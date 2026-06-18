use fastforge_store_api_core::{Listing, StoreError, StoreListingsApi};

use super::manager::GooglePlayManager;

#[async_trait::async_trait]
impl StoreListingsApi for GooglePlayManager {
    async fn list_listings(&self) -> Result<Vec<Listing>, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn get_listing(&self, _app_id: &str) -> Result<Listing, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn update_listing(&self, _app_id: &str, _listing: &Listing) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}
