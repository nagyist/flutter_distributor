use serde::{Deserialize, Serialize};

/// The status of an app review submission.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    /// Submitted, waiting for review
    Pending,
    /// Currently being reviewed
    InReview,
    /// Approved by the store
    Approved,
    /// Rejected by the store
    Reject { reason: String },
}

/// A review submission — a version sent to the store for approval.
///
/// Each version may have multiple review attempts (e.g. after rejection).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// Store-internal review ID
    pub id: String,
    /// The app this review belongs to
    pub app_id: String,
    /// The version being reviewed
    pub version_id: String,
    /// Current status
    pub status: ReviewStatus,
    /// ISO-8601 timestamp of when the review was submitted
    pub submitted_at: String,
    /// ISO-8601 timestamp of when the review was resolved (approved/rejected)
    pub resolved_at: Option<String>,
}
