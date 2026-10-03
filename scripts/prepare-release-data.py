#!/usr/bin/env python3
"""从固定 Rime 与 CC-CEDICT 快照生成发布词表；完全离线，拒绝覆盖不同数据。"""
import gzip
import hashlib
import json
import pathlib
import argparse
from cedict import convert
from rime import EXCLUDED_READINGS, parse_rime, compose_candidates

ROOT = pathlib.Path(__file__).resolve().parents[1]
EXPECTED = '05bb7cf923fd24cd636a703da2b0172d3b8686c3de28f613ac924e57ea44a95a'
RIME_EXPECTED = '1420536dec32cb4c7b070bdbdbc07f7a72f342ff1c22b7945f120cac0d5e6cae'
RIME_COMMIT = '0c6861ef7420ee780270ca6d993d18d4101049d0'


def prepare(root=ROOT, output=None):
    directory = root / 'assets/cedict'
    data = (directory / 'cedict-ts.txt.gz').read_bytes()
    if hashlib.sha256(data).hexdigest() != EXPECTED:
        raise ValueError('CC-CEDICT archive checksum mismatch')
    text = gzip.decompress(data).decode('utf-8')
    if '#! license=https://creativecommons.org/licenses/by-sa/4.0/' not in text:
        raise ValueError('CC-CEDICT license header missing')
    supplement = (root / 'assets/localgloss/common-phrases.tsv').read_bytes()
    fallback, glossary, counts = convert(text, supplement=supplement.decode('utf-8'))
    rime_data = (root / 'assets/rime/pinyin_simp.dict.yaml.gz').read_bytes()
    if hashlib.sha256(rime_data).hexdigest() != RIME_EXPECTED:
        raise ValueError('Rime archive checksum mismatch')
    rime_words, rime_counts = parse_rime(gzip.decompress(rime_data).decode('utf-8'))
    if rime_counts['rime_entries'] < 50000:
        raise ValueError('Unexpectedly small Rime dictionary')
    dictionary, fallback_added = compose_candidates(rime_words, fallback)
    counts.pop('frequency_matched_entries')
    counts['cedict_dictionary_entries'] = counts['dictionary_entries']
    counts['dictionary_entries'] = len(rime_words) + fallback_added
    gloss_words = {row.split('\t')[0] for row in glossary.splitlines() if row and not row.startswith('#')}
    rime_word_forms = {word for word, _ in rime_words}
    counts.update(rime_counts, fallback_added_entries=fallback_added,
                  rime_word_forms=len(rime_word_forms),
                  rime_word_forms_with_gloss=len(rime_word_forms & gloss_words))
    if counts['dictionary_entries'] < 100000 or counts['glossary_words'] < 100000:
        raise ValueError('Unexpectedly small dictionary')
    files = {'dict.tsv': dictionary.encode(), 'glossary-en.tsv': glossary.encode()}
    output = output or directory / 'generated'
    output.mkdir(parents=True, exist_ok=True)
    # 先检查全部输出，避免发现旧文件冲突时留下部分新文件。
    for name, value in files.items():
        path = output / name
        if path.exists() and path.read_bytes() != value:
            raise ValueError('Existing generated data differs; use --output with a fresh directory')
    for name, value in files.items():
        path = output / name
        if not path.exists():
            with path.open('xb') as target:
                target.write(value)
    report = {'source': 'Rime pinyin-simp candidates; CC-CEDICT glosses and fallback',
              'rime': {'commit': RIME_COMMIT, 'date': '2024-12-29T09:47:36Z', 'sha256': RIME_EXPECTED,
              'license': 'Apache-2.0', 'excluded_readings': sorted([list(key) for key in EXCLUDED_READINGS])},
              'cedict': {'source_date': '2026-09-27T12:50:23Z', 'source_sha256': EXPECTED, 'license': 'CC-BY-SA-4.0'},
              'supplement': {'source': 'LocalGloss contributors',
              'sha256': hashlib.sha256(supplement).hexdigest(), 'license': 'CC-BY-SA-4.0',
              'fallback_weights': 'Chinese fallback=1; glosses retained'}, 'ranking': {'source': 'Rime weights; non-Rime fallback=1',
              'unknown_word_weight': 1, 'personal_learning': False}, **counts,
              'sha256': {name: hashlib.sha256(data).hexdigest() for name, data in files.items()}}
    (output / 'manifest.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=pathlib.Path)
    prepare(output=parser.parse_args().output)
