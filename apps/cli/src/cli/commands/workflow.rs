use std::collections::HashMap;
use std::path::PathBuf;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use minact_core::actions::{Action, ActionContext, ActionOutput};
use minact_core::{ActionRegistry, Engine, StepConclusion, WorkflowError, WorkflowParser};
use serde_json::{Map, Value};

#[derive(Args)]
pub struct WorkflowArgs {
    #[command(subcommand)]
    pub command: WorkflowCommands,
}

#[derive(Subcommand)]
pub enum WorkflowCommands {
    /// Run a workflow
    Run {
        /// Path to a specific workflow file
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Event name to simulate (default: workflow_dispatch)
        #[arg(short, long, default_value = "workflow_dispatch")]
        event: String,

        /// Working directory (default: current directory)
        #[arg(short, long)]
        workspace: Option<PathBuf>,

        /// Input parameters (key=value)
        #[arg(short, long, value_parser = parse_key_val)]
        input: Vec<KeyVal>,
    },

    /// List available workflows
    List {
        /// Project directory to search
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Show detailed info
        #[arg(short, long)]
        verbose: bool,
    },

    /// Validate a workflow file
    Validate {
        /// Path to workflow file
        file: PathBuf,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct KeyVal {
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal, String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("Invalid KEY=value format: {}", s))?;
    Ok(KeyVal {
        key: s[..pos].to_string(),
        value: s[pos + 1..].to_string(),
    })
}

fn init_tracing() {
    // Initialize tracing only once for minact-core output
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with_target(false)
            .init();
    });
}

pub async fn execute(args: &WorkflowArgs) -> anyhow::Result<()> {
    init_tracing();

    match &args.command {
        WorkflowCommands::Run {
            file,
            event,
            workspace,
            input,
        } => cmd_run(file, event, workspace, input).await,
        WorkflowCommands::List { dir, verbose } => cmd_list(dir, *verbose),
        WorkflowCommands::Validate { file } => cmd_validate(file),
    }
}

async fn cmd_run(
    file: &Option<PathBuf>,
    event: &str,
    workspace: &Option<PathBuf>,
    input: &[KeyVal],
) -> anyhow::Result<()> {
    let workspace = workspace
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let inputs: HashMap<String, String> = input
        .iter()
        .map(|kv| (kv.key.clone(), kv.value.clone()))
        .collect();

    // Load workflow
    let workflow = if let Some(file_path) = file {
        log::info!("Loading workflow from: {}", file_path.display());
        WorkflowParser::parse_file(file_path)?
    } else {
        // Discover workflows in workspace
        let workflows = WorkflowParser::discover_workflows(&workspace)?;
        match workflows.len() {
            0 => {
                anyhow::bail!(
                    "No workflow files found in {}\n\
                     Looked in: .fastforge/workflows/",
                    workspace.display()
                );
            }
            1 => workflows.into_iter().next().unwrap(),
            n => {
                log::warn!("Found {} workflows. Use --file to specify one.\n", n);
                for (i, wf) in workflows.iter().enumerate() {
                    let source = wf
                        .file_path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    log::warn!("  {}. {} ({})", i + 1, wf.name, source);
                }
                anyhow::bail!("Please specify a workflow file with --file");
            }
        }
    };

    log::info!("Workflow: {}", workflow.name);
    log::info!("Workspace: {}", workspace.display());
    log::info!("Event: {}", event);
    if !inputs.is_empty() {
        log::info!("Inputs: {:?}", inputs);
    }

    // Create engine with fastforge-specific built-in actions
    let mut engine = Engine::with_actions(workspace.clone(), ActionRegistry::new());
    engine.register_action(Box::new(FastforgePackageAction));
    engine.register_action(Box::new(FastforgePublishAction));
    let result = engine.run_workflow(&workflow, event, inputs).await?;

    // Print summary
    println!();
    println!("═══════════════════════════════════════");
    println!("  Workflow: {}", result.workflow_name);
    println!(
        "  Result: {}",
        if result.success {
            "✓ SUCCESS"
        } else {
            "✗ FAILED"
        }
    );
    println!("═══════════════════════════════════════");

    for (job_id, job_result) in &result.job_results {
        let status = match job_result.conclusion {
            minact_core::StepConclusion::Success => "✓",
            minact_core::StepConclusion::Failure => "✗",
            minact_core::StepConclusion::Cancelled => "◯",
            minact_core::StepConclusion::Skipped => "–",
        };
        println!("  {} {} ({})", status, job_result.job_name, job_id);
    }

    if !result.success {
        std::process::exit(1);
    }

    Ok(())
}

