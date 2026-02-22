use std::collections::HashMap;
use std::sync::Arc;

pub type PublishProgressCallback = Arc<dyn Fn(u64, u64) + Send + Sync + 'static>;

#[derive(Debug, Clone)]
pub struct PublishConfig {
    pub app_version: Option<String>,
    pub artifact_path: Option<String>,
    pub publish_arguments: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct PublishResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug)]
pub enum PublishError {
    General(String),
}

impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for PublishError {}
