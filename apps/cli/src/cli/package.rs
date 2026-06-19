use anyhow::{Context, Result, anyhow};
use clap::Args;
use fastforge_app_builder::{FlutterAppBuilder, Platform};
use fastforge_app_packager::{
    AppPackager, MacOSDmgPackager, MacOSPkgPackager, MacOSZipPackager, PackageConfig,
};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Args)]
pub struct PackageArgs {
    #[arg(short, long = "platform")]
    pub platform: Option<String>,
    #[arg(short, long = "target")]
    pub target: Option<String>,
    #[arg(long = "skip-clean", default_value_t = false)]
    pub skip_clean: bool,
    #[arg(long = "build-target")]
    pub build_target: Option<String>,
}

pub async fn execute(args: &PackageArgs) -> Result<()> {
    log::info!("Executing package command");
    let platform = args
        .platform
        .as_deref()
        .ok_or_else(|| anyhow!("The 'platform' option is mandatory!"))?;
    let target = args
        .target
        .as_deref()
        .ok_or_else(|| anyhow!("The 'target' option is mandatory!"))?;

    let mut build_args = Map::new();
    if let Some(value) = &args.build_target {
        build_args.insert("target".to_string(), Value::String(value.clone()));
    }

    let artifacts = package_flutter_artifact(
        platform,
        target,
        build_args,
        std::env::vars().collect(),
        "dist/",
        None,
        !args.skip_clean,
    )?;

    for artifact in artifacts {
        println!("{}", artifact.display());
    }
    Ok(())
}

pub fn package_flutter_artifact(
    platform_str: &str,
    target: &str,
    build_args: Map<String, Value>,
    environment: HashMap<String, String>,
    output: &str,
    artifact_name: Option<String>,
    clean_before_build: bool,
) -> Result<Vec<PathBuf>> {
    let platform = Platform::from_str(platform_str)
        .map_err(|e| anyhow!("Invalid platform '{}': {}", platform_str, e))?;

    let builder = FlutterAppBuilder::default();
    if clean_before_build {
        builder
            .clean(Some(&environment))
            .map_err(|e| anyhow!("{}", e))?;
    }

    let build = builder
        .build(&platform, Some(target), build_args, Some(environment))
        .map_err(|e| anyhow!("{}", e))?;

    let pubspec = ProjectPubspec::load()?;
    let package_config = PackageConfig {
        app_name: pubspec.name.clone(),
        app_binary_name: pubspec.name,
        app_version: pubspec.version,
        build_mode: build.config.mode().as_str().to_string(),
        platform,
        flavor: build.config.flavor().map(ToOwned::to_owned),
        channel: None,
        artifact_name,
        package_format: target.to_string(),
        is_installer: matches!(
            target,
            "pkg" | "dmg" | "deb" | "rpm" | "pacman" | "exe" | "msix"
        ),
        build_output_dir: build.output_directory,
        build_output_files: build.output_files,
        output_dir: PathBuf::from(output),
    };

    let packager = macos_packager(target)?;
    if !packager.is_supported_on_current_platform() {
        return Err(anyhow!(
            "Packager '{}' is not supported on the current platform",
            target
        ));
    }

    let result = packager
        .package(&package_config)
        .map_err(|e| anyhow!("{}", e))?;
    Ok(result.artifacts)
}

fn macos_packager(target: &str) -> Result<Box<dyn AppPackager + Send + Sync>> {
    match target {
        "pkg" => Ok(Box::new(MacOSPkgPackager::from_yaml_file(
            std::path::Path::new("macos/packaging/pkg/make_config.yaml"),
        )?)),
        "dmg" => Ok(Box::new(MacOSDmgPackager)),
        "zip" => Ok(Box::new(MacOSZipPackager)),
        other => Err(anyhow!(
            "Unsupported package target: `{}`. Currently supported for CLI packaging: pkg, dmg, zip",
            other
        )),
    }
}

#[derive(Debug, Deserialize)]
struct ProjectPubspec {
    name: String,
    #[serde(default = "default_version")]
    version: String,
}

impl ProjectPubspec {
    fn load() -> Result<Self> {
        let content =
            std::fs::read_to_string("pubspec.yaml").context("Failed to read pubspec.yaml")?;
        serde_yaml::from_str(&content).context("Failed to parse pubspec.yaml")
    }
}

fn default_version() -> String {
    "0.1.0+1".to_string()
}
