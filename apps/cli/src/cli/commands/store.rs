use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};

use crate::config::Config;

#[derive(Args)]
pub struct StoreArgs {
    #[command(subcommand)]
    pub command: StoreCommands,
}

#[derive(Subcommand)]
pub enum StoreCommands {
    #[command(about = "List configured distribution stores")]
    List,
}

pub async fn execute(args: &StoreArgs) -> Result<()> {
    match args.command {
        StoreCommands::List => list_stores(),
    }
}

fn list_stores() -> Result<()> {
    let config = Config::load()?;
    if config.stores.is_empty() {
        return Err(anyhow!("No stores found in {}.", Config::DEFAULT_PATH));
    }

    if let Some(appstore) = &config.stores.appstore {
        println!("appstore ({})", appstore.auth.auth_type());
        if appstore.apps.is_empty() {
            println!("  apps: none");
        } else {
            for app in &appstore.apps {
                let identifier = app.identifier().unwrap_or("<unknown>");
                let label = app.name.as_deref().unwrap_or(identifier);
                println!("  - {} [ios] {}", label, identifier);
            }
        }
    }

    if let Some(googleplay) = &config.stores.googleplay {
        println!("googleplay ({})", googleplay.auth.auth_type());
        if googleplay.apps.is_empty() {
            println!("  apps: none");
        } else {
            for app in &googleplay.apps {
                let identifier = app.identifier().unwrap_or("<unknown>");
                println!("  - {} [android] {}", identifier, identifier);
            }
        }
    }

    Ok(())
}
