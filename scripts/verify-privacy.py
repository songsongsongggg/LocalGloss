#!/usr/bin/env python3
"""检查实际构建依赖闭包和 Mac 输入接口；不代替安装后的动态隐私验收。"""
import pathlib
import re
import subprocess

ROOT=pathlib.Path(__file__).resolve().parents[1]
tree=subprocess.check_output(['bash', 'scripts/dev.sh', 'cargo', 'tree', '--offline', '--locked',
    '-p', 'localgloss-macos', '--edges', 'normal', '--prefix', 'none', '--format', '{p}'],
    cwd=ROOT,text=True)
packages=set(re.findall(r'^([\w-]+) v',tree,re.M))
denied={'qingjian-predict','qingjian-update','qingjian-learning','qingjian-platform',
        'qingjian-neural','reqwest','async-openai','tokio','hyper','ureq','curl',
        'tracing-appender','tracing-subscriber','objc2-cloud-kit'}
assert not packages & denied, f'forbidden packages: {sorted(packages & denied)}'
sources=list((ROOT/'apps/localgloss-macos/src').rglob('*.rs'))
sources+=list((ROOT/'crates/localgloss-engine/src').rglob('*.rs'))
denied_api=re.compile(r'NSPasteboard|CGEventTap|AXUIElement|attributedSubstringFromRange|'
    r'selectedRange|surrounding_text|std::fs::write|File::create|OpenOptions|'
    r'with_learner\s*\(|with_usage_meter\s*\(|with_vocabulary_tracker\s*\(|'
    r'with_predictor\s*\(|set_predictor\s*\(|with_input_logger\s*\(')
for path in sources:
    assert not denied_api.search(path.read_text()), f'forbidden API: {path.relative_to(ROOT)}'
engine=(ROOT/'crates/localgloss-engine/src/engine/mod.rs').read_text()
assert 'engine.set_private(true)' in engine
assert 'engine.set_learning(false)' in engine
manifest=(ROOT/'Cargo.toml').read_text()
assert '"max_level_off"' in manifest and '"release_max_level_off"' in manifest
# 设置编辑器只允许模型文件执行主动保存；禁止网络、剪贴板与输入监控接口。
settings_sources=list((ROOT/'apps/localgloss-settings').glob('*.swift'))
for path in settings_sources:
    text=path.read_text()
    assert not re.search(r'URLSession|NSPasteboard|CGEventTap|AXUIElement|addGlobalMonitor',text), path.name
    if path.name != 'SettingsModel.swift':
        assert '.write(' not in text, path.name
# 用户设置保存仅由 saveSettings 显式触发。
assert 'try model.save(expected: baseline, checkConflict: true)' in (ROOT/'apps/localgloss-settings/SettingsWindow.swift').read_text()
print(f'privacy structure checks passed: {len(packages)} runtime packages, {len(sources)} source files')
print('pending: installed IMK network, filesystem, clipboard, password-field and focus checks')
