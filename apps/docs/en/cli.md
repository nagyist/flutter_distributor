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

---

## Resource Usage

### `distribute_options.yaml`

Refer to the [Distribute Options](./distribute-options.md) page for the full configuration reference.
