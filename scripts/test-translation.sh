#!/usr/bin/env bash
# 使用明确指定的随包词表做离线回归；不读取个人设置、不安装或切换输入法。
set -euo pipefail
if [[ $# -ne 1 ]]; then
  echo 'usage: bash scripts/test-translation.sh <bundle-resource-directory>' >&2
  exit 2
fi
LOCALGLOSS_RESOURCE_DIR="$1"
if [[ ! -f "$LOCALGLOSS_RESOURCE_DIR/dict.tsv" || ! -f "$LOCALGLOSS_RESOURCE_DIR/glossary-en.tsv" ]]; then
  echo 'Resource directory must contain dict.tsv and glossary-en.tsv' >&2
  exit 2
fi
LOCALGLOSS_TEST_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$LOCALGLOSS_TEST_ROOT/scripts/dev.sh" test
for LOCALGLOSS_EXAMPLE in translation_check mixed_check; do
  bash "$LOCALGLOSS_TEST_ROOT/scripts/dev.sh" cargo run --offline --locked \
    -p localgloss-engine --example "$LOCALGLOSS_EXAMPLE" -- "$LOCALGLOSS_RESOURCE_DIR"
done
