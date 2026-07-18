# 打包器

本节按平台组织 Fastforge 的打包能力。每个平台页面都会标注当前接入的构建系统、输出格式和环境要求。

## 平台

| 平台                         | 格式                                                          | `package` 当前接入范围 |
| ---------------------------- | ------------------------------------------------------------- | ---------------------- |
| [Android](android/README.md) | [APK](android/apk.md)、[AAB](android/aab.md)                  | Gradle                 |
| [iOS](ios/README.md)         | [IPA](ios/ipa.md)                                             | Xcode                  |
| [macOS](macos/README.md)     | [DMG](macos/dmg.md)、[PKG](macos/pkg.md)、[ZIP](macos/zip.md) | Xcode、兼容构建适配器  |

仓库中还包含 Linux、Windows、Web 和 OpenHarmony 的 packager，但当前 `fastforge package` 尚未把这些路径接入 CLI。本节只为当前可验证的平台和格式提供详细页面。

## 命令选择

- 需要可分发格式：优先使用 `fastforge package`。
- 只需要兼容构建适配器的原始产物：使用 `fastforge build`。
- 需要多任务、输出目录或发布：使用 `fastforge workflow`。

通用流程、参数与钩子见[打包](../packaging.md)。
