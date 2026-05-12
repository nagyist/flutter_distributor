use fastforge_core::{AnalyzeConfig, AnalyzeError, AnalyzeResult, AppAnalyzer};
use plist::Value;
use serde_json::json;
use std::fs::File;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub struct IOSIpaAnalyzer;

impl AppAnalyzer for IOSIpaAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        "ios-ipa"
    }

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        let file = File::open(&config.path)
            .map_err(AnalyzeError::Io)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| AnalyzeError::Parse(format!("Invalid ipa zip archive: {}", e)))?;

        let mut plist_bytes: Option<Vec<u8>> = None;
        for index in 0..archive.len() {
            let mut entry = archive
                .by_index(index)
                .map_err(|e| AnalyzeError::Parse(format!("Failed to read ipa entry: {}", e)))?;

            if !entry.is_file() || !entry.name().ends_with(".app/Info.plist") {
                continue;
            }

            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .map_err(AnalyzeError::Io)?;
            plist_bytes = Some(buf);
            break;
        }

        let plist_bytes = plist_bytes.ok_or_else(|| AnalyzeError::Parse("Can't parse .ipa file.".to_string()))?;
        let plist_value = Value::from_reader(Cursor::new(plist_bytes))
            .map_err(|e| AnalyzeError::Parse(format!("Failed to parse Info.plist: {}", e)))?;

        let plist_dict = plist_value
            .as_dictionary()
            .ok_or_else(|| AnalyzeError::Parse("Info.plist root is not a dictionary".to_string()))?;

        let identifier = read_required_plist_string(plist_dict, "CFBundleIdentifier")?;
        let name = read_optional_plist_string(plist_dict, "CFBundleDisplayName")
            .or_else(|| read_optional_plist_string(plist_dict, "CFBundleName"))
            .ok_or_else(|| {
                AnalyzeError::Parse("Missing CFBundleDisplayName/CFBundleName in Info.plist".to_string())
            })?;
        let version = read_required_plist_string(plist_dict, "CFBundleShortVersionString")?;
        let build_number_raw = read_required_plist_string(plist_dict, "CFBundleVersion")?;
        let build_number = build_number_raw
            .parse::<i32>()
            .map_err(|_| AnalyzeError::Parse("Failed to parse CFBundleVersion as integer".to_string()))?;

        let data = json!({
            "platform": "ios",
            "identifier": identifier,
            "name": name,
            "version": version,
            "buildNumber": build_number
        });

        log::info!("IPA analysis completed for {}", config.path);
        Ok(AnalyzeResult::new(true, data))
    }
}

fn read_required_plist_string(dict: &plist::Dictionary, key: &str) -> Result<String, AnalyzeError> {
    read_optional_plist_string(dict, key)
        .ok_or_else(|| AnalyzeError::Parse(format!("Missing {} in Info.plist", key)))
}

fn read_optional_plist_string(dict: &plist::Dictionary, key: &str) -> Option<String> {
    dict.get(key)
        .and_then(|value| value.as_string())
        .map(|value| value.to_string())
}
