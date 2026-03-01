use std::path::{Path, PathBuf};

/// Artifact naming template (mirrors Dart's `_kArtifactName`):
/// `{name}[-{flavor}]-{build_name}[+{build_number}][-{build_mode}]-{platform}[-setup][.{ext}]`
fn render_artifact_name(config: &PackageConfig) -> String {
    let build_name = config
        .app_version
        .split('+')
        .next()
        .unwrap_or(&config.app_version)
        .to_string();
    let build_number = config
        .app_version
        .split('+')
        .nth(1)
        .map(|s| s.to_string());

    let mut name = config.app_name.clone();
    if let Some(flavor) = &config.flavor {
        name.push('-');
        name.push_str(flavor);
    }
    name.push('-');
    name.push_str(&build_name);
    if let Some(number) = &build_number {
        name.push('+');
        name.push_str(number);
    }
    if config.build_mode == "profile" {
        name.push('-');
        name.push_str(&config.build_mode);
    }
    name.push('-');
    name.push_str(&config.platform);
    if config.is_installer {
        name.push_str("-setup");
    }
    if !config.package_format.is_empty() {
        name.push('.');
        name.push_str(&config.package_format);
    }
    name
}

/// Configuration passed to each packager, mirroring Dart's `MakeConfig`.
#[derive(Debug, Clone)]
pub struct PackageConfig {
    /// Application name (from pubspec).
    pub app_name: String,
    /// Binary name on the filesystem (may differ from `app_name` on Linux).
    pub app_binary_name: String,
    /// Full semver string, e.g. `"1.2.3+4"`.
    pub app_version: String,
    /// `"debug"`, `"profile"`, or `"release"`.
    pub build_mode: String,
    /// Target platform: `"android"`, `"ios"`, `"linux"`, `"macos"`, `"windows"`, `"web"`, `"ohos"`.
    pub platform: String,
    /// Optional build flavor.
    pub flavor: Option<String>,
    /// Optional release channel.
    pub channel: Option<String>,
    /// Override for the artifact file name (mustache template string).
    pub artifact_name: Option<String>,
    /// Package format extension, e.g. `"deb"`, `"zip"`, `""` for direct.
    pub package_format: String,
    /// Whether this package is an installer (appended `-setup` in artifact name).
    pub is_installer: bool,
    /// Directory containing the flutter build output.
    pub build_output_dir: PathBuf,
    /// Individual output files from the flutter build (e.g. a single `.apk`).
    pub build_output_files: Vec<PathBuf>,
    /// Root output directory where versioned sub-directories are created.
    pub output_dir: PathBuf,
}

impl PackageConfig {
    /// Returns the file name (without directory) for the output artifact.
    pub fn output_file_name(&self) -> String {
        if let Some(template) = &self.artifact_name {
            return template.clone();
        }
        render_artifact_name(self)
    }

    /// Returns the versioned output sub-directory, creating it if necessary.
    pub fn version_output_dir(&self) -> PathBuf {
        let dir = self.output_dir.join(&self.app_version);
        if !dir.exists() {
            std::fs::create_dir_all(&dir).ok();
        }
        dir
    }

    /// Full path to the output artifact file.
    pub fn output_file(&self) -> PathBuf {
        self.version_output_dir().join(self.output_file_name())
    }

    /// Full path to a temporary packaging directory (cleaned before use).
    pub fn packaging_dir(&self) -> PathBuf {
        let stem = if self.package_format.is_empty() {
            format!("{}_direct", self.output_file_name())
        } else {
            self.output_file_name().replace(
                &format!(".{}", self.package_format),
                &format!("_{}", self.package_format),
            )
        };
        let dir = self.version_output_dir().join(stem);
        if dir.exists() {
            std::fs::remove_dir_all(&dir).ok();
        }
        std::fs::create_dir_all(&dir).ok();
        dir
    }

    /// Path to the first build output file (convenience helper).
    pub fn first_build_output_file(&self) -> Option<&Path> {
        self.build_output_files.first().map(|p| p.as_path())
    }
}

/// Returned on successful packaging.
#[derive(Debug)]
pub struct PackageResult {
    /// All artifacts produced by the packager.
    pub artifacts: Vec<PathBuf>,
}

/// Error produced by a packager.
#[derive(Debug)]
pub enum PackageError {
    /// Generic error with a human-readable message.
    General(String),
    /// A required external tool (e.g. `dpkg-deb`) was not found.
    MissingTool(String),
    /// An invoked subprocess exited with a non-zero status.
    CommandFailed { command: String, stderr: String },
    /// A required file or directory was not found.
    NotFound(String),
    /// IO error.
    Io(std::io::Error),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageError::General(msg) => write!(f, "{}", msg),
            PackageError::MissingTool(tool) => {
                write!(f, "required tool not found: {}", tool)
            }
            PackageError::CommandFailed { command, stderr } => {
                write!(f, "command '{}' failed: {}", command, stderr)
            }
            PackageError::NotFound(path) => write!(f, "not found: {}", path),
            PackageError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for PackageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PackageError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PackageError {
    fn from(e: std::io::Error) -> Self {
        PackageError::Io(e)
    }
}
