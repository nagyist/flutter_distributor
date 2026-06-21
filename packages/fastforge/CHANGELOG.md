## 0.6.7

* feat: add custom packager support (#326)
* feat: add prepackage/postpackage hooks support (#342)
* refactor: install Linux app files under `/opt` instead of `/usr/share` (#327)
* feat(rpm): add support for RPM spec macros configuration
* fix(appimage): skip running ldd on directories
* feat: support custom Inno Setup installation path via `INNO_SETUP_PATH` env var
* feat(Inno Setup): support extra environment variables and locale filtering
* fix(macos): use `--component` flag for productbuild and add scripts support
* bump `unified_distributor` to ^0.2.7

## 0.6.6

* feat: add comprehensive pgyer upload parameters support #297

## 0.6.5

* MakeDebConfig Add StartupWMClass support #290
* feat: Add AppGallery publisher support

## 0.6.4

* fix: The `savekey-prefix` argument in the minio publisher is now properly ignored when empty

## 0.6.3

* feat: Support `minio` publisher

## 0.6.2

* feat: Support `app-version` argument to override version from pubspec.yaml (future versions will not read from pubspec.yaml)

## 0.6.1

* GitHub publisher supports `repo` argument to replace `repo-owner` and `repo-name` arguments
* GitHub publisher supports `release-draft` and `release-prerelease` arguments
* Remove AppCenter publisher

## 0.6.0

* [FIX] Google Play Bundle is uploaded but the result is ignored (#261)
* Better error if entity is not a file otherwise it looks like this: (#266)
* feat: Support ohos platform.

## 0.5.1

* Bump `unified_distributor` to 0.1.1

## 0.5.0

* Keeping version in sync with flutter_distributor

## 0.1.0

* First release.
