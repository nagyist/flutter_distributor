use chrono::Utc;
use fastforge_app_store_connect as asc;
use fastforge_store_api_core::StoreError;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::Serialize;
use std::env;

use super::manager::AppStoreManager;

const KEY_ID_ENV: &str = "APP_STORE_CONNECT_KEY_ID";
const ISSUER_ID_ENV: &str = "APP_STORE_CONNECT_ISSUER_ID";
const KEY_PATH_ENV: &str = "APP_STORE_CONNECT_KEY_PATH";
const API_BASE: &str = "https://api.appstoreconnect.apple.com";

#[derive(Debug, Serialize)]
struct JwtClaims {
    iss: String,
    iat: i64,
    exp: i64,
    aud: String,
}

impl AppStoreManager {
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

    pub(super) fn authed_client(&self) -> Result<asc::Client, StoreError> {
        let token = self.jwt()?;
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
        Ok(asc::Client::new_with_client(API_BASE, http_client))
    }
}
