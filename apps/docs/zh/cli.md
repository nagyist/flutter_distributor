# CLI

如何使用 Fastforge 的命令行界面（CLI）

## 安装

```shell
dart pub global activate fastforge
```

> **Windows 用户请注意：** 激活后，请确保 pub 缓存 bin 目录已添加到 PATH 环境变量中：
> 1. 打开 **系统属性** → **高级** → **环境变量**
> 2. 在 **用户变量** 中找到 `Path`，点击 **编辑**
> 3. 添加 `%APPDATA%\Pub\Cache\bin` 并点击 **确定**
> 4. 重启终端，然后运行 `fastforge --help` 验证

---

## 命令

> 这些命令按字母顺序排列，最常用的是 package、publish 和 release。

### Package

将应用程序打包为特定于平台的格式，并将结果放入文件夹中。

| Flag | Value | Required |
|------|-------|:--------:|
| `--platform` | 平台, 如 `android` | ✅ |
| `--targets` | 以逗号分隔的 maker 名称列表 | ✅ |
| `--skip-clean` | 跳过构建前的清理 | ❌ |
| `--hook-pre` | 打包前执行的 shell 命令 | ❌ |
| `--hook-post` | 打包后执行的 shell 命令 | ❌ |

示例：

```shell
fastforge package --platform=android --targets=aab,apk

fastforge package --platform=macos --target=zip --hook-pre 'echo "before"' --hook-post 'echo "after"'
```

### Publish

| Flag | Value | Required |
|------|-------|:--------:|
| `--path` | 路径, 如 `hello_world-1.0.0+1-android.apk` | ✅ |
| `--targets` | 以逗号分隔的 publisher 名称列表 | ✅ |

示例：

```shell
fastforge publish --path hello_world-1.0.0+1-android.apk --targets fir,pgyer
```

### Release

会根据配置文件（`distribute_options.yaml`），将你的应用打包成特定的格式并发布到分发平台。

| Flag | Value | Required |
|------|-------|:--------:|
| `--name` | 名称, e.g. `dev` | ✅ |
| `--skip-clean` | 跳过构建前的清理 | ❌ |

示例：

```shell
fastforge release --name dev
```

### Store catalog

通过 `.fastforge/config.yaml` 配置需要同步的 App Store 和 Google Play 应用：

```yaml
stores:
  appstore:
    apps:
      - bundle_id: com.example.myapp
        app_id: "1234567890" # 省略 bundle_id 时的可选回退值
  googleplay:
    apps:
      - package_name: com.example.myapp
```

然后使用统一命令拉取或推送所有已配置应用的 catalog 数据：

```shell
fastforge store catalog pull
fastforge store catalog push
```

统一命令从配置文件读取应用标识，认证继续使用现有环境变量：

| 商店 | 环境变量 |
|------|----------|
| App Store | `APP_STORE_CONNECT_KEY_ID`、`APP_STORE_CONNECT_ISSUER_ID`、`APP_STORE_CONNECT_KEY_PATH` |
| Google Play | `GOOGLE_PLAY_SERVICE_ACCOUNT_JSON`（服务账号 JSON 内容或 JSON 文件路径） |

Catalog 文件仍使用 `.fastforge/stores/appstore/` 和
`.fastforge/stores/googleplay/` 下的默认目录。命令会按配置顺序处理全部应用；
单个应用失败不会中断后续应用，最终会打印汇总并以错误状态退出。

对于 App Store 版本，`pull` 会把 `copyright` 等版本级元数据写入
`versions/<platform>/<version>/version.yaml`。修改该值后运行 `push`，即可更新
App Store Connect 中对应版本的版权信息。

对于 App Store 截图，`pull` 会在每个显示类型目录中写入隐藏 manifest。
`push` 会根据远端 ID 和校验值复用未变化的截图，替换内容已修改或处理失败的截图，
删除非空本地显示类型目录中已经不存在的远端截图，并按照本地文件名重新排序。
同步前可以使用独立命令
`fastforge appstore catalog push --app <bundle-id> --dry-run` 检查本地截图集。

---

## 资源说明

### `distribute_options.yaml`

完整的配置参考请查看[分发选项](./distribute-options.md)页面。
