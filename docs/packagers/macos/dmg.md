# DMG

DMG 是 macOS 常用的磁盘映像分发格式。

当前版本使用内置 DMG maker，不再要求全局安装 `appdmg` Node.js 工具。

## 打包

项目能够生成 macOS `.app` 后即可使用：

```bash
fastforge package --platform macos --target dmg
```

签名、公证等额外流程可以通过 `package.hooks.post` 执行。

返回 [macOS](README.md)。
