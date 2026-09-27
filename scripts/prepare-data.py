#!/usr/bin/env python3
"""显式下载锁定的上游词表，校验哈希；不执行远程脚本、不上传文本。"""
import argparse
import hashlib
import pathlib
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[1]
REVISION = 'f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5'
FILES = {
    'assets/lexicon/dict.tsv': '7787d6d219674dd51297652456c55ea2d49a65b5e571d2309a62add0ec2d2279',
    'assets/glossary/glossary-en.tsv': '7b9676979aa227bde7a2d541354a59b73a04f33acd36ef5d6c433ce789b81625',
}

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--download', action='store_true', help='明确允许下载词表；运行时输入不联网')
    args = parser.parse_args()
    for name, expected in FILES.items():
        path = ROOT / name
        if path.exists():
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
                raise SystemExit(f'Existing file differs; not overwritten: {name}')
            print('verified:', name)
            continue
        if not args.download:
            raise SystemExit('Missing data. Review DATA-SOURCES.md, then run with --download.')
        url = f'https://raw.githubusercontent.com/qingjian-team/qingjian/{REVISION}/{name}'
        with urllib.request.urlopen(url, timeout=60) as response:
            data = response.read(32 * 1024 * 1024 + 1)
        if hashlib.sha256(data).hexdigest() != expected:
            raise SystemExit(f'Checksum mismatch: {name}')
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open('xb') as output:
            output.write(data)
        print('downloaded and verified:', name)

if __name__ == '__main__':
    main()
