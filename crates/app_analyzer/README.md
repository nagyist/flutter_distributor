# fastforge_app_analyzer

A unified application package analyzer — reads metadata from compiled app packages (APK, IPA, AAB, DMG, etc.).

## Status

Work in progress (WIP). Currently supports analyzing Android (APK, AAB) and iOS (IPA) packages, with macOS (DMG) support in development.

## Supported Formats

| Format | Analyzer | Status |
|---|---|---|
| `android/apk` | `AndroidApkAnalyzer` | ✅ Implemented |
| `android/aab` | `AndroidAabAnalyzer` | ✅ Implemented |
| `ios/ipa` | `IOSIpaAnalyzer` | ✅ Implemented |
| `macos/dmg` | `MacOSDmgAnalyzer` | 🚧 In development |

## API Usage

```rust
use fastforge_app_analyzer::AndroidApkAnalyzer;
use fastforge_core::{AnalyzeConfig, AppAnalyzer};

fn analyze_apk() -> anyhow::Result<()> {
    let analyzer = AndroidApkAnalyzer;
    let config = AnalyzeConfig::new("path/to/app.apk");
    let result = analyzer.analyze(&config)?;

    println!("App name: {}", result.app_name);
    println!("Version: {} ({})", result.version_name, result.version_code);
    println!("Package: {}", result.package_name);

    Ok(())
}
```

## Run Tests

```bash
cargo test -p fastforge_app_analyzer --offline
```
