use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use fastforge_store_api::{AppStoreManager, GooglePlayManager, StoreManager};

#[derive(Args)]
pub struct StoreArgs {
    #[command(subcommand)]
    pub action: StoreAction,
}

#[derive(Subcommand)]
pub enum StoreAction {
    #[command(about = "List all apps in the store account")]
    ListApps {
        #[arg(short, long, help = "Store name: googleplay or appstore")]
        store: String,
    },
    #[command(about = "Get details of a specific app")]
    GetApp {
        #[arg(short, long, help = "Store name: googleplay or appstore")]
        store: String,
        #[arg(
            long,
            help = "App ID (package name for Google Play, App Store Connect ID for App Store)"
        )]
        app_id: String,
    },
}

pub async fn execute(args: &StoreArgs) -> Result<()> {
    match &args.action {
        StoreAction::ListApps { store } => {
            let apps = match store.to_ascii_lowercase().as_str() {
                "googleplay" => GooglePlayManager.list_apps().await?,
                "appstore" => AppStoreManager.list_apps().await?,
                other => {
                    return Err(anyhow!(
                        "Unsupported store: `{other}`. Supported: googleplay, appstore"
                    ));
                }
            };

            println!("{}", serde_json::to_string_pretty(&apps)?);
        }
        StoreAction::GetApp { store, app_id } => {
            let app = match store.to_ascii_lowercase().as_str() {
                "googleplay" => GooglePlayManager.get_app(app_id).await?,
                "appstore" => AppStoreManager.get_app(app_id).await?,
                other => {
                    return Err(anyhow!(
                        "Unsupported store: `{other}`. Supported: googleplay, appstore"
                    ));
                }
            };

            println!("{}", serde_json::to_string_pretty(&app)?);
        }
    }

    Ok(())
}
