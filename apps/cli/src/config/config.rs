use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FastforgeProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FastforgeDefaults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_name: Option<String>,
}

// ── Store config ───────────────────────────────────────────────────────────────

/// Per-store configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoresConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appstore: Option<StoreTargetConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub googleplay: Option<StoreTargetConfig>,
}

/// Credentials and app list for a single store target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreTargetConfig {
    /// App Store Connect API key ID (appstore) / path to service account JSON (googleplay)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// App Store Connect issuer ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer_id: Option<String>,
    /// Path to the .p8 key file (appstore) / service account JSON file (googleplay)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
    /// Apps managed in this store
    #[serde(default)]
    pub apps: Vec<AppConfig>,
}

/// A single app registered in the store config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Store-internal app ID (package name for Google Play, ASC ID for App Store)
    pub id: String,
    /// Optional alias used in CLI (e.g. `--app production`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<FastforgeProject>,
    #[serde(default = "default_output")]
    pub output: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<FastforgeDefaults>,
    /// Store credentials and app registrations
    #[serde(default)]
    pub stores: StoresConfig,
}

/// Resolve the app ID to use for a store operation.
///
/// Priority: CLI `--app-id` > CLI `--app` (alias lookup in config) >
/// single app from config > error.
pub fn resolve_app_id(
    store_config: &Option<StoreTargetConfig>,
    cli_app_id: Option<&str>,
    cli_app: Option<&str>,
    store_name: &str,
) -> anyhow::Result<String> {
    // 1. Explicit --app-id
    if let Some(id) = cli_app_id {
        return Ok(id.to_string());
    }

    let config = match store_config {
        Some(c) => c,
        None => anyhow::bail!(
            "No `{}` configuration found in `.fastforge/config.yaml`. Either set it up or pass --app-id directly.",
            store_name
        ),
    };

    // 2. --app alias lookup
    if let Some(alias) = cli_app {
        if let Some(app) = config
            .apps
            .iter()
            .find(|a| a.name.as_deref() == Some(alias))
        {
            return Ok(app.id.clone());
        }
        anyhow::bail!(
            "App `{alias}` not found in `{}` configuration. Available: {}",
            store_name,
            config
                .apps
                .iter()
                .filter_map(|a| a.name.as_deref())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // 3. Single app from config
    if config.apps.len() == 1 {
        return Ok(config.apps[0].id.clone());
    }

    if config.apps.is_empty() {
        anyhow::bail!(
            "No apps configured for `{}`. Add them to `.fastforge/config.yaml` or pass --app-id.",
            store_name
        );
    }

    anyhow::bail!(
        "Multiple apps configured for `{}`. Specify which one with --app-id or --app. \
         Available: {}",
        store_name,
        config
            .apps
            .iter()
            .map(|a| {
                a.name
                    .as_deref()
                    .map(|n| format!("{n} ({})", a.id))
                    .unwrap_or_else(|| a.id.clone())
            })
            .collect::<Vec<_>>()
            .join(", ")
    );
}

fn default_version() -> u32 {
    1
}

fn default_output() -> String {
    "dist/".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_version(),
            project: None,
            output: default_output(),
            env: None,
            defaults: None,
            stores: StoresConfig::default(),
        }
    }
}

impl Config {
    pub fn from_root(root: &Path) -> Result<Self> {
        let path = root.join(".fastforge").join("config.yaml");
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let parsed: Self = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;
        Ok(parsed)
    }

    #[allow(dead_code)]
    pub fn resolved_env(&self) -> HashMap<String, String> {
        let mut vars: HashMap<String, String> = std::env::vars().collect();
        if let Some(env) = &self.env {
            for (key, value) in env {
                vars.insert(key.clone(), expand_env_value(value, &vars));
            }
        }
        vars
    }

    #[allow(dead_code)]
    pub fn output_dir(&self, root: &Path) -> PathBuf {
        root.join(&self.output)
    }
}

#[allow(dead_code)]
fn expand_env_value(value: &str, vars: &HashMap<String, String>) -> String {
    let mut output = String::new();
    let mut rest = value;

    while let Some(start) = rest.find("${") {
        output.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        if let Some(end) = after.find('}') {
            let key = &after[..end];
            if let Some(replacement) = vars.get(key).cloned().or_else(|| std::env::var(key).ok()) {
                output.push_str(&replacement);
            }
            rest = &after[end + 1..];
        } else {
            output.push_str(&rest[start..]);
            rest = "";
        }
    }

    output.push_str(rest);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn defaults_when_no_config_exists() {
        let dir = TempDir::new().unwrap();
        let config = Config::from_root(dir.path()).unwrap();
        assert_eq!(config.version, 1);
        assert_eq!(config.output, "dist/");
    }
}
