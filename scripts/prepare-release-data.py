#!/usr/bin/env python3
"""从仓库中固定的 CC-CEDICT 快照生成发布词表；完全离线，不修改旧词库。"""
import gzip
import hashlib
import json
import pathlib
import argparse
from cedict import convert, parse_frequencies

ROOT = pathlib.Path(__file__).resolve().parents[1]
EXPECTED = '05bb7cf923fd24cd636a703da2b0172d3b8686c3de28f613ac924e57ea44a95a'
FREQUENCY_EXPECTED = '35e47c1fb9baf2a351a179afb8c22878caa0305e7294c855f05af524f46d51a3'


def prepare(root=ROOT, output=None):
    directory = root / 'assets/cedict'
    data = (directory / 'cedict-ts.txt.gz').read_bytes()
    if hashlib.sha256(data).hexdigest() != EXPECTED:
        raise ValueError('CC-CEDICT archive checksum mismatch')
    text = gzip.decompress(data).decode('utf-8')
    if '#! license=https://creativecommons.org/licenses/by-sa/4.0/' not in text:
        raise ValueError('CC-CEDICT license header missing')
    frequency_data = (root / 'assets/jieba/dict.txt.gz').read_bytes()
    if hashlib.sha256(frequency_data).hexdigest() != FREQUENCY_EXPECTED:
        raise ValueError('jieba archive checksum mismatch')
    frequencies = parse_frequencies(gzip.decompress(frequency_data).decode('utf-8'))
    supplement = (root / 'assets/localgloss/common-phrases.tsv').read_bytes()
    dictionary, glossary, counts = convert(text, frequencies, supplement.decode('utf-8'))
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
    report = {'source': 'CC-CEDICT', 'source_date': '2026-09-27T12:50:23Z', 'source_sha256': EXPECTED,
              'license': 'CC-BY-SA-4.0', 'supplement': {'source': 'LocalGloss contributors',
              'sha256': hashlib.sha256(supplement).hexdigest(), 'license': 'CC-BY-SA-4.0',
              'fallback_weights': 'curated priorities, not measured frequencies'}, 'ranking': {'source': 'jieba', 'license': 'MIT',
              'commit': '67fa2e36e72f69d9134b8a1037b83fbb070b9775', 'source_sha256': FREQUENCY_EXPECTED,
              'unknown_word_weight': 1, 'personal_learning': False}, **counts,
              'sha256': {name: hashlib.sha256(data).hexdigest() for name, data in files.items()}}
    (output / 'manifest.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=pathlib.Path)
    prepare(output=parser.parse_args().output)
