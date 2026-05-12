use fastforge_core::{AnalyzeConfig, AnalyzeError, AnalyzeResult, AppAnalyzer};
use regex::Regex;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct AndroidAabAnalyzer;

impl AppAnalyzer for AndroidAabAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        "android-aab"
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        if let Some(aapt2_path) = find_aapt2_path()
            && let Ok(result) = analyze_with_aapt2(&aapt2_path, config)
        {
            return Ok(result);
        }

        analyze_with_bundletool(config)
    }
}

fn find_aapt2_path() -> Option<String> {
    let android_home = env::var("ANDROID_HOME").ok()?;
    if android_home.is_empty() {
        return None;
    }

    let build_tools_dir = Path::new(&android_home).join("build-tools");
    if !build_tools_dir.exists() {
        return None;
    }

    let entries = fs::read_dir(&build_tools_dir).ok()?;
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_dir()
            && !path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .contains(".DS_Store")
        {
            let aapt2_path = path.join("aapt2");
            if aapt2_path.exists() {
                return Some(aapt2_path.to_string_lossy().to_string());
            }
        }
    }

    None
}

fn analyze_with_aapt2(
    aapt2_path: &str,
    config: &AnalyzeConfig,
) -> Result<AnalyzeResult, AnalyzeError> {
    let output = Command::new(aapt2_path)
        .args(["dump", "badging", &config.path])
        .output()
        .map_err(AnalyzeError::Io)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AnalyzeError::CommandFailed { command: "aapt2".to_string(), stderr: stderr.to_string() });
    }

    let aapt_output = String::from_utf8_lossy(&output.stdout);
    parse_aapt_badging_output(&aapt_output)
}

fn parse_aapt_badging_output(aapt_output: &str) -> Result<AnalyzeResult, AnalyzeError> {
    let name_regex = Regex::new(r"name='([^']+)'").unwrap();
    let label_regex = Regex::new(r"application-label:'([^']+)'").unwrap();
    let version_name_regex = Regex::new(r"versionName='([^']+)").unwrap();
    let version_code_regex = Regex::new(r"versionCode='(\d+)'").unwrap();

    let package_name = name_regex
        .captures(aapt_output)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| AnalyzeError::Parse("Failed to extract package name from aapt output".to_string()))?;

    let app_name = label_regex
        .captures(aapt_output)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| package_name.clone());

    let version_name = version_name_regex
        .captures(aapt_output)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| AnalyzeError::Parse("Failed to extract version name from aapt output".to_string()))?;

    let version_code_str = version_code_regex
        .captures(aapt_output)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| AnalyzeError::Parse("Failed to extract version code from aapt output".to_string()))?;

    let version_code = version_code_str
        .parse::<i32>()
        .map_err(|_| AnalyzeError::Parse("Failed to parse version code as integer".to_string()))?;

    let data = json!({
        "platform": "android",
        "identifier": package_name,
        "name": app_name,
        "version": version_name,
        "buildNumber": version_code
    });

    Ok(AnalyzeResult::new(true, data))
}

fn analyze_with_bundletool(config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
    let bundletool_env = env::var("BUNDLETOOL").ok();

    let mut command = if let Some(bundletool_path) = bundletool_env {
        if bundletool_path.ends_with(".jar") {
            let mut cmd = Command::new("java");
            cmd.args(["-jar", &bundletool_path]);
            cmd
        } else {
            Command::new(bundletool_path)
        }
    } else {
        Command::new("bundletool")
    };

    let output = command
        .args([
            "dump",
            "manifest",
            "--bundle",
            &config.path,
            "--module",
            "base",
        ])
        .output()
        .map_err(AnalyzeError::Io)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AnalyzeError::CommandFailed { command: "bundletool".to_string(), stderr: stderr.to_string() });
    }

    let manifest_output = String::from_utf8_lossy(&output.stdout);
    parse_manifest_xml(&manifest_output)
}

fn parse_manifest_xml(manifest_xml: &str) -> Result<AnalyzeResult, AnalyzeError> {
    let package_regex = Regex::new(r#"package="([^"]+)""#).unwrap();
    let version_name_regex = Regex::new(r#"android:versionName="([^"]+)""#).unwrap();
    let version_code_regex = Regex::new(r#"android:versionCode="(\d+)""#).unwrap();
    let label_regex = Regex::new(r#"android:label="([^"]+)""#).unwrap();

    let package_name = package_regex
        .captures(manifest_xml)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| AnalyzeError::Parse("Failed to extract package name from manifest".to_string()))?;

    let version_name = version_name_regex
        .captures(manifest_xml)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| AnalyzeError::Parse("Failed to extract version name from manifest".to_string()))?;

    let version_code_str = version_code_regex
        .captures(manifest_xml)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| AnalyzeError::Parse("Failed to extract version code from manifest".to_string()))?;

    let version_code = version_code_str
        .parse::<i32>()
        .map_err(|_| AnalyzeError::Parse("Failed to parse version code as integer".to_string()))?;

    let app_name = label_regex
        .captures(manifest_xml)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .map(|label| {
            if label.starts_with('@') {
                package_name.clone()
            } else {
                label
            }
        })
        .unwrap_or_else(|| package_name.clone());

    let data = json!({
        "platform": "android",
        "identifier": package_name,
        "name": app_name,
        "version": version_name,
        "buildNumber": version_code
    });

    Ok(AnalyzeResult::new(true, data))
}
