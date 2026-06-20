# fastforge_store_api_core

Core traits and types for app store management APIs — the shared abstractions and data models used by both the App Store Connect and Google Play Console clients.

## Status

Stable. This crate provides the foundational store API interfaces used by `fastforge_store_api`.

## Key Types

| Type | Description |
|---|---|
| `StoreManager` | Core trait for store API operations |
| `StoreAppsApi` | API trait for listing and fetching apps |
| `StoreReleasesApi` | API trait for release management |
| `StoreListingsApi` | API trait for store listing management |
| `StoreReviewsApi` | API trait for review management |
| `App` | Model representing an app in a store |
| `Release` | Model representing a release/version |
| `Listing` | Model representing a store listing |
| `Review` & `ReviewStatus` | Models for user reviews |
| `ReleaseStatus` | Enum for release states |
| `StoreError` | Unified error type for store operations |

## Usage

```rust
use fastforge_store_api_core::{
    StoreManager, StoreAppsApi, StoreError, App,
    StoreReleasesApi, Release,
};
```

Any store-specific client (App Store, Google Play) implements `StoreManager` and the associated API traits.

## Run Tests

```bash
cargo test -p fastforge_store_api_core --offline
```
