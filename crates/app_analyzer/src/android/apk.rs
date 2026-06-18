use fastforge_core::{AnalyzeConfig, AnalyzeError, AnalyzeResult, AppAnalyzer};
use regex::Regex;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct AndroidApkAnalyzer;

impl AppAnalyzer for AndroidApkAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        "android-apk"
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        // Check for ANDROID_HOME environment variable
        let android_home = env::var("ANDROID_HOME")
            .map_err(|_| AnalyzeError::MissingEnv("ANDROID_HOME".to_string()))?;

        if android_home.is_empty() {
            return Err(AnalyzeError::MissingEnv("ANDROID_HOME".to_string()));
        }

        // Find aapt tool in Android SDK build-tools directory
        let build_tools_dir = Path::new(&android_home).join("build-tools");

        if !build_tools_dir.exists() {
            return Err(AnalyzeError::NotFound(
                "build-tools directory in ANDROID_HOME".to_string(),
            ));
        }

        let entries = fs::read_dir(&build_tools_dir).map_err(AnalyzeError::Io)?;

        // Find the first build-tools version directory (excluding .DS_Store)
        let mut aapt2_path = None;
        for entry in entries {
            let entry = entry.map_err(AnalyzeError::Io)?;

            let path = entry.path();
            if path.is_dir()
                && !path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .contains(".DS_Store")
            {
                let aapt_tool_path = path.join("aapt2");
                if aapt_tool_path.exists() {
                    aapt2_path = Some(aapt_tool_path.to_string_lossy().to_string());
                    break;
                }
            }
        }

        let aapt2_path = aapt2_path
            .ok_or_else(|| AnalyzeError::NotFound("aapt2 in Android build-tools".to_string()))?;

        // Execute aapt command to extract APK information
        let output = Command::new(&aapt2_path)
            .args(["dump", "badging", &config.path])
            .output()
            .map_err(AnalyzeError::Io)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AnalyzeError::CommandFailed {
                command: "aapt2".to_string(),
                stderr: stderr.to_string(),
            });
        }

        let aapt_output = String::from_utf8_lossy(&output.stdout);

        // Parse aapt output using regex
        let name_regex = Regex::new(r"name='([^']+)'").unwrap();
        let label_regex = Regex::new(r"application-label:'([^']+)'").unwrap();
        let version_name_regex = Regex::new(r"versionName='([^']+)").unwrap();
        let version_code_regex = Regex::new(r"versionCode='(\d+)'").unwrap();

        // Extract package name
        let package_name = name_regex
            .captures(&aapt_output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                AnalyzeError::Parse("Failed to extract package name from aapt output".to_string())
            })?;

        // Extract application label
        let app_name = label_regex
            .captures(&aapt_output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                AnalyzeError::Parse(
                    "Failed to extract application label from aapt output".to_string(),
                )
            })?;

        // Extract version name
        let version_name = version_name_regex
            .captures(&aapt_output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                AnalyzeError::Parse("Failed to extract version name from aapt output".to_string())
            })?;

        // Extract version code
        let version_code_str = version_code_regex
            .captures(&aapt_output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                AnalyzeError::Parse("Failed to extract version code from aapt output".to_string())
            })?;

        let version_code = version_code_str.parse::<i32>().map_err(|_| {
            AnalyzeError::Parse("Failed to parse version code as integer".to_string())
        })?;

        let data = json!({
            "platform": "android",
            "identifier": package_name,
            "name": app_name,
            "version": version_name,
            "buildNumber": version_code
        });

        log::info!("APK analysis completed for {}", config.path);
        Ok(AnalyzeResult::new(true, data))
    }
}
