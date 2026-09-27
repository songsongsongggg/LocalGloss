#!/usr/bin/env python3
"""导出可审查的公开源码到新目录；不复制 Git 历史、个人文件或完整词表，不执行上传。"""
import argparse
import hashlib
import json
import pathlib
import shutil
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
FILES = [
    "Cargo.toml", "Cargo.lock", "LICENSE", "README.md", "NOTICE.md", "PRIVACY.md", "DATA-SOURCES.md",
    "docs/user/guide.md", "docs/development.md",
    "assets/lexicon/README.md", "assets/lexicon/00_meta/THUOCL_LICENSE.txt", "assets/glossary/README.md",
]
SCRIPTS = ["dev.sh", "bundle-localgloss.sh", "build-settings.sh", "test-settings.sh", "make-menu-icon.py", "verify-privacy.py", "prepare-data.py", "export-source.py"]
DIRECTORIES = [
    "crates/qingjian-core", "crates/qingjian-dictionary", "crates/qingjian-translate", "crates/qingjian-format", "crates/localgloss-engine",
    "apps/localgloss-macos", "apps/localgloss-settings",
]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("destination", type=pathlib.Path)
    args = parser.parse_args()
    destination = args.destination.resolve()
    if destination.exists():
        raise SystemExit("Destination already exists; refusing to overwrite")
    paths = [ROOT / name for name in FILES] + [ROOT / "scripts" / name for name in SCRIPTS]
    for directory in DIRECTORIES:
        paths.extend(path for path in (ROOT / directory).rglob("*") if path.is_file())
    for path in paths:
        name = path.relative_to(ROOT)
        if path.is_symlink() or any(part.startswith(".") for part in name.parts):
            raise SystemExit(f"Unexpected private or symlink path: {name}")
        if path.suffix not in {".rs", ".toml", ".lock", ".md", ".swift", ".sh", ".py", ".plist", ".strings", ".txt"} and path.name != "LICENSE":
            raise SystemExit(f"Unexpected file type: {name}")
        if path.stat().st_size > 1024 * 1024:
            raise SystemExit(f"Unexpected large file: {name}")
        # 只报告文件名；即使检查发现疑似秘密，也不输出匹配内容。
        text = path.read_text()
        private_path = re.search("/" + r"Users/[^/\s]+/|/" + "private/tmp/", text)
        credential = re.search(r"(?:gh[pousr]_)[A-Za-z0-9]{25,}|(?:sk-)[A-Za-z0-9]{30,}|-----BEGIN [A-Z ]*PRIVATE KEY-----", text)
        if private_path or credential:
            raise SystemExit(f"Publication review required: {name}")
    destination.mkdir(parents=True)
    for source in paths:
        target = destination / source.relative_to(ROOT)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    manifest = destination / "Cargo.toml"
    manifest.write_text(manifest.read_text().replace("https://github.com/qingjian-team/qingjian", "https://github.com/songsongsongggg/LocalGloss"))
    (destination / ".gitignore").write_text("/target/\n/dist/\n/.tools/\n*.app\n.env*\n.DS_Store\n__pycache__/\n*.pyc\n/assets/lexicon/dict.tsv\n/assets/glossary/glossary-en.tsv\nsettings.json\n")
    (destination / "rust-toolchain.toml").write_text('[toolchain]\nchannel = "1.98.1"\nprofile = "minimal"\ncomponents = ["rustfmt", "clippy"]\n')
    files = {str(path.relative_to(destination)): hashlib.sha256(path.read_bytes()).hexdigest()
             for path in destination.rglob("*") if path.is_file()}
    (destination.parent / (destination.name + "-manifest.json")).write_text(json.dumps(files, indent=2) + "\n")
    print(f"Exported {len(files)} reviewed source files to {destination}")

if __name__ == "__main__":
    main()
