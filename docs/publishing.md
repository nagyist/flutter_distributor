# 发布

`publish` 用于把已经存在的文件或目录发送到指定发布目标。

## 基本用法

```bash
fastforge publish \
  --path dist/my-app.apk \
  --target fir \
  --publish-arg bundle_id=com.example.app
```

`--path` 和 `--target` 为必填参数。发布参数使用可重复的 `--publish-arg KEY=VALUE`：

```bash
fastforge publish \
  --path dist/app.zip \
  --target github \
  --publish-arg repo=owner/repository \
  --publish-arg release-tag=v1.0.0
```

## 发布目标

当前已接通 S3 兼容存储、应用分发、应用商店、Web 托管和自定义命令等 target：

- [S3 / MinIO、七牛、OSS、COS](publishers/s3.md)
- [fir.im](publishers/fir.md)与 [Firebase](publishers/firebase.md)
- [GitHub Releases](publishers/github.md)
- [App Store](publishers/appstore.md)与 [AppGallery](publishers/appgallery.md)
- [Vercel](publishers/vercel.md)与 [Custom](publishers/custom.md)

完整列表见[发布器总览](publishers/README.md)。

## 多步骤自动化

需要组合构建、打包、发布和 shell 命令时，使用[本地工作流](workflows.md)。发布凭证通过进程环境变量传入，非敏感参数写入 action 的 `with` 字段。

## 当前未接通

- `pgyer` 当前不是有效 target。
- `playstore` 当前不是通用 target；Google Play 操作请使用 `fastforge googleplay`。

可用 target 以 `fastforge publish` 的命令提示为准。
