use fastforge_core::{AnalyzeConfig, AnalyzeError, AnalyzeResult, AppAnalyzer};
use plist::Value;
use serde_json::json;
use std::fs;
use std::path::Path;

pub struct MacOSAppAnalyzer;

impl AppAnalyzer for MacOSAppAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        "macos-app"
    }

    fn is_supported_on_current_platform(&self) -> bool {
        cfg!(target_os = "macos")
    }

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        if !self.is_supported_on_current_platform() {
            return Err(AnalyzeError::General(
                "macOS .app bundle analysis is only supported on macOS.".to_string(),
            ));
        }

        let app_path = Path::new(&config.path);

        // Verify the path exists and is a directory with .app extension
        if !app_path.exists() {
            return Err(AnalyzeError::NotFound(format!(
                "App bundle not found: {}",
                config.path
            )));
        }

        if !app_path.is_dir() {
            return Err(AnalyzeError::Parse(format!(
                "Expected a .app bundle directory, but path is not a directory: {}",
                config.path
            )));
        }

        let extension = app_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        if extension != "app" {
            return Err(AnalyzeError::Parse(format!(
                "Expected a .app bundle, got '.{}' extension",
                extension
            )));
        }

        let info_plist = app_path.join("Contents").join("Info.plist");
        if !info_plist.exists() {
            return Err(AnalyzeError::NotFound(format!(
                "Info.plist not found at: {}",
                info_plist.display()
            )));
        }

        let plist_value = Value::from_file(&info_plist)
            .map_err(|e| AnalyzeError::Parse(format!("Failed to parse Info.plist: {}", e)))?;

        let plist_dict = plist_value.as_dictionary().ok_or_else(|| {
            AnalyzeError::Parse("Info.plist root is not a dictionary".to_string())
        })?;

        let identifier = read_required_plist_string(plist_dict, "CFBundleIdentifier")?;
        let name = read_optional_plist_string(plist_dict, "CFBundleDisplayName")
            .or_else(|| read_optional_plist_string(plist_dict, "CFBundleName"))
            .ok_or_else(|| {
                AnalyzeError::Parse(
                    "Missing CFBundleDisplayName/CFBundleName in Info.plist".to_string(),
                )
            })?;
        let version = read_required_plist_string(plist_dict, "CFBundleShortVersionString")?;
        let build_number = read_required_plist_string(plist_dict, "CFBundleVersion")?;

        // Collect additional metadata
        let executable = read_optional_plist_string(plist_dict, "CFBundleExecutable");
        let bundle_type = read_optional_plist_string(plist_dict, "CFBundlePackageType");
        let min_os_version = read_optional_plist_string(plist_dict, "LSMinimumSystemVersion");

        // Detect architectures from the executable inside the bundle
        let architectures = detect_architectures(app_path, &executable);

        // Calculate app bundle size
        let size_bytes = calculate_directory_size(app_path);

        let data = json!({
            "platform": "macos",
            "identifier": identifier,
            "name": name,
            "version": version,
            "buildNumber": build_number,
            "executable": executable,
            "bundleType": bundle_type,
            "minOSVersion": min_os_version,
            "architectures": architectures,
            "sizeBytes": size_bytes,
        });

        log::info!("macOS app bundle analysis completed for {}", config.path);
        Ok(AnalyzeResult::new(true, data))
    }
}

fn read_required_plist_string(dict: &plist::Dictionary, key: &str) -> Result<String, AnalyzeError> {
    read_optional_plist_string(dict, key)
        .ok_or_else(|| AnalyzeError::Parse(format!("Missing {} in Info.plist", key)))
}

fn read_optional_plist_string(dict: &plist::Dictionary, key: &str) -> Option<String> {
    dict.get(key)
        .and_then(|value| value.as_string())
        .map(|value| value.to_string())
}

fn calculate_directory_size(path: &Path) -> u64 {
    fn walk(dir: &Path) -> u64 {
        let Ok(entries) = fs::read_dir(dir) else {
            return 0;
        };
        let mut total = 0u64;
        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };
            let path = entry.path();
            if path.is_dir() {
                total += walk(&path);
            } else if let Ok(metadata) = fs::metadata(&path) {
                total += metadata.len();
            }
        }
        total
    }
    walk(path)
}

fn detect_architectures(app_path: &Path, executable: &Option<String>) -> Vec<String> {
    let Some(exec_name) = executable else {
        return Vec::new();
    };
    let exec_path = app_path.join("Contents").join("MacOS").join(exec_name);
    if !exec_path.exists() {
        return Vec::new();
    }

    let output = match std::process::Command::new("file").arg(&exec_path).output() {
        Ok(output) if output.status.success() => output,
        _ => return Vec::new(),
    };

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut archs = Vec::new();

    if output_str.contains("x86_64") || output_str.contains("x86-64") {
        archs.push("x86_64".to_string());
    }
    if output_str.contains("arm64") || output_str.contains("aarch64") {
        archs.push("arm64".to_string());
    }
    if output_str.contains("ppc") {
        archs.push("ppc".to_string());
    }
    if output_str.contains("i386") || output_str.contains("i686") {
        archs.push("i386".to_string());
    }

    // If `file` didn't detect anything, try `lipo -info`
    if archs.is_empty() {
        if let Ok(lipo_output) = std::process::Command::new("lipo")
            .args(["-info", &exec_path.to_string_lossy()])
            .output()
        {
            let lipo_str = String::from_utf8_lossy(&lipo_output.stdout);
            // lipo output typically: "Architectures in the fat file: ... are: x86_64 arm64"
            if let Some(caps) = lipo_str.split(':').nth(1) {
                for arch in caps.split_whitespace() {
                    let arch = arch.trim().to_string();
                    if !arch.is_empty() {
                        archs.push(arch);
                    }
                }
            }
        }
    }

    archs
}
