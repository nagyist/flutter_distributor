# App Store Connect CLI 设计

本文档定义一个独立的 App Store Connect CLI。它可以脱离 Fastforge 使用，
但只保留核心发布能力，避免 profile、配置文件、插件、复杂交互等额外设计。

推荐二进制名称：`fastforge_app_store_connect`。

## 目标

- 提供一个类似 `gh` 的资源化命令行体验。
- 作为独立二进制运行，不依赖 Flutter 或 Fastforge 项目。
- 只通过环境变量读取 App Store Connect API 凭据。
- 支持 CI 友好的 JSON 输出和明确退出码。
- 复用 `fastforge_app_store_connect` 作为底层 App Store Connect API client。
- 为 Fastforge 提供稳定的命令/JSON 契约，方便后续集成。

## 非目标

- 不支持本地配置文件。
- 不支持 profile、账号切换、keychain 存储或交互式登录。
- 不覆盖全部 App Store Connect API endpoint。
- 不实现 certificates、profiles、devices、价格、截图、复杂 metadata sync 等扩展能力。
- 不替代 Xcode、Transporter 或 `notarytool` 的全部 Apple 工作流。

## 认证

CLI 只从环境变量读取凭据：

```rust
const KEY_ID_ENV: &str = "APP_STORE_CONNECT_KEY_ID";
const ISSUER_ID_ENV: &str = "APP_STORE_CONNECT_ISSUER_ID";
const KEY_PATH_ENV: &str = "APP_STORE_CONNECT_KEY_PATH";
const API_BASE: &str = "https://api.appstoreconnect.apple.com";
```

对应 shell 用法：

```console
export APP_STORE_CONNECT_KEY_ID=ABC123DEFG
export APP_STORE_CONNECT_ISSUER_ID=00000000-0000-0000-0000-000000000000
export APP_STORE_CONNECT_KEY_PATH=/secure/path/AuthKey_ABC123DEFG.p8
```

认证规则：

- 启动命令时检查 `APP_STORE_CONNECT_KEY_ID`、`APP_STORE_CONNECT_ISSUER_ID` 和 `APP_STORE_CONNECT_KEY_PATH`。
- 缺少任一变量时直接失败，并提示缺少的变量名。
- `APP_STORE_CONNECT_KEY_PATH` 指向 `.p8` 私钥文件。
- CLI 内部生成短期 JWT，请求 `API_BASE`。
- 不把凭据写入磁盘，不提供 `auth login`。

## 全局体验

```console
fastforge_app_store_connect <resource> <action> [flags]

fastforge_app_store_connect app list
fastforge_app_store_connect app view com.example.app
fastforge_app_store_connect build list --app com.example.app
fastforge_app_store_connect build upload ./build/MyApp.ipa --app com.example.app --wait
fastforge_app_store_connect build wait <build-id>
fastforge_app_store_connect version list --app com.example.app
fastforge_app_store_connect version submit 1.4.0 --app com.example.app --build latest --wait
fastforge_app_store_connect api get /v1/apps
```

全局 flags：

```console
--json <fields>         以 JSON 输出所选字段。
--limit <n>             list 最大返回数量。
--paginate             支持时拉取所有分页。
--verbose              显示关键请求和重试信息。
--debug                显示 request ID 和已脱敏的 HTTP 诊断信息。
--no-color             禁用颜色。
```

不提供这些全局能力：

- `--profile`
- `--team`
- `--hostname`
- `--template`
- `--jq`
- `--web`

这些能力可以以后再加，但不属于当前核心版本。

## 命令树

核心命令只保留 4 组：

```console
fastforge_app_store_connect app
fastforge_app_store_connect build
fastforge_app_store_connect version
fastforge_app_store_connect api
```

### `apps`

查询 App Store Connect 中的 app。

```console
fastforge_app_store_connect app list
fastforge_app_store_connect app view <app>
```

`<app>` 支持：

- App Store Connect app id
- bundle id，例如 `com.example.app`
- SKU

默认表格字段：

```text
ID  NAME  BUNDLE_ID  SKU  PLATFORM
```

JSON 示例：

```console
fastforge_app_store_connect app view com.example.app --json id,name,bundleId,sku,platforms
```

### `builds`

上传、查询和等待 build processing。

