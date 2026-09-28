#!/usr/bin/env python3
"""从仓库中固定的 CC-CEDICT 快照生成发布词表；完全离线，不修改旧词库。"""
import gzip
import hashlib
import json
import pathlib
from cedict import convert

ROOT = pathlib.Path(__file__).resolve().parents[1]
EXPECTED = '05bb7cf923fd24cd636a703da2b0172d3b8686c3de28f613ac924e57ea44a95a'


def prepare(root=ROOT):
    directory = root / 'assets/cedict'
    data = (directory / 'cedict-ts.txt.gz').read_bytes()
    if hashlib.sha256(data).hexdigest() != EXPECTED:
        raise ValueError('CC-CEDICT archive checksum mismatch')
    text = gzip.decompress(data).decode('utf-8')
    if '#! license=https://creativecommons.org/licenses/by-sa/4.0/' not in text:
        raise ValueError('CC-CEDICT license header missing')
    dictionary, glossary, counts = convert(text)
    if counts['dictionary_entries'] < 100000 or counts['glossary_words'] < 100000:
        raise ValueError('Unexpectedly small dictionary')
    files = {'dict.tsv': dictionary.encode(), 'glossary-en.tsv': glossary.encode()}
    output = directory / 'generated'
    output.mkdir(exist_ok=True)
    for name, value in files.items():
        path = output / name
        if path.exists() and path.read_bytes() != value:
            raise ValueError('Existing generated data differs; use a fresh checkout')
        if not path.exists():
            with path.open('xb') as target:
                target.write(value)
    report = {'source': 'CC-CEDICT', 'source_date': '2026-09-27T12:50:23Z', 'source_sha256': EXPECTED,
              'license': 'CC-BY-SA-4.0', **counts,
              'sha256': {name: hashlib.sha256(data).hexdigest() for name, data in files.items()}}
    (output / 'manifest.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    prepare()
