use std::process::Command;

use crate::{
    traits::AppPackager,
    types::{PackageConfig, PackageError, PackageResult},
};

/// Builds a Windows `.exe` installer using Inno Setup (`iscc`), mirroring
/// Dart's `AppPackageMakerExe`.
///
/// Requires Inno Setup (`iscc`) to be on `%PATH%` (Windows only).
pub struct WindowsExePackager;

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

impl AppPackager for WindowsExePackager {
    fn name(&self) -> &str {
        "exe"
    }

    fn platform(&self) -> &str {
        "windows"
    }

    fn package_format(&self) -> &str {
        "exe"
    }

    #[cfg(not(target_os = "windows"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let pkg_dir = config.packaging_dir();

        // Copy the flutter build output into the packaging directory
        run(Command::new("xcopy").args([
            "/E",
            "/I",
            "/Q",
            &config.build_output_dir.display().to_string(),
            &pkg_dir.display().to_string(),
        ]))?;

        let output_file = config.output_file();
        let output_dir = output_file
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .display()
            .to_string();
        let output_basename = output_file
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        // Build a minimal Inno Setup .iss script
        let iss = format!(
            "[Setup]\n\
             AppName={name}\n\
             AppVersion={ver}\n\
             DefaultDirName={{autopf}}\\{name}\n\
             OutputDir={out_dir}\n\
             OutputBaseFilename={out_name}\n\
             [Files]\n\
             Source: \"{src}\\*\"; DestDir: \"{{app}}\"; Flags: ignoreversion recursesubdirs\n",
            name = config.app_name,
            ver = config.app_version,
            out_dir = output_dir,
            out_name = output_basename,
            src = pkg_dir.display(),
        );
        let iss_path = pkg_dir.join(format!("{}.iss", config.app_name));
        std::fs::write(&iss_path, &iss)?;

        run(Command::new("iscc").arg(&iss_path))?;

        std::fs::remove_dir_all(&pkg_dir).ok();
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