fn cmd_list(dir: &Option<PathBuf>, verbose: bool) -> anyhow::Result<()> {
    let dir = dir
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let workflows = WorkflowParser::discover_workflows(&dir)?;

    if workflows.is_empty() {
        println!("No workflows found in {}", dir.display());
        println!("Looked in: .fastforge/workflows/");
        return Ok(());
    }

    println!("Workflows in {}:", dir.display());
    println!();

    for workflow in &workflows {
        let source = workflow
            .file_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        if verbose {
            println!("  ┌─ {}", workflow.name);
            println!("  ├─ File: {}", source);
            println!("  ├─ Jobs: {}", workflow.jobs.len());
            println!("  ├─ Events:");
            if workflow.on.push.is_some() {
                println!("  │    • push");
            }
            if workflow.on.pull_request.is_some() {
                println!("  │    • pull_request");
            }
            if workflow.on.release.is_some() {
                println!("  │    • release");
            }
            if workflow.on.workflow_dispatch.is_some() {
                println!("  │    • workflow_dispatch");
            }
            if workflow.on.schedule.is_some() {
                println!("  │    • schedule");
            }
            println!("  └─ Jobs:");
            for (job_id, job) in &workflow.jobs {
                println!("       • {} ({})", job.name, job_id);
            }
            println!();
        } else {
            println!("  • {} ({})", workflow.name, source);
        }
    }

    Ok(())
}

