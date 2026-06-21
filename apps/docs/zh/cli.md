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

---

## 资源说明

### `distribute_options.yaml`

完整的配置参考请查看[分发选项](./distribute-options.md)页面。
