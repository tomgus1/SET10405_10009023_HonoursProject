#!/usr/bin/env bash
# Packages a macOS build into a distributable, ad-hoc signed .dmg.
#
# Usage:
#   make-macos-dmg.sh <app_id> --bin <bin_path>        wraps a raw executable
#                                                       in a minimal .app bundle
#   make-macos-dmg.sh <app_id> --app <app_bundle_path>  uses an existing .app as-is
set -euo pipefail

APP_ID="$1"
MODE="$2"
SRC="$3"

if [ "$MODE" = "--bin" ]; then
  EXEC_NAME="$(basename "$SRC")"
  APP_BUNDLE="$APP_ID.app"
  rm -rf "$APP_BUNDLE"
  mkdir -p "$APP_BUNDLE/Contents/MacOS"
  cp "$SRC" "$APP_BUNDLE/Contents/MacOS/$EXEC_NAME"
  chmod +x "$APP_BUNDLE/Contents/MacOS/$EXEC_NAME"

  cat > "$APP_BUNDLE/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key><string>$EXEC_NAME</string>
  <key>CFBundleIdentifier</key><string>dev.timgraham.$APP_ID</string>
  <key>CFBundleName</key><string>$APP_ID</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>1.0</string>
</dict>
</plist>
EOF
elif [ "$MODE" = "--app" ]; then
  APP_BUNDLE="$SRC"
else
  echo "usage: make-macos-dmg.sh <app_id> --bin <path> | --app <path>" >&2
  exit 1
fi

codesign --force --deep --sign - "$APP_BUNDLE"
hdiutil create -volname "$APP_ID" -srcfolder "$APP_BUNDLE" -ov -format UDZO "$APP_ID.dmg"
