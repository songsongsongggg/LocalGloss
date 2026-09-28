# LocalGloss v0.2.0-beta.1

首个可下载的开源测试版，面向 Apple Silicon Mac，最低声明系统为 macOS 13。应用和数据均在本机运行，无需 ChatGPT、服务器或 API Key。

新增原生设置、手动词条置顶、Ctrl＋Shift＋Space 英文直通、可选中文标点，以及 F1 完整释义分页。保留 Tab 译词和输入状态清除机制。

## 下载

- `LocalGloss-v0.2.0-beta.1-macOS-arm64.zip`：已内置词库的应用。
- `LocalGloss-v0.2.0-beta.1-source-with-dependencies.tar.gz`：包含锁定 Rust 依赖和数据快照的源码。
- `SHA256SUMS`：下载校验值。
- `INSTALL.zh-CN.md`：安装、升级与回退说明。
- `build-manifest.json`：源码提交、构建状态和应用内部文件哈希。

数据改用 CC-CEDICT 2026-09-27 快照，按 CC BY-SA 4.0 保留署名和分发转换结果。包含 121,256 个拼音条目、120,541 个有释义词形。候选排序采用固定优先级，与此前本机原型不同；不自动学习个人输入。

## 已知限制

这是 Pre-release，尚未完成所有真实键盘、密码框、跨应用与网络/落盘动态验收。候选排序、词义及兼容性仍可能需要改进。仅提供 arm64 包，没有 Intel 包。

应用只做 ad-hoc 开发签名，未做 Developer ID 签名和公证，macOS 可能提示来源未验证；不要关闭系统安全保护。详见安装说明与验证记录。普通文本框中的密码、密钥无法自动识别，敏感输入请切回系统输入源。

反馈请提供版本、系统、应用与虚构复现步骤，不上传真实输入历史或个人配置。
