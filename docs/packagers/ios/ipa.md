# IPA

IPA 是 iOS 应用的归档分发格式。

## 打包

Xcode 项目可以直接执行：

```bash
fastforge package --platform ios --target ipa
```

## 兼容构建

当前兼容构建适配器尚未接入 iOS packager。单独构建 IPA 时，可以使用 export options plist：

使用 export options plist：

```bash
fastforge build \
  --platform ios \
  --target ipa \
  --build-export-options-plist ios/ExportOptions.plist
```

或使用 export method：

```bash
fastforge build \
  --platform ios \
  --target ipa \
  --build-export-method app-store
```

## 发布到 App Store

```bash
fastforge publish --path dist/MyApp.ipa --target appstore
```

凭证、上传和审核流程见 [App Store 发布器](../../publishers/appstore.md)与 [App Store Connect](../../stores/appstore.md)。

返回 [iOS](README.md)。
