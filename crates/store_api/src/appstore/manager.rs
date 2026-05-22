use chrono::Utc;
use fastforge_store_api_core::{App, Release, Review, StoreError, StoreManager};
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

// ── API response types ─────────────────────────────────────────────────────────

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

// ── AppStoreManager ────────────────────────────────────────────────────────────

pub struct AppStoreManager;

impl AppStoreManager {
    fn jwt() -> Result<String, StoreError> {
        let key_id = env::var(KEY_ID_ENV)
            .map_err(|_| StoreError::MissingCredential(KEY_ID_ENV.to_string()))?;
        let issuer_id = env::var(ISSUER_ID_ENV)
            .map_err(|_| StoreError::MissingCredential(ISSUER_ID_ENV.to_string()))?;
        let key_path = env::var(KEY_PATH_ENV)
            .map_err(|_| StoreError::MissingCredential(KEY_PATH_ENV.to_string()))?;
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
                401 => Err(StoreError::NotAuthenticated(format!(
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

#[async_trait::async_trait]
impl StoreManager for AppStoreManager {
    fn store_display_name(&self) -> &str {
        "App Store Connect"
    }

    async fn list_apps(&self) -> Result<Vec<App>, StoreError> {
        let token = Self::jwt()?;
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

        let token = Self::jwt()?;
        let url = format!("{API_BASE}/apps/{app_id}");
        let resp: SingleAppResponse = Self::request(&token, &url).await?;
        Ok(Self::resource_to_app(resp.data))
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
