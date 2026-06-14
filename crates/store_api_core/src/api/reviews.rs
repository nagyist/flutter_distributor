use crate::error::StoreError;
use crate::model::Review;

/// Review / submission API.
#[async_trait::async_trait]
pub trait StoreReviewsApi: Send + Sync {
    /// List all review submissions across all releases.
    ///
    /// Each release may have at most one active review at a time.
    async fn list_reviews(&self, app_id: &str) -> Result<Vec<Review>, StoreError>;

    /// Get details of a specific review submission.
    async fn get_review(&self, app_id: &str, review_id: &str) -> Result<Review, StoreError>;

    /// Submit a release for review.
    ///
    /// Returns the newly created review.
    async fn submit_review(
        &self,
        app_id: &str,
        release_id: &str,
    ) -> Result<Review, StoreError>;

    /// Cancel a pending review submission.
    async fn cancel_review(&self, app_id: &str, review_id: &str) -> Result<(), StoreError>;
}
