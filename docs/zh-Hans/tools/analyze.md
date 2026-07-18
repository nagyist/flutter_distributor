# 应用包分析

[English](../../en/tools/analyze.md) | 简体中文

`fastforge analyze` 从应用产物读取名称、标识符、版本和构建号，并输出 JSON。

## 支持格式

| 格式          | 平台限制       | 依赖                                    |
| ------------- | -------------- | --------------------------------------- |
| APK           | 无固定宿主限制 | `ANDROID_HOME` 中的 `aapt2`             |
| AAB           | 无固定宿主限制 | `aapt2`，或 `BUNDLETOOL`                |
| IPA           | 无固定宿主限制 | 无外部解析工具                          |
| DMG           | 仅 macOS       | `hdiutil`                               |
| `.app` bundle | 仅 macOS       | 本地 `Info.plist`；架构检测使用系统工具 |

## 输出到终端

```bash
fastforge analyze dist/app-release.apk
```

示例结构：

```json
{
  "platform": "android",
  "identifier": "com.example.app",
  "name": "Example",
  "version": "1.0.0",
  "buildNumber": 1
}
```

## 输出到文件

```bash
fastforge analyze dist/app-release.apk \
  --output analysis.json
```

## AAB 的 bundletool 回退

找不到可用 `aapt2` 时，可以通过 `BUNDLETOOL` 指向 bundletool JAR：

```bash
export BUNDLETOOL=/path/to/bundletool.jar
fastforge analyze dist/app-release.aab
```

## CI 用法

```bash
fastforge analyze "$ARTIFACT" --output artifact-metadata.json
```

命令遇到不支持的扩展名、缺少工具或无法解析的产物时会以非零状态退出。
