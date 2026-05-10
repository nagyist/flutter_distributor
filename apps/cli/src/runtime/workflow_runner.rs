use anyhow::{Context, Result, anyhow};
use fastforge_app_builder::{BuildResult, FlutterAppBuilder};
use fastforge_app_packager::{
    AndroidAabPackager, AndroidApkPackager, AppPackager, IOSIpaPackager, LinuxAppImagePackager,
    LinuxDebPackager, LinuxDirectPackager, LinuxPacmanPackager, LinuxRpmPackager, LinuxZipPackager,
    MacOSDmgPackager, MacOSPkgPackager, MacOSZipPackager, OHOSAppPackager, OHOSHapPackager,
    PackageConfig, WebDirectPackager, WebZipPackager, WindowsDirectPackager, WindowsExePackager,
    WindowsMsixPackager, WindowsZipPackager,
};
use fastforge_app_publisher::AppPublisher;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{
    FastforgeConfig, LoadedWorkflow, WorkflowJob, WorkflowStep, find_workflow,
    resolve_packaging_config,
};

#[derive(Debug, Clone)]
pub struct WorkflowRunArgs {
    pub workflow: String,
    pub job: Option<String>,
    pub env: Vec<String>,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunRecord {
    pub id: String,
    pub workflow_id: String,
    pub workflow_name: String,
    pub workflow_path: String,
    pub dry_run: bool,
    pub status: String,
    pub started_at_epoch_ms: u128,
    pub finished_at_epoch_ms: Option<u128>,
    pub jobs: Vec<WorkflowJobRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowJobRecord {
    pub id: String,
    pub status: String,
    pub steps: Vec<WorkflowStepRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepRecord {
    pub name: String,
    pub run: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
}

struct JobExecutionState {
    build_result: Option<BuildResult>,
    packaged_artifacts: Vec<PathBuf>,
}

pub fn run_workflow(root: &Path, args: &WorkflowRunArgs) -> Result<WorkflowRunRecord> {
    let workflow = find_workflow(root, &args.workflow)?;
    let config = FastforgeConfig::from_root(root)?;
    let mut record = WorkflowRunRecord {
        id: generate_run_id()?,
        workflow_id: workflow.id.clone(),
        workflow_name: workflow.display_name().to_string(),
        workflow_path: workflow.path.display().to_string(),
        dry_run: args.dry_run,
        status: "running".to_string(),
        started_at_epoch_ms: now_millis()?,
        finished_at_epoch_ms: None,
        jobs: Vec::new(),
    };

    let result = execute_workflow(root, &config, &workflow, args, &mut record);
    record.finished_at_epoch_ms = Some(now_millis()?);
    record.status = if result.is_ok() {
        "success".to_string()
    } else {
        "failed".to_string()
    };
    store_run_record(root, &record)?;
    result?;
    Ok(record)
}

fn execute_workflow(
    root: &Path,
    config: &FastforgeConfig,
    workflow: &LoadedWorkflow,
    args: &WorkflowRunArgs,
    record: &mut WorkflowRunRecord,
) -> Result<()> {
    let cli_env = parse_cli_env(&args.env)?;

    for (job_id, job) in &workflow.definition.jobs {
        if args.job.as_deref().is_some_and(|name| name != job_id) {
            continue;
        }

        let env = merge_env(
            &config.resolved_env(),
            workflow.definition.env.as_ref(),
            job.env.as_ref(),
            None,
            Some(&cli_env),
        );
        let mut state = JobExecutionState {
            build_result: None,
            packaged_artifacts: Vec::new(),
        };
        let mut job_record = WorkflowJobRecord {
            id: job_id.clone(),
            status: "running".to_string(),
            steps: Vec::new(),
        };

        execute_job(
            root,
            config,
            job,
            &env,
            args.dry_run,
            &mut state,
            &mut job_record,
        )
        .with_context(|| format!("Workflow job `{}` failed", job_id))?;
        job_record.status = "success".to_string();
        record.jobs.push(job_record);
    }

    if record.jobs.is_empty() {
        return Err(anyhow!(
            "Workflow `{}` has no matching jobs to run",
            workflow.id
        ));
    }

    Ok(())
}

fn execute_job(
    root: &Path,
    config: &FastforgeConfig,
    job: &WorkflowJob,
    env: &HashMap<String, String>,
    dry_run: bool,
    state: &mut JobExecutionState,
    record: &mut WorkflowJobRecord,
) -> Result<()> {
    for (index, step) in job.steps.iter().enumerate() {
        let step_env = merge_env(env, None, None, step.env.as_ref(), None);
        let step_name = step
            .name
            .clone()
            .unwrap_or_else(|| format!("step-{}", index + 1));
        let mut step_record = WorkflowStepRecord {
            name: step_name,
            run: step.run.clone(),
            status: "running".to_string(),
            detail: None,
        };

        let detail = if dry_run {
            build_dry_run_detail(root, step)?
        } else {
            execute_step(root, config, step, &step_env, state)?
        };

        step_record.detail = Some(detail);
        step_record.status = "success".to_string();
        record.steps.push(step_record);
    }

    Ok(())
}

fn execute_step(
    root: &Path,
    config: &FastforgeConfig,
    step: &WorkflowStep,
    env: &HashMap<String, String>,
    state: &mut JobExecutionState,
) -> Result<Value> {
    match step.run.as_str() {
        "build" => execute_build_step(step, env, state),
        "package" => execute_package_step(root, config, step, state),
        "publish" => execute_publish_step(step, env, state),
        other => Err(anyhow!(
            "Unsupported workflow step run `{}`. Supported values: build, package, publish",
            other
        )),
    }
}

fn build_dry_run_detail(root: &Path, step: &WorkflowStep) -> Result<Value> {
    let mut detail = serde_json::Map::new();
    detail.insert("run".to_string(), Value::String(step.run.clone()));
    if let Some(with) = &step.with {
        detail.insert("with".to_string(), serde_json::to_value(with)?);
        if step.run == "package" {
            let platform = get_required_string(with, "platform")?;
            let target = get_required_string(with, "target")?;
            let explicit = get_optional_string(with, "packaging_config")
                .or_else(|| get_optional_string(with, "make_config"));
            let resolved: Option<String> =
                resolve_packaging_config(root, &platform, &target, explicit.as_deref())?
                    .map(|path| path.display().to_string());
            detail.insert(
                "resolvedPackagingConfig".to_string(),
                serde_json::to_value(resolved)?,
            );
        }
    }
    Ok(Value::Object(detail))
}

fn execute_build_step(
    step: &WorkflowStep,
    env: &HashMap<String, String>,
    state: &mut JobExecutionState,
) -> Result<Value> {
    let with = step
        .with
        .as_ref()
        .ok_or_else(|| anyhow!("Build step is missing `with` configuration"))?;
    let platform = get_required_string(with, "platform")?;
    let target = get_optional_string(with, "target");
    let build_args = get_json_object(with, "build_args")?;
    let builder = FlutterAppBuilder::default();
    let result = builder
        .build(&platform, target.as_deref(), build_args, Some(env.clone()))
        .map_err(|err| anyhow!(err.to_string()))?;

    let detail = result.to_json_compatible();
    state.build_result = Some(result);
    Ok(detail)
}

fn execute_package_step(
    root: &Path,
    config: &FastforgeConfig,
    step: &WorkflowStep,
    state: &mut JobExecutionState,
) -> Result<Value> {
    let with = step
        .with
        .as_ref()
        .ok_or_else(|| anyhow!("Package step is missing `with` configuration"))?;
    let build_result = state
        .build_result
        .as_ref()
        .ok_or_else(|| anyhow!("Package step requires a prior build step in the same job"))?;
    let platform = get_required_string(with, "platform")?;
    let target = get_required_string(with, "target")?;
    let explicit = get_optional_string(with, "packaging_config")
        .or_else(|| get_optional_string(with, "make_config"));
    let packaging_config = resolve_packaging_config(root, &platform, &target, explicit.as_deref())?;
    let pubspec = read_pubspec_metadata(root)?;
    let artifact_name = get_optional_string(with, "artifact_name").or_else(|| {
        config
            .defaults
            .as_ref()
            .and_then(|defaults| defaults.artifact_name.clone())
    });

    let package_config = PackageConfig {
        app_name: pubspec.name.clone(),
        app_binary_name: pubspec.name,
        app_version: pubspec.version,
        build_mode: build_result.config.mode().as_str().to_string(),
        platform: platform.clone(),
        flavor: build_result.config.flavor().map(|value| value.to_string()),
        channel: get_optional_string(with, "channel"),
        artifact_name,
        package_format: target.clone(),
        is_installer: matches!(target.as_str(), "exe" | "msix" | "pkg"),
        build_output_dir: build_result.output_directory.clone(),
        build_output_files: build_result.output_files.clone(),
        output_dir: config.output_dir(root),
    };

    let packager = select_packager(&platform, &target)?;
    let result = packager
        .package(&package_config)
        .map_err(|err| anyhow!(err.to_string()))?;
    state.packaged_artifacts = result.artifacts.clone();

    Ok(serde_json::json!({
        "outputFiles": result.artifacts.iter().map(|path| path.display().to_string()).collect::<Vec<_>>(),
        "packagingConfig": packaging_config.map(|path| path.display().to_string()),
    }))
}

fn execute_publish_step(
    step: &WorkflowStep,
    _env: &HashMap<String, String>,
    state: &mut JobExecutionState,
) -> Result<Value> {
    let with = step
        .with
        .as_ref()
        .ok_or_else(|| anyhow!("Publish step is missing `with` configuration"))?;
    let target = get_required_string(with, "target")?;
    let artifact_path = if let Some(path) = get_optional_string(with, "path") {
        path
    } else {
        state
            .packaged_artifacts
            .first()
            .map(|path| path.display().to_string())
            .ok_or_else(|| anyhow!("Publish step requires `path` or a prior package step"))?
    };
    let publish_args = get_string_map(with, "publish_args");
    let publish_config = fastforge_app_publisher::PublishConfig {
        app_version: None,
        artifact_path: Some(artifact_path.clone()),
        publish_arguments: publish_args,
    };

    let result = match target.to_ascii_lowercase().as_str() {
        "s3" | "minio" => fastforge_app_publisher::S3Publisher::new().publish(publish_config, None),
        "qiniu" => fastforge_app_publisher::QiniuPublisher::new().publish(publish_config, None),
        "oss" => fastforge_app_publisher::OssPublisher::new().publish(publish_config, None),
        "cos" => fastforge_app_publisher::CosPublisher::new().publish(publish_config, None),
        "fir" => fastforge_app_publisher::FirPublisher::new().publish(publish_config, None),
        "firebase" => {
            fastforge_app_publisher::FirebasePublisher::new().publish(publish_config, None)
        }
        "firebase-hosting" => {
            fastforge_app_publisher::FirebaseHostingPublisher::new().publish(publish_config, None)
        }
        "github" => fastforge_app_publisher::GitHubPublisher::new().publish(publish_config, None),
        "appgallery" => {
            fastforge_app_publisher::AppGalleryPublisher::new().publish(publish_config, None)
        }
        "vercel" => fastforge_app_publisher::VercelPublisher::new().publish(publish_config, None),
        "custom" => fastforge_app_publisher::CustomPublisher::new().publish(publish_config, None),
        other => {
            return Err(anyhow!("Unsupported publish target: `{}`", other));
        }
    }
    .map_err(|err: fastforge_app_publisher::PublishError| anyhow!(err.to_string()))?;

    Ok(serde_json::json!({
        "target": target,
        "path": artifact_path,
        "message": result.message,
    }))
}

fn parse_cli_env(items: &[String]) -> Result<HashMap<String, String>> {
    let mut env = HashMap::new();
    for item in items {
        let (key, value) = item
            .split_once('=')
            .ok_or_else(|| anyhow!("Invalid --env value `{}`; expected KEY=VALUE", item))?;
        env.insert(key.trim().to_string(), value.to_string());
    }
    Ok(env)
}

fn merge_env(
    base: &HashMap<String, String>,
    workflow_env: Option<&HashMap<String, String>>,
    job_env: Option<&HashMap<String, String>>,
    step_env: Option<&HashMap<String, String>>,
    cli_env: Option<&HashMap<String, String>>,
) -> HashMap<String, String> {
    let mut merged = base.clone();
    for layer in [workflow_env, job_env, step_env, cli_env] {
        if let Some(values) = layer {
            merged.extend(values.clone());
        }
    }
    merged
}

fn get_required_string(map: &HashMap<String, serde_yaml::Value>, key: &str) -> Result<String> {
    get_optional_string(map, key).ok_or_else(|| anyhow!("Missing required key `{}`", key))
}

fn get_optional_string(map: &HashMap<String, serde_yaml::Value>, key: &str) -> Option<String> {
    map.get(key)
        .and_then(serde_yaml::Value::as_str)
        .map(str::to_string)
}

fn get_json_object(
    map: &HashMap<String, serde_yaml::Value>,
    key: &str,
) -> Result<serde_json::Map<String, Value>> {
    let Some(value) = map.get(key) else {
        return Ok(serde_json::Map::new());
    };
    let json_value = serde_json::to_value(value)?;
    match json_value {
        Value::Object(object) => Ok(object),
        other => Err(anyhow!("`{}` must be an object, got {}", key, other)),
    }
}

fn get_string_map(
    map: &HashMap<String, serde_yaml::Value>,
    key: &str,
) -> Option<HashMap<String, String>> {
    let value = map.get(key)?;
    let object = value.as_mapping()?;
    let mut result = HashMap::new();
    for (item_key, item_value) in object {
        let key = item_key.as_str()?;
        let value = item_value.as_str()?;
        result.insert(key.to_string(), value.to_string());
    }
    Some(result)
}

fn select_packager(platform: &str, target: &str) -> Result<Box<dyn AppPackager>> {
    let packager: Box<dyn AppPackager> = match (platform, target) {
        ("android", "apk") => Box::new(AndroidApkPackager),
        ("android", "aab") => Box::new(AndroidAabPackager),
        ("ios", "ipa") => Box::new(IOSIpaPackager),
        ("linux", "deb") => Box::new(LinuxDebPackager),
        ("linux", "rpm") => Box::new(LinuxRpmPackager),
        ("linux", "appimage") => Box::new(LinuxAppImagePackager),
        ("linux", "pacman") => Box::new(LinuxPacmanPackager),
        ("linux", "zip") => Box::new(LinuxZipPackager),
        ("linux", "direct") => Box::new(LinuxDirectPackager),
        ("macos", "dmg") => Box::new(MacOSDmgPackager),
        ("macos", "pkg") => Box::new(MacOSPkgPackager::default()),
        ("macos", "zip") => Box::new(MacOSZipPackager),
        ("ohos", "hap") => Box::new(OHOSHapPackager),
        ("ohos", "app") => Box::new(OHOSAppPackager),
        ("web", "zip") => Box::new(WebZipPackager),
        ("web", "direct") => Box::new(WebDirectPackager),
        ("windows", "exe") => Box::new(WindowsExePackager),
        ("windows", "msix") => Box::new(WindowsMsixPackager::default()),
        ("windows", "zip") => Box::new(WindowsZipPackager),
        ("windows", "direct") => Box::new(WindowsDirectPackager),
        _ => {
            return Err(anyhow!(
                "Unsupported package target `{}/{}'",
                platform,
                target
            ));
        }
    };
    Ok(packager)
}

#[derive(Debug)]
struct PubspecMetadata {
    name: String,
    version: String,
}

fn read_pubspec_metadata(root: &Path) -> Result<PubspecMetadata> {
    let path = root.join("pubspec.yaml");
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
    let name = yaml
        .get("name")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or("app")
        .to_string();
    let version = yaml
        .get("version")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or("0.1.0+1")
        .to_string();
    Ok(PubspecMetadata { name, version })
}

pub fn list_runs(root: &Path) -> Result<Vec<WorkflowRunRecord>> {
    let runs_dir = root.join(".fastforge").join("runs");
    if !runs_dir.exists() {
        return Ok(Vec::new());
    }
    let mut runs = Vec::new();
    for entry in std::fs::read_dir(&runs_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let content = std::fs::read_to_string(&path)?;
        let record: WorkflowRunRecord = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;
        runs.push(record);
    }
    runs.sort_by(|left, right| right.started_at_epoch_ms.cmp(&left.started_at_epoch_ms));
    Ok(runs)
}

pub fn get_run(root: &Path, run_id: &str) -> Result<WorkflowRunRecord> {
    list_runs(root)?
        .into_iter()
        .find(|record| record.id == run_id)
        .ok_or_else(|| anyhow!("Run `{}` not found", run_id))
}

fn store_run_record(root: &Path, record: &WorkflowRunRecord) -> Result<()> {
    let runs_dir = root.join(".fastforge").join("runs");
    std::fs::create_dir_all(&runs_dir)
        .with_context(|| format!("Failed to create {}", runs_dir.display()))?;
    let path = runs_dir.join(format!("{}.json", record.id));
    std::fs::write(&path, serde_json::to_string_pretty(record)?)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn generate_run_id() -> Result<String> {
    Ok(format!("run-{}", now_millis()?))
}

fn now_millis() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| anyhow!(err.to_string()))?
        .as_millis())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FastforgeConfig;
    use tempfile::TempDir;

    #[test]
    fn store_and_load_runs() {
        let dir = TempDir::new().unwrap();
        let record = WorkflowRunRecord {
            id: "run-1".to_string(),
            workflow_id: "release".to_string(),
            workflow_name: "Release".to_string(),
            workflow_path: ".fastforge/workflows/release.yaml".to_string(),
            dry_run: true,
            status: "success".to_string(),
            started_at_epoch_ms: 1,
            finished_at_epoch_ms: Some(2),
            jobs: vec![],
        };
        store_run_record(dir.path(), &record).unwrap();

        let runs = list_runs(dir.path()).unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].id, "run-1");
        assert_eq!(get_run(dir.path(), "run-1").unwrap().status, "success");
    }

    #[test]
    fn dry_run_resolves_packaging_path() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".fastforge/packaging/macos")).unwrap();
        std::fs::write(
            dir.path().join("pubspec.yaml"),
            "name: demo\nversion: 1.0.0+1\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join(".fastforge/packaging/macos/dmg.yaml"),
            "title: demo\n",
        )
        .unwrap();

        let step = WorkflowStep {
            name: Some("Package".to_string()),
            run: "package".to_string(),
            env: None,
            with: Some(HashMap::from([
                (
                    "platform".to_string(),
                    serde_yaml::Value::String("macos".to_string()),
                ),
                (
                    "target".to_string(),
                    serde_yaml::Value::String("dmg".to_string()),
                ),
            ])),
        };

        let detail = build_dry_run_detail(dir.path(), &step).unwrap();
        assert_eq!(
            detail["resolvedPackagingConfig"].as_str(),
            Some(
                dir.path()
                    .join(".fastforge/packaging/macos/dmg.yaml")
                    .display()
                    .to_string()
                    .as_str()
            )
        );

        let _ = FastforgeConfig::default();
    }
}
