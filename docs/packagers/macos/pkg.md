# PKG

PKG 是 macOS 安装器包格式。

## 配置文件

PKG packager 会读取：

```text
macos/packaging/pkg/make_config.yaml
```

该文件缺失或无效时打包会失败。

## 打包

```bash
fastforge package --platform macos --target pkg
```

需要上传到 App Store 时：

```bash
fastforge publish --path dist/MyApp.pkg --target appstore
```

返回 [macOS](README.md)。
