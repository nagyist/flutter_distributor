use crate::traits::AppAnalyzer;
use crate::types::{AnalyzeConfig, AnalyzeError, AnalyzeResult};
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
            .map_err(|_| AnalyzeError::new("Missing `ANDROID_HOME` environment variable."))?;

        if android_home.is_empty() {
            return Err(AnalyzeError::new(
                "Missing `ANDROID_HOME` environment variable.",
            ));
        }

        // Find aapt tool in Android SDK build-tools directory
        let build_tools_dir = Path::new(&android_home).join("build-tools");

        if !build_tools_dir.exists() {
            return Err(AnalyzeError::new(
                "build-tools directory not found in ANDROID_HOME",
            ));
        }

        let entries = fs::read_dir(&build_tools_dir).map_err(|e| {
            AnalyzeError::new(&format!("Failed to read build-tools directory: {}", e))
        })?;

        // Find the first build-tools version directory (excluding .DS_Store)
        let mut aapt2_path = None;
        for entry in entries {
            let entry = entry.map_err(|e| {
                AnalyzeError::new(&format!("Failed to read build-tools entry: {}", e))
            })?;

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
            .ok_or_else(|| AnalyzeError::new("aapt tool not found in any build-tools directory"))?;

        // Execute aapt command to extract APK information
        let output = Command::new(&aapt2_path)
            .args(["dump", "badging", &config.path])
            .output()
            .map_err(|e| AnalyzeError::new(&format!("Failed to execute aapt: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AnalyzeError::new(&format!(
                "aapt command failed: {}",
                stderr
            )));
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
            .ok_or_else(|| AnalyzeError::new("Failed to extract package name from aapt output"))?;

        // Extract application label
        let app_name = label_regex
            .captures(&aapt_output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                AnalyzeError::new("Failed to extract application label from aapt output")
            })?;

        // Extract version name
        let version_name = version_name_regex
            .captures(&aapt_output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| AnalyzeError::new("Failed to extract version name from aapt output"))?;

        // Extract version code
        let version_code_str = version_code_regex
            .captures(&aapt_output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| AnalyzeError::new("Failed to extract version code from aapt output"))?;

        let version_code = version_code_str
            .parse::<i32>()
            .map_err(|_| AnalyzeError::new("Failed to parse version code as integer"))?;

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
