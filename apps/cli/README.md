# fastforge

English | [简体中文](./README-ZH.md)

`fastforge` is the Rust CLI entry point of Fastforge. It orchestrates `analyze/build/package/publish/store/upgrade` subcommands.

## Quick Start

### 1. Requirements

- Rust stable (latest stable is recommended)
- Run commands from the repository root

### 2. Build and run

```bash
# From repository root
cargo build -p fastforge

# Show help
cargo run -p fastforge -- --help
```

### 3. Try the analyze command

```bash
# Analyze an app package. Supported: .aab/.apk/.ipa/.dmg
cargo run -p fastforge -- analyze /path/to/app.apk

# Write output to a JSON file
cargo run -p fastforge -- analyze /path/to/app.apk -o analysis.json
```

### 4. Try the build command

```bash
# Build Android APK directly (without package/publish)
cargo run -p fastforge -- build --platform android --target apk --build-flavor dev --build-dart-define APP_ENV=dev
```

### 5. Install as a local command

```bash
# Install from repository root
cargo install --path apps/cli

fastforge --help
```

## Available subcommands

- `analyze`: Analyze application package metadata
- `build`: Build Flutter app outputs directly
- `package`: Package (command entry is in place)
- `publish`: Publish (command entry is in place)
- `store`: Manage apps in app stores (Google Play, App Store)
- `upgrade`: Upgrade (command entry is in place)

## Store management

The `store` subcommand provides app store management operations for Google Play Console and App Store Connect.

### Credentials

Set up credentials via environment variables or project configuration (`.fastforge/config.yaml`):

**App Store Connect:**
```bash
export APP_STORE_CONNECT_KEY_ID=D83848D23
export APP_STORE_CONNECT_ISSUER_ID=227b0bbf-ada8-458c-9d62-3d8022b7d07f
export APP_STORE_CONNECT_KEY_PATH=./AuthKey.p8
```

**Google Play Console:**
```bash
export GOOGLE_PLAY_SERVICE_ACCOUNT_JSON=./service-account.json
```

### Project configuration

Add store configuration to `.fastforge/config.yaml` to avoid passing `--app-id` every time:

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

### Usage examples

```bash
# List apps (App Store only, Google Play API limitation)
fastforge store list-apps --store appstore

# Get app details (with config, single app — app-id is optional)
fastforge store get-app --store appstore

# Get app details (multiple apps — use alias or ID)
fastforge store get-app --store appstore --app ios_production
fastforge store get-app --store googleplay --app-id com.example.app

# List releases
fastforge store list-releases --store appstore --app ios_production
fastforge store list-releases --store googleplay --app-id com.example.app
```

## Development tips

```bash
# Check only this crate
cargo check -p fastforge_cli

# Run tests for this crate (if any)
cargo test -p fastforge_cli
```