```console
fastforge_app_store_connect build list --app <app>
fastforge_app_store_connect build list --app <app> --version 1.4.0
fastforge_app_store_connect build view <build-id>
fastforge_app_store_connect build upload <ipa-or-pkg> --app <app>
fastforge_app_store_connect build upload <ipa-or-pkg> --app <app> --wait
fastforge_app_store_connect build wait <build-id>
fastforge_app_store_connect build wait <build-id> --timeout 30m
```

说明：

- `build upload` 是核心能力，但具体上传实现可以先调用 Apple 官方上传工具。
- 上传完成后输出 build number、version、build id 和 processing state。
- `--wait` 会轮询直到 build 进入可用、失败或超时状态。
- `builds wait` 只负责状态轮询，不做额外发布动作。

默认表格字段：

```text
ID  VERSION  BUILD  PLATFORM  STATE  UPLOADED
```

### `versions`

查询和提交 App Store version。

```console
fastforge_app_store_connect version list --app <app>
fastforge_app_store_connect version view <version-id|version-string> --app <app>
fastforge_app_store_connect version submit <version-id|version-string> --app <app> --build <build-id|latest>
fastforge_app_store_connect version submit <version-id|version-string> --app <app> --build <build-id|latest> --wait
```

当前核心版不包含 `versions create/edit/release/cancel-submission`。
这些命令依赖更多 metadata、review 信息和状态约束，后续再设计。

`--build latest` 的含义：

- 在指定 app 下查找最新可用 build。
- 如果提供 version，则优先查找匹配 version 的 build。
- 若存在多个候选，选择上传时间最新的一个。
- 找不到候选 build 时失败。

### `api`

提供 raw API 逃生口，用于尚未包装的 endpoint。

```console
fastforge_app_store_connect api get /v1/apps
fastforge_app_store_connect api get /v1/apps --query 'filter[bundleId]=com.example.app'
fastforge_app_store_connect api post /v1/appStoreVersions --input payload.json
fastforge_app_store_connect api patch /v1/appStoreVersions/123 --input payload.json
fastforge_app_store_connect api delete /v1/someResources/123
```

规则：

- 路径必须以 `/v1/` 开头。
- 自动附加 `Authorization: Bearer <jwt>`。
- `--input` 读取 JSON 文件作为请求体。
- 默认输出远端返回的 JSON。

## 输出

默认输出面向人类，使用紧凑表格或简短摘要。

```console
$ fastforge_app_store_connect build list --app com.example.app
ID          VERSION  BUILD  PLATFORM  STATE       UPLOADED
123456789   1.4.0    42     IOS       VALID       2026-07-01 10:30
```

Mutation 输出保持短：

```console
Uploaded build 42 for com.example.app
Processing state: PROCESSING
Build id: 123456789
```

JSON 输出：

```console
fastforge_app_store_connect build list --app com.example.app --json id,version,buildNumber,processingState
```

字段规则：

- `--json` 不带值时输出该命令的稳定字段。
- `--json field1,field2` 只输出所选字段。
- 未知字段应失败，并列出可用字段。
- raw API JSON 只通过 `fastforge_app_store_connect api` 输出。

## 错误语义

退出码：

```text
0 success
1 general failure
2 command usage error
3 authentication or authorization failure
4 resource not found
5 validation failed before network request
6 remote API rejected the request
7 timeout while waiting for async state
8 partial success
```

错误信息应包含：

- 简短的人类可读摘要。
- 缺失环境变量时打印具体变量名。
- 可用时包含 App Store Connect request ID。
- `--debug` 模式下输出已脱敏诊断信息。

示例：

```text
missing required environment variable: APP_STORE_CONNECT_KEY_PATH
```

```text
failed to wait for build 123456789
reason: timed out after 30m
request id: ABCDEFGHIJ
```

## 实现形态

CLI 直接实现在现有 `crates/stores/app_store_connect` crate 下，不新增 crate，也不在 `apps/` 下新增入口。

推荐目录结构：

```text
crates/stores/app_store_connect/
  Cargo.toml
  src/
    main.rs                        CLI entrypoint, same dispatch style as apps/cli
    lib.rs                         exports generated client and CLI helpers
    client.rs                      generated low-level client
    auth.rs                        env auth and JWT generation
    context.rs                     create authenticated Client from env
    cli/
      mod.rs                       re-export command args
      commands/
        mod.rs                     command module exports
        app.rs                     fastforge_app_store_connect app ...
        build.rs                   fastforge_app_store_connect build ...
        version.rs                 fastforge_app_store_connect version ...
        api.rs                     fastforge_app_store_connect api ...
    output.rs                      table and JSON output models
```

