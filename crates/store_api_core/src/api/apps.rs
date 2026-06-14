use crate::error::StoreError;
use crate::model::App;

/// App management API.
#[async_trait::async_trait]
pub trait StoreAppsApi: Send + Sync {
    /// List all apps available in the store account.
    async fn list_apps(&self) -> Result<Vec<App>, StoreError>;

    /// Get details of a specific app by its store-internal ID.
    async fn get_app(&self, app_id: &str) -> Result<App, StoreError>;
}
