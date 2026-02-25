# fastforge

English | [简体中文](./README-ZH.md)

`fastforge` is the Rust CLI entry point of Fastforge. It orchestrates `analyze/build/package/publish/upgrade` subcommands.

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

### 4. Install as a local command

```bash
cargo install --path crates/fastforge
fastforge --help
```

## Available subcommands

- `analyze`: Analyze application package metadata
- `build`: Build (command entry is in place)
- `package`: Package (command entry is in place)
- `publish`: Publish (command entry is in place)
- `upgrade`: Upgrade (command entry is in place)

## Development tips

```bash
# Check only this crate
cargo check -p fastforge

# Run tests for this crate (if any)
cargo test -p fastforge
```
