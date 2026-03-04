use crate::types::{AnalyzeConfig, AnalyzeError, AnalyzeResult};

pub trait AppAnalyzer {
    /// Create a new instance of this analyzer.
    fn new() -> Self;

    /// Get the name/identifier of this analyzer.
    fn name(&self) -> &str;

    /// Check if this analyzer is supported on the current platform.
    fn is_supported_on_current_platform(&self) -> bool;

    /// Performs the core analysis logic using the provided configuration.
    /// This method should be implemented by concrete analyzers to define their specific analysis behavior.
    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError>;

    /// Analyzes the application using the provided configuration.
    /// This is the main entry point for running analysis and internally calls `perform_analyze`.
    fn analyze(&self, config: AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        self.perform_analyze(&config)
    }
}
