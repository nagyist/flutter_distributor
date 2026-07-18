# ZIP

ZIP 将 macOS `.app` bundle 压缩为便于下载和发布的归档文件。

## 打包

```bash
fastforge package --platform macos --target zip
```

## 发布

```bash
fastforge publish --path dist/MyApp.zip --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

ZIP 适合发布到 GitHub Releases、S3 兼容存储或自定义下载服务。

返回 [macOS](README.md)。
