use crate::cli::commands::app::resolve_app;
use crate::cli::GlobalArgs;
use crate::AppStoreConnectContext;
use crate::types::{
    self as asc_types,
    AppInfoLocalizationAttributes,
    AppStoreVersionAttributes,
    AppStoreVersionLocalizationAttributes,
    Platform,
};
use anyhow::{Context, Result, anyhow};
use clap::Args;
use serde_json::{Value, json};
use std::path::Path;

#[derive(Args, Debug)]
pub struct PushArgs {
    #[arg(long = "app")]
    pub app: String,
    #[arg(long = "input")]
    pub input: Option<String>,
    #[arg(long = "dry-run", default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
struct PushAction {
    resource_type: String,
    resource_id: Option<String>,
    locale: Option<String>,
    action: &'static str,
    details: String,
}

pub async fn execute(args: &PushArgs, _global: &GlobalArgs) -> Result<()> {
    let ctx = AppStoreConnectContext::from_env()?;
    let mut actions: Vec<PushAction> = Vec::new();

    eprintln!("🔍 Resolving app '{}'...", args.app);
    let app_row = resolve_app(&ctx, &args.app).await?;
    let bundle_id = &app_row.bundle_id;
    let base_dir = args.input.as_deref().map(Path::new)
        .unwrap_or_else(|| Path::new(".fastforge/stores/appstore"))
        .join(bundle_id);
    eprintln!("📂 Reading sync directory: {}", base_dir.display());

    // Scan phase
    let loc_dir = base_dir.join("app_info_localizations");
    if loc_dir.exists() {
        for entry in std::fs::read_dir(&loc_dir).context("reading app_info_localizations")? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
                let loc: AppInfoLocalizationAttributes = read_yaml(&path)?;
                let locale = loc.locale.clone().unwrap_or_default();
                actions.push(PushAction {
                    resource_type: "appInfoLocalizations".into(), resource_id: None,
                    locale: Some(locale.clone()), action: "create",
                    details: format!("app_info_localizations/{locale}.yaml"),
                });
            }
        }
    }

    let versions_dir = base_dir.join("versions");
    if versions_dir.exists() {
        for version_entry in std::fs::read_dir(&versions_dir).context("reading versions")? {
            let version_entry = version_entry?;
            let version_path = version_entry.path();
            if !version_path.is_dir() { continue; }
            let dir_name = version_path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            let version_yaml_path = version_path.join("app_store_version.yaml");
            if version_yaml_path.exists() {
                let _v: AppStoreVersionAttributes = read_yaml(&version_yaml_path)?;
                actions.push(PushAction {
                    resource_type: "appStoreVersions".into(), resource_id: None,
                    locale: None, action: "update",
                    details: format!("versions/{dir_name}/app_store_version.yaml"),
                });
            }

            let vloc_dir = version_path.join("localizations");
            if vloc_dir.exists() {
                for vloc_entry in std::fs::read_dir(&vloc_dir).context("reading version localizations")? {
                    let vloc_entry = vloc_entry?;
                    let vloc_path = vloc_entry.path();
                    if !vloc_path.is_dir() { continue; }
                    let locale = vloc_path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                    let vloc_yaml_path = vloc_path.join("app_store_version_localization.yaml");
                    if vloc_yaml_path.exists() {
                        let _v: AppStoreVersionLocalizationAttributes = read_yaml(&vloc_yaml_path)?;
                        actions.push(PushAction {
                            resource_type: "appStoreVersionLocalizations".into(), resource_id: None,
                            locale: Some(locale.clone()), action: "create",
                            details: format!("versions/{dir_name}/localizations/{locale}/app_store_version_localization.yaml"),
                        });
                    }
                }
            }
        }
    }

    if args.dry_run {
        eprintln!("\n📋 Dry run — would push {} actions:", actions.len());
        for a in &actions { eprintln!("  [{:>8}] {} {} {}", a.action, a.resource_type, a.locale.as_deref().unwrap_or(""), a.details); }
        return Ok(());
    }

    eprintln!("\n🚀 Executing push...");
    let app_id = app_row.id.clone();

    // Execute: app info localizations
    let current_app_info = fetch_current_app_info(&ctx, &app_id).await?;
    let existing_locales = fetch_current_app_info_localizations(&ctx, &current_app_info.id).await?;
    if loc_dir.exists() {
        for entry in std::fs::read_dir(&loc_dir).context("reading app_info_localizations")? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
                let loc: AppInfoLocalizationAttributes = read_yaml(&path)?;
                let locale = loc.locale.clone().unwrap_or_default();
                if let Some(existing) = existing_locales.iter().find(|e| e.locale == locale) {
                    push_update_app_info_localization(&ctx, &existing.id, &loc).await?;
                    eprintln!("  ✓ app_info_localizations/{locale}.yaml (id: {})", existing.id);
                } else {
                    push_create_app_info_localization(&ctx, &current_app_info.id, &loc).await?;
                    eprintln!("  ✓ app_info_localizations/{locale}.yaml (new)");
                }
            }
        }
    }

    // Execute: versions
    if versions_dir.exists() {
        for version_entry in std::fs::read_dir(&versions_dir)? {
            let version_entry = version_entry?;
            let version_path = version_entry.path();
            if !version_path.is_dir() { continue; }
            let dir_name = version_path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            let version_yaml_path = version_path.join("app_store_version.yaml");
            if !version_yaml_path.exists() { continue; }

            let version_yaml: AppStoreVersionAttributes = read_yaml(&version_yaml_path)?;
            let platform = version_yaml.platform.as_ref().map(|p: &Platform| p.to_string()).unwrap_or_default();
            let version_str = version_yaml.version_string.as_deref().unwrap_or_default().to_string();
            let existing_versions = fetch_all_versions_simple(&ctx, &app_id, &platform, &version_str).await?;
            let version_id = if let Some(v) = existing_versions.first() { v.id.clone() }
                else { eprintln!("  ⚠ Skipping {dir_name}: version not found"); continue; };

            let existing_vlocs = fetch_version_localizations_simple(&ctx, &version_id).await?;
            let vloc_dir = version_path.join("localizations");
            if vloc_dir.exists() {
                for vloc_entry in std::fs::read_dir(&vloc_dir)? {
                    let vloc_entry = vloc_entry?;
                    let vloc_path = vloc_entry.path();
                    if !vloc_path.is_dir() { continue; }
                    let locale = vloc_path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                    let vloc_yaml_path = vloc_path.join("app_store_version_localization.yaml");
                    if !vloc_yaml_path.exists() { continue; }

                    let vloc_yaml: AppStoreVersionLocalizationAttributes = read_yaml(&vloc_yaml_path)?;
                    if let Some(existing) = existing_vlocs.iter().find(|e| e.locale == locale) {
                        push_update_version_localization(&ctx, &existing.id, &vloc_yaml).await?;
                    } else {
                        push_create_version_localization(&ctx, &version_id, &vloc_yaml).await?;
                    }
                }
            }
        }
    }

    eprintln!("\n✅ Push complete.");
    Ok(())
}

