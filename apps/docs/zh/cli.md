# CLI

如何使用 Fastforge 的命令行界面（CLI）

## 安装

```shell
dart pub global activate fastforge
```

> **Windows 用户请注意：** 激活后，请确保 pub 缓存 bin 目录已添加到 PATH 环境变量中：
> 1. 打开 **系统属性** → **高级** → **环境变量**
> 2. 在 **用户变量** 中找到 `Path`，点击 **编辑**
> 3. 添加 `%APPDATA%\Pub\Cache\bin` 并点击 **确定**
> 4. 重启终端，然后运行 `fastforge --help` 验证

---

## Dart CLI 命令

> 这些命令按字母顺序排列，最常用的是 package、publish 和 release。

### Package

将应用程序打包为特定于平台的格式，并将结果放入文件夹中。

| Flag | Value | Required |
|------|-------|:--------:|
| `--platform` | 平台, 如 `android` | ✅ |
| `--targets` | 以逗号分隔的 maker 名称列表 | ✅ |
| `--skip-clean` | 跳过构建前的清理 | ❌ |
| `--hook-pre` | 打包前执行的 shell 命令 | ❌ |
| `--hook-post` | 打包后执行的 shell 命令 | ❌ |

示例：

```shell
fastforge package --platform=android --targets=aab,apk

fastforge package --platform=macos --target=zip --hook-pre 'echo "before"' --hook-post 'echo "after"'
```

### Publish

| Flag | Value | Required |
|------|-------|:--------:|
| `--path` | 路径, 如 `hello_world-1.0.0+1-android.apk` | ✅ |
| `--targets` | 以逗号分隔的 publisher 名称列表 | ✅ |

示例：

```shell
fastforge publish --path hello_world-1.0.0+1-android.apk --targets fir,pgyer
```

### Release

会根据配置文件（`distribute_options.yaml`），将你的应用打包成特定的格式并发布到分发平台。

| Flag | Value | Required |
|------|-------|:--------:|
| `--name` | 名称, e.g. `dev` | ✅ |
| `--skip-clean` | 跳过构建前的清理 | ❌ |

示例：

```shell
fastforge release --name dev
```

---

## Rust CLI 命令

Rust CLI 提供了 Dart CLI 的所有能力，并增加了更多功能。
在仓库根目录运行或安装二进制后执行。

### Analyze

分析应用包元信息（APK、AAB、IPA、DMG 等）。

```bash
fastforge analyze /path/to/app.apk

# 输出到 JSON 文件
fastforge analyze /path/to/app.apk -o analysis.json
```

输出包含平台、标识符、版本、构建号和应用名称。

### Build

直接构建 Flutter 应用产物（不打包/不发布）。

```bash
# 构建 Android APK（带构建参数）
fastforge build \
  --platform android \
  --target apk \
  --build-flavor dev \
  --build-dart-define APP_ENV=dev
```

| Flag | 描述 | 必填 |
|------|------|:----:|
| `--platform` | 目标平台（`android`, `ios`, `macos`, `windows`, `linux`, `web`, `ohos`） | ✅ |
| `--target` | 构建目标（`apk`, `aab`, `ipa`, `hap`, `app` 等） | ❌ |
| `--build-flavor` | Build flavor（映射为 `--flavor`） | ❌ |
| `--build-dart-define` | Dart 定义（`KEY=VALUE`），可重复 | ❌ |

### Package

（P1 - 命令入口已就绪，正在积极开发中）

### Publish

（P1 - 命令入口已就绪，正在积极开发中）

### Release

（P1 - 命令入口已就绪，正在积极开发中）

### Store

管理应用商店中的应用（Google Play Console 和 App Store Connect）。

**凭证设置：**

通过环境变量或项目配置（`.fastforge/config.yaml`）设置：

```bash
# App Store Connect
export APP_STORE_CONNECT_KEY_ID=D83848D23
export APP_STORE_CONNECT_ISSUER_ID=227b0bbf-ada8-458c-9d62-3d8022b7d07f
export APP_STORE_CONNECT_KEY_PATH=./AuthKey.p8

# Google Play Console
export GOOGLE_PLAY_SERVICE_ACCOUNT_JSON=./service-account.json
```

**项目配置（`.fastforge/config.yaml`）：**

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

**使用示例：**

```bash
# 列出应用（仅 App Store 支持，Google Play 有 API 限制）
fastforge store list-apps --store appstore

# 获取应用详情（有配置且只有一个 app 时，可省略 app-id）
fastforge store get-app --store appstore

# 获取应用详情（多个 app 时使用别名或 ID）
fastforge store get-app --store appstore --app ios_production
fastforge store get-app --store googleplay --app-id com.example.app

# 列出版本
fastforge store list-releases --store appstore --app ios_production
fastforge store list-releases --store googleplay --app-id com.example.app
```

### Upgrade

更新 Fastforge 到最新版本。

### Version Check

检查 fastforge 是否有新版本。

```bash
fastforge version-check
```

---

## 资源说明

### `distribute_options.yaml`（Dart CLI）

完整的配置参考请查看[分发选项](./distribute-options.md)页面。

### `.fastforge/config.yaml`（Rust CLI）

Rust CLI 使用 `.fastforge/config.yaml` 来存储商店凭证和项目级别设置。
如果此文件包含敏感凭证，**不应**提交到版本控制。
