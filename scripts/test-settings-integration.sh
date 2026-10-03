#!/usr/bin/env bash
# 只在新临时目录生成固定虚构配置，不使用个人 settings.json。
set -euo pipefail
LOCALGLOSS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOCALGLOSS_FIXTURE="$(mktemp -d "${TMPDIR:-/tmp}/localgloss-integration.XXXXXX")"
bash "$LOCALGLOSS_ROOT/scripts/test-settings.sh" --integration-output "$LOCALGLOSS_FIXTURE"
bash "$LOCALGLOSS_ROOT/scripts/dev.sh" cargo run --offline --locked -p localgloss-engine --example settings_roundtrip -- \
  "$LOCALGLOSS_FIXTURE/settings.json" "$LOCALGLOSS_ROOT/assets/cedict/generated"
