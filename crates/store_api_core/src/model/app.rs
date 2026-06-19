use serde::{Deserialize, Serialize};

/// An app managed in a store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    /// Store-internal app ID
    pub id: String,
    /// Package name (Android) or bundle ID (iOS) — the unique identifier used
    /// by the store to distinguish apps.
    pub package_name: String,
    /// App name / title
    pub title: String,
    /// A short description (subtitle / tagline)
    pub description: Option<String>,
    /// URL to the app icon
    pub icon_url: Option<String>,
    /// Whether the app is currently live on the store
    pub is_live: bool,
    /// Latest version string, if known
    pub latest_version: Option<String>,
    /// ISO-8601 timestamp of when the app was first created in the store
    pub created_at: Option<String>,
    /// ISO-8601 timestamp of the last update
    pub updated_at: Option<String>,
}
