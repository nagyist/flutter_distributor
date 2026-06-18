use fastforge_store_api_core::{Release, StoreError, StoreReleasesApi};

use super::manager::GooglePlayManager;

#[async_trait::async_trait]
impl StoreReleasesApi for GooglePlayManager {
    async fn list_releases(&self, _app_id: &str) -> Result<Vec<Release>, StoreError> {
        let _client = self.authed_client().await?;
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn get_release(&self, _app_id: &str, _release_id: &str) -> Result<Release, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn create_release(
        &self,
        _app_id: &str,
        _release: &Release,
    ) -> Result<String, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn update_release(&self, _app_id: &str, _release: &Release) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}
