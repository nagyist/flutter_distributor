use chrono::Utc;
use fastforge_store_api_core::{
    App, Listing, Release, ReleaseStatus, Review, ReviewStatus, StoreAppsApi, StoreError,
    StoreListingsApi, StoreManager, StoreReleasesApi, StoreReviewsApi,
};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

// ── Constants ──────────────────────────────────────────────────────────────────

const KEY_ID_ENV: &str = "APP_STORE_CONNECT_KEY_ID";
const ISSUER_ID_ENV: &str = "APP_STORE_CONNECT_ISSUER_ID";
const KEY_PATH_ENV: &str = "APP_STORE_CONNECT_KEY_PATH";
const API_BASE: &str = "https://api.appstoreconnect.apple.com/v1";

// ── Auth types ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct JwtClaims {
    iss: String,
    iat: i64,
    exp: i64,
    aud: String,
}

// ── App response types ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct AppsResponse {
    data: Vec<AppResource>,
    links: Option<Links>,
}

#[derive(Debug, Deserialize)]
struct AppResource {
    id: String,
    attributes: AppAttributes,
}

#[derive(Debug, Deserialize)]
struct AppAttributes {
    name: String,
    #[serde(rename = "bundleId")]
    bundle_id: String,
    #[serde(default)]
    is_live: bool,
    #[serde(default)]
    is_removed: bool,
}

#[derive(Debug, Deserialize)]
struct Links {
    next: Option<String>,
}

// ── App Store Version response types ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct VersionsResponse {
    data: Vec<VersionResource>,
    links: Option<Links>,
    #[serde(default)]
    included: Vec<IncludedResource>,
}

#[derive(Debug, Deserialize)]
struct VersionResource {
    id: String,
    attributes: VersionAttributes,
    #[serde(default)]
    relationships: Option<VersionRelationships>,
}

#[derive(Debug, Deserialize)]
struct VersionRelationships {
    build: Option<Relationship>,
    #[serde(default, rename = "appStoreVersionLocalizations")]
    pub app_store_version_localizations: Option<RelationshipList>,
}

#[derive(Debug, Deserialize)]
struct RelationshipList {
    data: Vec<RelationshipData>,
}

#[derive(Debug, Deserialize)]
struct Relationship {
    data: RelationshipData,
}

#[derive(Debug, Deserialize)]
struct RelationshipData {
    id: String,
}

#[derive(Debug, Deserialize)]
struct VersionAttributes {
    #[serde(rename = "versionString")]
    version_string: String,
    #[serde(rename = "appStoreState")]
    app_store_state: String,
    #[serde(rename = "earliestReleaseDate")]
    earliest_release_date: Option<String>,
    #[serde(rename = "createdDate")]
    created_date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IncludedResource {
    id: String,
    #[serde(rename = "type")]
    resource_type: String,
    attributes: serde_json::Value,
}

// ── AppStoreManager ────────────────────────────────────────────────────────────

pub struct AppStoreManager {
    key_id: Option<String>,
    issuer_id: Option<String>,
    key_path: Option<String>,
}

impl AppStoreManager {
    /// Create a new manager that reads credentials from environment variables.
    pub fn new() -> Self {
        Self {
            key_id: None,
            issuer_id: None,
            key_path: None,
        }
    }

    /// Create a manager with explicit credential values.
    ///
    /// Any value not provided will fall back to the corresponding environment variable.
    pub fn with_config(
        key_id: Option<String>,
        issuer_id: Option<String>,
        key_path: Option<String>,
    ) -> Self {
        Self {
            key_id,
            issuer_id,
            key_path,
        }
    }

    fn resolve_key_id(&self) -> Result<String, StoreError> {
        self.key_id
            .clone()
            .or_else(|| env::var(KEY_ID_ENV).ok())
            .ok_or_else(|| StoreError::MissingCredential(KEY_ID_ENV.to_string()))
    }

    fn resolve_issuer_id(&self) -> Result<String, StoreError> {
        self.issuer_id
            .clone()
            .or_else(|| env::var(ISSUER_ID_ENV).ok())
            .ok_or_else(|| StoreError::MissingCredential(ISSUER_ID_ENV.to_string()))
    }