fn read_yaml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let content = std::fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_yaml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))
}

struct CurrentAppInfo { id: String }

async fn fetch_current_app_info(ctx: &AppStoreConnectContext, app_id: &str) -> Result<CurrentAppInfo> {
    let resp: Value = ctx.http.get(ctx.url(&format!("/v1/apps/{app_id}/appInfos")))
        .query(&[("limit", 1)]).send().await?.error_for_status()?.json().await?;
    let info_id = resp["data"][0]["id"].as_str()
        .ok_or_else(|| anyhow!("no appInfo found for app {app_id}"))?.to_string();
    Ok(CurrentAppInfo { id: info_id })
}

struct CurrentAppInfoLocalization { id: String, locale: String }

async fn fetch_current_app_info_localizations(ctx: &AppStoreConnectContext, app_info_id: &str) -> Result<Vec<CurrentAppInfoLocalization>> {
    let resp: Value = ctx.http.get(ctx.url(&format!("/v1/appInfos/{app_info_id}/appInfoLocalizations")))
        .query(&[("limit", 200i64)]).send().await?.error_for_status()?.json().await?;
    Ok(resp["data"].as_array().cloned().unwrap_or_default().into_iter().filter_map(|item| {
        Some(CurrentAppInfoLocalization { id: item["id"].as_str()?.to_string(), locale: item["attributes"]["locale"].as_str()?.to_string() })
    }).collect())
}

async fn push_update_app_info_localization(ctx: &AppStoreConnectContext, id: &str, yaml: &AppInfoLocalizationAttributes) -> Result<()> {
    let mut attrs = json!({});
    if let Some(ref v) = yaml.name { attrs["name"] = json!(v); }
    if let Some(ref v) = yaml.subtitle { attrs["subtitle"] = json!(v); }
    if let Some(ref v) = yaml.privacy_policy_url { attrs["privacyPolicyUrl"] = json!(v); }
    if let Some(ref v) = yaml.privacy_policy_text { attrs["privacyPolicyText"] = json!(v); }
    if let Some(ref v) = yaml.privacy_choices_url { attrs["privacyChoicesUrl"] = json!(v); }
    ctx.http.patch(ctx.url(&format!("/v1/appInfoLocalizations/{id}")))
        .json(&json!({"data": {"type": "appInfoLocalizations", "id": id, "attributes": attrs}}))
        .send().await?.error_for_status()?;
    Ok(())
}

