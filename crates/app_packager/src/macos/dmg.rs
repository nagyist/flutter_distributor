use std::path::Path;

use dmg_maker::{CreateOptions, DmgMakerError, create};
use fastforge_core::{AppPackager, PackageConfig, PackageError, PackageResult, Platform};
use serde_json::json;

/// Builds a macOS `.dmg` using the Rust-native `dmg_maker` crate.
///
/// This replaces the previous implementation that shelled out to `appdmg`
/// (a Node.js tool), removing the Node.js runtime dependency.
pub struct MacOSDmgPackager;

impl AppPackager for MacOSDmgPackager {
    fn name(&self) -> &str {
        "dmg"
    }

    fn platform(&self) -> Platform {
        Platform::MacOS
    }

    fn package_format(&self) -> &str {
        "dmg"
    }

    #[cfg(not(target_os = "macos"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let pkg_dir = config.packaging_dir();

        // Find the .app bundle in the build output directory
        let app_bundle = std::fs::read_dir(&config.build_output_dir)?
            .filter_map(|e| e.ok())
            .find(|e| e.path().extension().is_some_and(|x| x == "app"))
            .ok_or_else(|| PackageError::NotFound(".app bundle in build output".into()))?;

        // Copy the .app into the packaging directory
        run_cp_r(&app_bundle.path(), &pkg_dir)?;

        // Copy the project's dmg packaging assets (background, icon, etc.)
        // These are expected at macos/packaging/dmg/ relative to the project root.
        let dmg_assets = Path::new("macos/packaging/dmg");
        if dmg_assets.exists() {
            run_cp_r_dir_contents(dmg_assets, &pkg_dir)?;
        }

        let output_file = config.output_file();

        // Build the DMG spec programmatically.
        // Paths in the spec are relative to the packaging directory (basepath).
        let escaped_name = config.app_name.replace('\\', "\\\\").replace('"', "\\\"");
        let mut contents = vec![
            json!({"x": 448, "y": 344, "type": "link", "path": "/Applications"}),
            json!({"x": 192, "y": 344, "type": "file", "path": format!("{escaped_name}.app")}),
        ];

        // If a background.png exists in the packaging dir (from dmg_assets),
        // include it in the spec so dmg_maker handles it automatically.
        if pkg_dir.join("background.png").exists() {
            contents.push(json!({"x": 0, "y": 0, "type": "position"}));
        }

        let spec = json!({
            "title": escaped_name,
            "background": "background.png",
            "icon-size": 80,
            "contents": contents,
        });

        // Delegate DMG creation to the native dmg_maker crate.
        create(CreateOptions {
            target: output_file.clone(),
            source: None,
            basepath: Some(pkg_dir.clone()),
            specification: Some(spec),
        })
        .map_err(map_dmg_error)?;

        // Clean up the packaging directory.
        std::fs::remove_dir_all(&pkg_dir).ok();

        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}

/// Copy a file or directory recursively.
fn run_cp_r(source: &Path, dest_dir: &Path) -> Result<(), PackageError> {
    let status = std::process::Command::new("cp")
        .args(["-RH", &source.display().to_string(), &dest_dir.display().to_string()])
        .status()
        .map_err(|e| {
            PackageError::MissingTool(format!("cp: {}", e))
        })?;
    if !status.success() {
        return Err(PackageError::CommandFailed {
            command: "cp".to_string(),
            stderr: format!("Failed to copy {} to {}", source.display(), dest_dir.display()),
        });
    }
    Ok(())
}

/// Copy all contents of a source directory into a destination directory.
fn run_cp_r_dir_contents(source: &Path, dest_dir: &Path) -> Result<(), PackageError> {
    let status = std::process::Command::new("cp")
        .args([
            "-RH",
            &format!("{}/.", source.display()),
            &dest_dir.display().to_string(),
        ])
        .status()
        .map_err(|e| {
            PackageError::MissingTool(format!("cp: {}", e))
        })?;
    if !status.success() {
        return Err(PackageError::CommandFailed {
            command: "cp".to_string(),
            stderr: format!("Failed to copy {} contents to {}", source.display(), dest_dir.display()),
        });
    }
    Ok(())
}

/// Map `DmgMakerError` to `PackageError`.
fn map_dmg_error(err: DmgMakerError) -> PackageError {
    match err {
        DmgMakerError::UnsupportedPlatform(os) => {
            PackageError::General(format!("DMG creation not supported on {os}"))
        }
        DmgMakerError::TargetExists(path) => {
            PackageError::General(format!("Target already exists: {}", path.display()))
        }
        DmgMakerError::FileNotFound(msg) => PackageError::NotFound(msg),
        DmgMakerError::InvalidConfig(msg) => {
            PackageError::General(format!("Invalid DMG configuration: {msg}"))
        }
        DmgMakerError::CommandFailed { command, stderr } => PackageError::CommandFailed {
            command,
            stderr,
        },
        DmgMakerError::Io(e) => PackageError::Io(e),
        DmgMakerError::Json(e) => PackageError::General(format!("JSON error: {e}")),
        DmgMakerError::General(msg) => PackageError::General(msg),
    }
}
