use crate::cli::commands::app::resolve_app;
use crate::cli::GlobalArgs;
use crate::AppStoreConnectContext;
use anyhow::{Context, Result};
use clap::Args;
use serde_json::Value;
use std::path::Path;

use crate::types as asc_types;

/// Helper: fetch first page via typed client then follow `links.next` via raw HTTP.
async fn collect_pages<T>(
    ctx: &AppStoreConnectContext,
    resp: crate::ResponseValue<T>,
) -> Result<Vec<Value>>
where
    T: serde::Serialize + HasLinks,
{
    let inner = resp.into_inner();
    let mut all_data = Vec::new();
    for item in inner.data_iter() {
        all_data.push(serde_json::to_value(item)?);
    }
    let mut next_url = inner.next_link();
    while let Some(url) = next_url {
        let page: Value = ctx
            .http
            .get(&url)
            .send()
            .await
            .context("failed to fetch page")?
            .error_for_status()
            .context("API error")?
            .json()
            .await
            .context("failed to parse page")?;
        if let Some(data) = page.get("data").and_then(Value::as_array) {
            all_data.extend(data.iter().cloned());
        }
        next_url = page
            .get("links")
            .and_then(|l| l.get("next"))
            .and_then(Value::as_str)
            .map(|s| s.to_string());
    }
    Ok(all_data)
}

/// Trait for paginated typed responses.
trait HasLinks {
    type Item: serde::Serialize;
    fn data_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_>;
    fn next_link(&self) -> Option<String>;
}

macro_rules! impl_has_links {
    ($resp:ty, $item:ty) => {
        impl HasLinks for $resp {
            type Item = $item;
            fn data_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
                Box::new(self.data.iter())
            }
            fn next_link(&self) -> Option<String> {
                self.links.next.clone()
            }
        }
    };
}

impl_has_links!(asc_types::AppStoreVersionsResponse, asc_types::AppStoreVersion);
impl_has_links!(asc_types::AppInfosResponse, asc_types::AppInfo);
impl_has_links!(asc_types::AppInfoLocalizationsResponse, asc_types::AppInfoLocalization);
impl_has_links!(asc_types::AppStoreVersionLocalizationsResponse, asc_types::AppStoreVersionLocalization);
impl_has_links!(asc_types::AppScreenshotSetsResponse, asc_types::AppScreenshotSet);
impl_has_links!(asc_types::AppScreenshotsResponse, asc_types::AppScreenshot);
impl_has_links!(asc_types::AppPreviewSetsResponse, asc_types::AppPreviewSet);
impl_has_links!(asc_types::AppPreviewsResponse, asc_types::AppPreview);