`Cargo.toml` 增加 binary target，入口使用 `src/main.rs`，和 `apps/cli` 保持一致：

```toml
[[bin]]
name = "fastforge_app_store_connect"
path = "src/main.rs"
```

`fastforge_app_store_connect` 同时承担生成 client 和 CLI。`client.rs` 已经包含 App Store Connect API 的具体 endpoint 方法，不再额外增加 `services/apps.rs`、`services/builds.rs`、`services/versions.rs` 这类重复封装。

内部边界保持轻量：

- 从环境变量读取认证信息。
- 根据 `.p8` 私钥生成 JWT。
- 创建带 `Authorization: Bearer <jwt>` 的 `Client`。
- 在命令文件中直接调用 `client.rs` 生成的方法。
- 提供稳定的 JSON 输出模型。
- 在 `src/main.rs` 中处理 clap 顶层入口和命令分发。
- 在 `src/cli/commands/*.rs` 中定义各命令的 `Args` 和 `execute`。
- 只在确有必要时添加小型 helper，例如 app id 解析、`latest` build 选择、wait 轮询。

`src/main.rs` 结构参考 `apps/cli/src/main.rs`：

```rust
use clap::{Parser, Subcommand};

mod cli;

use cli::{ApiArgs, AppArgs, BuildArgs, VersionArgs};

#[derive(Parser)]
#[command(name = "fastforge_app_store_connect")]
#[command(about = "App Store Connect command line tool.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    App(AppArgs),
    Build(BuildArgs),
    Version(VersionArgs),
    Api(ApiArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::App(args) => cli::commands::app::execute(args).await?,
        Commands::Build(args) => cli::commands::build::execute(args).await?,
        Commands::Version(args) => cli::commands::version::execute(args).await?,
        Commands::Api(args) => cli::commands::api::execute(args).await?,
    }

    Ok(())
}
```

`src/cli/commands/app.rs` 等文件采用 `apps/cli` 的命令文件模式：

```rust
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: AppCommand,
}

#[derive(Subcommand)]
pub enum AppCommand {
    List,
    View(AppViewArgs),
}

pub async fn execute(args: &AppArgs) -> Result<()> {
    match &args.command {
        AppCommand::List(args) => list(args).await,
        AppCommand::View(args) => view(args).await,
    }
}
```

建议的 Rust 依赖：

```text
clap          command parser
tokio         async runtime
reqwest       HTTP transport
serde         JSON models
serde_json    JSON output and input
jsonwebtoken JWT generation
tabled        table output
indicatif     upload/wait progress
```

不需要：

```text
keyring
dialoguer
toml
directories
```

## Fastforge 集成

Fastforge 只需要调用核心发布路径：

```console
fastforge publish --target appstore --path build/ios/ipa/App.ipa
```

可以映射为：

```console
fastforge_app_store_connect build upload build/ios/ipa/App.ipa --app <app> --wait \
  --json app,bundleId,version,buildNumber,buildId,processingState
```

如果需要提交审核，再调用：

```console
fastforge_app_store_connect version submit <version> --app <app> --build latest --wait \
  --json app,version,buildId,state
```

Fastforge 应显式传入 `--app`，认证仍统一来自环境变量：

```console
APP_STORE_CONNECT_KEY_ID
APP_STORE_CONNECT_ISSUER_ID
APP_STORE_CONNECT_KEY_PATH
```

## MVP 范围

第一版只做这些：

- 环境变量认证。
- `app list/view`。
- `build list/view/upload/wait`。
- `version list/view/submit`。
- `api get/post/patch/delete`。
- 表格输出和 `--json` 输出。
- 稳定退出码和基础错误提示。

后续再考虑：

- metadata sync。
- screenshots sync。
- TestFlight testers/groups。
- App Store version create/edit/release。
- certificates/profiles/devices。

## 开放问题

- `build upload` 首版应直接调用哪个 Apple 上传工具？
- `version submit --wait` 应等待到哪个终态才算成功？
- `--json` 字段名的兼容性策略如何定义？
