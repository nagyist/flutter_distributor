use crate::cli::GlobalArgs;
use crate::cli::commands::app::resolve_app;
use crate::cli::commands::build::{get_build, latest_build};
use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use crate::{AppStoreConnectContext, print_json, print_table};
use serde::Serialize;
use serde_json::{Value, json};
use std::time::{Duration, Instant};

#[derive(Args, Debug)]
pub struct VersionArgs {
    #[command(subcommand)]
    pub command: VersionCommand,
}

#[derive(Subcommand, Debug)]
pub enum VersionCommand {
    List(VersionListArgs),
    View(VersionViewArgs),
    Submit(VersionSubmitArgs),
}

#[derive(Args, Debug)]
pub struct VersionListArgs {
    #[arg(long = "app")]
    pub app: String,
}

#[derive(Args, Debug)]
pub struct VersionViewArgs {
    #[arg(value_name = "VERSION")]
    pub version: String,
    #[arg(long = "app")]
    pub app: String,
}

#[derive(Args, Debug)]
pub struct VersionSubmitArgs {
    #[arg(value_name = "VERSION")]
    pub version: String,
    #[arg(long = "app")]
    pub app: String,
    #[arg(long = "build")]
    pub build: String,
    #[arg(long = "wait", default_value_t = false)]
    pub wait: bool,
}

#[derive(Serialize, Clone)]
pub struct VersionRow {
    pub id: String,
    pub version: String,
    pub platform: String,
    pub state: String,
}

#[derive(Serialize)]
struct SubmissionRow {
    app: String,
    version: String,
    #[serde(rename = "buildId")]
    build_id: String,
    state: String,
}

pub async fn execute(args: &VersionArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = AppStoreConnectContext::from_env()?;
    match &args.command {
        VersionCommand::List(args) => list(&ctx, args, global).await,
        VersionCommand::View(args) => view(&ctx, args, global).await,
        VersionCommand::Submit(args) => submit(&ctx, args, global).await,
    }
}

async fn list(
    ctx: &AppStoreConnectContext,
    args: &VersionListArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let app = resolve_app(ctx, &args.app).await?;
    let versions = list_versions(ctx, &app.id, global.limit).await?;
    print_versions(versions, global)
}

async fn view(
    ctx: &AppStoreConnectContext,
    args: &VersionViewArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let app = resolve_app(ctx, &args.app).await?;
    let version = resolve_version(ctx, &app.id, &args.version).await?;
    print_versions(vec![version], global)
}

async fn submit(
    ctx: &AppStoreConnectContext,
    args: &VersionSubmitArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let app = resolve_app(ctx, &args.app).await?;
    let version = resolve_version(ctx, &app.id, &args.version).await?;
    let build = if args.build == "latest" {
        latest_build(ctx, &app.id, Some(&version.version)).await?
    } else {
        get_build(ctx, &args.build).await?
    };

    set_version_build(ctx, &version.id, &build.id).await?;
    let submission = create_submission(ctx, &version.id).await?;
    let state = submission_state(&submission);

    let mut row = SubmissionRow {
        app: app.bundle_id,
        version: version.version,
        build_id: build.id,
        state,
    };

    if args.wait {
        row.state =
            wait_for_submission(ctx, submission_id(&submission)?, Duration::from_secs(1800))
                .await?;
    }

    if global.json.is_some() {
        return print_json(&row, global.json.as_deref());
    }
    println!(
        "Submitted version {} for {} with build {}",
        row.version, row.app, row.build_id
    );
    println!("State: {}", row.state);
    Ok(())
}

pub async fn list_versions(
    ctx: &AppStoreConnectContext,
    app_id: &str,
    limit: Option<i64>,
) -> Result<Vec<VersionRow>> {
    let mut request = ctx
        .http
        .get(ctx.url(&format!("/v1/apps/{app_id}/appStoreVersions")))
        .query(&[("sort", "-createdDate")]);
    if let Some(limit) = limit {
        request = request.query(&[("limit", limit)]);
    }
    let response: Value = request.send().await?.error_for_status()?.json().await?;
    response
        .get("data")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .map(version_row)
        .collect()
}

