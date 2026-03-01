use anyhow::{Result, anyhow};
use app_publisher::{AppPublisher, PublishConfig, S3Publisher};
use clap::Args;
use std::collections::HashMap;

#[derive(Args)]
pub struct PublishArgs {
    #[arg(long = "path")]
    pub path: Option<String>,
    #[arg(short, long = "target")]
    pub target: Option<String>,
    #[arg(long = "publish-arg", value_name = "KEY=VALUE")]
    pub publish_args: Vec<String>,
}

pub async fn execute(args: &PublishArgs) -> Result<()> {
    log::info!("Executing publish command");

    let artifact_path = args
        .path
        .clone()
        .ok_or_else(|| anyhow!("The 'path' option is mandatory!"))?;

    let target = args
        .target
        .as_deref()
        .ok_or_else(|| anyhow!("The 'target' option is mandatory!"))?
        .to_ascii_lowercase();

    let publish_arguments = parse_publish_args(&args.publish_args)?;
    let publish_config = PublishConfig {
        app_version: None,
        artifact_path: Some(artifact_path),
        publish_arguments: if publish_arguments.is_empty() {
            None
        } else {
            Some(publish_arguments)
        },
    };

    let result = match target.as_str() {
        "s3" | "minio" => S3Publisher::new().publish(publish_config, None),
        _ => {
            return Err(anyhow!(
                "Unsupported publish target: `{}`. Currently supported: s3",
                target
            ));
        }
    }
    .map_err(|e| anyhow!(e.to_string()))?;

    println!("{}", result.message);
    Ok(())
}

fn parse_publish_args(items: &[String]) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for item in items {
        let (key, value) = item
            .split_once('=')
            .ok_or_else(|| anyhow!("Invalid --publish-arg item: `{item}`; expected KEY=VALUE"))?;
        let key = key.trim();
        if key.is_empty() {
            return Err(anyhow!(
                "Invalid --publish-arg item: `{item}`; key cannot be empty"
            ));
        }
        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
}
