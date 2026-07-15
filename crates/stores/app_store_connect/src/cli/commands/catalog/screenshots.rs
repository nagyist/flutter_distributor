use crate::AppStoreConnectContext;
use anyhow::{Context, Result, anyhow};
use md5::{Digest, Md5};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const PROCESSING_TIMEOUT: Duration = Duration::from_secs(120);
const PROCESSING_POLL_INTERVAL: Duration = Duration::from_secs(2);
const MANIFEST_FILE: &str = ".fastforge.yaml";

#[derive(Debug)]
struct LocalScreenshot {
    path: PathBuf,
    file_name: String,
    bytes: Vec<u8>,
    checksum: String,
    pulled_checksum: Option<String>,
    remote_id_hint: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct ScreenshotManifest {
    #[serde(default)]
    screenshots: Vec<ScreenshotManifestEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct ScreenshotManifestEntry {
    pub file_name: String,
    pub remote_id: String,
    pub checksum: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ScreenshotResource {
    id: String,
    #[serde(default)]
    attributes: ScreenshotAttributes,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScreenshotAttributes {
    source_file_checksum: Option<String>,
    #[serde(default)]
    upload_operations: Vec<UploadOperation>,
    asset_delivery_state: Option<AssetDeliveryState>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct AssetDeliveryState {
    state: Option<String>,
    #[serde(default)]
    errors: Vec<Value>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadOperation {
    length: Option<i64>,
    method: Option<String>,
    offset: Option<i64>,
    #[serde(default)]
    request_headers: Vec<UploadHeader>,
    url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct UploadHeader {
    name: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResourceResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct ScreenshotSetResource {
    id: String,
    #[serde(default)]
    attributes: ScreenshotSetAttributes,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScreenshotSetAttributes {
    screenshot_display_type: Option<String>,
}

#[derive(Debug)]
enum DesiredScreenshot {
    Reuse {
        remote_id: String,
        verify_processing: bool,
    },
    Upload(usize),
}

#[derive(Debug)]
struct ScreenshotSyncPlan {
    desired: Vec<DesiredScreenshot>,
    delete_ids: Vec<String>,
}

#[derive(Debug, Default)]
pub(super) struct ScreenshotSyncSummary {
    pub reused: usize,
    pub uploaded: usize,
    pub deleted: usize,
}

pub(super) fn discover_screenshot_sets(localization_dir: &Path) -> Result<Vec<(String, usize)>> {
    let root = localization_dir.join("screenshots");
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut sets = Vec::new();
    for entry in std::fs::read_dir(&root).context("reading screenshot display types")? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let display_type = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_owned();
        if display_type.is_empty() {
            continue;
        }
        let files = screenshot_paths(&path)?;
        if !files.is_empty() {
            sets.push((display_type, files.len()));
        }
    }
    sets.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(sets)
}

pub(super) fn write_screenshot_manifest(
    display_dir: &Path,
    entries: &[ScreenshotManifestEntry],
) -> Result<()> {
    let manifest = ScreenshotManifest {
        screenshots: entries.to_vec(),
    };
    let content =
        serde_yaml::to_string(&manifest).context("failed to serialize screenshot manifest")?;
    std::fs::write(display_dir.join(MANIFEST_FILE), content).with_context(|| {
        format!(
            "failed to write screenshot manifest in {}",
            display_dir.display()
        )
    })
}

pub(super) fn prepare_screenshot_directory(display_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(display_dir)
        .with_context(|| format!("failed to create {}", display_dir.display()))?;
    for path in screenshot_paths(display_dir)? {
        std::fs::remove_file(&path)
            .with_context(|| format!("failed to remove stale screenshot {}", path.display()))?;
    }
    let manifest = display_dir.join(MANIFEST_FILE);
    if manifest.exists() {
        std::fs::remove_file(&manifest)
            .with_context(|| format!("failed to remove {}", manifest.display()))?;
    }
    Ok(())
}

pub(super) fn screenshot_checksum(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("failed to read screenshot {}", path.display()))?;
    Ok(format!("{:x}", Md5::digest(bytes)))
}

pub(super) async fn sync_localization_screenshots(
    ctx: &AppStoreConnectContext,
    localization_id: &str,
    localization_dir: &Path,
) -> Result<ScreenshotSyncSummary> {
    let mut summary = ScreenshotSyncSummary::default();

    for (display_type, _) in discover_screenshot_sets(localization_dir)? {
        let display_dir = localization_dir.join("screenshots").join(&display_type);
        let local = load_local_screenshots(&display_dir).await?;
        if local.is_empty() {
            continue;
        }

        eprintln!(
            "  syncing screenshots/{display_type} ({} files)...",
            local.len()
        );
        let set_id = get_or_create_screenshot_set(ctx, localization_id, &display_type).await?;
        let remote = fetch_screenshots(ctx, &set_id).await?;
        let plan = build_sync_plan(&local, &remote);

        for screenshot_id in &plan.delete_ids {
            delete_screenshot(ctx, screenshot_id).await?;
            eprintln!("    − removed remote screenshot {screenshot_id}");
            summary.deleted += 1;
        }

        let mut desired_ids = Vec::with_capacity(plan.desired.len());
        let mut verify_ids = Vec::new();
        for desired in plan.desired {
            match desired {
                DesiredScreenshot::Reuse {
                    remote_id,
                    verify_processing,
                } => {
                    eprintln!("    • unchanged {remote_id}");
                    if verify_processing {
                        verify_ids.push(remote_id.clone());
                    }
                    desired_ids.push(remote_id);
                    summary.reused += 1;
                }
                DesiredScreenshot::Upload(local_index) => {
                    let screenshot = &local[local_index];
                    let remote_id = upload_screenshot(ctx, &set_id, screenshot).await?;
                    eprintln!("    + uploaded {} ({remote_id})", screenshot.file_name);
                    verify_ids.push(remote_id.clone());
                    desired_ids.push(remote_id);
                    summary.uploaded += 1;
                }
            }
        }

        wait_for_processing(ctx, &verify_ids).await?;
        replace_screenshot_order(ctx, &set_id, &desired_ids).await?;
    }

    Ok(summary)
}

fn screenshot_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("reading screenshots from {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file()
            || path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with('.'))
        {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !matches!(extension.as_str(), "png" | "jpg" | "jpeg") {
            return Err(anyhow!(
                "unsupported screenshot file {}; expected PNG or JPEG",
                path.display()
            ));
        }
        files.push(path);
    }
    files.sort();
    Ok(files)
}

async fn load_local_screenshots(dir: &Path) -> Result<Vec<LocalScreenshot>> {
    let manifest = read_screenshot_manifest(dir)?;
    let manifest_by_file: HashMap<_, _> = manifest
        .screenshots
        .into_iter()
        .map(|entry| (entry.file_name.clone(), entry))
        .collect();
    let mut screenshots = Vec::new();
    for path in screenshot_paths(dir)? {
        let bytes = tokio::fs::read(&path)
            .await
            .with_context(|| format!("failed to read {}", path.display()))?;
        if bytes.is_empty() {
            return Err(anyhow!("screenshot file is empty: {}", path.display()));
        }
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| anyhow!("screenshot filename is not valid UTF-8: {}", path.display()))?
            .to_owned();
        let checksum = format!("{:x}", Md5::digest(&bytes));
        let manifest_entry = manifest_by_file.get(&file_name);
        screenshots.push(LocalScreenshot {
            remote_id_hint: manifest_entry
                .map(|entry| entry.remote_id.clone())
                .or_else(|| remote_id_from_filename(&file_name)),
            pulled_checksum: manifest_entry.map(|entry| entry.checksum.clone()),
            checksum,
            path,
            file_name,
            bytes,
        });
    }
    Ok(screenshots)
}

fn read_screenshot_manifest(dir: &Path) -> Result<ScreenshotManifest> {
    let path = dir.join(MANIFEST_FILE);
    if !path.exists() {
        return Ok(ScreenshotManifest::default());
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read screenshot manifest {}", path.display()))?;
    serde_yaml::from_str(&content)
        .with_context(|| format!("failed to parse screenshot manifest {}", path.display()))
}

fn remote_id_from_filename(file_name: &str) -> Option<String> {
    let stem = Path::new(file_name).file_stem()?.to_str()?;
    let (_, remote_id) = stem.split_once('_')?;
    (!remote_id.is_empty()).then(|| remote_id.to_owned())
}

fn remote_state(remote: &ScreenshotResource) -> Option<&str> {
    remote
        .attributes
        .asset_delivery_state
        .as_ref()?
        .state
        .as_deref()
}

fn is_reusable(remote: &ScreenshotResource) -> bool {
    matches!(remote_state(remote), Some("COMPLETE" | "UPLOAD_COMPLETE"))
}

fn build_sync_plan(local: &[LocalScreenshot], remote: &[ScreenshotResource]) -> ScreenshotSyncPlan {
    let mut reused_remote = HashSet::new();
    let mut desired = Vec::with_capacity(local.len());

    for (local_index, screenshot) in local.iter().enumerate() {
        let checksum_match = remote.iter().find(|candidate| {
            !reused_remote.contains(&candidate.id)
                && is_reusable(candidate)
                && candidate
                    .attributes
                    .source_file_checksum
                    .as_deref()
                    .is_some_and(|checksum| checksum.eq_ignore_ascii_case(&screenshot.checksum))
        });

        let id_match = screenshot.remote_id_hint.as_deref().and_then(|id| {
            remote
                .iter()
                .find(|candidate| candidate.id == id && !reused_remote.contains(&candidate.id))
        });

        let reusable = checksum_match.or_else(|| {
            id_match.filter(|candidate| {
                is_reusable(candidate)
                    && (candidate
                        .attributes
                        .source_file_checksum
                        .as_deref()
                        .is_some_and(|checksum| {
                            checksum.eq_ignore_ascii_case(&screenshot.checksum)
                        })
                        || screenshot
                            .pulled_checksum
                            .as_deref()
                            .is_some_and(|checksum| {
                                checksum.eq_ignore_ascii_case(&screenshot.checksum)
                            }))
            })
        });

        if let Some(candidate) = reusable {
            reused_remote.insert(candidate.id.clone());
            desired.push(DesiredScreenshot::Reuse {
                remote_id: candidate.id.clone(),
                verify_processing: remote_state(candidate) == Some("UPLOAD_COMPLETE"),
            });
        } else {
            desired.push(DesiredScreenshot::Upload(local_index));
        }
    }

    let delete_ids = remote
        .iter()
        .filter(|candidate| !reused_remote.contains(&candidate.id))
        .map(|candidate| candidate.id.clone())
        .collect();

    ScreenshotSyncPlan {
        desired,
        delete_ids,
    }
}

async fn get_or_create_screenshot_set(
    ctx: &AppStoreConnectContext,
    localization_id: &str,
    display_type: &str,
) -> Result<String> {
    let response = ctx
        .http
        .get(ctx.url(&format!(
            "/v1/appStoreVersionLocalizations/{localization_id}/appScreenshotSets"
        )))
        .query(&[
            ("filter[screenshotDisplayType]", display_type),
            ("limit", "200"),
        ])
        .send()
        .await
        .context("failed to list App Store screenshot sets")?;
    let response = checked(response, "list App Store screenshot sets").await?;
    let sets: ResourceResponse<Vec<ScreenshotSetResource>> = response
        .json()
        .await
        .context("failed to decode App Store screenshot sets")?;
    let matching: Vec<_> = sets
        .data
        .into_iter()
        .filter(|set| set.attributes.screenshot_display_type.as_deref() == Some(display_type))
        .collect();
    if matching.len() > 1 {
        return Err(anyhow!(
            "multiple App Store screenshot sets found for display type {display_type}"
        ));
    }
    if let Some(set) = matching.into_iter().next() {
        return Ok(set.id);
    }

    let response = ctx
        .http
        .post(ctx.url("/v1/appScreenshotSets"))
        .json(&json!({
            "data": {
                "type": "appScreenshotSets",
                "attributes": { "screenshotDisplayType": display_type },
                "relationships": {
                    "appStoreVersionLocalization": {
                        "data": {
                            "type": "appStoreVersionLocalizations",
                            "id": localization_id
                        }
                    }
                }
            }
        }))
        .send()
        .await
        .context("failed to create App Store screenshot set")?;
    let response = checked(response, "create App Store screenshot set").await?;
    let set: ResourceResponse<ScreenshotSetResource> = response
        .json()
        .await
        .context("failed to decode created App Store screenshot set")?;
    eprintln!(
        "    + created screenshot set {display_type} ({})",
        set.data.id
    );
    Ok(set.data.id)
}

async fn fetch_screenshots(
    ctx: &AppStoreConnectContext,
    set_id: &str,
) -> Result<Vec<ScreenshotResource>> {
    let response = ctx
        .http
        .get(ctx.url(&format!("/v1/appScreenshotSets/{set_id}/appScreenshots")))
        .query(&[
            (
                "fields[appScreenshots]",
                "sourceFileChecksum,uploadOperations,assetDeliveryState",
            ),
            ("limit", "200"),
        ])
        .send()
        .await
        .context("failed to list App Store screenshots")?;
    let response = checked(response, "list App Store screenshots").await?;
    let screenshots: ResourceResponse<Vec<ScreenshotResource>> = response
        .json()
        .await
        .context("failed to decode App Store screenshots")?;
    Ok(screenshots.data)
}

async fn delete_screenshot(ctx: &AppStoreConnectContext, screenshot_id: &str) -> Result<()> {
    let response = ctx
        .http
        .delete(ctx.url(&format!("/v1/appScreenshots/{screenshot_id}")))
        .send()
        .await
        .with_context(|| format!("failed to delete App Store screenshot {screenshot_id}"))?;
    checked(response, "delete App Store screenshot").await?;
    Ok(())
}

async fn upload_screenshot(
    ctx: &AppStoreConnectContext,
    set_id: &str,
    screenshot: &LocalScreenshot,
) -> Result<String> {
    let file_size =
        i64::try_from(screenshot.bytes.len()).context("screenshot file is too large to upload")?;
    let response = ctx
        .http
        .post(ctx.url("/v1/appScreenshots"))
        .json(&json!({
            "data": {
                "type": "appScreenshots",
                "attributes": {
                    "fileSize": file_size,
                    "fileName": screenshot.file_name
                },
                "relationships": {
                    "appScreenshotSet": {
                        "data": { "type": "appScreenshotSets", "id": set_id }
                    }
                }
            }
        }))
        .send()
        .await
        .with_context(|| format!("failed to reserve upload for {}", screenshot.path.display()))?;
    let response = checked(response, "reserve App Store screenshot upload").await?;
    let reservation: ResourceResponse<ScreenshotResource> = response
        .json()
        .await
        .context("failed to decode App Store screenshot upload reservation")?;

    let screenshot_id = reservation.data.id.clone();
    let upload_result = async {
        upload_parts(
            &screenshot.bytes,
            &reservation.data.attributes.upload_operations,
        )
        .await?;
        commit_screenshot(ctx, &screenshot_id, &screenshot.checksum).await
    }
    .await;

    if let Err(error) = upload_result {
        let _ = delete_screenshot(ctx, &screenshot_id).await;
        return Err(error);
    }

    Ok(screenshot_id)
}

async fn upload_parts(bytes: &[u8], operations: &[UploadOperation]) -> Result<()> {
    if operations.is_empty() {
        return Err(anyhow!(
            "App Store returned no screenshot upload operations"
        ));
    }

    // Upload URLs are pre-signed and must not receive the App Store JWT.
    let client = reqwest::Client::new();
    for operation in operations {
        let offset = usize::try_from(
            operation
                .offset
                .ok_or_else(|| anyhow!("upload operation is missing offset"))?,
        )
        .context("upload operation has a negative offset")?;
        let length = usize::try_from(
            operation
                .length
                .ok_or_else(|| anyhow!("upload operation is missing length"))?,
        )
        .context("upload operation has a negative length")?;
        let end = offset
            .checked_add(length)
            .filter(|end| *end <= bytes.len())
            .ok_or_else(|| anyhow!("upload operation byte range exceeds screenshot size"))?;
        let method = Method::from_bytes(
            operation
                .method
                .as_deref()
                .ok_or_else(|| anyhow!("upload operation is missing method"))?
                .as_bytes(),
        )
        .context("upload operation contains an invalid HTTP method")?;
        let url = operation
            .url
            .as_deref()
            .ok_or_else(|| anyhow!("upload operation is missing URL"))?;
        let mut request = client.request(method, url);
        for header in &operation.request_headers {
            if let (Some(name), Some(value)) = (&header.name, &header.value) {
                request = request.header(name, value);
            }
        }
        let response = request
            .body(bytes[offset..end].to_vec())
            .send()
            .await
            .context("failed to upload an App Store screenshot part")?;
        checked(response, "upload App Store screenshot part").await?;
    }
    Ok(())
}

async fn commit_screenshot(
    ctx: &AppStoreConnectContext,
    screenshot_id: &str,
    checksum: &str,
) -> Result<()> {
    let response = ctx
        .http
        .patch(ctx.url(&format!("/v1/appScreenshots/{screenshot_id}")))
        .json(&json!({
            "data": {
                "type": "appScreenshots",
                "id": screenshot_id,
                "attributes": {
                    "uploaded": true,
                    "sourceFileChecksum": checksum
                }
            }
        }))
        .send()
        .await
        .context("failed to commit App Store screenshot upload")?;
    checked(response, "commit App Store screenshot upload").await?;
    Ok(())
}

async fn replace_screenshot_order(
    ctx: &AppStoreConnectContext,
    set_id: &str,
    screenshot_ids: &[String],
) -> Result<()> {
    let data: Vec<_> = screenshot_ids
        .iter()
        .map(|id| json!({ "type": "appScreenshots", "id": id }))
        .collect();
    let response = ctx
        .http
        .patch(ctx.url(&format!(
            "/v1/appScreenshotSets/{set_id}/relationships/appScreenshots"
        )))
        .json(&json!({ "data": data }))
        .send()
        .await
        .context("failed to update App Store screenshot order")?;
    checked(response, "update App Store screenshot order").await?;
    Ok(())
}

async fn wait_for_processing(
    ctx: &AppStoreConnectContext,
    screenshot_ids: &[String],
) -> Result<()> {
    let mut pending: HashSet<String> = screenshot_ids.iter().cloned().collect();
    if pending.is_empty() {
        return Ok(());
    }

    let started = Instant::now();
    while !pending.is_empty() {
        let ids: Vec<_> = pending.iter().cloned().collect();
        for screenshot_id in ids {
            let screenshot = fetch_screenshot(ctx, &screenshot_id).await?;
            match remote_state(&screenshot) {
                Some("COMPLETE") => {
                    pending.remove(&screenshot_id);
                }
                Some("FAILED") => {
                    let errors = screenshot
                        .attributes
                        .asset_delivery_state
                        .as_ref()
                        .map(|state| &state.errors);
                    let _ = delete_screenshot(ctx, &screenshot_id).await;
                    return Err(anyhow!(
                        "App Store failed to process screenshot {screenshot_id}: {}",
                        serde_json::to_string(errors.unwrap_or(&Vec::new()))?
                    ));
                }
                _ => {}
            }
        }

        if pending.is_empty() {
            break;
        }
        if started.elapsed() >= PROCESSING_TIMEOUT {
            return Err(anyhow!(
                "timed out waiting for App Store to process screenshots: {}",
                pending.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        tokio::time::sleep(PROCESSING_POLL_INTERVAL).await;
    }
    Ok(())
}

async fn fetch_screenshot(
    ctx: &AppStoreConnectContext,
    screenshot_id: &str,
) -> Result<ScreenshotResource> {
    let response = ctx
        .http
        .get(ctx.url(&format!("/v1/appScreenshots/{screenshot_id}")))
        .query(&[(
            "fields[appScreenshots]",
            "sourceFileChecksum,assetDeliveryState",
        )])
        .send()
        .await
        .context("failed to read App Store screenshot processing state")?;
    let response = checked(response, "read App Store screenshot processing state").await?;
    let screenshot: ResourceResponse<ScreenshotResource> = response
        .json()
        .await
        .context("failed to decode App Store screenshot processing state")?;
    Ok(screenshot.data)
}

async fn checked(response: Response, operation: &str) -> Result<Response> {
    if response.status().is_success() {
        return Ok(response);
    }
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    Err(anyhow!("{operation} failed: {status}\n{body}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local(name: &str, checksum: &str) -> LocalScreenshot {
        LocalScreenshot {
            path: PathBuf::from(name),
            file_name: name.to_owned(),
            bytes: vec![1],
            checksum: checksum.to_owned(),
            pulled_checksum: None,
            remote_id_hint: remote_id_from_filename(name),
        }
    }

    fn remote(id: &str, checksum: Option<&str>, state: &str) -> ScreenshotResource {
        ScreenshotResource {
            id: id.to_owned(),
            attributes: ScreenshotAttributes {
                source_file_checksum: checksum.map(str::to_owned),
                asset_delivery_state: Some(AssetDeliveryState {
                    state: Some(state.to_owned()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }

    #[test]
    fn extracts_remote_id_from_pulled_filename() {
        assert_eq!(
            remote_id_from_filename("001_abc-123.png").as_deref(),
            Some("abc-123")
        );
        assert_eq!(remote_id_from_filename("screenshot.png"), None);
    }

    #[test]
    fn reuses_complete_screenshot_with_matching_checksum() {
        let plan = build_sync_plan(
            &[local("001_remote-id.png", "abc")],
            &[remote("remote-id", Some("ABC"), "COMPLETE")],
        );

        assert!(matches!(
            plan.desired.as_slice(),
            [DesiredScreenshot::Reuse {
                remote_id,
                verify_processing: false
            }] if remote_id == "remote-id"
        ));
        assert!(plan.delete_ids.is_empty());
    }

    #[test]
    fn reuses_checksum_match_even_when_file_was_renamed() {
        let plan = build_sync_plan(
            &[local("new-name.png", "abc")],
            &[remote("remote-id", Some("abc"), "COMPLETE")],
        );

        assert!(matches!(
            plan.desired.as_slice(),
            [DesiredScreenshot::Reuse { remote_id, .. }] if remote_id == "remote-id"
        ));
        assert!(plan.delete_ids.is_empty());
    }

    #[test]
    fn replaces_changed_and_failed_screenshots() {
        let plan = build_sync_plan(
            &[
                local("001_changed.png", "new"),
                local("002_failed.png", "same"),
            ],
            &[
                remote("changed", Some("old"), "COMPLETE"),
                remote("failed", Some("same"), "FAILED"),
            ],
        );

        assert!(matches!(plan.desired[0], DesiredScreenshot::Upload(0)));
        assert!(matches!(plan.desired[1], DesiredScreenshot::Upload(1)));
        assert_eq!(plan.delete_ids, vec!["changed", "failed"]);
    }

    #[test]
    fn manifest_prevents_reupload_of_unchanged_processed_download() {
        let mut screenshot = local("001_remote-id.png", "downloaded");
        screenshot.pulled_checksum = Some("downloaded".to_owned());
        let plan = build_sync_plan(
            &[screenshot],
            &[remote("remote-id", Some("original"), "COMPLETE")],
        );

        assert!(matches!(
            plan.desired.as_slice(),
            [DesiredScreenshot::Reuse { remote_id, .. }] if remote_id == "remote-id"
        ));
        assert!(plan.delete_ids.is_empty());
    }

    #[test]
    fn edited_manifest_file_is_replaced() {
        let mut screenshot = local("001_remote-id.png", "edited");
        screenshot.pulled_checksum = Some("downloaded".to_owned());
        let plan = build_sync_plan(
            &[screenshot],
            &[remote("remote-id", Some("original"), "COMPLETE")],
        );

        assert!(matches!(
            plan.desired.as_slice(),
            [DesiredScreenshot::Upload(0)]
        ));
        assert_eq!(plan.delete_ids, vec!["remote-id"]);
    }

    #[test]
    fn removes_remote_screenshots_missing_locally() {
        let plan = build_sync_plan(
            &[local("001_keep.png", "keep")],
            &[
                remote("keep", Some("keep"), "COMPLETE"),
                remote("remove", Some("remove"), "COMPLETE"),
            ],
        );

        assert_eq!(plan.delete_ids, vec!["remove"]);
    }

    #[test]
    fn keeps_upload_complete_screenshot_and_requests_verification() {
        let plan = build_sync_plan(
            &[local("001_remote-id.png", "abc")],
            &[remote("remote-id", Some("abc"), "UPLOAD_COMPLETE")],
        );

        assert!(matches!(
            plan.desired.as_slice(),
            [DesiredScreenshot::Reuse {
                verify_processing: true,
                ..
            }]
        ));
    }
}