/// Fetch all app store versions via the generated typed client.
async fn fetch_versions(
    ctx: &AppStoreConnectContext,
    app_id: &str,
    platform: Option<&str>,
    version: Option<&str>,
) -> Result<Vec<Value>> {
    let filter_platform = Some(vec![match platform.unwrap_or("IOS") {
        "MAC_OS" => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::MacOs,
        "TV_OS" => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::TvOs,
        "VISION_OS" => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::VisionOs,
        _ => asc_types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem::Ios,
    }]);
    let filter_version_string = version.map(|v| vec![v.to_string()]);

    let resp = ctx
        .client
        .apps_app_store_versions_get_to_many_related(
            app_id,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None,
            None, None,
            filter_platform.as_ref().map(|v| v.as_ref()),
            filter_version_string.as_ref().map(|v| v.as_ref()),
            None, Some(200), None, None, None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch versions: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Fetch all app infos via the generated typed client.
async fn fetch_app_infos(
    ctx: &AppStoreConnectContext,
    app_id: &str,
) -> Result<Vec<Value>> {
    let resp = ctx
        .client
        .apps_app_infos_get_to_many_related(
            app_id,
            None, None, None, None, None, None, Some(200), None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch app infos: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Fetch all app info localizations via the generated typed client.
async fn fetch_app_info_localizations(
    ctx: &AppStoreConnectContext,
    app_info_id: &str,
) -> Result<Vec<Value>> {
    let resp = ctx
        .client
        .app_infos_app_info_localizations_get_to_many_related(
            app_info_id,
            None, None, None, None, Some(200),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch app info localizations: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Fetch all version localizations via the generated typed client.
async fn fetch_version_localizations(
    ctx: &AppStoreConnectContext,
    version_id: &str,
) -> Result<Vec<Value>> {
    let resp = ctx
        .client
        .app_store_versions_app_store_version_localizations_get_to_many_related(
            version_id,
            None, None, None, None, None, None, None, Some(200), None, None, None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch version localizations: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Fetch all screenshot sets via the generated typed client.
async fn fetch_screenshot_sets(
    ctx: &AppStoreConnectContext,
    vloc_id: &str,
) -> Result<Vec<Value>> {
    let resp = ctx
        .client
        .app_store_version_localizations_app_screenshot_sets_get_to_many_related(
            vloc_id,
            None, None, None, None, None, None, None, None, None, Some(200), None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch screenshot sets: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Fetch all screenshots in a set via the generated typed client.
async fn fetch_screenshots(
    ctx: &AppStoreConnectContext,
    ss_set_id: &str,
) -> Result<Vec<Value>> {
    let resp = ctx
        .client
        .app_screenshot_sets_app_screenshots_get_to_many_related(
            ss_set_id,
            None, None, None, Some(200),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch screenshots: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Fetch all preview sets via the generated typed client.
async fn fetch_preview_sets(
    ctx: &AppStoreConnectContext,
    vloc_id: &str,
) -> Result<Vec<Value>> {
    let resp = ctx
        .client
        .app_store_version_localizations_app_preview_sets_get_to_many_related(
            vloc_id,
            None, None, None, None, None, None, None, None, None, Some(200), None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch preview sets: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Fetch all previews in a set via the generated typed client.
async fn fetch_previews(
    ctx: &AppStoreConnectContext,
    pv_set_id: &str,
) -> Result<Vec<Value>> {
    let resp = ctx
        .client
        .app_preview_sets_app_previews_get_to_many_related(
            pv_set_id,
            None, None, None, Some(200),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch previews: {e}"))?;
    collect_pages(ctx, resp).await
}

/// Resolve an image download URL from imageAsset templateUrl + dimensions.
fn resolve_image_url(attrs: &serde_json::Map<String, Value>) -> Option<String> {
    let image_asset = attrs.get("imageAsset")?;
    let template = image_asset.get("templateUrl")?.as_str()?;
    let width = image_asset.get("width")?.as_i64()?;
    let height = image_asset.get("height")?.as_i64()?;
    let ext = Path::new(attrs.get("fileName")?.as_str()?)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    Some(template
        .replace("{w}", &width.to_string())
        .replace("{h}", &height.to_string())
        .replace("{f}", ext))
}

/// Resolve a preview video download URL from previewFrameTimeCode + videoUrl/templateUrl.
fn resolve_video_url(attrs: &serde_json::Map<String, Value>) -> Option<String> {
    // Prefer direct videoUrl
    if let Some(url) = attrs.get("videoUrl").and_then(Value::as_str).filter(|s| !s.is_empty()) {
        return Some(url.to_string());
    }
    // Fallback: videoUrl may need a separate fetch with specific fields
    // For now, return None — will log a warning
    None
}

#[derive(Args, Debug)]
pub struct PullArgs {
    /// App bundle ID or app ID
    #[arg(long = "app")]
    pub app: String,

    /// Optional version string filter (e.g. "1.2.3"). If omitted, pulls all versions.
    #[arg(long = "version")]
    pub version: Option<String>,

    /// Platform filter (e.g. "IOS", "MAC_OS"). Defaults to "IOS".
    #[arg(long = "platform")]
    pub platform: Option<String>,

    /// Output directory root. Defaults to `.fastforge/stores/appstore/{bundle_id}/`
    #[arg(long = "output")]
    pub output: Option<String>,
}

/// Download a file from a URL and save it to disk.
async fn download_file(ctx: &AppStoreConnectContext, url: &str, dest: &Path) -> Result<()> {
    let response = ctx
        .http
        .get(url)
        .send()
        .await
        .context("failed to download file")?
        .error_for_status()
        .context("download returned error")?;
    let bytes = response.bytes().await.context("failed to read response body")?;
    tokio::fs::write(dest, &bytes)
        .await
        .with_context(|| format!("failed to write {}", dest.display()))?;
    Ok(())
}

/// Ensure a directory exists.
fn ensure_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .with_context(|| format!("failed to create directory {}", path.display()))
}

/// Write a YAML value to a file.
fn write_yaml<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let content = serde_yaml::to_string(value).context("failed to serialize YAML")?;
    std::fs::write(path, &content)
        .with_context(|| format!("failed to write {}", path.display()))
}

pub async fn execute(args: &PullArgs, _global: &GlobalArgs) -> Result<()> {
    let ctx = AppStoreConnectContext::from_env()?;
    let start = std::time::Instant::now();
    let mut pulled_count = 0u64;

    // 1. Resolve app
    eprintln!("🔍 Resolving app '{}'...", args.app);
    let app_row = resolve_app(&ctx, &args.app).await?;
    let bundle_id = &app_row.bundle_id;
    let app_id = &app_row.id;

    let output_root = args
        .output
        .as_deref()
        .map(Path::new)
        .unwrap_or_else(|| Path::new(".fastforge/stores/appstore"))
        .join(bundle_id);
    ensure_dir(&output_root)?;

    // Write app.yaml
    let app_yaml = asc_types::AppAttributes {
        bundle_id: Some(bundle_id.clone()),
        name: Some(app_row.name.clone()),
        primary_locale: Some(app_row.locale.clone()),
        sku: Some(app_row.sku.clone()),
        ..Default::default()
    };
    write_yaml(&output_root.join("app.yaml"), &app_yaml)?;
    eprintln!("  ✓ app.yaml");
    pulled_count += 1;

    // 2. Fetch appInfos
    eprintln!("📦 Fetching app info...");
    let app_infos = fetch_app_infos(&ctx, &app_id).await?;

    for app_info in &app_infos {
        let info_id = app_info["id"].as_str().unwrap_or_default().to_string();

        // 3. Fetch appInfoLocalizations
        let localizations = fetch_app_info_localizations(&ctx, &info_id).await?;
        let localizations_dir = output_root.join("app_info_localizations");
        ensure_dir(&localizations_dir)?;

        for loc in &localizations {
            let locale = loc["attributes"]["locale"]
                .as_str()
                .unwrap_or("unknown")
                .to_string();

            let yaml: asc_types::AppInfoLocalizationAttributes =
                serde_json::from_value(loc["attributes"].clone())
                    .context("failed to parse app info localization attributes")?;
            let loc_path = localizations_dir.join(format!("{locale}.yaml"));
            write_yaml(&loc_path, &yaml)?;
            eprintln!("  ✓ app_info_localizations/{locale}.yaml");
            pulled_count += 1;
        }
    }

    // 4. Fetch appStoreVersions
    eprintln!("📱 Fetching app store versions...");
    let versions = fetch_versions(&ctx, &app_id, args.platform.as_deref(), args.version.as_deref()).await?;

    for version in &versions {
        let version_id = version["id"].as_str().unwrap_or_default().to_string();
        let platform = version["attributes"]["platform"].as_str().unwrap_or("IOS").to_string();
        let version_string = version["attributes"]["versionString"].as_str().unwrap_or("0.0.0").to_string();
        let version_dir = output_root.join("versions").join(format!("{platform}_{version_string}"));
        ensure_dir(&version_dir)?;

        let version_attrs: asc_types::AppStoreVersionAttributes =
            serde_json::from_value(version["attributes"].clone())
                .context("failed to parse version attributes")?;
        write_yaml(&version_dir.join("app_store_version.yaml"), &version_attrs)?;
        eprintln!("  ✓ versions/{platform}_{version_string}/app_store_version.yaml");
        pulled_count += 1;

        // 5. Fetch version localizations
        let version_locs = fetch_version_localizations(&ctx, &version_id).await?;

        for vloc in &version_locs {
            let locale = vloc["attributes"]["locale"].as_str().unwrap_or("unknown").to_string();
            let vloc_dir = version_dir.join("localizations").join(&locale);
            ensure_dir(&vloc_dir)?;

            let vloc_attrs: asc_types::AppStoreVersionLocalizationAttributes =
                serde_json::from_value(vloc["attributes"].clone())
                    .context("failed to parse version localization attributes")?;
            write_yaml(&vloc_dir.join("app_store_version_localization.yaml"), &vloc_attrs)?;
            eprintln!("  ✓ versions/{platform}_{version_string}/localizations/{locale}/app_store_version_localization.yaml");
            pulled_count += 1;

            // 6. Fetch screenshot sets
            let vloc_id = vloc["id"].as_str().unwrap_or_default().to_string();
            let screenshot_sets = fetch_screenshot_sets(&ctx, &vloc_id).await?;

            for ss_set in &screenshot_sets {
                let ss_set_id = ss_set["id"].as_str().unwrap_or_default().to_string();
                let display_type = ss_set["attributes"]["screenshotDisplayType"]
                    .as_str().unwrap_or("UNKNOWN").to_string();
                let screenshots_dir = vloc_dir.join("screenshots").join(&display_type);
                ensure_dir(&screenshots_dir)?;

                let set_yaml: asc_types::AppScreenshotSetAttributes =
                    serde_json::from_value(ss_set["attributes"].clone())
                        .context("failed to parse screenshot set attributes")?;
                write_yaml(&screenshots_dir.join("screenshot_set.yaml"), &set_yaml)?;
                eprintln!("  ✓ .../screenshots/{display_type}/screenshot_set.yaml");
                pulled_count += 1;

                // 7. Fetch screenshots
                let screenshots = fetch_screenshots(&ctx, &ss_set_id).await?;

                for (idx, screenshot) in screenshots.iter().enumerate() {
                    let ss_id = screenshot["id"].as_str().unwrap_or_default().to_string();
                    let ss_attrs = screenshot["attributes"].as_object().cloned().unwrap_or_default();

                    let seq = idx + 1;
                    let file_name = ss_attrs.get("fileName").and_then(Value::as_str).unwrap_or("screenshot.png");
                    let extension = Path::new(file_name).extension().and_then(|e| e.to_str()).unwrap_or("png");
                    let screenshot_filename = format!("{:03}_{}.{}", seq, ss_id, extension);
                    let screenshot_path = screenshots_dir.join(&screenshot_filename);

                    if let Some(image_url) = resolve_image_url(&ss_attrs) {
                        match download_file(&ctx, &image_url, &screenshot_path).await {
                            Ok(_) => eprintln!("  ✓ .../screenshots/{display_type}/{screenshot_filename}"),
                            Err(e) => eprintln!("  ⚠ failed to download screenshot {ss_id}: {e}"),
                        }
                        pulled_count += 1;
                    } else {
                        eprintln!("  ⚠ screenshot {ss_id} has no downloadable URL (state: not ready)");
                        std::fs::write(&screenshot_path, "")?;
                    }
                }
            }

            // 8. Fetch preview sets
            let vloc_id = vloc["id"].as_str().unwrap_or_default().to_string();
            let preview_sets = fetch_preview_sets(&ctx, &vloc_id).await?;

            for pv_set in &preview_sets {
                let pv_set_id = pv_set["id"].as_str().unwrap_or_default().to_string();
                let preview_type = pv_set["attributes"]["previewType"]
                    .as_str().unwrap_or("UNKNOWN").to_string();
                let previews_dir = vloc_dir.join("previews").join(&preview_type);
                ensure_dir(&previews_dir)?;

                let set_yaml: asc_types::AppPreviewSetAttributes =
                    serde_json::from_value(pv_set["attributes"].clone())
                        .context("failed to parse preview set attributes")?;
                write_yaml(&previews_dir.join("preview_set.yaml"), &set_yaml)?;
                eprintln!("  ✓ .../previews/{preview_type}/preview_set.yaml");
                pulled_count += 1;

                // 9. Fetch previews
                let previews = fetch_previews(&ctx, &pv_set_id).await?;

                for (idx, preview) in previews.iter().enumerate() {
                    let pv_id = preview["id"].as_str().unwrap_or_default().to_string();
                    let pv_attrs = preview["attributes"].as_object().cloned().unwrap_or_default();

                    let seq = idx + 1;
                    let file_name = pv_attrs.get("fileName").and_then(Value::as_str).unwrap_or("preview.mov");
                    let extension = Path::new(file_name).extension().and_then(|e| e.to_str()).unwrap_or("mov");
                    let preview_filename = format!("{:03}_{}.{}", seq, pv_id, extension);
                    let preview_path = previews_dir.join(&preview_filename);

                    if let Some(video_url) = resolve_video_url(&pv_attrs) {
                        match download_file(&ctx, &video_url, &preview_path).await {
                            Ok(_) => eprintln!("  ✓ .../previews/{preview_type}/{preview_filename}"),
                            Err(e) => eprintln!("  ⚠ failed to download preview {pv_id}: {e}"),
                        }
                        pulled_count += 1;
                    } else {
                        eprintln!("  ⚠ preview {pv_id} has no downloadable URL (state: not ready)");
                        std::fs::write(&preview_path, "")?;
                    }
                }
            }
        }
    }

    let elapsed = start.elapsed();
    eprintln!("\n✅ Pull complete: {pulled_count} resources synced to {} in {elapsed:?}", output_root.display());
    Ok(())
}
