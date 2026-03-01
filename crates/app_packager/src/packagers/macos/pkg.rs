use std::process::Command;

use crate::{
    traits::AppPackager,
    types::{PackageConfig, PackageError, PackageResult},
};

/// Builds a macOS `.pkg` installer using `xcrun productbuild` (and optionally
/// `productsign`), mirroring Dart's `AppPackageMakerPkg`.
///
/// Requires Xcode command-line tools.
pub struct MacOSPkgPackager {
    /// Optional code-signing identity (e.g. `"Developer ID Installer: ..."`)
    pub sign_identity: Option<String>,
    /// Installation path prefix (defaults to `/Applications/`)
    pub install_path: Option<String>,
}

impl Default for MacOSPkgPackager {
    fn default() -> Self {
        Self {
            sign_identity: None,
            install_path: None,
        }
    }
}

fn run(cmd: &mut Command) -> Result<(), PackageError> {
    let out = cmd.output().map_err(|e| {
        PackageError::MissingTool(format!(
            "{}: {}",
            cmd.get_program().to_string_lossy(),
            e
        ))
    })?;
    if !out.status.success() {
        return Err(PackageError::CommandFailed {
            command: cmd.get_program().to_string_lossy().into(),
            stderr: String::from_utf8_lossy(&out.stderr).into(),
        });
    }
    Ok(())
}

impl AppPackager for MacOSPkgPackager {
    fn name(&self) -> &str {
        "pkg"
    }

    fn platform(&self) -> &str {
        "macos"
    }

    fn package_format(&self) -> &str {
        "pkg"
    }

    #[cfg(not(target_os = "macos"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        // `productbuild --root <dir> <install-path> <output>` – the first argument
        // must be the directory that will become the installer's payload root.
        // Flutter's iOS/macOS builds expose the .app bundle as the first entry
        // in build_output_files (which is itself a directory).
        let app_path = config
            .first_build_output_file()
            .ok_or_else(|| PackageError::General("no build output files".into()))?;

        let output_file = config.output_file();
        let install_path = self
            .install_path
            .as_deref()
            .unwrap_or("/Applications/");

        let unsigned_path = {
            let mut p = output_file.clone();
            let stem = p
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            p.set_file_name(format!("{}-unsigned.pkg", stem));
            p
        };

        run(Command::new("xcrun").args([
            "productbuild",
            "--root",
            &app_path.display().to_string(),
            install_path,
            &unsigned_path.display().to_string(),
        ]))?;

        if let Some(identity) = &self.sign_identity {
            run(Command::new("xcrun").args([
                "productsign",
                "--sign",
                identity,
                &unsigned_path.display().to_string(),
                &output_file.display().to_string(),
            ]))?;
            std::fs::remove_file(&unsigned_path)?;
        } else {
            std::fs::rename(&unsigned_path, &output_file)?;
        }

        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
