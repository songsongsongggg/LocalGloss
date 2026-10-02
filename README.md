# LocalGloss 本地译词输入法

LocalGloss 是面向 macOS 的离线拼音输入法。输入中文拼音时，候选旁显示英文译词；按 Tab 可直接输入译词，无需复制粘贴。

**当前稳定版为 v0.2.0；测试版为 v0.3.0-alpha.2。** [下载应用与源码](https://github.com/songsongsongggg/LocalGloss/releases/tag/v0.2.0) · [安装与回退](INSTALL.zh-CN.md) · [验证记录](docs/validation-0.2.0.md)

仅提供 Apple Silicon、macOS 13+ 构建。ZIP 内已包含开放词库，解压后按安装说明添加输入源。本机用户已确认常见输入和候选排序可用；尚未覆盖所有应用与系统版本。应用仍采用 ad-hoc 签名，未做 Developer ID 签名与公证。

[下载 v0.3.0-alpha.2 测试版](https://github.com/songsongsongggg/LocalGloss/releases/tag/v0.3.0-alpha.2)：增加可搜索词条编辑器与草稿保护，适合愿意反馈问题的使用者。完整 IMK、密码框和跨应用验收尚未完成；日常使用仍可选择稳定版。

## 功能

- 全拼、简拼中文候选与本地英文释义；常用词按随包静态词频排序，缺少译词时不请求云服务。
- Tab / Shift＋Tab 输入第一、第二条译词，Space 输入中文。
- F1 展开完整释义，Page Up / Page Down 翻阅；Esc 返回候选。
- Ctrl＋Shift＋Space 切换英文直通；支持网址、邮箱的原样输入路径。
- 原生设置窗口：候选字号、每页数量、Tab 译词、符号翻页和可选中文标点。
- 主动词条：手动指定输入码、上屏文字和译词，同码置顶；不自动学习输入历史。

## 隐私边界

运行时不接入云服务、遥测、自动更新、输入日志或自动学习；不监听剪贴板，不读取输入框正文。只有点击「保存设置」才会保存偏好和主动词条。输入内容仍会交给目标应用，目标应用及系统自身的行为不由 LocalGloss 控制。

详见 [隐私说明](PRIVACY.md)、[使用指南](docs/user/guide.md) 和 [开发与验收](docs/development.md)。

## 构建

需要 Apple Silicon Mac、macOS 13 或更新系统、Command Line Tools、Rust 1.98.1（本次验证版本）和 Python 3.11 或更新版本。首次获取 Rust 依赖需要联网；发布词表已随源码以固定归档提供，后续构建与词表转换均离线。

```sh
bash scripts/dev.sh cargo fetch --locked
bash scripts/dev.sh test
bash scripts/dev.sh check
bash scripts/test-settings.sh
```

阅读 [数据来源](DATA-SOURCES.md) 后，从固定快照离线转换词表并构建：

```sh
python3 scripts/prepare-release-data.py
bash scripts/dev.sh bundle
python3 scripts/verify-privacy.py
```

产物位于 `dist/<时间戳>/LocalGloss.app`。脚本不安装输入法、不关闭应用、不改变当前输入源。首次安装与升级步骤见使用指南。开发签名为 ad-hoc，尚未公证。

## 来源与许可

本项目派生自 [青简](https://github.com/qingjian-team/qingjian) 的离线核心，沿用 **GPL-3.0-or-later**。macOS 外壳、离线装配及设置应用为 LocalGloss 的独立修改，详见 [NOTICE](NOTICE.md)。

发布数据采用 CC-CEDICT，原始快照和派生 TSV 按 CC BY-SA 4.0 分发；静态词频来自 MIT 许可的 jieba，保留独立声明；代码与数据许可分开保留。不会上传个人设置、主动词条或本机开发记录。

欢迎按 [贡献指南](CONTRIBUTING.md) 提交问题和 PR；安全反馈见 [SECURITY.md](SECURITY.md)。
