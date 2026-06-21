# CLI

How to use the command line interface (CLI) for Fastforge

## Installation

```shell
dart pub global activate fastforge
```

> **Windows users:** After activation, ensure the pub cache bin directory is in your PATH:
> 1. Open **System Properties** → **Advanced** → **Environment Variables**
> 2. Under **User variables**, select `Path` → **Edit**
> 3. Add `%APPDATA%\Pub\Cache\bin` and click **OK**
> 4. Restart your terminal, then try `fastforge --help`

---

## Dart CLI Commands

> These commands are sorted in alphabetical order. The most commonly used are package, publish, and release.

### Package

Will package your application into a platform specific format and put the result in a folder.

| Flag | Value | Required |
|------|-------|:--------:|
| `--platform` | Platform, e.g. `android` | ✅ |
| `--targets` | Comma separated list of maker names | ✅ |
| `--skip-clean` | Skip clean once before build | ❌ |
| `--hook-pre` | Shell command to run before packaging | ❌ |
| `--hook-post` | Shell command to run after packaging | ❌ |

Example:

```shell
fastforge package --platform=android --targets=aab,apk

fastforge package --platform=macos --target=zip --hook-pre 'echo "before"' --hook-post 'echo "after"'
```

### Publish

| Flag | Value | Required |
|------|-------|:--------:|
| `--path` | Path, e.g. `hello_world-1.0.0+1-android.apk` | ✅ |
| `--targets` | Comma separated list of publisher names | ✅ |

Example:

```shell
fastforge publish --path hello_world-1.0.0+1-android.apk --targets fir,pgyer
```

### Release

Will according to the configuration file (`distribute_options.yaml`), package your application into a specific format and publish it to the distribution platform.

| Flag | Value | Required |
|------|-------|:--------:|
| `--name` | Name, e.g. `dev` | ✅ |
| `--skip-clean` | Skip clean once before build | ❌ |

Example:

```shell
fastforge release --name dev
```

---

## Rust CLI Commands

The Rust CLI provides all the capabilities of the Dart CLI plus additional features.
Run from the repository root or after installing the binary.

### Analyze

Analyze application package metadata (APK, AAB, IPA, DMG, etc.).

```bash
fastforge analyze /path/to/app.apk

# Write output to a JSON file
fastforge analyze /path/to/app.apk -o analysis.json
```

Output includes platform, identifier, version, build number, and app name.

### Build

Build Flutter app outputs directly (without packaging/publishing).

```bash
# Build Android APK with build arguments
fastforge build \
  --platform android \
  --target apk \
  --build-flavor dev \
  --build-dart-define APP_ENV=dev
```

| Flag | Description | Required |
|------|-------------|:--------:|
| `--platform` | Target platform (`android`, `ios`, `macos`, `windows`, `linux`, `web`, `ohos`) | ✅ |
| `--target` | Build target (`apk`, `aab`, `ipa`, `hap`, `app`, etc.) | ❌ |
| `--build-flavor` | Build flavor (maps to `--flavor`) | ❌ |
| `--build-dart-define` | Dart defines (`KEY=VALUE`), repeatable | ❌ |

### Package

(P1 - Command entry is in place, under active development)

### Publish

(P1 - Command entry is in place, under active development)

### Release

(P1 - Command entry is in place, under active development)

### Store

Manage apps in app stores (Google Play Console and App Store Connect).

**Credentials:**

Set up via environment variables or project configuration (`.fastforge/config.yaml`):

```bash
# App Store Connect
export APP_STORE_CONNECT_KEY_ID=D83848D23
export APP_STORE_CONNECT_ISSUER_ID=227b0bbf-ada8-458c-9d62-3d8022b7d07f
export APP_STORE_CONNECT_KEY_PATH=./AuthKey.p8

# Google Play Console
export GOOGLE_PLAY_SERVICE_ACCOUNT_JSON=./service-account.json
```

**Project configuration (`.fastforge/config.yaml`):**

```yaml
stores:
  appstore:
    key_id: "D83848D23"
    issuer_id: "227b0bbf-ada8-458c-9d62-3d8022b7d07f"
    key_path: "./AuthKey.p8"
    apps:
      - id: "1234567890"
        name: ios_production
  googleplay:
    key_path: "./service-account.json"
    apps:
      - id: "com.example.app"
        name: android_production
```

**Usage examples:**

```bash
# List apps (App Store only, Google Play API limitation)
fastforge store list-apps --store appstore

# Get app details (single app with config)
fastforge store get-app --store appstore

# Get app details (multiple apps — use alias or ID)
fastforge store get-app --store appstore --app ios_production
fastforge store get-app --store googleplay --app-id com.example.app

# List releases
fastforge store list-releases --store appstore --app ios_production
fastforge store list-releases --store googleplay --app-id com.example.app
```

### Upgrade

Update Fastforge to the latest version.

### Version Check

Check for a newer version of fastforge.

```bash
fastforge version-check
```

---

## Resource Usage

### `distribute_options.yaml` (Dart CLI)

Refer to the [Distribute Options](./distribute-options.md) page for the full configuration reference.

### `.fastforge/config.yaml` (Rust CLI)

The Rust CLI uses `.fastforge/config.yaml` for store credentials and project-level settings.
This file should NOT be committed to version control if it contains sensitive credentials.
