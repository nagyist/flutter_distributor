use fastforge_core::{
    AppPublisher, PublishConfig, PublishError, PublishProgressCallback, PublishResult,
};
use std::env;
use std::path::Path;
use std::process::Command;

pub struct AppStorePublisher;

const PUBLISHER_NAME: &str = "appstore";
const ENV_APPSTORE_USERNAME: &str = "APPSTORE_USERNAME";
const ENV_APPSTORE_PASSWORD: &str = "APPSTORE_PASSWORD";
const ENV_APPSTORE_API_KEY: &str = "APPSTORE_APIKEY";
const ENV_APPSTORE_API_ISSUER: &str = "APPSTORE_APIISSUER";
const APPSTORE_CONNECT_APPS_URL: &str = "https://appstoreconnect.apple.com/apps";

impl AppPublisher for AppStorePublisher {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        PUBLISHER_NAME
    }

    fn is_supported_on_current_platform(&self) -> bool {
        cfg!(target_os = "macos")
    }

    fn perform_publish(
        &self,
        config: &PublishConfig,
        _on_progress: Option<&PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError> {
        if !self.is_supported_on_current_platform() {
            return Err(PublishError::General(
                "AppStore publisher is only supported on macOS.".to_string(),
            ));
        }

        let artifact_path = config.artifact_path.as_deref().ok_or_else(|| {
            PublishError::General("Missing `artifact_path` in publish config.".to_string())
        })?;
        let artifact_type = appstore_artifact_type(artifact_path)?;
        let auth = AppStoreAuth::from_config(config)?;

        let mut args = vec![
            "altool".to_string(),
            "--upload-app".to_string(),
            "--file".to_string(),
            artifact_path.to_string(),
            "--type".to_string(),
            artifact_type.to_string(),
        ];
        args.extend(auth.to_cli_args());

        let output = Command::new("xcrun")
            .args(args)
            .output()
            .map_err(to_publish_error)?;

        if !output.status.success() {
            return Err(PublishError::General(format!(
                "Upload of appstore failed: exit_code={:?}, stderr={}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(PublishResult {
            success: true,
            message: APPSTORE_CONNECT_APPS_URL.to_string(),
        })
    }
}

struct AppStoreAuth {
    username: Option<String>,
    password: Option<String>,
    api_key: Option<String>,
    api_issuer: Option<String>,
}

impl AppStoreAuth {
    fn from_config(config: &PublishConfig) -> Result<Self, PublishError> {
        let username = optional_value(config, &["username"], &[ENV_APPSTORE_USERNAME]);
        let password = optional_value(config, &["password"], &[ENV_APPSTORE_PASSWORD]);
        let api_key = optional_value(config, &["api-key"], &[ENV_APPSTORE_API_KEY]);
        let api_issuer = optional_value(config, &["api-issuer"], &[ENV_APPSTORE_API_ISSUER]);

        let has_userpass = is_non_empty(&username) || is_non_empty(&password);
        let has_api = is_non_empty(&api_key) || is_non_empty(&api_issuer);

        if !has_userpass && !has_api {
            return Err(PublishError::General(format!(
                "Missing `{ENV_APPSTORE_USERNAME}` & `{ENV_APPSTORE_PASSWORD}` or `{ENV_APPSTORE_API_KEY}` & `{ENV_APPSTORE_API_ISSUER}`."
            )));
        }
        if is_non_empty(&username) ^ is_non_empty(&password) {
            return Err(PublishError::General(format!(
                "Missing `{ENV_APPSTORE_USERNAME}` & `{ENV_APPSTORE_PASSWORD}`."
            )));
        }
        if is_non_empty(&api_key) ^ is_non_empty(&api_issuer) {
            return Err(PublishError::General(format!(
                "Missing `{ENV_APPSTORE_API_KEY}` & `{ENV_APPSTORE_API_ISSUER}`."
            )));
        }

        Ok(Self {
            username,
            password,
            api_key,
            api_issuer,
        })
    }

    fn to_cli_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        push_flag(&mut args, "--username", self.username.as_deref());
        push_flag(&mut args, "--password", self.password.as_deref());
        push_flag(&mut args, "--apiKey", self.api_key.as_deref());
        push_flag(&mut args, "--apiIssuer", self.api_issuer.as_deref());
        args
    }
}

fn appstore_artifact_type(path: &str) -> Result<&'static str, PublishError> {
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| {
            PublishError::General(format!("Cannot infer artifact type from file path: {path}"))
        })?;
    match extension {
        "ipa" => Ok("ios"),
        "pkg" => Ok("osx"),
        _ => Ok("osx"),
    }
}

fn optional_value(
    config: &PublishConfig,
    argument_keys: &[&str],
    env_keys: &[&str],
) -> Option<String> {
    config
        .publish_arguments
        .as_ref()
        .and_then(|arguments| {
            argument_keys
                .iter()
                .find_map(|key| arguments.get(*key).cloned())
        })
        .or_else(|| env_keys.iter().find_map(|key| env::var(key).ok()))
        .filter(|value| !value.trim().is_empty())
}

fn is_non_empty(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|v| !v.trim().is_empty())
}

fn push_flag(args: &mut Vec<String>, flag: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|v| !v.trim().is_empty()) {
        args.push(flag.to_string());
        args.push(value.to_string());
    }
}

fn to_publish_error(error: impl std::fmt::Display) -> PublishError {
    PublishError::General(error.to_string())
}
