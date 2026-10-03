#!/usr/bin/env bash
# 编译原生设置应用；只有用户在界面点击保存才写入偏好。
set -euo pipefail
LOCALGLOSS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOCALGLOSS_SETTINGS_APP="${1:?output app path required}"
test ! -e "$LOCALGLOSS_SETTINGS_APP"
mkdir -p "$LOCALGLOSS_SETTINGS_APP/Contents/MacOS" "$LOCALGLOSS_SETTINGS_APP/Contents/Resources"
if test -d /Library/Developer/CommandLineTools; then
  export DEVELOPER_DIR=/Library/Developer/CommandLineTools
else
  export DEVELOPER_DIR="$(/usr/bin/xcode-select -p)"
fi
xcrun swiftc -O -file-prefix-map "$LOCALGLOSS_ROOT=." -debug-prefix-map "$LOCALGLOSS_ROOT=." -target arm64-apple-macos13.0 -framework AppKit \
  "$LOCALGLOSS_ROOT"/apps/localgloss-settings/*.swift \
  -o "$LOCALGLOSS_SETTINGS_APP/Contents/MacOS/localgloss-settings"
python3 - "$LOCALGLOSS_SETTINGS_APP" <<'PY'
import plistlib,pathlib,sys,os
p=pathlib.Path(sys.argv[1])/'Contents/Info.plist'
info={'CFBundleIdentifier':'local.localgloss.settings','CFBundleName':'LocalGloss Settings','CFBundleDisplayName':'LocalGloss 设置','CFBundleExecutable':'localgloss-settings','CFBundlePackageType':'APPL','CFBundleVersion':'12','CFBundleIconFile':'LocalGloss.icns','LocalGlossRelease':'0.3.0-alpha.5','CFBundleShortVersionString':'0.3.0','LSMinimumSystemVersion':'13.0','NSPrincipalClass':'NSApplication'}
if os.environ.get('LOCALGLOSS_SETTINGS_PREVIEW') == '1':
    info.update(CFBundleIdentifier='local.localgloss.settings.preview', LocalGlossPreview=True, CFBundleDisplayName='LocalGloss 词条预览')
p.write_bytes(plistlib.dumps(info))
PY
bash "$LOCALGLOSS_ROOT/scripts/make-app-icon.sh" "$LOCALGLOSS_SETTINGS_APP/Contents/Resources/LocalGloss.icns"
codesign --sign - "$LOCALGLOSS_SETTINGS_APP"
