use crate::types::{BuildConfig, BuildError, BuildResult};
use std::collections::HashMap;

pub trait AppBuilder {
    fn name(&self) -> &str;
    fn matches(&self, platform: &str, target: Option<&str>) -> bool;
    fn is_supported_on_current_platform(&self) -> bool;
    fn build_subcommand(&self) -> &str;
    fn validate_arguments(&self, _config: &BuildConfig) -> Result<(), BuildError> {
        Ok(())
    }
    fn resolve_output_files(
        &self,
        config: &BuildConfig,
        flutter_version: Option<&crate::types::FlutterVersion>,
        environment: Option<&HashMap<String, String>>,
    ) -> Result<(std::path::PathBuf, Vec<std::path::PathBuf>), BuildError>;
    fn build_result(
        &self,
        config: BuildConfig,
        output_directory: std::path::PathBuf,
        output_files: Vec<std::path::PathBuf>,
        duration_ms: u128,
    ) -> BuildResult;
}
