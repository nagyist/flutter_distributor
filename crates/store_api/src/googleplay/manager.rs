use chrono::Utc;
use fastforge_store_api_core::{App, Release, ReleaseStatus, Review, ReviewStatus, StoreError, StoreManager};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

// ── Constants ──────────────────────────────────────────────────────────────────

const CREDENTIALS_ENV: &str = "GOOGLE_PLAY_SERVICE_ACCOUNT_JSON";
const TOKEN_URI: &str = "https://oauth2.googleapis.com/token";
const ANDROID_PUBLISHER_SCOPE: &str = "https://www.googleapis.com/auth/androidpublisher";
const PUBLISHER_API: &str = "https://androidpublisher.googleapis.com/androidpublisher/v3";

// ── Auth types ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ServiceAccountCredentials {
    client_email: String,
    private_key: String,
    token_uri: Option<String>,
}

#[derive(Debug, Serialize)]
struct AccessTokenClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: i64,
    exp: i64,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}

// ── API response types ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TracksResponse {
    tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
struct Track {
    track: String,
    releases: Vec<TrackRelease>,
}

#[derive(Debug, Deserialize)]
struct TrackRelease {
    status: String,
    name: Option<String>,
    #[serde(default)]
    version_codes: Vec<String>,
    #[serde(rename = "releaseTime", default)]
    release_time: Option<String>,
}

// ── GooglePlayManager ──────────────────────────────────────────────────────────

pub struct GooglePlayManager {
    credential_file: Option<String>,
}

impl GooglePlayManager {
    /// Create a new manager that reads credentials from `GOOGLE_PLAY_SERVICE_ACCOUNT_JSON`.
    pub fn new() -> Self {
        Self { credential_file: None }
    }

    /// Create a manager with an explicit credential file path.
    pub fn with_credential_file(path: impl Into<String>) -> Self {
        Self {
            credential_file: Some(path.into()),
        }
    }

    fn credential_path(&self) -> Result<String, StoreError> {
        match &self.credential_file {
            Some(p) => Ok(p.clone()),
            None => env::var(CREDENTIALS_ENV)
                .map_err(|_| StoreError::MissingCredential(CREDENTIALS_ENV.to_string())),
        }
    }

    fn credentials(&self) -> Result<ServiceAccountCredentials, StoreError> {
        let path = self.credential_path()?;
        let content = std::fs::read_to_string(&path).map_err(StoreError::Io)?;
        serde_json::from_str(&content)
            .map_err(|e| StoreError::General(format!("Failed to parse credentials: {e}")))
    }

    async fn exchange_token(credentials: &ServiceAccountCredentials) -> Result<String, StoreError> {
        let token_uri = credentials
            .token_uri
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or(TOKEN_URI);
        let now = Utc::now().timestamp();
        let claims = AccessTokenClaims {
            iss: &credentials.client_email,
            scope: ANDROID_PUBLISHER_SCOPE,
            aud: token_uri,
            iat: now,
            exp: now + 3600,
        };
        let jwt = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(credentials.private_key.as_bytes())
                .map_err(|e| StoreError::General(format!("Invalid private key: {e}")))?,
        )
        .map_err(|e| StoreError::General(format!("JWT encoding failed: {e}")))?;

        let resp = Client::new()
            .post(token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| StoreError::HttpError(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(StoreError::ApiError {
                status: status.to_string(),
                message: format!("Token exchange failed: {body}"),
            });
        }

        let body: AccessTokenResponse = resp
            .json()
            .await
            .map_err(|e| StoreError::General(format!("Failed to parse token response: {e}")))?;
        Ok(body.access_token)
    }

    async fn request(token: &str, url: &str) -> Result<String, StoreError> {
        let resp = Client::new()
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| StoreError::HttpError(e.to_string()))?;

        if resp.status().as_u16() == 404 {
            return Err(StoreError::NotFound(format!("Resource not found: {url}")));
        }
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(StoreError::ApiError {
                status: status.to_string(),
                message: body,
            });
        }

        resp.text().await.map_err(|e| StoreError::HttpError(e.to_string()))
    }

    fn release_status_from_track(s: &str) -> ReleaseStatus {
        match s {
            "completed" | "inProgress" => ReleaseStatus::Published,
            "draft" => ReleaseStatus::Draft,
            "halted" => ReleaseStatus::Removed,
            _ => ReleaseStatus::Draft,
        }
    }
}

impl Default for GooglePlayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl StoreManager for GooglePlayManager {
    fn store_display_name(&self) -> &str {
        "Google Play Console"
    }

    async fn list_apps(&self) -> Result<Vec<App>, StoreError> {
        Err(StoreError::General(
            "Google Play Developer API does not provide a \"list apps\" endpoint. \
             Use `get_app` with a specific package name instead."
                .to_string(),
        ))
    }

    async fn get_app(&self, app_id: &str) -> Result<App, StoreError> {
        let creds = self.credentials()?;
        let token = Self::exchange_token(&creds).await?;
        let url = format!("{PUBLISHER_API}/applications/{app_id}/tracks");
        let body = Self::request(&token, &url).await?;
        let tracks: TracksResponse = serde_json::from_str(&body)
            .map_err(|e| StoreError::General(format!("Failed to parse tracks: {e}")))?;

        let latest_version = tracks
            .tracks
            .iter()
            .flat_map(|t| t.releases.iter())
            .filter_map(|r| r.name.as_deref())
            .next()
            .map(|s| s.to_string());

        let is_live = tracks.tracks.iter().any(|t| {
            t.releases
                .iter()
                .any(|r| r.status == "completed" || r.status == "inProgress")
        });

        Ok(App {
            id: app_id.to_string(),
            package_name: app_id.to_string(),
            title: app_id.to_string(),
            description: None,
            icon_url: None,
            is_live,
            latest_version,
            created_at: None,
            updated_at: None,
        })
    }

    async fn list_releases(&self, app_id: &str) -> Result<Vec<Release>, StoreError> {
        let creds = self.credentials()?;
        let token = Self::exchange_token(&creds).await?;
        let url = format!("{PUBLISHER_API}/applications/{app_id}/tracks");
        let body = Self::request(&token, &url).await?;
        let tracks: TracksResponse = serde_json::from_str(&body)
            .map_err(|e| StoreError::General(format!("Failed to parse tracks: {e}")))?;

        let mut releases = Vec::new();
        for track in &tracks.tracks {
            for r in &track.releases {
                let build_number = r.version_codes.first().cloned().unwrap_or_default();
                let version = r.name.clone().unwrap_or_default();
                releases.push(Release {
                    id: Some(format!("{}/{}", track.track, build_number)),
                    version,
                    build_number,
                    status: Self::release_status_from_track(&r.status),
                    whats_new: String::new(),
                    created_at: r.release_time.clone(),
                    published_at: r.release_time.clone(),
                });
            }
        }

        Ok(releases)
    }

    async fn get_release(&self, _app_id: &str, _release_id: &str) -> Result<Release, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn create_release(&self, _app_id: &str, _release: &Release) -> Result<String, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn update_release(&self, _app_id: &str, _release: &Release) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn list_reviews(&self, app_id: &str) -> Result<Vec<Review>, StoreError> {
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

    async fn submit_review(&self, _app_id: &str, _release_id: &str) -> Result<Review, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }

    async fn cancel_review(&self, _app_id: &str, _review_id: &str) -> Result<(), StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
    }
}
