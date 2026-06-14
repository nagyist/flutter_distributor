use crate::error::StoreError;
use crate::model::Release;

/// Release management API.
#[async_trait::async_trait]
pub trait StoreReleasesApi: Send + Sync {
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
}
