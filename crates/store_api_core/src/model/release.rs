use serde::{Deserialize, Serialize};

/// The review / publish status of a release.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseStatus {
    /// Draft, not yet submitted
    Draft,
    /// Waiting for review
    WaitingForReview,
    /// Currently in review
    InReview,
    /// Approved and ready to publish
    Approved,
    /// Rejected by the store
    Rejected { reason: String },
    /// Published and live
    Published,
    /// Removed from the store
    Removed,
}

/// A release (version) of the app managed in a store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    /// Store-internal ID for this release
    pub id: Option<String>,
    /// Human-readable version string (e.g. "1.0.0")
    pub version: String,
    /// Build number (e.g. "1")
    pub build_number: String,
    /// Current status
    pub status: ReleaseStatus,
    /// What's new in this release (localized)
    pub whats_new: String,
    /// ISO-8601 timestamp of when the release was created
    pub created_at: Option<String>,
    /// ISO-8601 timestamp of when the release was published
    pub published_at: Option<String>,
}
