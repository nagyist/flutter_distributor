use crate::traits::AppAnalyzer;
use crate::types::{AnalyzeConfig, AnalyzeError, AnalyzeResult};
use plist::Value;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MacOSDmgAnalyzer;

impl AppAnalyzer for MacOSDmgAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        return "macos-dmg";
    }

    fn is_supported_on_current_platform(&self) -> bool {
        return cfg!(target_os = "macos");
    }

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        if !self.is_supported_on_current_platform() {
            return Err(AnalyzeError::new(
                "DMG analysis is only supported on macOS.",
            ));
        }

        let mount_point = create_mount_point()?;
        let _mount_guard = mount_dmg(&config.path, &mount_point)?;

        let app_bundle = find_first_app_bundle(&mount_point)?
            .ok_or_else(|| AnalyzeError::new("No .app bundle found in DMG"))?;
        let info_plist = app_bundle.join("Contents").join("Info.plist");
        if !info_plist.exists() {
            return Err(AnalyzeError::new(&format!(
                "Info.plist not found: {}",
                info_plist.display()
            )));
        }

        let plist_value = Value::from_file(&info_plist)
            .map_err(|e| AnalyzeError::new(&format!("Failed to parse Info.plist: {}", e)))?;
        let plist_dict = plist_value
            .as_dictionary()
            .ok_or_else(|| AnalyzeError::new("Info.plist root is not a dictionary"))?;

        let identifier = read_required_plist_string(plist_dict, "CFBundleIdentifier")?;
        let name = read_optional_plist_string(plist_dict, "CFBundleDisplayName")
            .or_else(|| read_optional_plist_string(plist_dict, "CFBundleName"))
            .ok_or_else(|| {
                AnalyzeError::new("Missing CFBundleDisplayName/CFBundleName in Info.plist")
            })?;
        let version = read_required_plist_string(plist_dict, "CFBundleShortVersionString")?;
        let build_number_raw = read_required_plist_string(plist_dict, "CFBundleVersion")?;
        let build_number = build_number_raw
            .parse::<i32>()
            .map_err(|_| AnalyzeError::new("Failed to parse CFBundleVersion as integer"))?;

        let data = json!({
            "platform": "macos",
            "identifier": identifier,
            "name": name,
            "version": version,
            "buildNumber": build_number
        });

        log::info!("DMG analysis completed for {}", config.path);
        Ok(AnalyzeResult::new(true, data))
    }
}

fn create_mount_point() -> Result<PathBuf, AnalyzeError> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AnalyzeError::new(&format!("Failed to create mount point timestamp: {}", e)))?
        .as_millis();
    let mount_point = std::env::temp_dir().join(format!("fastforge-dmg-mount-{}", ts));
    fs::create_dir_all(&mount_point)
        .map_err(|e| AnalyzeError::new(&format!("Failed to create mount point: {}", e)))?;
    Ok(mount_point)
}

fn mount_dmg(dmg_path: &str, mount_point: &Path) -> Result<MountedDmg, AnalyzeError> {
    let output = Command::new("hdiutil")
        .args([
            "attach",
            "-nobrowse",
            "-readonly",
            "-mountpoint",
            mount_point.to_string_lossy().as_ref(),
            dmg_path,
        ])
        .output()
        .map_err(|e| AnalyzeError::new(&format!("Failed to execute hdiutil attach: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AnalyzeError::new(&format!(
            "hdiutil attach failed: {}",
            stderr
        )));
    }

    Ok(MountedDmg::new(mount_point.to_path_buf()))
}

fn find_first_app_bundle(root: &Path) -> Result<Option<PathBuf>, AnalyzeError> {
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let entries = fs::read_dir(&dir).map_err(|e| {
            AnalyzeError::new(&format!(
                "Failed to read directory {}: {}",
                dir.display(),
                e
            ))
        })?;
        for entry in entries {
            let entry = entry.map_err(|e| {
                AnalyzeError::new(&format!("Failed to read directory entry: {}", e))
            })?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            if path.extension().and_then(|ext| ext.to_str()) == Some("app") {
                return Ok(Some(path));
            }
            stack.push(path);
        }
    }

    Ok(None)
}

fn read_required_plist_string(dict: &plist::Dictionary, key: &str) -> Result<String, AnalyzeError> {
    read_optional_plist_string(dict, key)
        .ok_or_else(|| AnalyzeError::new(&format!("Missing {} in Info.plist", key)))
}

fn read_optional_plist_string(dict: &plist::Dictionary, key: &str) -> Option<String> {
    dict.get(key)
        .and_then(|value| value.as_string())
        .map(|value| value.to_string())
}

struct MountedDmg {
    mount_point: PathBuf,
}

impl MountedDmg {
    fn new(mount_point: PathBuf) -> Self {
        Self { mount_point }
    }
}

impl Drop for MountedDmg {
    fn drop(&mut self) {
        let _ = Command::new("hdiutil")
            .args([
                "detach",
                self.mount_point.to_string_lossy().as_ref(),
                "-force",
            ])
            .output();
        let _ = fs::remove_dir_all(&self.mount_point);
    }
}
