# App Package Analysis

English | [简体中文](../../zh-Hans/tools/analyze.md)

`fastforge analyze` reads an application's name, identifier, version, and build number from an artifact and outputs JSON.

## Supported Formats

| Format        | Platform restrictions | Dependencies                                                 |
| ------------- | --------------------- | ------------------------------------------------------------ |
| APK           | No fixed host         | `aapt2` under `ANDROID_HOME`                                 |
| AAB           | No fixed host         | `aapt2`, or `BUNDLETOOL`                                     |
| IPA           | No fixed host         | No external parsing tool                                     |
| DMG           | macOS only            | `hdiutil`                                                    |
| `.app` bundle | macOS only            | Local `Info.plist`; architecture detection uses system tools |

## Write to the Terminal

```bash
fastforge analyze dist/app-release.apk
```

Example structure:

```json
{
  "platform": "android",
  "identifier": "com.example.app",
  "name": "Example",
  "version": "1.0.0",
  "buildNumber": 1
}
```

## Write to a File

```bash
fastforge analyze dist/app-release.apk \
  --output analysis.json
```

## bundletool Fallback for AAB

If no working `aapt2` can be found, point `BUNDLETOOL` to a bundletool JAR:

```bash
export BUNDLETOOL=/path/to/bundletool.jar
fastforge analyze dist/app-release.aab
```

## CI Usage

```bash
fastforge analyze "$ARTIFACT" --output artifact-metadata.json
```

The command exits with a nonzero status for unsupported extensions, missing tools, or artifacts that cannot be parsed.
