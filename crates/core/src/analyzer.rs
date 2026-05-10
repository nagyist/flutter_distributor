use serde_json::Value;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AnalyzeConfig {
    pub path: String,
}

impl AnalyzeConfig {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[derive(Debug)]
pub struct AnalyzeResult {
    pub success: bool,
    pub data: Value,
}

impl AnalyzeResult {
    pub fn new(success: bool, data: Value) -> Self {
        Self { success, data }
    }
}

#[derive(Debug)]
pub enum AnalyzeError {
    General(String),
}

impl AnalyzeError {
    pub fn new(message: &str) -> Self {
        Self::General(message.to_string())
    }
}

impl std::fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzeError::General(msg) => write!(f, "Analyze error: {}", msg),
        }
    }
}

impl std::error::Error for AnalyzeError {}

// ── Trait ─────────────────────────────────────────────────────────────────────

pub trait AppAnalyzer {
    fn new() -> Self;
    fn name(&self) -> &str;
    fn is_supported_on_current_platform(&self) -> bool;

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError>;

    fn analyze(&self, config: AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        self.perform_analyze(&config)
    }
}
