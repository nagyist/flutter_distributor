use crate::error::StoreError;
use crate::model::Listing;

/// Store listing / store page metadata API.
#[async_trait::async_trait]
pub trait StoreListingsApi: Send + Sync {
    /// List all store listings across all apps.
    async fn list_listings(&self) -> Result<Vec<Listing>, StoreError>;

    /// Get the store listing for a specific app.
    async fn get_listing(&self, app_id: &str) -> Result<Listing, StoreError>;

    /// Update the store listing metadata for an app.
    async fn update_listing(&self, app_id: &str, listing: &Listing) -> Result<(), StoreError>;
}
