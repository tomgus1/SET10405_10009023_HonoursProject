#!/usr/bin/env bash
# Packages a Linux build into a portable .AppImage that runs across distros.
#
# Uses linuxdeploy to bundle the executable's non-core shared library
# dependencies (GTK, fontconfig, etc.) into the AppImage, so it doesn't rely
# on those being present on the machine it's run on. This is paired with
# building on an older Ubuntu runner (see the workflow files) to keep the
# glibc baseline low, since glibc itself is forward- but not
# backward-compatible and is deliberately NOT bundled.
#
# Usage: make-appimage.sh <app_id> <bin_path> [<bundle_dir>] [<icon_path>]
#   app_id      short name used for the .desktop entry, icon and output file
#   bin_path    path to the executable that should be launched
#   bundle_dir  optional: directory whose full contents sit alongside the
#               executable at runtime (e.g. Flutter's data/ and lib/ dirs)
#   icon_path   optional: PNG to use as the app icon; a placeholder is
#               generated when omitted
set -euo pipefail

APP_ID="$1"
BIN_PATH="$2"
BUNDLE_DIR="${3:-}"
ICON_PATH="${4:-}"
EXEC_NAME="$(basename "$BIN_PATH")"
OUT_DIR="$PWD"

WORKDIR="$(mktemp -d)"
APPDIR="$WORKDIR/AppDir"
mkdir -p "$APPDIR/usr/bin"

if [ -n "$BUNDLE_DIR" ]; then
  cp -a "$BUNDLE_DIR"/. "$APPDIR/usr/bin/"
else
  cp "$BIN_PATH" "$APPDIR/usr/bin/$EXEC_NAME"
fi
chmod +x "$APPDIR/usr/bin/$EXEC_NAME"

if [ -n "$ICON_PATH" ] && [ -f "$ICON_PATH" ]; then
  ICON_FILE="$WORKDIR/$APP_ID.png"
  cp "$ICON_PATH" "$ICON_FILE"
else
  ICON_FILE="$WORKDIR/$APP_ID.png"
  python3 - "$ICON_FILE" <<'PYEOF'
import struct, sys, zlib

def chunk(tag, data):
    return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data))

w = h = 256
row = b"\x00" + bytes((58, 110, 165, 255)) * w
raw = row * h
png = b"\x89PNG\r\n\x1a\n"
png += chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
png += chunk(b"IDAT", zlib.compress(raw))
png += chunk(b"IEND", b"")
with open(sys.argv[1], "wb") as f:
    f.write(png)
PYEOF
fi

DESKTOP_FILE="$WORKDIR/$APP_ID.desktop"
cat > "$DESKTOP_FILE" <<EOF
[Desktop Entry]
Type=Application
Name=$APP_ID
Exec=$EXEC_NAME
Icon=$APP_ID
Categories=Utility;
EOF

curl -fsSL -o "$WORKDIR/linuxdeploy" \
  https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x "$WORKDIR/linuxdeploy"

(
  cd "$WORKDIR"
  ./linuxdeploy --appimage-extract-and-run \
    --appdir "$APPDIR" \
    --executable "$APPDIR/usr/bin/$EXEC_NAME" \
    --desktop-file "$DESKTOP_FILE" \
    --icon-file "$ICON_FILE" \
    --output appimage
)

mv "$WORKDIR"/*.AppImage "$OUT_DIR/$APP_ID-x86_64.AppImage"
