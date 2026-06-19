use fastforge_app_store_connect as asc;
use fastforge_store_api_core::{Listing, StoreAppsApi, StoreError, StoreListingsApi};
use std::env;

use super::manager::AppStoreManager;

const LISTING_LOCALE_ENV: &str = "FASTFORGE_STORE_LISTING_LOCALE";
const DEFAULT_LISTING_LOCALE: &str = "en-US";

struct AppStoreListingParts {
    app_info_localization: asc::types::AppInfoLocalization,
    version_localization: asc::types::AppStoreVersionLocalization,
}

fn listing_locale(listing: Option<&Listing>) -> String {
    listing
        .and_then(|listing| {
            let id = listing.id.trim();
            if id.is_empty() || id.contains(':') {
                None
            } else {
                Some(id.to_string())
            }
        })
        .or_else(|| env::var(LISTING_LOCALE_ENV).ok())
        .unwrap_or_else(|| DEFAULT_LISTING_LOCALE.to_string())
}

fn split_listing_id(id: &str) -> Option<(&str, &str)> {
    id.split_once(':')
        .filter(|(app_info_id, version_id)| !app_info_id.is_empty() && !version_id.is_empty())
}

fn map_listing(app_id: &str, parts: AppStoreListingParts) -> Listing {
    let app_info_attrs = parts.app_info_localization.attributes.as_ref();
    let version_attrs = parts.version_localization.attributes.as_ref();

    Listing {
        id: format!(
            "{}:{}",
            parts.app_info_localization.id, parts.version_localization.id
        ),
        app_id: app_id.to_string(),
        title: app_info_attrs
            .and_then(|attrs| attrs.name.clone())
            .unwrap_or_default(),
        short_description: app_info_attrs.and_then(|attrs| attrs.subtitle.clone()),
        description: version_attrs.and_then(|attrs| attrs.description.clone()),
        icon_url: None,
        screenshot_urls: Vec::new(),
        category: None,
        created_at: None,
        updated_at: None,
    }
}

async fn fetch_listing_parts(
    client: &asc::Client,
    app_id: &str,
    locale: &str,
) -> Result<AppStoreListingParts, StoreError> {
    let locale_filter = vec![locale.to_string()];
    let app_infos = client
        .apps_app_infos_get_to_many_related(
            app_id,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            None,
        )
        .await
        .map_err(|e| StoreError::General(format!("Failed to list App Store app infos: {e}")))?
        .into_inner()
        .data;
    let app_info = app_infos
        .into_iter()
        .next()
        .ok_or_else(|| StoreError::NotFound(format!("App Store app info for {app_id}")))?;

    let app_info_localization = client
        .app_infos_app_info_localizations_get_to_many_related(
            &app_info.id,
            None,
            None,
            Some(&locale_filter),
            None,
            Some(1),
        )
        .await
        .map_err(|e| {
            StoreError::General(format!(
                "Failed to list App Store app info localizations: {e}"
            ))
        })?
        .into_inner()
        .data
        .into_iter()
        .next()
        .ok_or_else(|| {
            StoreError::NotFound(format!("App Store app info localization {app_id}/{locale}"))
        })?;

    let versions = client
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
            None,
            Some(1),
            None,
            None,
            None,
        )
        .await
        .map_err(|e| StoreError::General(format!("Failed to list App Store versions: {e}")))?
        .into_inner()
        .data;
    let version = versions
        .into_iter()
        .next()
        .ok_or_else(|| StoreError::NotFound(format!("App Store version for {app_id}")))?;

    let version_localization = client
        .app_store_versions_app_store_version_localizations_get_to_many_related(
            &version.id,
            None,
            None,
            None,
            None,
            None,
            Some(&locale_filter),
            None,
            Some(1),
            None,
            None,
            None,
        )
        .await
        .map_err(|e| {
            StoreError::General(format!(
                "Failed to list App Store version localizations: {e}"
            ))
        })?
        .into_inner()
        .data
        .into_iter()
        .next()
        .ok_or_else(|| {
            StoreError::NotFound(format!("App Store version localization {app_id}/{locale}"))
        })?;

    Ok(AppStoreListingParts {
        app_info_localization,
        version_localization,
    })
}

#[async_trait::async_trait]
impl StoreListingsApi for AppStoreManager {
    async fn list_listings(&self) -> Result<Vec<Listing>, StoreError> {
        let client = self.authed_client()?;
        let locale = listing_locale(None);
        let apps = StoreAppsApi::list_apps(self).await?;
        let mut listings = Vec::new();

        for app in apps {
            listings.push(map_listing(
                &app.id,
                fetch_listing_parts(&client, &app.id, &locale).await?,
            ));
        }

        Ok(listings)
    }

    async fn get_listing(&self, app_id: &str) -> Result<Listing, StoreError> {
        let client = self.authed_client()?;
        let locale = listing_locale(None);
        Ok(map_listing(
            app_id,
            fetch_listing_parts(&client, app_id, &locale).await?,
        ))
    }

    async fn update_listing(&self, app_id: &str, listing: &Listing) -> Result<(), StoreError> {
        let client = self.authed_client()?;
        let (app_info_localization_id, version_localization_id) =
            if let Some((app_info_id, version_id)) = split_listing_id(&listing.id) {
                (app_info_id.to_string(), version_id.to_string())
            } else {
                let locale = listing_locale(Some(listing));
                let parts = fetch_listing_parts(&client, app_id, &locale).await?;
                (
                    parts.app_info_localization.id,
                    parts.version_localization.id,
                )
            };

        let app_info_body = asc::types::AppInfoLocalizationUpdateRequest {
            data: asc::types::AppInfoLocalizationUpdateRequestData {
                attributes: Some(asc::types::AppInfoLocalizationUpdateRequestDataAttributes {
                    name: Some(listing.title.clone()),
                    privacy_choices_url: None,
                    privacy_policy_text: None,
                    privacy_policy_url: None,
                    subtitle: listing.short_description.clone(),
                }),
                id: app_info_localization_id.clone(),
                type_: asc::types::AppInfoLocalizationUpdateRequestDataType::AppInfoLocalizations,
            },
        };
        client
            .app_info_localizations_update_instance(&app_info_localization_id, &app_info_body)
            .await
            .map_err(|e| {
                StoreError::General(format!("Failed to update App Store app info listing: {e}"))
            })?;

        let version_body = asc::types::AppStoreVersionLocalizationUpdateRequest {
            data: asc::types::AppStoreVersionLocalizationUpdateRequestData {
                attributes: Some(
                    asc::types::AppStoreVersionLocalizationUpdateRequestDataAttributes {
                        description: listing.description.clone(),
                        keywords: None,
                        marketing_url: None,
                        promotional_text: listing.short_description.clone(),
                        support_url: None,
                        whats_new: None,
                    },
                ),
                id: version_localization_id.clone(),
                type_: asc::types::AppStoreVersionLocalizationUpdateRequestDataType::AppStoreVersionLocalizations,
            },
        };
        client
            .app_store_version_localizations_update_instance(
                &version_localization_id,
                &version_body,
            )
            .await
            .map_err(|e| {
                StoreError::General(format!("Failed to update App Store version listing: {e}"))
            })?;

        Ok(())
    }
}
