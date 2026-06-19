use anyhow::{Result, anyhow};
use clap::Args;
use fastforge_app_publisher::{
    AppGalleryPublisher, AppPublisher, AppStorePublisher, CosPublisher, CustomPublisher,
    FirPublisher, FirebaseHostingPublisher, FirebasePublisher, GitHubPublisher, OssPublisher,
    PublishConfig, QiniuPublisher, S3Publisher, VercelPublisher,
};
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
    let message = publish_artifact(&artifact_path, &target, publish_arguments)?;
    println!("{}", message);
    Ok(())
}

pub fn publish_artifact(
    artifact_path: &str,
    target: &str,
    publish_arguments: HashMap<String, String>,
) -> Result<String> {
    let target = target.to_ascii_lowercase();
    let publish_config = PublishConfig {
        app_version: None,
        artifact_path: Some(artifact_path.to_string()),
        publish_arguments: if publish_arguments.is_empty() {
            None
        } else {
            Some(publish_arguments)
        },
    };

    let result = match target.as_str() {
        "s3" | "minio" => S3Publisher::new().publish(publish_config, None),
        "qiniu" => QiniuPublisher::new().publish(publish_config, None),
        "oss" => OssPublisher::new().publish(publish_config, None),
        "cos" => CosPublisher::new().publish(publish_config, None),
        "fir" => FirPublisher::new().publish(publish_config, None),
        "firebase" => FirebasePublisher::new().publish(publish_config, None),
        "firebase-hosting" => FirebaseHostingPublisher::new().publish(publish_config, None),
        "github" => GitHubPublisher::new().publish(publish_config, None),
        "appstore" => AppStorePublisher::new().publish(publish_config, None),
        "appgallery" => AppGalleryPublisher::new().publish(publish_config, None),
        "vercel" => VercelPublisher::new().publish(publish_config, None),
        "custom" => CustomPublisher::new().publish(publish_config, None),
        _ => {
            return Err(anyhow!(
                "Unsupported publish target: `{}`. Currently supported: s3, qiniu, oss, cos, fir, firebase, firebase-hosting, github, appstore, appgallery, vercel, custom",
                target
            ));
        }
    }
    .map_err(|e| anyhow!(e.to_string()))?;

    Ok(result.message)
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
