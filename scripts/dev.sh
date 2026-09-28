#!/usr/bin/env bash
# 仅使用项目内工具与 CLT，正常构建不联网、不安装输入法。
set -euo pipefail
LOCALGLOSS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if test -d "$LOCALGLOSS_ROOT/.tools/rust/bin"; then
  export PATH="$LOCALGLOSS_ROOT/.tools/rust/bin:$PATH"
fi
if test -d "$LOCALGLOSS_ROOT/.tools/cargo-home"; then
  export CARGO_HOME="$LOCALGLOSS_ROOT/.tools/cargo-home"
fi
if test -d /Library/Developer/CommandLineTools; then
  export DEVELOPER_DIR=/Library/Developer/CommandLineTools
else
  export DEVELOPER_DIR="$(/usr/bin/xcode-select -p)"
fi
export SDKROOT="$(/usr/bin/xcrun --show-sdk-path)"
cd "$LOCALGLOSS_ROOT"
case "${1:-}" in
  test) cargo test --offline --locked -p localgloss-engine -p localgloss-macos ;;
  check) cargo fmt --all -- --check; cargo clippy --offline --locked -p localgloss-engine -p localgloss-macos --all-targets -- -D warnings ;;
  bundle) bash scripts/bundle-localgloss.sh ;;
  cargo) shift; cargo "$@" ;;
  *) echo 'usage: scripts/dev.sh {test|check|bundle|cargo <args>}' >&2; exit 2 ;;
esac
