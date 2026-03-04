use clap::{Parser, Subcommand};

mod cli;
mod config;

use cli::{
    AnalyzeArgs, BuildArgs, PackageArgs, PublishArgs, ReleaseArgs, UpgradeArgs, VersionCheckArgs,
};

#[derive(Parser)]
#[command(name = "fastforge")]
#[command(about = "Package and publish your apps with ease.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Analyze your project")]
    Analyze(AnalyzeArgs),
    #[command(about = "Build your project")]
    Build(BuildArgs),
    #[command(about = "Package your project")]
    Package(PackageArgs),
    #[command(about = "Publish your project")]
    Publish(PublishArgs),
    #[command(about = "Release the current Flutter application")]
    Release(ReleaseArgs),
    #[command(about = "Update Fastforge to the latest version")]
    Upgrade(UpgradeArgs),
    #[command(
        name = "version-check",
        about = "Check for a newer version of fastforge"
    )]
    VersionCheck(VersionCheckArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze(args) => {
            cli::analyze::execute(args).await?;
        }
        Commands::Build(args) => {
            cli::build::execute(args).await?;
        }
        Commands::Package(args) => {
            cli::package::execute(args).await?;
        }
        Commands::Publish(args) => {
            cli::publish::execute(args).await?;
        }
        Commands::Release(args) => {
            cli::release::execute(args).await?;
        }
        Commands::Upgrade(args) => {
            cli::upgrade::execute(args).await?;
        }
        Commands::VersionCheck(args) => {
            cli::version_check::execute(args).await?;
        }
    }

    Ok(())
}
