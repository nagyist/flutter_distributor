# iOS

Fastforge 通过 Xcode 构建并整理 iOS 应用产物。所有 iOS 操作都需要 macOS 与 Xcode。

## 格式

- [IPA](ipa.md)：iOS 归档分发格式

## 当前状态

| 构建系统       | `package` 状态        |
| -------------- | --------------------- |
| Xcode          | IPA 已支持            |
| 兼容构建适配器 | 暂未接通 iOS packager |

Xcode 项目可以直接使用 `fastforge package`。使用兼容构建适配器的项目，目前只能通过 `fastforge build` 生成 IPA；该路径必须提供 export options plist 或 export method。

返回[打包器总览](../README.md)。
