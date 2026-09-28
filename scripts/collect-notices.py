#!/usr/bin/env python3
"""将实际运行依赖的许可证随二进制分发；不收集本机路径或用户文件。"""
import json
import pathlib
import re
import shutil
import subprocess
import sys

root = pathlib.Path(__file__).resolve().parents[1]
output = pathlib.Path(sys.argv[1])
metadata = json.loads(subprocess.check_output(['bash', 'scripts/dev.sh', 'cargo', 'metadata', '--offline', '--locked', '--format-version', '1'], cwd=root))
tree = subprocess.check_output(['bash', 'scripts/dev.sh', 'cargo', 'tree', '--offline', '--locked', '-p', 'localgloss-macos', '--edges', 'normal', '--prefix', 'none', '--format', '{p}'], cwd=root, text=True)
runtime = set(re.findall(r'^([\w-]+) v([\w.+-]+)', tree, re.M))
output.mkdir(parents=True, exist_ok=True)
notices = []
for package in metadata['packages']:
    if (package['name'], package['version']) not in runtime or package['source'] is None:
        continue
    directory = pathlib.Path(package['manifest_path']).parent
    destination = output / (package['name'] + '-' + package['version'])
    destination.mkdir()
    licenses = [p for p in directory.rglob('*') if p.is_file() and p.suffix.lower() not in {'.rs', '.c', '.h', '.swift'} and any(p.name.upper().startswith(prefix) for prefix in ['LICENSE', 'LICENCE', 'COPYING', 'NOTICE', 'COPYRIGHT'])]
    if not licenses:
        if package['name'].startswith('objc2') or package['name'] == 'dispatch2':
            fallback = ['MIT.txt', 'objc2-LICENSE.md', 'objc2-core-LICENSE.md', 'objc2-encode-LICENSE.md']
        elif package['name'].startswith('ferrous-opencc'):
            fallback = ['Apache-2.0.txt', 'OpenCC-LICENSE.txt']
        else:
            raise SystemExit('No license file found for ' + package['name'])
        for name in fallback:
            shutil.copyfile(root / 'assets/licenses' / name, destination / name)
        shutil.copyfile(directory / 'Cargo.toml', destination / 'PACKAGE.toml')
    for source in licenses:
        target = destination / source.relative_to(directory)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    notices.append({'name': package['name'], 'version': package['version'], 'license': package['license'], 'repository': package['repository'], 'authors': package['authors']})
(output / 'dependencies.json').write_text(json.dumps(notices, ensure_ascii=False, indent=2) + '\n')
print('Collected dependency notices:', len(notices))