    fn resolve_key_path(&self) -> Result<String, StoreError> {
        self.key_path
            .clone()
            .or_else(|| env::var(KEY_PATH_ENV).ok())
            .ok_or_else(|| StoreError::MissingCredential(KEY_PATH_ENV.to_string()))
    }

    fn jwt(&self) -> Result<String, StoreError> {
        let key_id = self.resolve_key_id()?;
        let issuer_id = self.resolve_issuer_id()?;
        let key_path = self.resolve_key_path()?;
        let private_key_pem = std::fs::read_to_string(&key_path).map_err(StoreError::Io)?;

        let now = Utc::now().timestamp();
        let claims = JwtClaims {
            iss: issuer_id,
            iat: now,
            exp: now + 1200,
            aud: "appstoreconnect-v1".to_string(),
        };

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(key_id);
        header.typ = None;

        encode(
            &header,
            &claims,
            &EncodingKey::from_ec_pem(private_key_pem.as_bytes())
                .map_err(|e| StoreError::General(format!("Invalid EC private key: {e}")))?,
        )
        .map_err(|e| StoreError::General(format!("JWT encoding failed: {e}")))
    }

    fn resource_to_app(r: AppResource) -> App {
        App {
            id: r.id,
            package_name: r.attributes.bundle_id,
            title: r.attributes.name,
            description: None,
            icon_url: None,
            is_live: r.attributes.is_live && !r.attributes.is_removed,
            latest_version: None,
            created_at: None,
            updated_at: None,
        }
    }

    fn release_status_from_store_state(state: &str) -> ReleaseStatus {
        match state {
            "PREPARE_FOR_SUBMISSION" | "DEVELOPMENT" => ReleaseStatus::Draft,
            "WAITING_FOR_REVIEW" => ReleaseStatus::WaitingForReview,
            "IN_REVIEW" => ReleaseStatus::InReview,
            "PENDING_APPLE_RELEASE" => ReleaseStatus::Approved,
            "READY_FOR_SALE" => ReleaseStatus::Published,
            "REJECTED" => ReleaseStatus::Rejected {
                reason: "Rejected by App Store review".to_string(),
            },
            "REMOVED_FROM_SALE" => ReleaseStatus::Removed,
            _ => ReleaseStatus::Draft,
        }
    }

    async fn request<T: for<'de> Deserialize<'de>>(token: &str, url: &str) -> Result<T, StoreError> {
        let resp = Client::new()
            .get(url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| StoreError::HttpError(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return match status {
                404 => Err(StoreError::NotFound(format!("App not found: {body}"))),
                401 | 403 => Err(StoreError::NotAuthenticated(format!(
                    "Invalid or expired App Store Connect credentials: {body}"
                ))),
                _ => Err(StoreError::ApiError {
                    status: status.to_string(),
                    message: body,
                }),
            };
        }

        resp.json()
            .await
            .map_err(|e| StoreError::General(format!("Failed to parse response: {e}")))
    }
}

impl Default for AppStoreManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── StoreManager (combined trait) ────────────────────────────────────────────

impl StoreManager for AppStoreManager {
    fn store_display_name(&self) -> &str {
        "App Store Connect"
    }
}

// ── StoreAppsApi ─────────────────────────────────────────────────────────

#[async_trait::async_trait]
impl StoreAppsApi for AppStoreManager {
    async fn list_apps(&self) -> Result<Vec<App>, StoreError> {
        let token = self.jwt()?;
        let mut all = Vec::new();
        let mut next_url: Option<String> = Some(format!("{API_BASE}/apps"));

        while let Some(url) = next_url.take() {
            let resp: AppsResponse = Self::request(&token, &url).await?;
            for resource in resp.data {
                all.push(Self::resource_to_app(resource));
            }
            next_url = resp.links.and_then(|l| l.next);
        }

        Ok(all)
    }

