use std::process::Command;

use fastforge_core::{AppPackager, PackageConfig, PackageError, PackageResult};

/// Builds a Linux AppImage using `appimagetool`, mirroring Dart's
/// `AppPackageMakerAppImage`.
///
/// Requires `appimagetool` to be on `$PATH`.
pub struct LinuxAppImagePackager;

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

impl AppPackager for LinuxAppImagePackager {
    fn name(&self) -> &str {
        "appimage"
    }

    fn platform(&self) -> &str {
        "linux"
    }

    fn package_format(&self) -> &str {
        "AppImage"
    }

    #[cfg(not(target_os = "linux"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let pkg_dir = config.packaging_dir();
        let app_name = &config.app_name;
        let binary_name = &config.app_binary_name;

        let app_dir = pkg_dir.join(format!("{}.AppDir", app_name));
        std::fs::create_dir_all(&app_dir)?;

        // Copy flutter build output contents into AppDir
        run(Command::new("cp").args([
            "-r",
            &format!("{}/.", config.build_output_dir.display()),
            &app_dir.display().to_string(),
        ]))?;

        // Write AppRun script – flutter Linux builds place the binary in lib/
        // (named after the app), or directly in the AppDir root.
        let app_run_path = app_dir.join("AppRun");
        std::fs::write(
            &app_run_path,
            format!(
                "#!/usr/bin/env sh\nHERE=\"$(dirname \"$(readlink -f \"$0\")\")\"\nexec \"$HERE/{bin}\" \"$@\"\n",
                bin = binary_name,
            ),
        )?;
        run(Command::new("chmod").args(["+x", &app_run_path.display().to_string()]))?;

        // Write .desktop file
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName={name}\nExec={bin}\nIcon={bin}\nCategories=Utility;\n",
            name = config.app_name,
            bin = binary_name,
        );
        let desktop_path = app_dir.join(format!("{}.desktop", app_name));
        std::fs::write(&desktop_path, &desktop)?;

        // Build the AppImage (output format is `.AppImage`)
        let output_file = config.output_file();
        run(Command::new("appimagetool")
            .args([
                "--no-appstream",
                &app_dir.display().to_string(),
                &output_file.display().to_string(),
            ])
            .env("ARCH", "x86_64"))?;

        std::fs::remove_dir_all(&pkg_dir).ok();
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
