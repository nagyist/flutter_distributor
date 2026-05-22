use crate::error::StoreError;
use crate::model::{App, Release, Review};

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
#[async_trait::async_trait]
pub trait StoreManager: Send + Sync {
    /// A human-readable name for the store (e.g. "Google Play Store").
    fn store_display_name(&self) -> &str;

    // ── App Management ─────────────────────────────────────────────

    /// List all apps available in the store account.
    async fn list_apps(&self) -> Result<Vec<App>, StoreError>;

    /// Get details of a specific app by its store-internal ID.
    async fn get_app(&self, app_id: &str) -> Result<App, StoreError>;

    // ── Release Management ─────────────────────────────────────────

    /// List all releases across all tracks for the given app.
    async fn list_releases(&self, app_id: &str) -> Result<Vec<Release>, StoreError>;

    /// Get details of a specific release by its store-internal ID.
    async fn get_release(&self, app_id: &str, release_id: &str) -> Result<Release, StoreError>;

    /// Create a new release / draft.
    ///
    /// Returns the store-internal ID of the newly created release.
    async fn create_release(
        &self,
        app_id: &str,
        release: &Release,
    ) -> Result<String, StoreError>;

    /// Update an existing release's metadata (e.g. what's new notes).
    async fn update_release(
        &self,
        app_id: &str,
        release: &Release,
    ) -> Result<(), StoreError>;

    // ── Review / Submission ─────────────────────────────────────────

    /// List all review submissions for a given release.
    ///
    /// A release may be reviewed multiple times (e.g. after rejection).
    async fn list_reviews(
        &self,
        app_id: &str,
        release_id: &str,
    ) -> Result<Vec<Review>, StoreError>;

    /// Get details of a specific review submission.
    async fn get_review(
        &self,
        app_id: &str,
        review_id: &str,
    ) -> Result<Review, StoreError>;

    /// Submit a release for review.
    ///
    /// Returns the newly created review.
    async fn submit_review(
        &self,
        app_id: &str,
        release_id: &str,
    ) -> Result<Review, StoreError>;

    /// Cancel a pending review submission.
    async fn cancel_review(&self, app_id: &str, review_id: &str) -> Result<(), StoreError>;
}
