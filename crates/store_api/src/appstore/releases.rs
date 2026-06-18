use fastforge_app_store_connect as asc;
use fastforge_store_api_core::{Release, ReleaseStatus, StoreError, StoreReleasesApi};

use super::manager::AppStoreManager;

fn map_release(
    version: asc::types::AppStoreVersion,
    included: &[asc::types::AppStoreVersionsResponseIncludedItem],
) -> Release {
    let attrs = version.attributes.as_ref();
    let version_string = attrs
        .and_then(|a| a.version_string.as_deref())
        .unwrap_or("")
        .to_string();
    let app_store_state = attrs.and_then(|a| a.app_store_state.as_ref());
    let created_date = attrs
        .and_then(|a| a.created_date.as_ref())
        .map(ToString::to_string);
    let earliest_release_date = attrs
        .and_then(|a| a.earliest_release_date.as_ref())
        .map(ToString::to_string);

    let build_versions: std::collections::HashMap<&str, &str> = included
        .iter()
        .filter_map(|item| match item {
            asc::types::AppStoreVersionsResponseIncludedItem::Build(build) => build
                .attributes
                .as_ref()
                .and_then(|a| a.version.as_deref())
                .map(|ver| (build.id.as_str(), ver)),
            _ => None,
        })
        .collect();

    let localization_whats_new: std::collections::HashMap<&str, &str> = included
        .iter()
        .filter_map(|item| match item {
            asc::types::AppStoreVersionsResponseIncludedItem::AppStoreVersionLocalization(
                localization,
            ) => localization
                .attributes
                .as_ref()
                .and_then(|a| a.whats_new.as_deref())
                .map(|text| (localization.id.as_str(), text)),
            _ => None,
        })
        .collect();

    Release {
        id: Some(version.id),
        version: version_string,
        build_number: build_versions.values().next().unwrap_or(&"").to_string(),
        status: map_release_status(app_store_state),
        whats_new: localization_whats_new
            .values()
            .next()
            .unwrap_or(&"")
            .to_string(),
        created_at: created_date,
        published_at: earliest_release_date,
    }
}

fn map_release_status(state: Option<&asc::types::AppStoreVersionState>) -> ReleaseStatus {
    match state {
        Some(asc::types::AppStoreVersionState::PrepareForSubmission) | None => ReleaseStatus::Draft,
        Some(asc::types::AppStoreVersionState::WaitingForReview) => ReleaseStatus::WaitingForReview,
        Some(asc::types::AppStoreVersionState::InReview) => ReleaseStatus::InReview,
        Some(asc::types::AppStoreVersionState::PendingAppleRelease) => ReleaseStatus::Approved,
        Some(asc::types::AppStoreVersionState::ReadyForSale) => ReleaseStatus::Published,
        Some(asc::types::AppStoreVersionState::Rejected) => ReleaseStatus::Rejected {
            reason: "Rejected by App Store review".to_string(),
        },
        Some(asc::types::AppStoreVersionState::RemovedFromSale) => ReleaseStatus::Removed,
        _ => ReleaseStatus::Draft,
    }
}

#[async_trait::async_trait]
impl StoreReleasesApi for AppStoreManager {
    async fn list_releases(&self, app_id: &str) -> Result<Vec<Release>, StoreError> {
        let client = self.authed_client()?;
        let include = vec![
            asc::types::AppsAppStoreVersionsGetToManyRelatedIncludeItem::Build,
            asc::types::AppsAppStoreVersionsGetToManyRelatedIncludeItem::AppStoreVersionLocalizations,
        ];

        let resp = client
            .apps_app_store_versions_get_to_many_related(
                app_id,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(&include),
                None,
                None,
                None,
                None,
            )
            .await
            .map_err(|e| StoreError::General(format!("Failed to list releases: {e}")))?;

        let body = resp.into_inner();
        Ok(body
            .data
            .into_iter()
            .map(|version| map_release(version, &body.included))
            .collect())
    }

    async fn get_release(&self, _app_id: &str, _release_id: &str) -> Result<Release, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn create_release(
        &self,
        _app_id: &str,
        _release: &Release,
    ) -> Result<String, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn update_release(&self, _app_id: &str, _release: &Release) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}
