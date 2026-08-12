const { execFileSync } = require('child_process');

// electron-builder skips signing entirely in CI (no Developer ID cert available).
// Ad-hoc sign the .app here, before it gets sealed into the .dmg, so the build
// isn't left completely unsigned (still triggers Gatekeeper's "unidentified
// developer" warning on downloaded copies, but avoids "app is damaged" errors).
exports.default = async function afterSign(context) {
  if (context.electronPlatformName !== 'darwin') {
    return;
  }
  const appPath = `${context.appOutDir}/${context.packager.appInfo.productFilename}.app`;
  execFileSync('codesign', ['--force', '--deep', '--sign', '-', appPath]);
};
