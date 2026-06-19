use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use fastforge_store_api::{AppStoreManager, GooglePlayManager};
use fastforge_store_api_core::{StoreAppsApi, StoreReleasesApi, StoreReviewsApi};
use std::path::Path;

use crate::config::{FastforgeConfig, resolve_app_id};

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
    #[command(about = "List all releases for an app")]
    ListReleases {
        #[arg(short, long, help = "Store name: googleplay or appstore")]
        store: String,
        #[arg(long, help = "App ID or alias from config")]
        app: Option<String>,
        #[arg(long, help = "App ID (overrides config and --app)")]
        app_id: Option<String>,
    },
    #[command(about = "List review history for a release")]
    ListReviews {
        #[arg(short, long, help = "Store name: googleplay or appstore")]
        store: String,
        #[arg(long, help = "App ID or alias from config")]
        app: Option<String>,
        #[arg(long, help = "App ID (overrides config and --app)")]
        app_id: Option<String>,
    },
    #[command(about = "Get details of a specific app")]
    GetApp {
        #[arg(short, long, help = "Store name: googleplay or appstore")]
        store: String,
        #[arg(long, help = "App ID or alias from config")]
        app: Option<String>,
        #[arg(long, help = "App ID (overrides config and --app)")]
        app_id: Option<String>,
    },
}

pub async fn execute(args: &StoreArgs) -> Result<()> {
    // Load config from current directory (or parents).
    let config = FastforgeConfig::from_root(&find_project_root()?)?;

    match &args.action {
        StoreAction::ListApps { store } => {
            let result = match store.to_ascii_lowercase().as_str() {
                "googleplay" => build_googleplay(&config, store)?.list_apps().await?,
                "appstore" => build_appstore(&config, store)?.list_apps().await?,
                other => {
                    anyhow::bail!("Unsupported store: `{other}`. Supported: googleplay, appstore")
                }
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        StoreAction::ListReleases { store, app, app_id } => {
            let store_name = store.to_ascii_lowercase();
            let store_config = config_for(&config, &store_name)?;
            let resolved =
                resolve_app_id(store_config, app_id.as_deref(), app.as_deref(), &store_name)?;

            let result = match store_name.as_str() {
                "googleplay" => {
                    build_googleplay(&config, &store_name)?
                        .list_releases(&resolved)
                        .await?
                }
                "appstore" => {
                    build_appstore(&config, &store_name)?
                        .list_releases(&resolved)
                        .await?
                }
                _ => unreachable!(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        StoreAction::GetApp { store, app, app_id } => {
            let store_name = store.to_ascii_lowercase();
            let store_config = config_for(&config, &store_name)?;
            let resolved =
                resolve_app_id(store_config, app_id.as_deref(), app.as_deref(), &store_name)?;

            let result = match store_name.as_str() {
                "googleplay" => {
                    build_googleplay(&config, &store_name)?
                        .get_app(&resolved)
                        .await?
                }
                "appstore" => {
                    build_appstore(&config, &store_name)?
                        .get_app(&resolved)
                        .await?
                }
                _ => unreachable!(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        StoreAction::ListReviews { store, app, app_id } => {
            let store_name = store.to_ascii_lowercase();
            let store_config = config_for(&config, &store_name)?;
            let resolved =
                resolve_app_id(store_config, app_id.as_deref(), app.as_deref(), &store_name)?;

            let result = match store_name.as_str() {
                "googleplay" => {
                    build_googleplay(&config, &store_name)?
                        .list_reviews(&resolved)
                        .await?
                }
                "appstore" => {
                    build_appstore(&config, &store_name)?
                        .list_reviews(&resolved)
                        .await?
                }
                _ => unreachable!(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

fn find_project_root() -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    let mut dir: Option<&Path> = Some(cwd.as_path());
    while let Some(d) = dir {
        if d.join(".fastforge").is_dir() {
            return Ok(d.to_path_buf());
        }
        dir = d.parent();
    }
    Ok(cwd)
}

fn config_for<'a>(
    config: &'a FastforgeConfig,
    store: &str,
) -> Result<&'a Option<crate::config::StoreTargetConfig>> {
    match store {
        "googleplay" => Ok(&config.stores.googleplay),
        "appstore" => Ok(&config.stores.appstore),
        _ => anyhow::bail!("Unsupported store: `{store}`"),
    }
}

fn build_googleplay(config: &FastforgeConfig, store: &str) -> Result<GooglePlayManager> {
    if let Some(cfg) = config_for(config, store)? {
        if let Some(path) = &cfg.key_path {
            return Ok(GooglePlayManager::with_credential_file(path));
        }
    }
    Ok(GooglePlayManager::new())
}

fn build_appstore(config: &FastforgeConfig, store: &str) -> Result<AppStoreManager> {
    if let Some(cfg) = config_for(config, store)? {
        return Ok(AppStoreManager::with_config(
            cfg.key_id.clone(),
            cfg.issuer_id.clone(),
            cfg.key_path.clone(),
        ));
    }
    Ok(AppStoreManager::new())
}
