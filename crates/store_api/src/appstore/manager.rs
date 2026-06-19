use fastforge_store_api_core::StoreManager;

pub struct AppStoreManager {
    pub(super) key_id: Option<String>,
    pub(super) issuer_id: Option<String>,
    pub(super) key_path: Option<String>,
}

impl AppStoreManager {
    pub fn new() -> Self {
        Self {
            key_id: None,
            issuer_id: None,
            key_path: None,
        }
    }

    pub fn with_config(
        key_id: Option<String>,
        issuer_id: Option<String>,
        key_path: Option<String>,
    ) -> Self {
        Self {
            key_id,
            issuer_id,
            key_path,
        }
    }
}

impl Default for AppStoreManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreManager for AppStoreManager {
    fn store_display_name(&self) -> &str {
        "App Store Connect"
    }
}
