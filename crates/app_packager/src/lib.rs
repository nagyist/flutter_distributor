pub mod packagers;
pub mod traits;
pub mod types;

pub use traits::AppPackager;
pub use types::{PackageConfig, PackageError, PackageResult};

// Re-export all packagers for convenient access.
pub use packagers::android::aab::AndroidAabPackager;
pub use packagers::android::apk::AndroidApkPackager;
pub use packagers::ios::ipa::IOSIpaPackager;
pub use packagers::linux::appimage::LinuxAppImagePackager;
pub use packagers::linux::deb::LinuxDebPackager;
pub use packagers::linux::direct::LinuxDirectPackager;
pub use packagers::linux::pacman::LinuxPacmanPackager;
pub use packagers::linux::rpm::LinuxRpmPackager;
pub use packagers::linux::zip::LinuxZipPackager;
pub use packagers::macos::dmg::MacOSDmgPackager;
pub use packagers::macos::pkg::MacOSPkgPackager;
pub use packagers::macos::zip::MacOSZipPackager;
pub use packagers::ohos::app::OHOSAppPackager;
pub use packagers::ohos::hap::OHOSHapPackager;
pub use packagers::web::direct::WebDirectPackager;
pub use packagers::web::zip::WebZipPackager;
pub use packagers::windows::direct::WindowsDirectPackager;
pub use packagers::windows::exe::WindowsExePackager;
pub use packagers::windows::msix::WindowsMsixPackager;
pub use packagers::windows::zip::WindowsZipPackager;
