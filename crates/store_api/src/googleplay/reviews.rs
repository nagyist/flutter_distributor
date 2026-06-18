use fastforge_store_api_core::{Review, StoreError, StoreReviewsApi};

use super::manager::GooglePlayManager;

#[async_trait::async_trait]
impl StoreReviewsApi for GooglePlayManager {
    async fn list_reviews(&self, _app_id: &str) -> Result<Vec<Review>, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn get_review(&self, _app_id: &str, _review_id: &str) -> Result<Review, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn submit_review(&self, _app_id: &str, _release_id: &str) -> Result<Review, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn cancel_review(&self, _app_id: &str, _review_id: &str) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}
