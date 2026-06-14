use serde::{Deserialize, Serialize};

/// A store listing — the public-facing metadata displayed on the store page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    /// Store-internal listing ID
    pub id: String,
    /// The app this listing belongs to
    pub app_id: String,
    /// App title displayed on the store
    pub title: String,
    /// Short description / tagline
    pub short_description: Option<String>,
    /// Full description
    pub description: Option<String>,
    /// URL to the app icon
    pub icon_url: Option<String>,
    /// URLs to screenshots shown on the store page
    pub screenshot_urls: Vec<String>,
    /// App category (e.g. "Games", "Productivity")
    pub category: Option<String>,
    /// ISO-8601 timestamp of when the listing was created
    pub created_at: Option<String>,
    /// ISO-8601 timestamp of the last update
    pub updated_at: Option<String>,
}
