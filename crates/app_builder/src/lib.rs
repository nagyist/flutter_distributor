mod custom;
mod flutter;
mod gradle;

use crate::flutter::command::FlutterCommand;
use crate::flutter::{
    AndroidAabBuilder, AndroidApkBuilder, IOSBuilder, LinuxBuilder, MacOSBuilder, OhosAppBuilder,
    OhosHapBuilder, WebBuilder, WindowsBuilder,
};
pub use fastforge_core::AppBuilder;
pub use fastforge_core::{
    BuildConfig, BuildError, BuildMode, BuildRequest, BuildResult,
};
pub use crate::flutter::{FlutterVersion, PubspecInfo};
use serde_json::{Map, Value};
use std::path::Path;
use std::time::Instant;

pub use crate::custom::{CustomAppBuilder, CustomBuilder};
pub use crate::gradle::{
    GradleAndroidAabBuilder, GradleAndroidApkBuilder, GradleAppBuilder, GradleKmpAndroidAabBuilder,
    GradleKmpAndroidApkBuilder, GradleKmpDesktopBuilder, GradleKmpIosFrameworkBuilder,
};

pub struct FlutterAppBuilder {
    builders: Vec<Box<dyn AppBuilder + Send + Sync>>,
}

impl Default for FlutterAppBuilder {
    fn default() -> Self {
        Self {
            builders: vec![
                Box::new(AndroidAabBuilder),
                Box::new(AndroidApkBuilder),
                Box::new(IOSBuilder),
                Box::new(LinuxBuilder),
                Box::new(MacOSBuilder),
                Box::new(OhosHapBuilder),
                Box::new(OhosAppBuilder),
                Box::new(WebBuilder),
                Box::new(WindowsBuilder),
            ],
        }
    }
}

impl FlutterAppBuilder {
    pub fn clean(
        &self,
        environment: Option<&std::collections::HashMap<String, String>>,
    ) -> Result<(), BuildError> {
        FlutterCommand::new(environment).clean()
    }

    pub fn build(
        &self,
        platform: &str,
        target: Option<&str>,
        arguments: Map<String, Value>,
        environment: Option<std::collections::HashMap<String, String>>,
    ) -> Result<BuildResult, BuildError> {
        let builder = self
            .builders
            .iter()
            .find(|b| b.matches(platform, target))
            .ok_or_else(|| {
                BuildError::UnsupportedBuilder(format!(
                    "No builder found for platform={} target={}",
                    platform,
                    target.unwrap_or("")
                ))
            })?;

        if !builder.is_supported_on_current_platform() {
            return Err(BuildError::UnsupportedPlatform(format!(
                "{} is not supported on the current platform",
                builder.name()
            )));
        }

        let config = BuildConfig::new(arguments);
        builder.validate_arguments(&config)?;

        let pubspec = read_pubspec(Path::new("pubspec.yaml"))?;
        let mut build_arguments = encode_build_arguments(&config.arguments);
        build_arguments.push("--dart-define".to_string());
        build_arguments.push(format!("FLUTTER_BUILD_NAME={}", pubspec.build_name));
        build_arguments.push("--dart-define".to_string());
        build_arguments.push(format!("FLUTTER_BUILD_NUMBER={}", pubspec.build_number));

        let start = Instant::now();
        let flutter = FlutterCommand::new(environment.as_ref());
        let exit = flutter.build_with_echo(builder.build_subcommand(), &build_arguments)?;
        if exit != 0 {
            return Err(BuildError::CommandFailed(format!(
                "flutter build failed with exit code {}",
                exit
            )));
        }

        let (output_directory, output_files) = builder.resolve_output_files(
            &config,
            environment.as_ref(),
        )?;

        if output_files.is_empty() {
            return Err(BuildError::ArtifactNotFound(format!(
                "No build artifacts found in {}",
                output_directory.display()
            )));
        }

        Ok(builder.build_result(
            config,
            output_directory,
            output_files,
            start.elapsed().as_millis(),
        ))
    }
}

pub fn build(request: BuildRequest) -> Result<BuildResult, BuildError> {
    FlutterAppBuilder::default().build(
        &request.platform,
        request.target.as_deref(),
        request.arguments,
        request.environment,
    )
}

fn read_pubspec(path: &Path) -> Result<PubspecInfo, BuildError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| BuildError::Io(format!("Failed to read {}: {}", path.display(), e)))?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content)
        .map_err(|e| BuildError::Parse(format!("Failed to parse pubspec.yaml: {}", e)))?;

    let version = yaml
        .get("version")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or("0.1.0+1")
        .to_string();

    let mut split = version.split('+');
    let build_name = split.next().unwrap_or("0.1.0").to_string();
    let build_number = split.next_back().unwrap_or(&version).to_string();

    Ok(PubspecInfo {
        build_name,
        build_number,
    })
}

fn encode_build_arguments(arguments: &Map<String, Value>) -> Vec<String> {
    let mut output = Vec::new();

    for (key, value) in arguments {
        match value {
            Value::Null | Value::Bool(_) => {
                output.push(format!("--{}", key));
            }
            Value::Object(map) => {
                for (sub_key, sub_value) in map {
                    output.push(format!("--{}", key));
                    output.push(format!("{}={}", sub_key, value_to_cli_string(sub_value)));
                }
            }
            _ => {
                output.push(format!("--{}", key));
                output.push(value_to_cli_string(value));
            }
        }
    }

    output
}

fn value_to_cli_string(value: &Value) -> String {
    if let Some(s) = value.as_str() {
        s.to_string()
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn encode_arguments_matches_dart_behavior() {
        let mut args = Map::new();
        args.insert("verbose".to_string(), Value::Bool(true));
        args.insert("flavor".to_string(), Value::String("dev".to_string()));
        args.insert("build-number".to_string(), Value::Number(42.into()));
        args.insert(
            "dart-define".to_string(),
            json!({"APP_ENV":"dev","FOO":"bar"}),
        );
        args.insert("null-flag".to_string(), Value::Null);

        let actual = encode_build_arguments(&args);
        let expected = vec![
            "--verbose",
            "--flavor",
            "dev",
            "--build-number",
            "42",
            "--dart-define",
            "APP_ENV=dev",
            "--dart-define",
            "FOO=bar",
            "--null-flag",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn build_config_mode_and_flavor() {
        let mut args = Map::new();
        args.insert("profile".to_string(), Value::Bool(true));
        args.insert("flavor".to_string(), Value::String("prod".to_string()));
        let config = BuildConfig::new(args);
        assert_eq!(config.mode(), BuildMode::Profile);
        assert_eq!(config.flavor(), Some("prod"));
    }
}
