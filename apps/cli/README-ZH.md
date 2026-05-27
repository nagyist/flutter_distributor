# fastforge

[English](./README.md) | 简体中文

`fastforge` 是 Fastforge 的 Rust 命令行入口，负责统一调度 `analyze/build/package/publish/store/upgrade` 子命令。

## 快速开始

### 1. 环境要求

- Rust stable（建议使用最新稳定版）
- 在仓库根目录执行命令

### 2. 编译与运行

```bash
# 在仓库根目录
cargo build -p fastforge

# 查看帮助
cargo run -p fastforge -- --help
```

### 3. 执行分析命令

```bash
# 分析应用包，支持: .aab/.apk/.ipa/.dmg
cargo run -p fastforge -- analyze /path/to/app.apk

# 输出到 JSON 文件
cargo run -p fastforge -- analyze /path/to/app.apk -o analysis.json
```

### 4. 执行构建命令

```bash
# 直接构建 Android APK（不打包/不发布）
cargo run -p fastforge -- build --platform android --target apk --build-flavor dev --build-dart-define APP_ENV=dev
```

### 5. 安装为本地命令

```bash
# 从仓库根目录安装
cargo install --path apps/cli

fastforge --help
```

## 可用子命令

- `analyze`: 分析应用包元信息
- `build`: 直接构建 Flutter 应用产物
- `package`: 打包（命令入口已就绪）
- `publish`: 发布（命令入口已就绪）
- `store`: 管理应用商店中的应用（Google Play、App Store）
- `upgrade`: 升级（命令入口已就绪）

## 商店管理

`store` 子命令提供了 Google Play Console 和 App Store Connect 的应用商店管理功能。

### 凭证设置

通过环境变量或项目配置（`.fastforge/config.yaml`）设置凭证：

**App Store Connect：**
```bash
export APP_STORE_CONNECT_KEY_ID=D83848D23
export APP_STORE_CONNECT_ISSUER_ID=227b0bbf-ada8-458c-9d62-3d8022b7d07f
export APP_STORE_CONNECT_KEY_PATH=./AuthKey.p8
```

**Google Play Console：**
```bash
export GOOGLE_PLAY_SERVICE_ACCOUNT_JSON=./service-account.json
```

### 项目配置

在 `.fastforge/config.yaml` 中添加商店配置，可以省去每次传入 `--app-id`：

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

### 使用示例

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

## 开发提示

```bash
# 只检查该 crate
cargo check -p fastforge_cli

# 运行该 crate 测试（如存在）
cargo test -p fastforge_cli
```
