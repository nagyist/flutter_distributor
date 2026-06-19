use fastforge_google_play_console as gpc;
use fastforge_store_api_core::{Listing, StoreError, StoreListingsApi};
use std::env;

use super::manager::GooglePlayManager;

const PACKAGE_NAMES_ENV: &str = "FASTFORGE_GOOGLE_PLAY_PACKAGE_NAMES";
const LISTING_LOCALE_ENV: &str = "FASTFORGE_STORE_LISTING_LOCALE";
const DEFAULT_LISTING_LOCALE: &str = "en-US";

fn map_listing(app_id: &str, listing: gpc::types::Listing) -> Listing {
    let language = listing
        .language
        .unwrap_or_else(|| DEFAULT_LISTING_LOCALE.to_string());

    Listing {
        id: language,
        app_id: app_id.to_string(),
        title: listing.title.unwrap_or_default(),
        short_description: listing.short_description,
        description: listing.full_description,
        icon_url: None,
        screenshot_urls: Vec::new(),
        category: None,
        created_at: None,
        updated_at: None,
    }
}

fn listing_locale(listing: Option<&Listing>) -> String {
    listing
        .and_then(|listing| {
            let id = listing.id.trim();
            if id.is_empty() {
                None
            } else {
                Some(id.to_string())
            }
        })
        .or_else(|| env::var(LISTING_LOCALE_ENV).ok())
        .unwrap_or_else(|| DEFAULT_LISTING_LOCALE.to_string())
}

async fn create_edit(client: &gpc::Client, app_id: &str) -> Result<String, StoreError> {
    let response = client
        .androidpublisher_edits_insert(
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
            &gpc::types::AppEdit::default(),
        )
        .await
        .map_err(|e| StoreError::General(format!("Failed to create Google Play edit: {e}")))?;

    response.into_inner().id.ok_or_else(|| {
        StoreError::General("Google Play edit response did not include an id".to_string())
    })
}

#[async_trait::async_trait]
impl StoreListingsApi for GooglePlayManager {
    async fn list_listings(&self) -> Result<Vec<Listing>, StoreError> {
        let package_names = env::var(PACKAGE_NAMES_ENV)
            .map_err(|_| StoreError::MissingCredential(PACKAGE_NAMES_ENV.to_string()))?;
        let client = self.authed_client().await?;
        let mut listings = Vec::new();

        for app_id in package_names
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let edit_id = create_edit(&client, app_id).await?;
            let response = client
                .androidpublisher_edits_listings_list(
                    app_id, &edit_id, None, None, None, None, None, None, None, None, None, None,
                    None,
                )
                .await
                .map_err(|e| {
                    StoreError::General(format!("Failed to list Google Play listings: {e}"))
                })?;

            listings.extend(
                response
                    .into_inner()
                    .listings
                    .into_iter()
                    .map(|listing| map_listing(app_id, listing)),
            );
        }

        Ok(listings)
    }

    async fn get_listing(&self, app_id: &str) -> Result<Listing, StoreError> {
        let client = self.authed_client().await?;
        let edit_id = create_edit(&client, app_id).await?;
        let locale = listing_locale(None);
        let response = client
            .androidpublisher_edits_listings_get(
                app_id, &edit_id, &locale, None, None, None, None, None, None, None, None, None,
                None, None,
            )
            .await
            .map_err(|e| StoreError::General(format!("Failed to get Google Play listing: {e}")))?;

        Ok(map_listing(app_id, response.into_inner()))
    }

    async fn update_listing(&self, app_id: &str, listing: &Listing) -> Result<(), StoreError> {
        let client = self.authed_client().await?;
        let edit_id = create_edit(&client, app_id).await?;
        let locale = listing_locale(Some(listing));
        let body = gpc::types::Listing {
            full_description: listing.description.clone(),
            language: Some(locale.clone()),
            short_description: listing.short_description.clone(),
            title: Some(listing.title.clone()),
            video: None,
        };

        client
            .androidpublisher_edits_listings_patch(
                app_id, &edit_id, &locale, None, None, None, None, None, None, None, None, None,
                None, None, &body,
            )
            .await
            .map_err(|e| {
                StoreError::General(format!("Failed to update Google Play listing: {e}"))
            })?;

        client
            .androidpublisher_edits_commit(
                app_id, &edit_id, None, None, None, None, None, None, None, None, None, None, None,
                None,
            )
            .await
            .map_err(|e| StoreError::General(format!("Failed to commit Google Play edit: {e}")))?;

        Ok(())
    }
}
