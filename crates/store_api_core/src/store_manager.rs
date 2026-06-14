use crate::api::{StoreAppsApi, StoreListingsApi, StoreReleasesApi, StoreReviewsApi};

/// Core abstraction for managing apps on a specific app store.
///
/// All operations are scoped by `app_id` (the store-internal app identifier,
/// e.g. package name on Play Store or bundle ID on App Store), making the
/// trait suitable for stores where multiple apps are managed under the
/// same account.
///
/// Method naming follows the S3 API conventions:
/// - `list_*`  for collection operations
/// - `get_*`   for single-item retrieval
/// - `create_*` for creating new resources
/// - `update_*` for modifying existing resources
/// - `put_*`   for idempotent upserts
/// - `delete_*` for removing resources
///
/// This trait does NOT handle binary upload — use `AppPublisher` from
/// `fastforge_app_publisher` for that.
pub trait StoreManager: StoreAppsApi + StoreReleasesApi + StoreReviewsApi + StoreListingsApi {
    /// A human-readable name for the store (e.g. "Google Play Store").
    fn store_display_name(&self) -> &str;
}
