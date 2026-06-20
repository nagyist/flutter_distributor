# fastforge_store_api

Concrete app store management API clients — provides high-level operations such as listing apps, fetching app details, and managing releases for both App Store Connect and Google Play Console.

## Status

Work in progress (WIP). Both App Store and Google Play store managers are implemented.

## Supported Stores

| Store | Manager | Key Operations |
|---|---|---|
| App Store Connect | `AppStoreManager` | `list-apps`, `get-app`, `list-releases` |
| Google Play Console | `GooglePlayManager` | `list-apps`, `get-app`, `list-releases` |

## API Usage

```rust
use fastforge_store_api::AppStoreManager;
use fastforge_store_api_core::StoreManager;

async fn list_apps() -> anyhow::Result<()> {
    let manager = AppStoreManager::new("issuer_id", "key_id", "private_key")?;
    let apps = manager.list_apps().await?;

    for app in apps {
        println!("{} ({})", app.name, app.bundle_id);
    }
    Ok(())
}
```

## Run Tests

```bash
cargo test -p fastforge_store_api --offline
```
