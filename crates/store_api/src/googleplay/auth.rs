use chrono::Utc;
use fastforge_google_play_console as gpc;
use fastforge_store_api_core::StoreError;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::env;

use super::manager::GooglePlayManager;

const CREDENTIALS_ENV: &str = "GOOGLE_PLAY_SERVICE_ACCOUNT_JSON";
const TOKEN_URI: &str = "https://oauth2.googleapis.com/token";
const ANDROID_PUBLISHER_SCOPE: &str = "https://www.googleapis.com/auth/androidpublisher";
const PUBLISHER_API: &str = "https://androidpublisher.googleapis.com/androidpublisher/v3";

#[derive(Debug, Deserialize)]
pub(crate) struct ServiceAccountCredentials {
    pub(crate) client_email: String,
    pub(crate) private_key: String,
    pub(crate) token_uri: Option<String>,
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

impl GooglePlayManager {
    fn credential_path(&self) -> Result<String, StoreError> {
        match &self.credential_file {
            Some(path) => Ok(path.clone()),
            None => env::var(CREDENTIALS_ENV)
                .map_err(|_| StoreError::MissingCredential(CREDENTIALS_ENV.to_string())),
        }
    }

    pub(crate) fn credentials(&self) -> Result<ServiceAccountCredentials, StoreError> {
        let path = self.credential_path()?;
        let content = std::fs::read_to_string(&path).map_err(StoreError::Io)?;
        serde_json::from_str(&content)
            .map_err(|e| StoreError::General(format!("Failed to parse credentials: {e}")))
    }

    pub(crate) async fn exchange_token(
        credentials: &ServiceAccountCredentials,
    ) -> Result<String, StoreError> {
        let token_uri = credentials
            .token_uri
            .as_deref()
            .filter(|value| !value.trim().is_empty())
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

        let resp = reqwest::Client::new()
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

    pub(crate) async fn authed_client(&self) -> Result<gpc::Client, StoreError> {
        let credentials = self.credentials()?;
        let token = Self::exchange_token(&credentials).await?;
        let bearer = format!("Bearer {token}");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&bearer)
                .map_err(|e| StoreError::General(format!("Invalid header: {e}")))?,
        );
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let http_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|e| StoreError::General(format!("HTTP client error: {e}")))?;
        Ok(gpc::Client::new_with_client(PUBLISHER_API, http_client))
    }
}
