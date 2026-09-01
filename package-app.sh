#!/usr/bin/env bash
# Builds the release binary (regenerating the icon via build.rs) and
# assembles a real .app bundle using Apple's own command-line tools:
#   plutil / PlistBuddy - build Info.plist from scratch, no template file
#   codesign            - ad-hoc sign so Gatekeeper doesn't just refuse to
#                          open an unsigned bundle
#   lsregister          - force Finder/LaunchServices to drop stale icon
#                          cache for this bundle
#
# Usage: ./package-app.sh
# Output: target/release/bundle/Tor Browser Installer.app

set -euo pipefail

APP_NAME="Tor Browser Installer"
BUNDLE_ID="org.torproject.browserinstaller"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Pull these from Cargo.toml directly rather than hardcoding, so the
# script can't silently drift out of sync with the crate.
BIN_NAME="$(awk -F'"' '/^name *=/{print $2; exit}' Cargo.toml)"
VERSION="$(awk -F'"' '/^version *=/{print $2; exit}' Cargo.toml)"

if [[ -z "$BIN_NAME" || -z "$VERSION" ]]; then
    echo "error: couldn't read name/version from Cargo.toml" >&2
    exit 1
fi

echo "==> cargo build --release (binary: $BIN_NAME, version: $VERSION)"
cargo build --release

BIN_PATH="target/release/${BIN_NAME}"
ICNS_PATH="target/generated-assets/icon.icns"

if [[ ! -f "$BIN_PATH" ]]; then
    echo "error: no binary at $BIN_PATH - contents of target/release:" >&2
    ls target/release >&2
    exit 1
fi
if [[ ! -f "$ICNS_PATH" ]]; then
    echo "error: no icon at $ICNS_PATH - did build.rs run and succeed?" >&2
    exit 1
fi

APP_DIR="target/release/bundle/${APP_NAME}.app"
CONTENTS="${APP_DIR}/Contents"

echo "==> assembling ${APP_DIR}"
rm -rf "$APP_DIR"
mkdir -p "${CONTENTS}/MacOS" "${CONTENTS}/Resources"

cp "$BIN_PATH" "${CONTENTS}/MacOS/${BIN_NAME}"
chmod +x "${CONTENTS}/MacOS/${BIN_NAME}"
cp "$ICNS_PATH" "${CONTENTS}/Resources/icon.icns"

# --- Build Info.plist with Apple's own tools, no template file ---
PLIST="${CONTENTS}/Info.plist"
PB="/usr/libexec/PlistBuddy"

plutil -create xml1 "$PLIST"
"$PB" -c "Add :CFBundleName string ${APP_NAME}" "$PLIST"
"$PB" -c "Add :CFBundleDisplayName string ${APP_NAME}" "$PLIST"
"$PB" -c "Add :CFBundleIdentifier string ${BUNDLE_ID}" "$PLIST"
"$PB" -c "Add :CFBundleVersion string ${VERSION}" "$PLIST"
"$PB" -c "Add :CFBundleShortVersionString string ${VERSION}" "$PLIST"
"$PB" -c "Add :CFBundleExecutable string ${BIN_NAME}" "$PLIST"
"$PB" -c "Add :CFBundleIconFile string icon.icns" "$PLIST"
"$PB" -c "Add :CFBundlePackageType string APPL" "$PLIST"
"$PB" -c "Add :LSMinimumSystemVersion string 11.0" "$PLIST"
"$PB" -c "Add :NSHighResolutionCapable bool true" "$PLIST"

# --- Ad-hoc code sign so Gatekeeper doesn't just refuse to launch it ---
echo "==> codesign (ad-hoc)"
codesign --force --deep --sign - "$APP_DIR"

# --- Refresh LaunchServices' icon cache for this bundle ---
LSREGISTER="/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister"
if [[ -x "$LSREGISTER" ]]; then
    "$LSREGISTER" -f "$APP_DIR"
fi

echo "==> done: ${APP_DIR}"
echo "    open \"${APP_DIR}\""