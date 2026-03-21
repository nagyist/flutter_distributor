use anyhow::Result;
use clap::{Args, Subcommand};

use crate::config::load_workflows;
use crate::runtime::workflow_runner::{WorkflowRunArgs, run_workflow};

#[derive(Args)]
pub struct WorkflowArgs {
    #[command(subcommand)]
    pub command: WorkflowCommand,
}

#[derive(Subcommand)]
pub enum WorkflowCommand {
    List,
    View(WorkflowViewArgs),
    Run(WorkflowRunCliArgs),
}

#[derive(Args)]
pub struct WorkflowViewArgs {
    pub workflow: String,
}

#[derive(Args)]
pub struct WorkflowRunCliArgs {
    pub workflow: String,
    #[arg(long = "job")]
    pub job: Option<String>,
    #[arg(long = "env", value_name = "KEY=VALUE")]
    pub env: Vec<String>,
    #[arg(long = "dry-run", default_value_t = false)]
    pub dry_run: bool,
}

pub async fn execute(args: &WorkflowArgs) -> Result<()> {
    let root = std::env::current_dir()?;

    match &args.command {
        WorkflowCommand::List => {
            for workflow in load_workflows(&root)? {
                println!("{}\t{}", workflow.id, workflow.display_name());
            }
        }
        WorkflowCommand::View(view) => {
            let workflow = crate::config::find_workflow(&root, &view.workflow)?;
            println!("{}", serde_json::to_string_pretty(&workflow.definition)?);
        }
        WorkflowCommand::Run(run) => {
            let record = run_workflow(
                &root,
                &WorkflowRunArgs {
                    workflow: run.workflow.clone(),
                    job: run.job.clone(),
                    env: run.env.clone(),
                    dry_run: run.dry_run,
                },
            )?;
            println!("{}", serde_json::to_string_pretty(&record)?);
        }
    }

    Ok(())
}
