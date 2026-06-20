# deb

Build your Flutter app as a Debian package (`.deb`) for installation on Debian-based Linux distributions such as Ubuntu, Debian, Linux Mint, and Pop!_OS. The DEB format is the standard package management format used by APT-based systems.

## Requirements

- Linux system (Debian/Ubuntu-based distribution recommended)
- Required tools: `dpkg`, `dpkg-deb` (typically pre-installed on Debian-based systems)

## Usage

Add `make_config.yaml` to your project `linux/packaging/deb` directory.

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
# You can also specify [metainfo](https://freedesktop.org/software/appstream/docs/chap-Quickstart.html) file
# which contains metadata of the app.
# metainfo: linux/packaging/myappid.appdata.xml
```

Run:

```
fastforge package --platform linux --targets deb
```

## Related Links

- [Build and release a Linux app](https://docs.flutter.dev/deployment/linux)
- [Packaging Debian packages, how it works](https://www.debian.org/doc/manuals/packaging-tutorial/packaging-tutorial.en.pdf)
