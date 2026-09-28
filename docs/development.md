# 开发与验收

## 代码范围

- `crates/localgloss-engine`：离线装配、会话清除、候选与释义、主动词条和混输状态机。
- `apps/localgloss-macos`：InputMethodKit 回调、候选渲染和设置菜单。
- `apps/localgloss-settings`：原生 AppKit 设置窗口；唯一的主动保存入口。
- `crates/qingjian-*`：保留的四个上游离线模块。部分通用 API 可写文件，但 LocalGloss 运行时不装配这些服务。

## 本地验证

首次按 README 准备依赖后执行：

```sh
bash scripts/dev.sh test
bash scripts/dev.sh check
bash scripts/dev.sh cargo test --offline --locked --workspace
bash scripts/test-settings.sh
python3 scripts/verify-privacy.py
```

Swift 测试使用独立临时目录与虚构词条，覆盖保存、0600 权限、替换、重复码拒绝、符号链接拒绝和未知字段拒绝。Rust 测试覆盖组合清除、缺译词、翻页、主动词条、英文直通和完整释义。

## 安装后人工验收

只使用虚构文本和物理键盘。此前自动化注入按键在 LocalGloss 和系统拼音中都未可靠触发组合，不能替代真实按键测试。

1. 在原生文本应用、浏览器和日常编辑器里，检查拼音、译词、F1、英文直通及中文标点。
2. 保存字号、候选数量和示例词条；切回后检查生效，重复码应拒绝保存。
3. 两个输入框及两种应用之间切换；旧拼音或释义不得出现在新会话。
4. 切到标准密码框；候选隐藏，旧组合清除，密码不进入输入法。
5. 断网输入；已有词表正常可用。观察进程流量与文件写入，区分目标应用行为。
6. 比较输入前后的剪贴板变化计数，不读取剪贴板内容；有变化时排除其他应用干扰。

核心测试、静态检查和签名校验通过不代表这些实际会话检查通过。0.2.0-beta.1 在完成上述检查前保持 Pre-release 状态。

## 本次变化

0.2.0 新增原生设置、主动词条置顶、Ctrl＋Shift＋Space 英文直通、可选中文标点和 F1 释义分页；保持无云服务、无自动学习、无输入日志。0.1.1 已有 Tab 译词、符号翻页与稳定候选宽度。

发布版使用 CC-CEDICT；执行 `python3 scripts/prepare-release-data.py` 和 `python3 -m unittest discover -s scripts/tests -v` 检查转换，再运行 `bash scripts/dev.sh cargo run --offline --locked -p localgloss-engine --example offline_check -- assets/cedict/generated` 验证实际词表。测试与发布记录见 [0.2.0-beta.1 验证](validation-0.2.0-beta.1.md)。
