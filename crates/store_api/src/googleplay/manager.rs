use chrono::Utc;
use fastforge_store_api_core::{App, Release, Review, StoreError, StoreManager};
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
    releases: Vec<TrackRelease>,
}

#[derive(Debug, Deserialize)]
struct TrackRelease {
    status: String,
    name: Option<String>,
}

// ── GooglePlayManager ──────────────────────────────────────────────────────────

pub struct GooglePlayManager;

impl GooglePlayManager {
    fn credentials() -> Result<ServiceAccountCredentials, StoreError> {
        let path = env::var(CREDENTIALS_ENV)
            .map_err(|_| StoreError::MissingCredential(CREDENTIALS_ENV.to_string()))?;
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

    async fn fetch_app(token: &str, package_name: &str) -> Result<App, StoreError> {
        let url = format!("{PUBLISHER_API}/applications/{package_name}/tracks");
        let resp = Client::new()
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| StoreError::HttpError(e.to_string()))?;

        if resp.status().as_u16() == 404 {
            return Err(StoreError::NotFound(format!(
                "App `{package_name}` not found or no access"
            )));
        }
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(StoreError::ApiError {
                status: status.to_string(),
                message: format!("Failed to get tracks for `{package_name}`: {body}"),
            });
        }

        let tracks: TracksResponse = resp
            .json()
            .await
            .map_err(|e| StoreError::General(format!("Failed to parse tracks response: {e}")))?;

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
            id: package_name.to_string(),
            package_name: package_name.to_string(),
            title: package_name.to_string(),
            description: None,
            icon_url: None,
            is_live,
            latest_version,
            created_at: None,
            updated_at: None,
        })
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
        let creds = Self::credentials()?;
        let token = Self::exchange_token(&creds).await?;
        Self::fetch_app(&token, app_id).await
    }

    async fn list_releases(&self, _app_id: &str) -> Result<Vec<Release>, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
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

    async fn list_reviews(&self, _app_id: &str, _release_id: &str) -> Result<Vec<Review>, StoreError> {
        Err(StoreError::General("Not yet implemented".to_string()))
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
