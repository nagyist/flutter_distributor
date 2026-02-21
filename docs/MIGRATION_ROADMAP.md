# Fastforge Dart → Rust Migration Roadmap

## Goal

Replace the Dart implementation of `fastforge` with Rust while keeping
user-facing behavior compatible. After GA, fully remove the Dart runtime dependency.

## Scope

**In scope:** CLI commands (`package`, `publish`, `release`, `upgrade`,
`version-check`), `distribute_options.yaml` parsing, all packagers/publishers,
CI/CD pipeline migration.

**Out of scope:** New product features; breaking config schema changes during migration.

## Milestones

### M1 — Analyzer

Migrate artifact analysis (APK/IPA parsing) to Rust.

- Move analyze/parse capabilities to `crates/app_analyzer`.
- Standardize JSON output shape to match Dart analyzer output.
- Add snapshot tests for representative artifact fixtures.

**Done when:** Parse outputs match Dart JSON shape for all supported artifact types.

### M2 — Builder

Migrate build orchestration to Rust.

- Implement build orchestration in `crates/app_builder`.
- Target-specific build matrix and environment resolution.
- Align build logs and failure surfaces with Dart behavior.

**Done when:** Core platform build scenarios pass in Rust-only mode; build output
metadata and failure codes match Dart baseline.

### M3 — Packager

Migrate all packagers to `crates/app_packager`.

| Priority | Formats |
|----------|---------|
| P1 | `apk`, `aab`, `ipa`, `hap`, `app`, `zip`, `direct` |
| P2 | `deb`, `rpm`, `pkg`, `msix`, `exe` |
| P3 | `appimage`, `dmg` (reuse `crates/dmg_maker`), `pacman` |

- Implement packager registry and per-packager config validation.
- Ensure artifact naming and output layout match Dart.

**Done when:** All packagers pass unit + integration tests; golden metadata
matches Dart baseline.

### M4 — Publisher

Migrate all publishers to `crates/app_publisher`.

| Priority | Providers |
|----------|-----------|
| P1 | `github`, `firebase`, `playstore`, `appstore`, `pgyer` |
| P2 | `qiniu`, `vercel`, `fir`, `minio` |
| P3 | `appcenter`, `appgallery`, `firebase-hosting` |

- Implement publisher trait + provider implementations.
- Mock-backed contract tests; credential loading and error handling.

**Done when:** P1 providers pass staging E2E; all providers have retry/error tests.

### M5 — CLI & Config

Complete the Rust CLI surface and `distribute_options.yaml` support.

- Implement `release` command and all remaining subcommands in `crates/fastforge`.
- Parse `distribute_options.yaml` with variable merge, `jobs`/`skip-jobs`/
  `skip-clean` semantics.
- Shared core crate for config, logging, error model, runtime context.

**Done when:** `fastforge` CLI help and error behavior match Dart; release dry-run
matches Dart job selection; config compatibility tests pass.

### M6 — CI Migration, GA & Dart Removal

Switch CI to Rust-first, ship GA, then remove Dart.

- GitHub Actions: `cargo fmt`, `clippy`, `test`, cross-platform matrix
  (`macos`, `ubuntu`, `windows`).
- Publish Rust binary releases; enable Rust release pipeline for production.
- Dart CLI enters deprecation → removed after 2 weeks of stable Rust pipeline.

**Done when:** No production workflow requires Dart to run `fastforge`; Dart
packages archived or removed.

## Risks

| Risk | Mitigation |
|------|------------|
| Provider API / auth complexity | Mock-contract tests + staging creds + phased rollout |
| Platform-specific toolchain drift | Per-OS CI runners + weekly smoke builds |
| Hidden Dart behavior drift | Snapshot baseline tests kept until Dart removal |