fn cmd_validate(file: &PathBuf) -> anyhow::Result<()> {
    if !file.exists() {
        anyhow::bail!("File not found: {}", file.display());
    }

    match WorkflowParser::parse_file(file) {
        Ok(workflow) => {
            println!("✓ Valid workflow: {}", workflow.name);
            println!("  Jobs: {}", workflow.jobs.len());
            for (job_id, job) in &workflow.jobs {
                println!(
                    "    • {} ({}) — {} step(s)",
                    job.name,
                    job_id,
                    job.steps.len()
                );
            }
            Ok(())
        }
        Err(e) => {
            println!("✗ Invalid workflow: {}", e);
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Built-in action: fastforge/package
// ---------------------------------------------------------------------------

struct FastforgePackageAction;

#[async_trait]
impl Action for FastforgePackageAction {
    fn id(&self) -> &'static str {
        "fastforge/package"
    }

    fn validate(&self, ctx: &ActionContext) -> Result<(), WorkflowError> {
        if !ctx.inputs.contains_key("platform") {
            return Err(WorkflowError::Other(
                "fastforge/package requires 'platform' input".to_string(),
            ));
        }
        if !ctx.inputs.contains_key("target") {
            return Err(WorkflowError::Other(
                "fastforge/package requires 'target' input".to_string(),
            ));
        }
        Ok(())
    }

    async fn run(&self, ctx: &ActionContext) -> Result<ActionOutput, WorkflowError> {
        let platform = &ctx.inputs["platform"];
        let target = &ctx.inputs["target"];
        let output = ctx
            .inputs
            .get("output")
            .cloned()
            .unwrap_or_else(|| "dist/".to_string());
        let artifact_name = ctx.inputs.get("artifact-name").cloned();
        let skip_clean = ctx
            .inputs
            .get("skip-clean")
            .map(|s| s == "true")
            .unwrap_or(false);
        let build_target = ctx.inputs.get("build-target").cloned();

        // Parse additional build-args from JSON string (if provided)
        let mut build_args = Map::new();
        if let Some(args_json) = ctx.inputs.get("build-args") {
            let parsed: Map<String, Value> = serde_json::from_str(args_json)
                .map_err(|e| WorkflowError::Other(format!("Invalid build-args JSON: {}", e)))?;
            build_args = parsed;
        }
        if let Some(bt) = &build_target {
            build_args.insert("target".to_string(), Value::String(bt.clone()));
        }

        // Resolve hooks from inputs
        let hooks: Option<HashMap<String, serde_yaml::Value>> = {
            let mut map = HashMap::new();
            if let Some(cmd) = ctx.inputs.get("hook-pre") {
                map.insert("pre".to_string(), serde_yaml::Value::String(cmd.clone()));
            }
            if let Some(cmd) = ctx.inputs.get("hook-post") {
                map.insert("post".to_string(), serde_yaml::Value::String(cmd.clone()));
            }
            if map.is_empty() { None } else { Some(map) }
        };

        tracing::info!(
            "[fastforge/package] Packaging platform={} target={}...",
            platform,
            target
        );

        let environment: HashMap<String, String> = ctx
            .env
            .iter()
            .chain(
                std::env::vars()
                    .into_iter()
                    .collect::<HashMap<_, _>>()
                    .iter(),
            )
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let artifacts = super::package::package_flutter_artifact(
            platform,
            target,
            build_args,
            environment,
            &output,
            artifact_name,
            !skip_clean,
            hooks.as_ref(),
        )
        .map_err(|e| WorkflowError::Other(format!("Package failed: {}", e)))?;

        let artifact_paths: Vec<String> = artifacts
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        tracing::info!(
            "[fastforge/package] Package completed: {} artifact(s)",
            artifacts.len()
        );

        Ok(ActionOutput {
            success: true,
            conclusion: StepConclusion::Success,
            outputs: HashMap::from([
                ("artifact-count".to_string(), artifacts.len().to_string()),
                ("artifact-paths".to_string(), artifact_paths.join(",")),
            ]),
            artifacts: artifacts
                .into_iter()
                .map(|p| minact_core::Artifact {
                    name: p
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    path: p,
                })
                .collect(),
        })
    }
}

// ---------------------------------------------------------------------------
// Built-in action: fastforge/publish
// ---------------------------------------------------------------------------

struct FastforgePublishAction;

#[async_trait]
impl Action for FastforgePublishAction {
    fn id(&self) -> &'static str {
        "fastforge/publish"
    }

    fn validate(&self, ctx: &ActionContext) -> Result<(), WorkflowError> {
        if !ctx.inputs.contains_key("path") {
            return Err(WorkflowError::Other(
                "fastforge/publish requires 'path' input".to_string(),
            ));
        }
        if !ctx.inputs.contains_key("target") {
            return Err(WorkflowError::Other(
                "fastforge/publish requires 'target' input (e.g. s3, qiniu, oss, fir, firebase, github, appstore)"
                    .to_string(),
            ));
        }
        Ok(())
    }

    async fn run(&self, ctx: &ActionContext) -> Result<ActionOutput, WorkflowError> {
        let artifact_path = &ctx.inputs["path"];
        let target = &ctx.inputs["target"];

        // Parse publish-args from JSON string (if provided)
        let publish_arguments: HashMap<String, String> = if let Some(args_json) =
            ctx.inputs.get("publish-args")
        {
            serde_json::from_str(args_json)
                .map_err(|e| WorkflowError::Other(format!("Invalid publish-args JSON: {}", e)))?
        } else {
            // Fall back to treating all extra inputs as publish arguments
            let mut args = HashMap::new();
            for (key, value) in &ctx.inputs {
                if key != "path" && key != "target" && key != "publish-args" {
                    args.insert(key.clone(), value.clone());
                }
            }
            args
        };

        tracing::info!(
            "[fastforge/publish] Publishing '{}' to target '{}'...",
            artifact_path,
            target
        );

        let message = super::publish::publish_artifact(artifact_path, target, publish_arguments)
            .map_err(|e| WorkflowError::Other(format!("Publish failed: {}", e)))?;

        tracing::info!("[fastforge/publish] Publish completed: {}", message);

        Ok(ActionOutput {
            success: true,
            conclusion: StepConclusion::Success,
            outputs: HashMap::from([("message".to_string(), message)]),
            artifacts: vec![],
        })
    }
}
