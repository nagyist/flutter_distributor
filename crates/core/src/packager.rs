use std::path::{Path, PathBuf};

// ── Types ─────────────────────────────────────────────────────────────────────

fn render_artifact_name(config: &PackageConfig) -> String {
    let build_name = config
        .app_version
        .split('+')
        .next()
        .unwrap_or(&config.app_version)
        .to_string();
    let build_number = config.app_version.split('+').nth(1).map(|s| s.to_string());

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

#[derive(Debug, Clone)]
pub struct PackageConfig {
    pub app_name: String,
    pub app_binary_name: String,
    pub app_version: String,
    pub build_mode: String,
    pub platform: String,
    pub flavor: Option<String>,
    pub channel: Option<String>,
    pub artifact_name: Option<String>,
    pub package_format: String,
    pub is_installer: bool,
    pub build_output_dir: PathBuf,
    pub build_output_files: Vec<PathBuf>,
    pub output_dir: PathBuf,
}

impl PackageConfig {
    pub fn output_file_name(&self) -> String {
        if let Some(template) = &self.artifact_name {
            return template.clone();
        }
        render_artifact_name(self)
    }

    pub fn version_output_dir(&self) -> PathBuf {
        let dir = self.output_dir.join(&self.app_version);
        if !dir.exists() {
            std::fs::create_dir_all(&dir).ok();
        }
        dir
    }

    pub fn output_file(&self) -> PathBuf {
        self.version_output_dir().join(self.output_file_name())
    }

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

    pub fn first_build_output_file(&self) -> Option<&Path> {
        self.build_output_files.first().map(|p| p.as_path())
    }
}

#[derive(Debug)]
pub struct PackageResult {
    pub artifacts: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum PackageError {
    General(String),
    MissingTool(String),
    CommandFailed { command: String, stderr: String },
    NotFound(String),
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

// ── Trait ─────────────────────────────────────────────────────────────────────

pub trait AppPackager {
    fn name(&self) -> &str;
    fn platform(&self) -> &str;
    fn package_format(&self) -> &str;

    fn is_supported_on_current_platform(&self) -> bool {
        true
    }

    fn matches(&self, platform: &str, target: Option<&str>) -> bool {
        self.platform() == platform && target.is_none_or(|t| self.name() == t)
    }

    fn package(&self, config: &PackageConfig) -> Result<PackageResult, PackageError>;
}
