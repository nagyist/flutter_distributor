# msix

将 Flutter 应用构建为 Windows MSIX 包。MSIX 是微软的现代应用打包格式，提供可靠、安全的安装体验，支持自动更新、应用权限管理和企业部署场景。

## 环境要求

- Windows 10 版本 1809 或更高版本，或 Windows 11
- [MSIX 打包 SDK](https://docs.microsoft.com/zh-cn/windows/msix/)

## 使用方法

将 `make_config.yaml` 添加到你的项目 `windows/packaging/msix` 目录。

```yaml
display_name: HelloWorld
msix_version: 1.0.0.0
# logo_path: C:\path\to\logo.png
```

> 查看所有配置选项：[msix](https://github.com/YehudaKremer/msix)

运行：

```
fastforge package --platform windows --targets msix
```

## 相关链接

- [MSIX 工具包](https://github.com/YehudaKremer/msix)
- [什么是 MSIX？](https://docs.microsoft.com/zh-cn/windows/msix/)
- [构建和发布 Windows 应用程序](https://docs.flutter.dev/deployment/windows)
