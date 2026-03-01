use std::process::Command;

use crate::{
    traits::AppPackager,
    types::{PackageConfig, PackageError, PackageResult},
};

/// Builds a Windows `.msix` package using the `makeappx` / `signtool` SDK tools,
/// mirroring Dart's `AppPackageMakerMsix`.
///
/// Requires the Windows 10 SDK tools (`makeappx`, `signtool`) on Windows.
pub struct WindowsMsixPackager {
    /// Optional path to a PFX certificate for signing.
    pub certificate_path: Option<String>,
    /// Optional PFX certificate password.
    pub certificate_password: Option<String>,
    /// Publisher distinguished name used in AppxManifest, e.g.
    /// `"CN=My Company, O=My Company, L=City, S=State, C=US"`.
    /// Defaults to `"CN=Publisher"` when not set.
    pub publisher: Option<String>,
}

impl Default for WindowsMsixPackager {
    fn default() -> Self {
        Self {
            certificate_path: None,
            certificate_password: None,
            publisher: None,
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

impl AppPackager for WindowsMsixPackager {
    fn name(&self) -> &str {
        "msix"
    }

    fn platform(&self) -> &str {
        "windows"
    }

    fn package_format(&self) -> &str {
        "msix"
    }

    #[cfg(not(target_os = "windows"))]
    fn is_supported_on_current_platform(&self) -> bool {
        false
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError> {
        let pkg_dir = config.packaging_dir();

        // Copy the flutter build output
        run(Command::new("xcopy").args([
            "/E", "/I", "/Q",
            &config.build_output_dir.display().to_string(),
            &pkg_dir.display().to_string(),
        ]))?;

        // Write a minimal AppxManifest.xml
        let publisher = self.publisher.as_deref().unwrap_or("CN=Publisher");
        let manifest = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
         xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
         xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities">
  <Identity Name="{name}" Publisher="{publisher}" Version="{ver}.0"/>
  <Properties>
    <DisplayName>{name}</DisplayName>
    <PublisherDisplayName>Publisher</PublisherDisplayName>
    <Logo>Assets\StoreLogo.png</Logo>
  </Properties>
  <Dependencies>
    <TargetDeviceFamily Name="Windows.Universal" MinVersion="10.0.17763.0" MaxVersionTested="10.0.19041.0"/>
  </Dependencies>
  <Resources><Resource Language="en-us"/></Resources>
  <Applications>
    <Application Id="App" Executable="{bin}.exe" EntryPoint="Windows.FullTrustApplication">
      <uap:VisualElements DisplayName="{name}" Description="{name}" BackgroundColor="transparent" Square150x150Logo="Assets\Square150x150Logo.png" Square44x44Logo="Assets\Square44x44Logo.png">
        <uap:DefaultTile/>
        <uap:SplashScreen Image="Assets\SplashScreen.png"/>
      </uap:VisualElements>
    </Application>
  </Applications>
  <Capabilities><rescap:Capability Name="runFullTrust"/></Capabilities>
</Package>
"#,
            name = config.app_name,
            publisher = publisher,
            ver = config.app_version.split('+').next().unwrap_or("0.0.1"),
            bin = config.app_binary_name,
        );
        std::fs::write(pkg_dir.join("AppxManifest.xml"), &manifest)?;

        let output_file = config.output_file();
        run(Command::new("makeappx").args([
            "pack",
            "/d",
            &pkg_dir.display().to_string(),
            "/p",
            &output_file.display().to_string(),
            "/nv",
        ]))?;

        // Optionally sign
        if let Some(cert) = &self.certificate_path {
            let mut args = vec![
                "sign".to_string(),
                "/fd".to_string(),
                "SHA256".to_string(),
                "/a".to_string(),
                "/f".to_string(),
                cert.clone(),
            ];
            if let Some(pwd) = &self.certificate_password {
                args.push("/p".to_string());
                args.push(pwd.clone());
            }
            args.push(output_file.display().to_string());
            run(Command::new("signtool").args(&args))?;
        }

        std::fs::remove_dir_all(&pkg_dir).ok();
        Ok(PackageResult {
            artifacts: vec![output_file],
        })
    }
}
