# AAB

AAB（Android App Bundle）用于 Google Play 分发。

## 打包

```bash
fastforge package --platform android --target aab
```

Fastforge 使用 Gradle builder，并把最终 AAB 整理到 `dist/`。

## 兼容构建

当前兼容构建适配器可以单独生成 AAB，但尚未接入 Android packager：

```bash
fastforge build --platform android --target aab
```

## 上传 Google Play

通用 `fastforge publish` 当前没有 `playstore` target。上传和轨道管理请使用：

```bash
fastforge googleplay bundle upload --help
fastforge googleplay track update --help
```

认证和命令说明见 [Google Play](../../stores/googleplay.md)。

返回 [Android](README.md)。
