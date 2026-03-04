use std::process::Command;

use crate::{
    traits::AppPackager,
    types::{PackageConfig, PackageError, PackageResult},
};

/// Builds a pacman `.pkg.tar.xz` using `bsdtar` and `xz`, mirroring
/// Dart's `AppPackageMakerPacman`.
///
/// Requires `bsdtar` (libarchive) and `xz` to be on `$PATH`.
pub struct LinuxPacmanPackager;

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

impl AppPackager for LinuxPacmanPackager {
    fn name(&self) -> &str {
        "pacman"
    }

    fn platform(&self) -> &str {
        "linux"
    }

    fn package_format(&self) -> &str {
        "pacman"
    }

    #[cfg(not(target_os = "linux"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let pkg_dir = config.packaging_dir();
        let binary_name = &config.app_binary_name;

        // Create directory tree
        let share_app_dir = pkg_dir.join("usr/share").join(binary_name);
        let applications_dir = pkg_dir.join("usr/share/applications");
        std::fs::create_dir_all(&share_app_dir)?;
        std::fs::create_dir_all(&applications_dir)?;

        // Copy the flutter build output into /usr/share/{binary_name}/
        run(Command::new("cp").args([
            "-fr",
            &format!("{}/.", config.build_output_dir.display()),
            &share_app_dir.display().to_string(),
        ]))?;

        // Write .PKGINFO
        let pkginfo = format!(
            "pkgname = {name}\npkgver = {ver}-1\npkgdesc = {name}\narch = x86_64\nurl = \nsize = 0\n",
            name = binary_name,
            ver = config.app_version.split('+').next().unwrap_or("0.0.1"),
        );
        std::fs::write(pkg_dir.join(".PKGINFO"), &pkginfo)?;

        // Write .INSTALL
        let install = format!(
            "post_install() {{\n  ln -s /usr/share/{n}/{n} /usr/bin/{n}\n  chmod +x /usr/bin/{n}\n}}\n\
             post_remove() {{\n  rm -f /usr/bin/{n}\n}}\n",
            n = binary_name,
        );
        std::fs::write(pkg_dir.join(".INSTALL"), &install)?;

        // Write .desktop entry
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName={name}\nExec={bin} %U\nIcon={bin}\n",
            name = config.app_name,
            bin = binary_name,
        );
        std::fs::write(
            applications_dir.join(format!("{}.desktop", binary_name)),
            &desktop,
        )?;

        // Create .MTREE metadata
        run(Command::new("bsdtar")
            .current_dir(&pkg_dir)
            .args([
                "-czf",
                ".MTREE",
                "--format=mtree",
                "--options=!all,use-set,type,uid,gid,mode,time,size,md5,sha256,link",
                ".PKGINFO",
                ".INSTALL",
                "usr",
            ])
            .env("LANG", "C"))?;

        // Archive with bsdtar
        run(Command::new("bsdtar")
            .current_dir(&pkg_dir)
            .args(["-cf", "temptar", ".MTREE", ".INSTALL", ".PKGINFO", "usr"])
            .env("LANG", "C"))?;

        // Compress with xz
        run(Command::new("xz")
            .current_dir(&pkg_dir)
            .args(["-z", "temptar"]))?;

        // Move to output
        let output_file = config.output_file();
        std::fs::rename(pkg_dir.join("temptar.xz"), &output_file)?;

        std::fs::remove_dir_all(&pkg_dir).ok();
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
