# LocalGloss 本地译词输入法

LocalGloss 是面向 macOS 的离线拼音输入法。输入中文拼音时，候选旁显示英文译词；按 Tab 可直接输入译词，无需复制粘贴。

**当前为 0.2.0 开发预览。** 仅支持 Apple Silicon；新增功能已通过核心测试和构建，安装后的真实键盘、密码框和跨应用回归仍待验收。暂不发布安装包或稳定版承诺。

## 功能

- 全拼中文候选与本地英文释义；缺少译词时不请求云服务。
- Tab / Shift＋Tab 输入第一、第二条译词，Space 输入中文。
- F1 展开完整释义，Page Up / Page Down 翻阅；Esc 返回候选。
- Ctrl＋Shift＋Space 切换英文直通；支持网址、邮箱的原样输入路径。
- 原生设置窗口：候选字号、每页数量、Tab 译词、符号翻页和可选中文标点。
- 主动词条：手动指定输入码、上屏文字和译词，同码置顶；不自动学习输入历史。

## 隐私边界

运行时不接入云服务、遥测、自动更新、输入日志或自动学习；不监听剪贴板，不读取输入框正文。只有点击「保存设置」才会保存偏好和主动词条。输入内容仍会交给目标应用，目标应用及系统自身的行为不由 LocalGloss 控制。

详见 [隐私说明](PRIVACY.md)、[使用指南](docs/user/guide.md) 和 [开发与验收](docs/development.md)。

## 构建

需要 Apple Silicon Mac、macOS 13 或更新系统、Command Line Tools、Rust 1.98.1（本次验证版本）和 Python 3.11 或更新版本。首次获取 Rust 依赖和词表需要联网，后续构建使用离线模式。

```sh
bash scripts/dev.sh cargo fetch --locked
bash scripts/dev.sh test
bash scripts/dev.sh check
bash scripts/test-settings.sh
```

阅读 [数据来源](DATA-SOURCES.md) 后，显式下载锁定版本的公开词表并构建：

```sh
python3 scripts/prepare-data.py --download
bash scripts/dev.sh bundle
python3 scripts/verify-privacy.py
```

产物位于 `dist/<时间戳>/LocalGloss.app`。脚本不安装输入法、不关闭应用、不改变当前输入源。首次安装与升级步骤见使用指南。开发签名为 ad-hoc，尚未公证。

## 来源与许可

本项目派生自 [青简](https://github.com/qingjian-team/qingjian) 的离线核心，沿用 **GPL-3.0-or-later**。macOS 外壳、离线装配及设置应用为 LocalGloss 的独立修改，详见 [NOTICE](NOTICE.md)。

源码许可不覆盖所有上游词库来源。公开仓库保留数据来源和许可说明，完整词表由显式下载步骤获取；不会上传个人设置、主动词条或本机开发记录。
