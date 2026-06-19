use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Not authenticated: {0}")]
    NotAuthenticated(String),

    #[error("Missing credential '{0}'")]
    MissingCredential(String),

    #[error("HTTP request failed: {0}")]
    HttpError(String),

    #[error("API error: {status} {message}")]
    ApiError { status: String, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    General(String),
}
