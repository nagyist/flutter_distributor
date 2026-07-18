# APK

APK 是 Android 可直接安装的应用包。

## 打包

```bash
fastforge package --platform android --target apk
```

Fastforge 使用 Gradle builder，并把最终 APK 整理到 `dist/`。

## 工作流示例

```yaml
- name: Package APK
  uses: fastforge/package
  with:
    platform: android
    target: apk
    output: artifacts/
```

## 兼容构建

当前兼容构建适配器可以单独生成 APK，但尚未接入 Android packager：

```bash
fastforge build --platform android --target apk
```

返回 [Android](README.md)。
