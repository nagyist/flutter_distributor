# deb

将 Flutter 应用构建为 Debian 软件包（`.deb`），用于安装在基于 Debian 的 Linux 发行版上，如 Ubuntu、Debian、Linux Mint 和 Pop!_OS。DEB 格式是基于 APT 的系统的标准包管理格式。

## 环境要求

- Linux 系统（推荐使用 Debian/Ubuntu 系发行版）
- 必需工具：`dpkg`、`dpkg-deb`（通常在 Debian 系系统中已预装）

## 使用方法

将 `make_config.yaml` 添加到您的项目 `linux/packaging/deb` 目录。

```yaml
display_name: Hello World
package_name: hello-world
maintainer:
  name: LiJianying
  email: lijy91@foxmail.com
co_authors:
  - name: Kingkor Roy Tirtho
    email: krtirtho@gmail.com
priority: optional
section: x11
installed_size: 6604
essential: false
icon: assets/logo.png

postinstall_scripts:
  - echo "Installed my awesome app"
postuninstall_scripts:
  - echo "Surprised Pickachu face"

keywords:
  - Hello
  - World
  - Test
  - Application

generic_name: Music Application

categories:
  - Music
  - Media

startup_notify: true
# 您也可以指定 [metainfo](https://freedesktop.org/software/appstream/docs/chap-Quickstart.html) 文件
# 其中包含应用的元数据。
# metainfo: linux/packaging/myappid.appdata.xml
```

运行：

```
fastforge package --platform linux --targets deb
```

## 相关链接

- [构建和发布 Linux 应用程序](https://docs.flutter.dev/deployment/linux)
- [Debian 打包教程](https://www.debian.org/doc/manuals/packaging-tutorial/packaging-tutorial.en.pdf)
