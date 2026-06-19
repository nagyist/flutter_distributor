use fastforge_store_api_core::StoreManager;

pub struct GooglePlayManager {
    pub(super) credential_file: Option<String>,
}

impl GooglePlayManager {
    /// Create a new manager that reads credentials from `GOOGLE_PLAY_SERVICE_ACCOUNT_JSON`.
    pub fn new() -> Self {
        Self {
            credential_file: None,
        }
    }

    /// Create a manager with an explicit credential file path.
    pub fn with_credential_file(path: impl Into<String>) -> Self {
        Self {
            credential_file: Some(path.into()),
        }
    }
}

impl Default for GooglePlayManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreManager for GooglePlayManager {
    fn store_display_name(&self) -> &str {
        "Google Play Console"
    }
}
