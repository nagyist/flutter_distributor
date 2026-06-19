import 'dart:io';

import 'package:flutter_app_packager/src/api/app_package_maker.dart';
import 'package:flutter_app_packager/src/makers/pkg/make_pkg_config.dart';
import 'package:shell_executor/shell_executor.dart';

class AppPackageMakerPkg extends AppPackageMaker {
  @override
  String get name => 'pkg';
  @override
  String get platform => 'macos';
  @override
  String get packageFormat => 'pkg';

  @override
  MakeConfigLoader get configLoader {
    return MakePkgConfigLoader()
      ..platform = platform
      ..packageFormat = packageFormat;
  }

  @override
  Future<MakeResult> make(MakeConfig config) async {
    MakePkgConfig makeConfig = config as MakePkgConfig;
    File appFile = config.buildOutputFiles.first;

    File outputFile = config.outputFile;
    File unsignedPkgFile = File(
      outputFile.path.replaceFirst(
        '.$packageFormat',
        '-unsigned.$packageFormat',
      ),
    );

    final pkgBuild = [
      'productbuild',
      '--component',
      appFile.path,
      makeConfig.installPath ?? '/Applications/',
      unsignedPkgFile.path,
    ];
    if (null != makeConfig.scriptsPath) {
      pkgBuild.add('--scripts');
      pkgBuild.add(makeConfig.scriptsPath!);
    }

    await $('xcrun', pkgBuild);

    // 修复 pkg 元数据：expand → 编辑 → flatten
    // productbuild --component 生成的包有两个问题：
    // 1. Distribution 缺少 <domains>，导致 GUI Installer 安装位置不正确
    // 2. 组件 PackageInfo 包含 <relocate>，导致安装器重定向到已存在的构建产物
    final expandDir = Directory('${unsignedPkgFile.path}.expanded');
    if (expandDir.existsSync()) expandDir.deleteSync(recursive: true);
    await $('pkgutil', ['--expand', unsignedPkgFile.path, expandDir.path]);

    // 修复 1：Distribution 注入 <domains>
    final distributionFile = File('${expandDir.path}/Distribution');
    if (distributionFile.existsSync()) {
      var content = distributionFile.readAsStringSync();
      content = content.replaceFirst(
        '<options ',
        '<domains enable_local="true" enable_currentUserHome="false" enable_anywhere="false" />\n    <options ',
      );
      distributionFile.writeAsStringSync(content);
    }

    // 修复 2：组件 PackageInfo 移除 <relocate> 等重定向元素
    // 这些元素会导致 Installer 将应用安装到已存在的构建路径而非 /Applications
    for (final entry in expandDir.listSync()) {
      if (entry is! Directory || !entry.path.endsWith('.pkg')) continue;
      final componentDir = Directory('${entry.path}.expanded');
      if (componentDir.existsSync()) componentDir.deleteSync(recursive: true);
      await $('pkgutil', ['--expand', entry.path, componentDir.path]);

      final pkgInfoFile = File('${componentDir.path}/PackageInfo');
      if (pkgInfoFile.existsSync()) {
        var content = pkgInfoFile.readAsStringSync();
        content = content.replaceAll(RegExp(r'<relocate>.*?</relocate>', dotAll: true), '');
        content = content.replaceAll(RegExp(r'<upgrade-bundle>.*?</upgrade-bundle>', dotAll: true), '');
        content = content.replaceAll(RegExp(r'<update-bundle\s*/>', dotAll: true), '');
        content = content.replaceAll(RegExp(r'<atomic-update-bundle\s*/>', dotAll: true), '');
        content = content.replaceAll(RegExp(r'<strict-identifier>.*?</strict-identifier>', dotAll: true), '');
        pkgInfoFile.writeAsStringSync(content);
      }

      entry.deleteSync(recursive: true);
      await $('pkgutil', ['--flatten', componentDir.path, entry.path]);
      componentDir.deleteSync(recursive: true);
    }

    unsignedPkgFile.deleteSync();
    await $('pkgutil', ['--flatten', expandDir.path, unsignedPkgFile.path]);
    expandDir.deleteSync(recursive: true);
    if (makeConfig.signIdentity != null) {
      await $('xcrun', [
        'productsign',
        '--sign',
        makeConfig.signIdentity!,
        unsignedPkgFile.path,
        outputFile.path,
      ]);
      unsignedPkgFile.deleteSync();
    } else {
      unsignedPkgFile.renameSync(outputFile.path);
    }
    return Future.value(resultResolver.resolve(config));
  }
}
