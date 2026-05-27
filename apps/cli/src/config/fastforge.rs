use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::{BTreeMap, HashMap};
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
pub struct FastforgeConfig {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(default)]
    pub jobs: BTreeMap<String, WorkflowJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(default)]
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub run: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone)]
pub struct LoadedWorkflow {
    pub id: String,
    pub path: PathBuf,
    pub definition: WorkflowDefinition,
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
        if let Some(app) = config.apps.iter().find(|a| a.name.as_deref() == Some(alias)) {
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

impl Default for FastforgeConfig {
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

impl FastforgeConfig {
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

    pub fn resolved_env(&self) -> HashMap<String, String> {
        let mut vars: HashMap<String, String> = std::env::vars().collect();
        if let Some(env) = &self.env {
            for (key, value) in env {
                vars.insert(key.clone(), expand_env_value(value, &vars));
            }
        }
        vars
    }

    pub fn output_dir(&self, root: &Path) -> PathBuf {
        root.join(&self.output)
    }
}

impl LoadedWorkflow {
    pub fn display_name(&self) -> &str {
        self.definition.name.as_deref().unwrap_or(&self.id)
    }
}

pub fn load_workflows(root: &Path) -> Result<Vec<LoadedWorkflow>> {
    let workflows_dir = root.join(".fastforge").join("workflows");
    if !workflows_dir.exists() {
        return Ok(Vec::new());
    }

    let mut workflows = Vec::new();
    for entry in std::fs::read_dir(&workflows_dir)
        .with_context(|| format!("Failed to read {}", workflows_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(ext) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if ext != "yaml" && ext != "yml" {
            continue;
        }
        let Some(id) = path.file_stem().and_then(|value| value.to_str()) else {
            continue;
        };
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let definition: WorkflowDefinition = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;
        workflows.push(LoadedWorkflow {
            id: id.to_string(),
            path,
            definition,
        });
    }

    workflows.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(workflows)
}

pub fn find_workflow(root: &Path, workflow: &str) -> Result<LoadedWorkflow> {
    let workflows = load_workflows(root)?;
    workflows
        .into_iter()
        .find(|item| item.id == workflow || item.display_name() == workflow)
        .ok_or_else(|| anyhow!("Workflow `{}` not found in .fastforge/workflows", workflow))
}

pub fn resolve_packaging_config(
    root: &Path,
    platform: &str,
    target: &str,
    explicit: Option<&str>,
) -> Result<Option<PathBuf>> {
    if let Some(explicit) = explicit {
        let candidate = root.join(explicit);
        if !candidate.exists() {
            return Err(anyhow!(
                "Packaging config `{}` does not exist",
                candidate.display()
            ));
        }
        return Ok(Some(candidate));
    }

    let preferred = root
        .join(".fastforge")
        .join("packaging")
        .join(platform)
        .join(format!("{target}.yaml"));
    if preferred.exists() {
        return Ok(Some(preferred));
    }

    let legacy = root
        .join(platform)
        .join("packaging")
        .join(target)
        .join("make_config.yaml");
    if legacy.exists() {
        return Ok(Some(legacy));
    }

    Ok(None)
}

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
        let config = FastforgeConfig::from_root(dir.path()).unwrap();
        assert_eq!(config.version, 1);
        assert_eq!(config.output, "dist/");
    }

    #[test]
    fn load_config_and_workflows() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".fastforge/workflows")).unwrap();
        std::fs::write(
            dir.path().join(".fastforge/config.yaml"),
            r#"
version: 1
output: build-dist/
env:
  API_KEY: ${HOME}
defaults:
  artifact_name: custom.{{ext}}
"#,
        )
        .unwrap();
        std::fs::write(
            dir.path().join(".fastforge/workflows/release.yaml"),
            r#"
name: Release
jobs:
  android:
    steps:
      - run: build
        with:
          platform: android
          target: apk
"#,
        )
        .unwrap();

        let config = FastforgeConfig::from_root(dir.path()).unwrap();
        assert_eq!(config.output, "build-dist/");
        assert_eq!(
            config.defaults.unwrap().artifact_name.as_deref(),
            Some("custom.{{ext}}")
        );

        let workflows = load_workflows(dir.path()).unwrap();
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].id, "release");
        assert_eq!(workflows[0].display_name(), "Release");
    }

    #[test]
    fn packaging_resolution_prefers_new_layout_then_legacy() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".fastforge/packaging/macos")).unwrap();
        std::fs::create_dir_all(dir.path().join("linux/packaging/deb")).unwrap();
        std::fs::write(
            dir.path().join(".fastforge/packaging/macos/dmg.yaml"),
            "title: demo\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("linux/packaging/deb/make_config.yaml"),
            "package_name: demo\n",
        )
        .unwrap();

        let dmg = resolve_packaging_config(dir.path(), "macos", "dmg", None).unwrap();
        assert_eq!(
            dmg.unwrap(),
            dir.path().join(".fastforge/packaging/macos/dmg.yaml")
        );

        let deb = resolve_packaging_config(dir.path(), "linux", "deb", None).unwrap();
        assert_eq!(
            deb.unwrap(),
            dir.path().join("linux/packaging/deb/make_config.yaml")
        );
    }
}