    async fn get_app(&self, app_id: &str) -> Result<App, StoreError> {
        #[derive(Deserialize)]
        struct SingleAppResponse {
            data: AppResource,
        }

        let token = self.jwt()?;
        let url = format!("{API_BASE}/apps/{app_id}");
        let resp: SingleAppResponse = Self::request(&token, &url).await?;
        Ok(Self::resource_to_app(resp.data))
    }
}

// ── StoreReleasesApi ─────────────────────────────────────────────────────

#[async_trait::async_trait]
impl StoreReleasesApi for AppStoreManager {
    async fn list_releases(&self, app_id: &str) -> Result<Vec<Release>, StoreError> {
        let token = self.jwt()?;
        let mut all = Vec::new();
        let mut next_url: Option<String> =
            Some(format!("{API_BASE}/apps/{app_id}/appStoreVersions?include=build,appStoreVersionLocalizations"));

        while let Some(url) = next_url.take() {
            let resp: VersionsResponse = Self::request(&token, &url).await?;

            // Build lookups from included resources
            let build_versions: std::collections::HashMap<&str, &str> = resp
                .included
                .iter()
                .filter(|r| r.resource_type == "builds")
                .filter_map(|r| {
                    let version = r.attributes.get("version")?.as_str()?;
                    Some((r.id.as_str(), version))
                })
                .collect();

            // Localization whats_new lookup: localization_id → whats_new
            let localization_whats_new: std::collections::HashMap<&str, &str> = resp
                .included
                .iter()
                .filter(|r| r.resource_type == "appStoreVersionLocalizations")
                .filter_map(|r| {
                    let text = r.attributes.get("whatsNew")?.as_str()?;
                    Some((r.id.as_str(), text))
                })
                .collect();

            for v in resp.data {
                // Extract build number from the version's build relationship
                let build_number = v
                    .relationships
                    .as_ref()
                    .and_then(|rel| rel.build.as_ref())
                    .and_then(|b| build_versions.get(b.data.id.as_str()))
                    .unwrap_or(&"")
                    .to_string();

                // Extract whats_new from the first localization (preferred locale)
                let whats_new = v
                    .relationships
                    .as_ref()
                    .and_then(|rel| rel.app_store_version_localizations.as_ref())
                    .and_then(|locs| locs.data.first())
                    .and_then(|loc| localization_whats_new.get(loc.id.as_str()))
                    .unwrap_or(&"")
                    .to_string();

                all.push(Release {
                    id: Some(v.id),
                    version: v.attributes.version_string,
                    build_number,
                    status: Self::release_status_from_store_state(&v.attributes.app_store_state),
                    whats_new,
                    created_at: v.attributes.created_date.clone(),
                    published_at: v.attributes.earliest_release_date.clone(),
                });
            }
            next_url = resp.links.and_then(|l| l.next);
        }

        Ok(all)
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

    async fn update_release(
        &self,
        _app_id: &str,
        _release: &Release,
    ) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}

// ── StoreReviewsApi ──────────────────────────────────────────────────────

#[async_trait::async_trait]
impl StoreReviewsApi for AppStoreManager {
    async fn list_reviews(
        &self,
        app_id: &str,
    ) -> Result<Vec<Review>, StoreError> {
        let releases = self.list_releases(app_id).await?;

        let mut reviews = Vec::new();
        for r in &releases {
            let status = match &r.status {
                ReleaseStatus::WaitingForReview => ReviewStatus::Pending,
                ReleaseStatus::InReview => ReviewStatus::InReview,
                ReleaseStatus::Approved => ReviewStatus::Approved,
                ReleaseStatus::Published => ReviewStatus::Approved,
                ReleaseStatus::Rejected { reason } => ReviewStatus::Reject {
                    reason: reason.clone(),
                },
                _ => continue,
            };

            reviews.push(Review {
                id: r.id.clone().unwrap_or_default(),
                app_id: app_id.to_string(),
                version_id: r.id.clone().unwrap_or_default(),
                status,
                submitted_at: r.created_at.clone().unwrap_or_default(),
                resolved_at: r.published_at.clone(),
            });
        }

        Ok(reviews)
    }

    async fn get_review(&self, _app_id: &str, _review_id: &str) -> Result<Review, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn submit_review(
        &self,
        _app_id: &str,
        _release_id: &str,
    ) -> Result<Review, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn cancel_review(&self, _app_id: &str, _review_id: &str) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}

// ── StoreListingsApi ─────────────────────────────────────────────────────

#[async_trait::async_trait]
impl StoreListingsApi for AppStoreManager {
    async fn list_listings(&self) -> Result<Vec<Listing>, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn get_listing(&self, _app_id: &str) -> Result<Listing, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn update_listing(&self, _app_id: &str, _listing: &Listing) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}