async fn push_create_app_info_localization(ctx: &AppStoreConnectContext, app_info_id: &str, yaml: &AppInfoLocalizationAttributes) -> Result<()> {
    let locale = yaml.locale.clone().unwrap_or_default();
    let mut attrs = json!({"locale": locale});
    if let Some(ref v) = yaml.name { attrs["name"] = json!(v); }
    if let Some(ref v) = yaml.subtitle { attrs["subtitle"] = json!(v); }
    if let Some(ref v) = yaml.privacy_policy_url { attrs["privacyPolicyUrl"] = json!(v); }
    if let Some(ref v) = yaml.privacy_policy_text { attrs["privacyPolicyText"] = json!(v); }
    if let Some(ref v) = yaml.privacy_choices_url { attrs["privacyChoicesUrl"] = json!(v); }
    ctx.http.post(ctx.url("/v1/appInfoLocalizations"))
        .json(&json!({"data": {"type": "appInfoLocalizations", "attributes": attrs, "relationships": {"appInfo": {"data": {"type": "appInfos", "id": app_info_id}}}}}))
        .send().await?.error_for_status()?;
    Ok(())
}

async fn push_update_version_localization(ctx: &AppStoreConnectContext, id: &str, yaml: &AppStoreVersionLocalizationAttributes) -> Result<()> {
    let mut attrs = json!({});
    if let Some(ref v) = yaml.description { attrs["description"] = json!(v); }
    if let Some(ref v) = yaml.keywords { attrs["keywords"] = json!(v); }
    if let Some(ref v) = yaml.marketing_url { attrs["marketingUrl"] = json!(v); }
    if let Some(ref v) = yaml.promotional_text { attrs["promotionalText"] = json!(v); }
    if let Some(ref v) = yaml.support_url { attrs["supportUrl"] = json!(v); }
    if let Some(ref v) = yaml.whats_new { attrs["whatsNew"] = json!(v); }
    ctx.http.patch(ctx.url(&format!("/v1/appStoreVersionLocalizations/{id}")))
        .json(&json!({"data": {"type": "appStoreVersionLocalizations", "id": id, "attributes": attrs}}))
        .send().await?.error_for_status()?;
    Ok(())
}

async fn push_create_version_localization(ctx: &AppStoreConnectContext, version_id: &str, yaml: &AppStoreVersionLocalizationAttributes) -> Result<()> {
    let locale = yaml.locale.clone().unwrap_or_default();
    let mut attrs = json!({"locale": locale});
    if let Some(ref v) = yaml.description { attrs["description"] = json!(v); }
    if let Some(ref v) = yaml.keywords { attrs["keywords"] = json!(v); }
    if let Some(ref v) = yaml.marketing_url { attrs["marketingUrl"] = json!(v); }
    if let Some(ref v) = yaml.promotional_text { attrs["promotionalText"] = json!(v); }
    if let Some(ref v) = yaml.support_url { attrs["supportUrl"] = json!(v); }
    if let Some(ref v) = yaml.whats_new { attrs["whatsNew"] = json!(v); }
    ctx.http.post(ctx.url("/v1/appStoreVersionLocalizations"))
        .json(&json!({"data": {"type": "appStoreVersionLocalizations", "attributes": attrs, "relationships": {"appStoreVersion": {"data": {"type": "appStoreVersions", "id": version_id}}}}}))
        .send().await?.error_for_status()?;
    Ok(())
}

struct CurrentAppVersion { id: String }

struct CurrentAppVersionLocalization { id: String, locale: String }

async fn fetch_all_versions_simple(ctx: &AppStoreConnectContext, app_id: &str, platform: &str, version_str: &str) -> Result<Vec<CurrentAppVersion>> {
    let filter_platform = Some(vec![match platform {
        "MAC_OS" => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::MacOs,
        "TV_OS" => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::TvOs,
        "VISION_OS" => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::VisionOs,
        _ => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::Ios,
    }]);
    let filter_version = Some(vec![version_str.to_string()]);
    let resp = ctx.client.apps_app_store_versions_get_to_many_related(
        app_id, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        filter_platform.as_ref().map(|v: &Vec<_>| v.as_ref()),
        filter_version.as_ref().map(|v| v.as_ref()),
        None, Some(1), None, None, None,
    ).await.map_err(|e| anyhow!("failed to fetch versions: {e}"))?;
    Ok(resp.into_inner().data.into_iter().map(|v| CurrentAppVersion { id: v.id }).collect())
}

async fn fetch_version_localizations_simple(ctx: &AppStoreConnectContext, version_id: &str) -> Result<Vec<CurrentAppVersionLocalization>> {
    let resp = ctx.client.app_store_versions_app_store_version_localizations_get_to_many_related(
        version_id, None, None, None, None, None, None, None, Some(200), None, None, None,
    ).await.map_err(|e| anyhow!("failed to fetch version localizations: {e}"))?;
    Ok(resp.into_inner().data.into_iter().filter_map(|v| {
        Some(CurrentAppVersionLocalization { id: v.id, locale: v.attributes.as_ref()?.locale.clone()? })
    }).collect())
}
