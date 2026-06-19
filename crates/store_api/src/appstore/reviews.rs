use fastforge_store_api_core::{
    ReleaseStatus, Review, ReviewStatus, StoreError, StoreReleasesApi, StoreReviewsApi,
};

use super::manager::AppStoreManager;

#[async_trait::async_trait]
impl StoreReviewsApi for AppStoreManager {
    async fn list_reviews(&self, app_id: &str) -> Result<Vec<Review>, StoreError> {
        let releases = self.list_releases(app_id).await?;

        let mut reviews = Vec::new();
        for release in &releases {
            let status = match &release.status {
                ReleaseStatus::WaitingForReview => ReviewStatus::Pending,
                ReleaseStatus::InReview => ReviewStatus::InReview,
                ReleaseStatus::Approved | ReleaseStatus::Published => ReviewStatus::Approved,
                ReleaseStatus::Rejected { reason } => ReviewStatus::Reject {
                    reason: reason.clone(),
                },
                _ => continue,
            };

            reviews.push(Review {
                id: release.id.clone().unwrap_or_default(),
                app_id: app_id.to_string(),
                version_id: release.id.clone().unwrap_or_default(),
                status,
                submitted_at: release.created_at.clone().unwrap_or_default(),
                resolved_at: release.published_at.clone(),
            });
        }

        Ok(reviews)
    }

    async fn get_review(&self, _app_id: &str, _review_id: &str) -> Result<Review, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn submit_review(&self, _app_id: &str, _release_id: &str) -> Result<Review, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn cancel_review(&self, _app_id: &str, _review_id: &str) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}
