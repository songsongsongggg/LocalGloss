#!/usr/bin/env bash
# 使用系统 AppKit 和 iconutil 生成应用图标，不下载素材或引入依赖。
set -euo pipefail
LOCALGLOSS_ICON_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOCALGLOSS_ICON_OUTPUT="${1:?output icns path required}"
test ! -e "$LOCALGLOSS_ICON_OUTPUT"
LOCALGLOSS_ICON_WORK="$(mktemp -d "${TMPDIR:-/tmp}/localgloss-icon.XXXXXXXX")"
trap 'rm -rf "$LOCALGLOSS_ICON_WORK"' EXIT
export DEVELOPER_DIR=/Library/Developer/CommandLineTools
xcrun swift "$LOCALGLOSS_ICON_ROOT/scripts/make-app-icon.swift" "$LOCALGLOSS_ICON_WORK/LocalGloss.iconset"
/usr/bin/iconutil --convert icns --output "$LOCALGLOSS_ICON_OUTPUT" "$LOCALGLOSS_ICON_WORK/LocalGloss.iconset"
