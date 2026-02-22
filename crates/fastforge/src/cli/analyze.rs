use anyhow::{Result, anyhow};
use app_analyzer::{
    AnalyzeConfig, AndroidAabAnalyzer, AndroidApkAnalyzer, AppAnalyzer, IOSIpaAnalyzer,
    MacOSDmgAnalyzer,
};
use clap::Args;
use serde_json::json;
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

    let output_json = match ext.as_deref() {
        Some("aab") => run_analyzer(AndroidAabAnalyzer::new(), path)?,
        Some("apk") => run_analyzer(AndroidApkAnalyzer::new(), path)?,
        Some("ipa") => run_analyzer(IOSIpaAnalyzer::new(), path)?,
        Some("dmg") => run_analyzer(MacOSDmgAnalyzer::new(), path)?,
        Some(other) => {
            return Err(anyhow!("Unsupported analyze file type: .{}", other));
        }
        None => {
            return Err(anyhow!("Analyze path has no file extension"));
        }
    };

    let serialized = serde_json::to_string_pretty(&output_json)?;

    if let Some(output_path) = &args.output {
        std::fs::write(output_path, serialized)?;
        log::info!("Analysis output written to {}", output_path);
    } else {
        println!("{}", serialized);
    }

    Ok(())
}

fn run_analyzer<A: AppAnalyzer>(analyzer: A, path: &str) -> Result<serde_json::Value> {
    if !analyzer.is_supported_on_current_platform() {
        return Err(anyhow!(
            "Analyzer {} is not supported on this platform",
            analyzer.name()
        ));
    }

    let config = AnalyzeConfig::new(path.to_string());
    let result = analyzer.analyze(config).map_err(|err| anyhow!("{}", err))?;

    Ok(json!({
        "success": result.success,
        "data": result.data,
    }))
}
