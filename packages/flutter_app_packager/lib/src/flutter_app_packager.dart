import 'dart:io';

import 'package:flutter_app_packager/src/api/app_package_maker.dart';
import 'package:flutter_app_packager/src/makers/makers.dart';
import 'package:flutter_app_packager/src/makers/pacman/app_package_maker_pacman.dart';

class FlutterAppPackager {
  final List<AppPackageMaker> _makers = [
    AppPackageMakerAab(),
    AppPackageMakerApk(),
    AppPackageMakerApp(),
    AppPackageMakerAppImage(),
    AppPackageMakerDeb(),
    AppPackageMakerDirect('linux'),
    AppPackageMakerDirect('windows'),
    AppPackageMakerDirect('web'),
    AppPackageMakerDmg(),
    AppPackageMakerExe(),
    AppPackageMakerHap(),
    AppPackageMakerIpa(),
    AppPackageMakerMsix(),
    AppPackageMakerPkg(),
    AppPackageMakerRPM(),
    AppPackageMakerPacman(),
    AppPackageMakerZip('linux'),
    AppPackageMakerZip('macos'),
    AppPackageMakerZip('windows'),
    AppPackageMakerZip('web'),
  ];

  Future<MakeResult> package(
    String platform,
    String target,
    Map<String, dynamic>? arguments,
    Directory outputDirectory, {
    required Directory buildOutputDirectory,
    required List<File> buildOutputFiles,
  }) {
    final maker = _makers.firstWhere((e) => e.match(platform, target));
    final config = maker.configLoader.load(
      arguments,
      outputDirectory,
      buildOutputDirectory: buildOutputDirectory,
      buildOutputFiles: buildOutputFiles,
    );
    return maker.make(config);
  }
}
