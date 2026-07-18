# 打包

`fastforge package` 负责准备项目、执行构建并把产物整理为可分发格式。

```bash
fastforge package --platform macos --target zip
```

`--platform` 和 `--target` 均为必填参数，一次处理一个 target。各平台的格式和环境要求见[打包器总览](packagers/README.md)。

## 打包流程

一次打包包含以下阶段：

1. 根据项目文件识别构建系统。
2. 调用对应平台工具完成构建。
3. 运行 pre-package hook。
4. 生成最终分发产物。
5. 运行 post-package hook。

Android 项目当前接入 Gradle，iOS 和 macOS 项目当前接入 Xcode。部分兼容项目会根据自身的清单文件选择对应构建适配器。

## 当前支持范围

| 构建系统       | 平台    | 格式                |
| -------------- | ------- | ------------------- |
| Gradle         | Android | `apk`、`aab`        |
| Xcode          | iOS     | `ipa`               |
| Xcode          | macOS   | `dmg`、`pkg`、`zip` |
| 兼容构建适配器 | macOS   | `dmg`、`pkg`、`zip` |

> [!NOTE]
> 当前 `fastforge package` 对不同构建系统的覆盖范围并不一致。底层模块存在不代表 CLI 已经完整接通，执行前请查看对应平台页面。

## 构建阶段

打包命令会自动执行所需构建。需要单独检查构建结果或调试构建参数时，可以直接运行：

```bash
fastforge build --platform <platform> [--target <target>]
```

### 平台与 target

| 平台      | target       | 宿主限制                |
| --------- | ------------ | ----------------------- |
| `android` | `apk`、`aab` | 需要 Android 工具链     |
| `ios`     | `ipa` 或省略 | 仅 macOS                |
| `macos`   | 可省略       | 仅 macOS                |
| `windows` | 可省略       | 仅 Windows              |
| `linux`   | 可省略       | 仅 Linux                |
| `web`     | 可省略       | 无固定宿主限制          |
| `ohos`    | `hap`、`app` | 需要 OpenHarmony 工具链 |

### 兼容构建参数

以下参数只作用于当前用于 Flutter 项目的兼容构建适配器，并不是所有项目打包都需要配置：

```bash
fastforge build \
  --platform android \
  --target apk \
  --clean \
  --build-target lib/main_dev.dart \
  --build-flavor dev \
  --build-target-platform android-arm64 \
  --build-dart-define APP_ENV=dev \
  --build-obfuscate \
  --build-split-debug-info build/debug-info \
  --build-tree-shake-icons
```

| Fastforge 参数                  | 对应构建参数             |
| ------------------------------- | ------------------------ |
| `--build-target`                | `--target`               |
| `--build-flavor`                | `--flavor`               |
| `--build-target-platform`       | `--target-platform`      |
| `--build-export-options-plist`  | `--export-options-plist` |
| `--build-export-method`         | `--export-method`        |
| `--build-dart-define KEY=VALUE` | 可重复的 `--dart-define` |
| `--build-obfuscate`             | `--obfuscate`            |
| `--build-split-debug-info`      | `--split-debug-info`     |
| `--build-tree-shake-icons`      | `--tree-shake-icons`     |
| `--build-profile`               | `--profile`              |

还可以通过逗号分隔的 `--flutter-build-args` 传递该适配器支持的额外参数：

```bash
fastforge build --platform web \
  --flutter-build-args source-maps,base-href=/console/
```

无等号的条目按布尔开关处理，`key=value` 按键值参数处理。值本身包含逗号时，不应使用此入口。

## 打包参数

### 跳过清理

需要复用已有构建缓存时，可以跳过打包前的清理步骤：

```bash
fastforge package --platform macos --target zip --skip-clean
```

### 自定义兼容项目入口

```bash
fastforge package --platform macos --target dmg \
  --build-target lib/main_production.dart
```

### 输出目录

直接运行 `package` 时输出目录为 `dist/`。需要自定义目录或产物名时，使用工作流 action 的 `output`、`artifact-name`。

## 生命周期钩子

```bash
fastforge package --platform macos --target zip \
  --hook-pre './scripts/before-package.sh' \
  --hook-post './scripts/after-package.sh'
```

钩子通过系统 shell 执行。Fastforge 会提供以下环境变量：

- `PLATFORM`
- `PACKAGE_FORMAT`
- `BUILD_MODE`
- `OUTPUT_DIRECTORY`
- `BUILD_OUTPUT_DIRECTORY`
- `BUILD_OUTPUT_FILES`（以 `:` 分隔）

任意钩子返回非零退出码时，打包立即失败。

## 自动化

需要组合多个打包目标、发布或其他命令时，使用[本地工作流](workflows.md)。
