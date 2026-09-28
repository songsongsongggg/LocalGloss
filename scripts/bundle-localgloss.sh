#!/usr/bin/env bash
# 只创建项目内新构建目录，不安装、注册输入源、结束进程或覆盖旧应用。
set -euo pipefail
LOCALGLOSS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$LOCALGLOSS_ROOT"
test "$(uname -m)" = arm64 || { echo 'Release bundles currently support Apple Silicon only' >&2; exit 2; }
python3 scripts/prepare-release-data.py
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$LOCALGLOSS_ROOT=. --remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=/cargo"
cargo build --offline --locked --release -p localgloss-macos
LOCALGLOSS_BUILD_ID="$(date -u +%Y%m%dT%H%M%SZ)"
LOCALGLOSS_OUTPUT="$LOCALGLOSS_ROOT/dist/$LOCALGLOSS_BUILD_ID"
LOCALGLOSS_APP="$LOCALGLOSS_OUTPUT/LocalGloss.app"
test ! -e "$LOCALGLOSS_OUTPUT"
mkdir -p "$LOCALGLOSS_APP/Contents/MacOS" "$LOCALGLOSS_APP/Contents/Resources"
cp "${CARGO_TARGET_DIR:-target}/release/localgloss-macos" "$LOCALGLOSS_APP/Contents/MacOS/"
/usr/bin/xcrun strip -x "$LOCALGLOSS_APP/Contents/MacOS/localgloss-macos"
cp apps/localgloss-macos/Info.plist "$LOCALGLOSS_APP/Contents/"
cp -R apps/localgloss-macos/resources/. "$LOCALGLOSS_APP/Contents/Resources/"
cp assets/cedict/generated/dict.tsv "$LOCALGLOSS_APP/Contents/Resources/"
cp assets/cedict/generated/glossary-en.tsv "$LOCALGLOSS_APP/Contents/Resources/"
cp LICENSE "$LOCALGLOSS_APP/Contents/Resources/"
cp README.md "$LOCALGLOSS_APP/Contents/Resources/"
mkdir -p "$LOCALGLOSS_APP/Contents/Resources/notices"
cp assets/cedict/LICENSE.txt "$LOCALGLOSS_APP/Contents/Resources/notices/CC-CEDICT-LICENSE.txt"
cp assets/cedict/README.md "$LOCALGLOSS_APP/Contents/Resources/notices/CC-CEDICT-README.md"
cp assets/cedict/generated/manifest.json "$LOCALGLOSS_APP/Contents/Resources/notices/data-manifest.json"
cp NOTICE.md DATA-SOURCES.md PRIVACY.md "$LOCALGLOSS_APP/Contents/Resources/notices/"
python3 scripts/collect-notices.py "$LOCALGLOSS_APP/Contents/Resources/notices/dependencies"
python3 scripts/make-menu-icon.py "$LOCALGLOSS_APP/Contents/Resources/localgloss-menu.pdf"
bash scripts/build-settings.sh "$LOCALGLOSS_APP/Contents/Resources/LocalGloss Settings.app"
/usr/bin/plutil -lint "$LOCALGLOSS_APP/Contents/Info.plist"
/usr/bin/codesign --sign - "$LOCALGLOSS_APP"
/usr/bin/codesign --verify --deep --strict "$LOCALGLOSS_APP"
python3 - "$LOCALGLOSS_APP" <<'PY'
import hashlib, json, pathlib, subprocess, sys
app=pathlib.Path(sys.argv[1])
files={str(path.relative_to(app)):hashlib.sha256(path.read_bytes()).hexdigest()
       for path in app.rglob('*') if path.is_file()}
root=pathlib.Path.cwd()
revision_file=root/'SOURCE-COMMIT'
revision=revision_file.read_text().strip() if revision_file.exists() else subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
dirty=None if revision_file.exists() else bool(subprocess.check_output(['git','status','--porcelain']))
report={'product':'LocalGloss','upstream_commit':'f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5',
        'source_commit':revision,
        'release':'0.2.0-beta.1', 'architecture':'arm64', 'source_dirty':dirty,
        'signing':'ad-hoc; no Developer ID; not notarized','installed':False,'files':files}
(app.parent/'build-manifest.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print('app:',app)
print('manifest:',app.parent/'build-manifest.json')
PY