pub async fn resolve_version(
    ctx: &AppStoreConnectContext,
    app_id: &str,
    version: &str,
) -> Result<VersionRow> {
    if version.chars().all(|c| c.is_ascii_digit()) {
        let response = ctx
            .http
            .get(ctx.url(&format!("/v1/appStoreVersions/{version}")))
            .send()
            .await?;
        if response.status().is_success() {
            let value: Value = response.json().await?;
            return version_row(
                value
                    .get("data")
                    .ok_or_else(|| anyhow!("missing version data"))?,
            );
        }
    }

    let versions = list_versions(ctx, app_id, Some(200)).await?;
    let matches = versions
        .into_iter()
        .filter(|row| row.version == version)
        .collect::<Vec<_>>();
    match matches.len() {
        0 => Err(anyhow!("version `{version}` not found")),
        1 => Ok(matches.into_iter().next().unwrap()),
        _ => Err(anyhow!("version `{version}` matched multiple records")),
    }
}

async fn set_version_build(
    ctx: &AppStoreConnectContext,
    version_id: &str,
    build_id: &str,
) -> Result<()> {
    let body = json!({
        "data": {
            "type": "builds",
            "id": build_id
        }
    });
    ctx.http
        .patch(ctx.url(&format!(
            "/v1/appStoreVersions/{version_id}/relationships/build"
        )))
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

async fn create_submission(ctx: &AppStoreConnectContext, version_id: &str) -> Result<Value> {
    let body = json!({
        "data": {
            "type": "appStoreVersionSubmissions",
            "relationships": {
                "appStoreVersion": {
                    "data": {
                        "type": "appStoreVersions",
                        "id": version_id
                    }
                }
            }
        }
    });
    Ok(ctx
        .http
        .post(ctx.url("/v1/appStoreVersionSubmissions"))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

async fn wait_for_submission(
    ctx: &AppStoreConnectContext,
    submission_id: &str,
    timeout: Duration,
) -> Result<String> {
    let start = Instant::now();
    loop {
        let response: Value = ctx
            .http
            .get(ctx.url(&format!("/v1/appStoreVersionSubmissions/{submission_id}")))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let state = submission_state(&response);
        match state.as_str() {
            "COMPLETE" | "ACCEPTED" | "SUBMITTED" => return Ok(state),
            "FAILED" | "REJECTED" => {
                return Err(anyhow!(
                    "submission {submission_id} failed with state {state}"
                ));
            }
            _ if start.elapsed() >= timeout => {
                return Err(anyhow!(
                    "submission {submission_id} timed out after {:?}",
                    timeout
                ));
            }
            _ => tokio::time::sleep(Duration::from_secs(30)).await,
        }
    }
}

fn print_versions(versions: Vec<VersionRow>, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&versions, global.json.as_deref());
    }
    let rows = versions
        .into_iter()
        .map(|version| vec![version.id, version.version, version.platform, version.state])
        .collect::<Vec<_>>();
    print_table(&["ID", "VERSION", "PLATFORM", "STATE"], &rows);
    Ok(())
}

fn version_row(value: &Value) -> Result<VersionRow> {
    let attrs = value
        .get("attributes")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("missing version attributes"))?;
    Ok(VersionRow {
        id: value
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        version: attr_string(attrs, "versionString"),
        platform: attr_string(attrs, "platform"),
        state: attr_string(attrs, "appVersionState"),
    })
}

fn submission_id(value: &Value) -> Result<&str> {
    value
        .get("data")
        .and_then(|data| data.get("id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("missing submission id"))
}

fn submission_state(value: &Value) -> String {
    value
        .get("data")
        .and_then(|data| data.get("attributes"))
        .and_then(|attrs| attrs.get("state"))
        .and_then(Value::as_str)
        .unwrap_or("SUBMITTED")
        .to_string()
}

fn attr_string(attrs: &serde_json::Map<String, Value>, key: &str) -> String {
    attrs
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}
