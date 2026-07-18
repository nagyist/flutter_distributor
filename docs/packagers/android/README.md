# Android

Fastforge 通过 Gradle 构建并整理 Android 应用产物。

## 格式

- [APK](apk.md)：可直接安装或侧载
- [AAB](aab.md)：用于 Google Play 分发

## 当前状态

| 构建系统       | `package` 状态            |
| -------------- | ------------------------- |
| Gradle         | APK、AAB 已支持           |
| 兼容构建适配器 | 暂未接通 Android packager |

## 环境要求

- Android SDK
- 可用的 Gradle 和 Android 工具链
- 需要分析 APK/AAB 时，配置 `ANDROID_HOME` 和 `aapt2`

`package` 会从项目中定位 Gradle 配置并调用 Android builder。使用兼容构建适配器的项目，目前可以通过 `fastforge build` 生成原始产物，但不能直接进入 Android packager。

返回[打包器总览](../README.md)。
