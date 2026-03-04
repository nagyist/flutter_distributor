use std::process::Command;

use crate::{
    traits::AppPackager,
    types::{PackageConfig, PackageError, PackageResult},
};

/// Builds a macOS `.dmg` using the `appdmg` Node.js tool, mirroring
/// Dart's `AppPackageMakerDmg`.
///
/// Requires `appdmg` (`npm install -g appdmg`) on the host.
pub struct MacOSDmgPackager;

fn run(cmd: &mut Command) -> Result<(), PackageError> {
    let out = cmd.output().map_err(|e| {
        PackageError::MissingTool(format!("{}: {}", cmd.get_program().to_string_lossy(), e))
    })?;
    if !out.status.success() {
        return Err(PackageError::CommandFailed {
            command: cmd.get_program().to_string_lossy().into(),
            stderr: String::from_utf8_lossy(&out.stderr).into(),
        });
    }
    Ok(())
}

impl AppPackager for MacOSDmgPackager {
    fn name(&self) -> &str {
        "dmg"
    }

    fn platform(&self) -> &str {
        "macos"
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
        run(Command::new("cp").args([
            "-RH",
            &app_bundle.path().display().to_string(),
            &pkg_dir.display().to_string(),
        ]))?;

        // Copy the project's dmg packaging assets (appdmg spec JSON, background, etc.)
        let dmg_assets = std::path::Path::new("macos/packaging/dmg");
        if dmg_assets.exists() {
            run(Command::new("cp").args([
                "-RH",
                &format!("{}/.", dmg_assets.display()),
                &pkg_dir.display().to_string(),
            ]))?;
        }

        // Write a minimal make_config.json that appdmg can consume.
        // Values are JSON-escaped to avoid injection from app_name.
        let make_config_json = pkg_dir.join("make_config.json");
        let escaped_name = config.app_name.replace('\\', "\\\\").replace('"', "\\\"");
        let json_content = format!(
            r#"{{"title":"{name}","background":"background.png","icon-size":80,"contents":[{{"x":448,"y":344,"type":"link","path":"/Applications"}},{{"x":192,"y":344,"type":"file","path":"{name}.app"}}]}}"#,
            name = escaped_name,
        );
        std::fs::write(&make_config_json, &json_content)?;

        let output_file = config.output_file();
        run(Command::new("appdmg").args([
            &make_config_json.display().to_string(),
            &output_file.display().to_string(),
        ]))?;

        std::fs::remove_dir_all(&pkg_dir).ok();
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
