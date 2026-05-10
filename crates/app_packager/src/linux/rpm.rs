use std::process::Command;

use fastforge_core::{AppPackager, PackageConfig, PackageError, PackageResult};

/// Builds an RPM package using `rpmbuild`, mirroring Dart's `AppPackageMakerRPM`.
///
/// Requires `rpmbuild` (from the `rpm-build` package) and optionally `patchelf`.
pub struct LinuxRpmPackager;

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

impl AppPackager for LinuxRpmPackager {
    fn name(&self) -> &str {
        "rpm"
    }

    fn platform(&self) -> &str {
        "linux"
    }

    fn package_format(&self) -> &str {
        "rpm"
    }

    #[cfg(not(target_os = "linux"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let pkg_dir = config.packaging_dir();
        let binary_name = &config.app_binary_name;
        let app_name = &config.app_name;

        // Create rpmbuild tree: BUILD BUILDROOT RPMS SOURCES SPECS SRPMS
        let rpmbuild_dir = pkg_dir.join("rpmbuild");
        for sub in &["BUILD", "BUILDROOT", "RPMS", "SOURCES", "SPECS", "SRPMS"] {
            std::fs::create_dir_all(rpmbuild_dir.join(sub))?;
        }

        // Copy app files into BUILD/{app_name}/
        let build_root = rpmbuild_dir.join("BUILD").join(app_name);
        std::fs::create_dir_all(&build_root)?;
        run(Command::new("cp").args([
            "-fr",
            &format!("{}/.", config.build_output_dir.display()),
            &build_root.display().to_string(),
        ]))?;

        // Write {app_name}.desktop into BUILD/
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName={name}\nExec=/usr/share/{bin}/{bin} %U\nIcon={bin}\n",
            name = config.app_name,
            bin = binary_name,
        );
        std::fs::write(
            rpmbuild_dir
                .join("BUILD")
                .join(format!("{}.desktop", app_name)),
            &desktop,
        )?;

        // Minimal .spec file
        let arch = "x86_64";
        let spec = format!(
            "%define _rpmdir %{{_topdir}}/RPMS\n\
             Name: {name}\n\
             Version: {ver}\n\
             Release: 1\n\
             Summary: {name}\n\
             License: Unknown\n\
             BuildArch: {arch}\n\n\
             %description\n{name}\n\n\
             %install\n\
             mkdir -p %{{buildroot}}/usr/share/{bin}\n\
             cp -r %{{_builddir}}/{name}/. %{{buildroot}}/usr/share/{bin}/\n\
             mkdir -p %{{buildroot}}/usr/share/applications\n\
             cp %{{_builddir}}/{name}.desktop %{{buildroot}}/usr/share/applications/\n\n\
             %files\n\
             /usr/share/{bin}\n\
             /usr/share/applications/{name}.desktop\n",
            name = app_name,
            ver = config.app_version.split('+').next().unwrap_or("0.0.1"),
            arch = arch,
            bin = binary_name,
        );
        let spec_path = rpmbuild_dir
            .join("SPECS")
            .join(format!("{}.spec", app_name));
        std::fs::write(&spec_path, &spec)?;

        run(Command::new("rpmbuild")
            .args([
                "--define",
                &format!("_topdir {}", rpmbuild_dir.display()),
                "-bb",
                &spec_path.display().to_string(),
            ])
            .env("QA_RPATHS", "17"))?;

        // Find the produced RPM and copy it to the output file
        let rpm_dir = rpmbuild_dir.join("RPMS").join(arch);
        let output_file = config.output_file();
        let entries: Vec<_> = std::fs::read_dir(&rpm_dir)
            .map_err(|_| PackageError::NotFound(rpm_dir.display().to_string()))?
            .filter_map(|e| e.ok())
            .collect();
        let first_rpm = entries
            .first()
            .ok_or_else(|| PackageError::General("rpmbuild produced no output".into()))?;
        std::fs::copy(first_rpm.path(), &output_file)?;

        std::fs::remove_dir_all(&pkg_dir).ok();
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
