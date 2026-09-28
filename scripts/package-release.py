#!/usr/bin/env python3
"""从干净源码与已验证应用生成下载包、含依赖源码包和校验文件；不安装或上传。"""
import argparse
import hashlib
import json
import pathlib
import plistlib
import shutil
import subprocess
import tarfile
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def run(*args, **kwargs):
    return subprocess.check_output(args, cwd=ROOT, text=True, **kwargs)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('app', type=pathlib.Path)
    parser.add_argument('destination', type=pathlib.Path)
    args = parser.parse_args()
    app, output = args.app.resolve(), args.destination.resolve()
    if output.exists():
        raise SystemExit('Destination exists; refusing to overwrite')
    if run('git', 'status', '--porcelain').strip():
        raise SystemExit('Release requires a clean source checkout')
    version = tomllib.loads((ROOT / 'apps/localgloss-macos/Cargo.toml').read_text())['package']['version']
    revision = run('git', 'rev-parse', 'HEAD').strip()
    manifest = json.loads((app.parent / 'build-manifest.json').read_text())
    if manifest['source_commit'] != revision or manifest['source_dirty'] is not False or manifest['release'] != version:
        raise SystemExit('App/source revision mismatch or dirty build')
    actual = {str(p.relative_to(app)): hashlib.sha256(p.read_bytes()).hexdigest() for p in app.rglob('*') if p.is_file()}
    if actual != manifest['files']:
        raise SystemExit('App differs from its build manifest')
    for relative in ['Contents/MacOS/localgloss-macos', 'Contents/Resources/LocalGloss Settings.app/Contents/MacOS/localgloss-settings']:
        binary = app / relative
        if run('/usr/bin/lipo', '-archs', str(binary)).strip() != 'arm64':
            raise SystemExit('Unexpected architecture')
        if b'/Users/' in binary.read_bytes():
            raise SystemExit('Build contains a local user path')
    if plistlib.loads((app / 'Contents/Info.plist').read_bytes())['LocalGlossRelease'] != version:
        raise SystemExit('Version mismatch')
    subprocess.run(['/usr/bin/codesign', '--verify', '--deep', '--strict', str(app)], check=True)
    output.mkdir(parents=True)
    name = f'LocalGloss-v{version}-macOS-arm64'
    stage = output / 'work'
    binary_stage = stage / name
    binary_stage.mkdir(parents=True)
    shutil.copytree(app, binary_stage / 'LocalGloss.app')
    for filename in ['INSTALL.zh-CN.md', 'LICENSE', 'DATA-SOURCES.md']:
        shutil.copyfile(ROOT / filename, binary_stage / filename)
    subprocess.run(['/usr/bin/ditto', '-c', '-k', '--norsrc', '--noextattr', '--keepParent', str(binary_stage), str(output / (name + '.zip'))], check=True)
    source_name = f'LocalGloss-v{version}-source-with-dependencies'
    source = stage / source_name
    source.mkdir()
    for filename in run('git', 'ls-files', '-z').split('\0'):
        if not filename:
            continue
        original = ROOT / filename
        if original.is_symlink():
            raise SystemExit('Symlinks require review before source packaging')
        destination = source / filename
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(original, destination)
    (source / 'SOURCE-COMMIT').write_text(revision + '\n')
    vendor = source / 'vendor'
    config = run('bash', 'scripts/dev.sh', 'cargo', 'vendor', '--offline', '--locked', '--versioned-dirs', str(vendor))
    (source / '.cargo').mkdir(exist_ok=True)
    (source / '.cargo/config.toml').write_text(config.replace(str(vendor), 'vendor'))
    def portable(info):
        info.uid = info.gid = 0
        info.uname = info.gname = ''
        info.mtime = 0
        info.pax_headers = {}
        return info
    with tarfile.open(output / (source_name + '.tar.gz'), 'w:gz') as archive:
        archive.add(source, arcname=source_name, filter=portable)
    shutil.copyfile(app.parent / 'build-manifest.json', output / 'build-manifest.json')
    shutil.copyfile(ROOT / 'INSTALL.zh-CN.md', output / 'INSTALL.zh-CN.md')
    assets = sorted(p for p in output.iterdir() if p.is_file())
    (output / 'SHA256SUMS').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest() + '  ' + p.name + '\n' for p in assets))
    print('Release assets:')
    for path in assets + [output / 'SHA256SUMS']:
        print(path.name, path.stat().st_size)


if __name__ == '__main__':
    main()
