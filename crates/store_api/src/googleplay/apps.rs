use fastforge_store_api_core::{App, StoreAppsApi, StoreError};

use super::manager::GooglePlayManager;

#[async_trait::async_trait]
impl StoreAppsApi for GooglePlayManager {
    async fn list_apps(&self) -> Result<Vec<App>, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn get_app(&self, app_id: &str) -> Result<App, StoreError> {
        Ok(App {
            id: app_id.to_string(),
            package_name: app_id.to_string(),
            title: String::new(),
            description: None,
            icon_url: None,
            is_live: false,
            latest_version: None,
            created_at: None,
            updated_at: None,
        })
    }
}
