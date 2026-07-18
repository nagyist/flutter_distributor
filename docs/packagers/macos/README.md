# macOS

Fastforge 支持把 macOS 应用打包为 DMG、PKG 或 ZIP。

## 格式

- [DMG](dmg.md)：磁盘映像
- [PKG](pkg.md)：安装器包
- [ZIP](zip.md)：压缩归档

## 当前状态

| 构建系统       | `package` 状态       |
| -------------- | -------------------- |
| Xcode          | DMG、PKG、ZIP 已支持 |
| 兼容构建适配器 | DMG、PKG、ZIP 已支持 |

## 环境要求

- macOS
- Xcode 命令行工具
- 项目自身需要的 SDK 和构建工具

Fastforge 根据项目文件选择 Xcode builder 或兼容构建适配器，随后把生成的 `.app` 交给对应 packager。

返回[打包器总览](../README.md)。
