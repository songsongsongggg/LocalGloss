#!/usr/bin/env bash
# 使用虚构词条在新临时目录测试，不访问真实用户设置。
set -euo pipefail
LOCALGLOSS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export DEVELOPER_DIR=/Library/Developer/CommandLineTools
mkdir -p "$LOCALGLOSS_ROOT/target"
xcrun swiftc "$LOCALGLOSS_ROOT"/apps/localgloss-settings/{UserTerm,SettingsModel,ValidationError}.swift \
  "$LOCALGLOSS_ROOT/apps/localgloss-settings/tests/main.swift" -o "$LOCALGLOSS_ROOT/target/settings-tests"
"$LOCALGLOSS_ROOT/target/settings-tests"
