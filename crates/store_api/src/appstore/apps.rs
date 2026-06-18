use fastforge_app_store_connect as asc;
use fastforge_store_api_core::{App, StoreAppsApi, StoreError};

use super::manager::AppStoreManager;

fn map_app(app: asc::types::App) -> App {
    App {
        id: app.id,
        package_name: app
            .attributes
            .as_ref()
            .and_then(|a| a.bundle_id.clone())
            .unwrap_or_default(),
        title: app
            .attributes
            .as_ref()
            .and_then(|a| a.name.clone())
            .unwrap_or_default(),
        description: None,
        icon_url: None,
        is_live: false,
        latest_version: None,
        created_at: None,
        updated_at: None,
    }
}

#[async_trait::async_trait]
impl StoreAppsApi for AppStoreManager {
    async fn list_apps(&self) -> Result<Vec<App>, StoreError> {
        let client = self.authed_client()?;
        let resp = client
            .apps_get_collection(
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None,
            )
            .await
            .map_err(|e| StoreError::General(format!("Failed to list apps: {e}")))?;

        Ok(resp.into_inner().data.into_iter().map(map_app).collect())
    }

    async fn get_app(&self, app_id: &str) -> Result<App, StoreError> {
        let client = self.authed_client()?;
        let resp = client
            .apps_get_instance(
                app_id, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None,
            )
            .await
            .map_err(|e| StoreError::General(format!("Failed to get app: {e}")))?;

        Ok(map_app(resp.into_inner().data))
    }
}
