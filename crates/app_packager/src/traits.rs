use crate::types::{PackageConfig, PackageError, PackageResult};

/// Trait implemented by every concrete packager.
pub trait AppPackager {
    /// Short identifier for this packager, e.g. `"deb"`, `"apk"`.
    fn name(&self) -> &str;

    /// Target platform, e.g. `"linux"`, `"android"`, `"macos"`.
    fn platform(&self) -> &str;

    /// Output file extension / package format, e.g. `"deb"`.
    /// Returns an empty string for directory-based outputs (direct).
    fn package_format(&self) -> &str;

    /// Returns `true` if this packager can run on the current host OS.
    /// Defaults to `true`; override for platform-restricted tools.
    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    /// Returns `true` if this packager matches the given `platform` and
    /// optional `target` name, mirroring Dart's `AppPackageMaker.match`.
    fn matches(&self, platform: &str, target: Option<&str>) -> bool {
        self.platform() == platform && target.is_none_or(|t| self.name() == t)
    }

    /// Package the application using the provided configuration.
    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError>;
}
