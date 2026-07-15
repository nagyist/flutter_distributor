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

## Commands

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

### Store catalog

Pull or push catalog data for every App Store and Google Play app configured in
`.fastforge/config.yaml`:

```yaml
stores:
  appstore:
    apps:
      - bundle_id: com.example.myapp
        app_id: "1234567890" # Optional fallback when bundle_id is omitted
  googleplay:
    apps:
      - package_name: com.example.myapp
```

```shell
fastforge store catalog pull
fastforge store catalog push
```

The unified command reads app identifiers from the configuration file and uses
the existing authentication environment variables:

| Store | Environment variables |
|-------|-----------------------|
| App Store | `APP_STORE_CONNECT_KEY_ID`, `APP_STORE_CONNECT_ISSUER_ID`, `APP_STORE_CONNECT_KEY_PATH` |
| Google Play | `GOOGLE_PLAY_SERVICE_ACCOUNT_JSON` (service account JSON or a JSON file path) |

Catalog files use the existing default directories under
`.fastforge/stores/appstore/` and `.fastforge/stores/googleplay/`. All configured
apps are processed in order. If one app fails, the remaining apps continue and
the command exits with an error after printing a summary.

For App Store screenshots, `pull` writes a hidden manifest beside each display
type directory. During `push`, unchanged screenshots are reused by remote ID and
checksum, changed or failed uploads are replaced, remote screenshots missing
from a non-empty local display type directory are deleted, and the remaining
screenshots are reordered by local filename. Use `--dry-run` with the standalone
`fastforge appstore catalog push --app <bundle-id> --dry-run` command to review
local screenshot sets before syncing.

---

## Resource Usage

### `distribute_options.yaml`

Refer to the [Distribute Options](./distribute-options.md) page for the full configuration reference.
