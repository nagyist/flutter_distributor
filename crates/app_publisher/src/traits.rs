use crate::types::{PublishConfig, PublishError, PublishProgressCallback, PublishResult};

pub trait AppPublisher {
    /// Create a new instance of this publisher.
    fn new() -> Self;

    /// Get the name/identifier of this publisher.
    fn name(&self) -> &str;

    /// Check if this publisher is supported on the current platform.
    fn is_supported_on_current_platform(&self) -> bool;

    /// Performs the core publishing logic using the provided configuration.
    /// This method should be implemented by concrete publishers to define their specific publishing behavior.
    fn perform_publish(
        &self,
        config: &PublishConfig,
        on_progress: Option<&PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError>;

    /// Publishes the application using the provided configuration.
    /// This is the main entry point for running publishing and internally calls `perform_publish`.
    fn publish(
        &self,
        config: PublishConfig,
        on_progress: Option<PublishProgressCallback>,
    ) -> Result<PublishResult, PublishError> {
        self.perform_publish(&config, on_progress.as_ref())
    }
}
