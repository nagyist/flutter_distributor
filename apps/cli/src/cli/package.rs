use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct PackageArgs {
    #[arg(short, long = "target")]
    pub target: Option<String>,
}

pub async fn execute(_args: &PackageArgs) -> Result<()> {
    log::info!("Executing package command");
    Ok(())
}
