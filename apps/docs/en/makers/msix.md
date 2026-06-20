# msix

Build your Flutter app as a Windows MSIX package. MSIX is Microsoft's modern application packaging format that provides a reliable, secure installation experience with support for automatic updates, app permissions management, and enterprise deployment scenarios.

## Requirements

- Windows 10 version 1809 or later, or Windows 11
- [MSIX Packaging SDK](https://docs.microsoft.com/en-us/windows/msix/)

## Usage

Add `make_config.yaml` to your project `windows/packaging/msix` directory.

```yaml
display_name: HelloWorld
msix_version: 1.0.0.0
# logo_path: C:\path\to\logo.png
```

> View all configuration options: [msix](https://github.com/YehudaKremer/msix)

Run:

```
fastforge package --platform windows --targets msix
```

## Related Links

- [MSIX Toolkit](https://github.com/YehudaKremer/msix)
- [What is MSIX?](https://docs.microsoft.com/en-us/windows/msix/)
- [Build and release a Windows app](https://docs.flutter.dev/deployment/windows)
