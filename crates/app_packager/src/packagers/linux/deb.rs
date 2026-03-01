use std::path::Path;
use std::process::Command;

use crate::{
    traits::AppPackager,
    types::{PackageConfig, PackageError, PackageResult},
};

/// Builds a Debian `.deb` package using `dpkg-deb`, mirroring
/// Dart's `AppPackageMakerDeb`.
///
/// Requires `dpkg-deb` to be installed on the host (`dpkg-dev` on Debian/Ubuntu).
pub struct LinuxDebPackager;

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

fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), PackageError> {
    run(Command::new("cp").args(["-fr", &format!("{}/.", src.display()), &dst.display().to_string()]))?;
    Ok(())
}

impl AppPackager for LinuxDebPackager {
    fn name(&self) -> &str {
        "deb"
    }

    fn platform(&self) -> &str {
        "linux"
    }

    fn package_format(&self) -> &str {
        "deb"
    }

    #[cfg(not(target_os = "linux"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let pkg_dir = config.packaging_dir();
        let binary_name = &config.app_binary_name;

        // Create the required directory tree
        let debian_dir = pkg_dir.join("DEBIAN");
        let share_app_dir = pkg_dir.join("usr/share").join(binary_name);
        let applications_dir = pkg_dir.join("usr/share/applications");
        std::fs::create_dir_all(&debian_dir)?;
        std::fs::create_dir_all(&share_app_dir)?;
        std::fs::create_dir_all(&applications_dir)?;

        // Copy the flutter build output into /usr/share/{binary_name}/
        copy_dir_contents(&config.build_output_dir, &share_app_dir)?;

        // Write DEBIAN/control (minimal placeholder; real deployments supply a
        // make_config.yaml via `DebPackageConfig`; keep compatible with Dart output)
        let control = format!(
            "Package: {}\nVersion: {}\nArchitecture: amd64\nMaintainer: unknown\nDescription: {}\n",
            binary_name, config.app_version, config.app_name,
        );
        std::fs::write(debian_dir.join("control"), &control)?;

        // Write DEBIAN/postinst
        let postinst = format!(
            "#!/usr/bin/env sh\nln -s /usr/share/{n}/{n} /usr/bin/{n}\nchmod +x /usr/bin/{n}\nexit 0\n",
            n = binary_name,
        );
        let postinst_path = debian_dir.join("postinst");
        std::fs::write(&postinst_path, &postinst)?;
        run(Command::new("chmod").args(["+x", &postinst_path.display().to_string()]))?;

        // Write DEBIAN/postrm
        let postrm = format!(
            "#!/usr/bin/env sh\nrm /usr/bin/{n}\nexit 0\n",
            n = binary_name,
        );
        let postrm_path = debian_dir.join("postrm");
        std::fs::write(&postrm_path, &postrm)?;
        run(Command::new("chmod").args(["+x", &postrm_path.display().to_string()]))?;

        // Write usr/share/applications/{binary_name}.desktop
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName={name}\nExec={bin} %U\nIcon={bin}\n",
            name = config.app_name,
            bin = binary_name,
        );
        std::fs::write(applications_dir.join(format!("{}.desktop", binary_name)), &desktop)?;

        let output_file = config.output_file();
        run(Command::new("dpkg-deb").args([
            "--build",
            "--root-owner-group",
            &pkg_dir.display().to_string(),
            &output_file.display().to_string(),
        ]))?;

        std::fs::remove_dir_all(&pkg_dir).ok();
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
