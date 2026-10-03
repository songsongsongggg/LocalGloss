#!/usr/bin/env python3
"""解析固定 Rime 简体词表并与 CC-CEDICT 低权重补充组合；不执行 YAML 或联网。"""
import re
from cedict import HAN

# 上游快照中的口语组合误读；只移除这些读音，不改变正确 dou 读音。
EXCLUDED_READINGS = {('都是', 'du shi'), ('都有', 'du you'), ('都要', 'du yao')}


def parse_rime(text):
    rows = text.splitlines()
    if '# Rime dictionary' not in rows or 'name: pinyin_simp' not in rows or '...' not in rows:
        raise ValueError('Unexpected Rime dictionary header')
    words, skipped, corrected = {}, 0, 0
    for number, row in enumerate(rows[rows.index('...') + 1:], rows.index('...') + 2):
        if not row or row.startswith('#'):
            continue
        fields = row.split('\t')
        if len(fields) != 3:
            raise ValueError(f'Malformed Rime row at line {number}')
        word, pinyin, weight = fields
        if not weight.isascii() or not weight.isdigit() or int(weight) > 0xffffffff:
            raise ValueError(f'Invalid Rime weight at line {number}')
        parts = pinyin.split(' ')
        if not HAN.fullmatch(word) or len(word) != len(parts) or not all(re.fullmatch('[a-z]+', part) for part in parts):
            skipped += 1
            continue
        key = (word, pinyin)
        if key in EXCLUDED_READINGS:
            corrected += 1
            continue
        words[key] = max(words.get(key, 1), int(weight), 1)
    return words, {'rime_entries': len(words), 'rime_skipped': skipped, 'rime_corrected': corrected}


def compose_candidates(rime, fallback):
    words = dict(rime)
    added = 0
    for row in fallback.splitlines():
        if not row or row.startswith('#'):
            continue
        word, pinyin, _ = row.split('\t')
        key = (word, pinyin)
        if key not in words and key not in EXCLUDED_READINGS:
            words[key] = 1
            added += 1
    header = '# Rime pinyin-simp / Rime and AOSP contributors: Apache-2.0; CC-CEDICT fallback / MDBG and LocalGloss contributors: CC-BY-SA-4.0. Preserve both source licenses; see bundled notices.\n'
    return header + ''.join(f'{word}\t{pinyin}\t{weight}\n' for (word, pinyin), weight in sorted(words.items())), added
