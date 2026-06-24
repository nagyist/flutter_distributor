use anyhow::{Result, anyhow};
use clap::Args;
use fastforge_app_analyzer::{
    AnalyzeConfig, AndroidAabAnalyzer, AndroidApkAnalyzer, AppAnalyzer, IOSIpaAnalyzer,
    MacOSAppAnalyzer, MacOSDmgAnalyzer,
};
use std::path::Path;

#[derive(Args)]
pub struct AnalyzeArgs {
    #[arg(value_name = "PATH")]
    pub path: String,
    #[arg(short, long = "output")]
    pub output: Option<String>,
}

pub async fn execute(args: &AnalyzeArgs) -> Result<()> {
    log::info!("Executing analyze command");
    let path = args.path.trim();
    if path.is_empty() {
        return Err(anyhow!("Analyze path cannot be empty"));
    }

    let ext = Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_lowercase());

    let config = AnalyzeConfig::new(path.to_string());
    let info = match ext.as_deref() {
        Some("apk") => AndroidApkAnalyzer::new().analyze(config)?,
        Some("aab") => AndroidAabAnalyzer::new().analyze(config)?,
        Some("ipa") => IOSIpaAnalyzer::new().analyze(config)?,
        Some("dmg") => MacOSDmgAnalyzer::new().analyze(config)?,
        Some("app") => MacOSAppAnalyzer::new().analyze(config)?,
        Some(ext) => return Err(anyhow!("Unsupported file extension: .{}", ext)),
        None => return Err(anyhow!("Unable to determine file extension")),
    };
    let output_json = info.data;

    let formatted = serde_json::to_string_pretty(&output_json)?;
    match &args.output {
        Some(output) => std::fs::write(output, &formatted)?,
        None => println!("{}", formatted),
    }

    Ok(())
}
