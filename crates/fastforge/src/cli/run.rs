use anyhow::Result;
use clap::{Args, Subcommand};

use crate::runtime::workflow_runner::{get_run, list_runs};

#[derive(Args)]
pub struct RunArgs {
    #[command(subcommand)]
    pub command: RunCommand,
}

#[derive(Subcommand)]
pub enum RunCommand {
    List,
    View(RunViewArgs),
}

#[derive(Args)]
pub struct RunViewArgs {
    pub run_id: String,
}

pub async fn execute(args: &RunArgs) -> Result<()> {
    let root = std::env::current_dir()?;

    match &args.command {
        RunCommand::List => {
            for run in list_runs(&root)? {
                println!(
                    "{}\t{}\t{}\t{}",
                    run.id, run.workflow_id, run.status, run.started_at_epoch_ms
                );
            }
        }
        RunCommand::View(view) => {
            let run = get_run(&root, &view.run_id)?;
            println!("{}", serde_json::to_string_pretty(&run)?);
        }
    }

    Ok(())
}
