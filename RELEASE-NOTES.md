# LocalGloss v0.2.0

首个稳定版，面向 Apple Silicon Mac。修复测试版中常用词排在生僻词之后的问题：`jix`、`jixu` 首选「继续」，`wenti` 首选「问题」，`shijian` 首选「时间」。维护者已在本机使用排序修复版，确认常见输入合理、体验良好，并批准发布稳定版。

## 功能

- 输入拼音即可查看中文候选与英文释义；Space 输入中文，Tab / Shift＋Tab 输入译词。
- CC-CEDICT 开放词库与 jieba 静态词频随包提供，保留 121,256 个拼音条目；对少数生僻读音降权。
- 原生设置、主动词条、英文直通、可选中文标点和 F1 完整释义分页。
- 运行时完全离线，不接入 ChatGPT、服务器或 API Key；不监听剪贴板，不记录输入，不自动学习。

## 下载

- `LocalGloss-v0.2.0-macOS-arm64.zip`：已内置词库的应用。
- `LocalGloss-v0.2.0-source-with-dependencies.tar.gz`：含锁定 Rust 依赖与数据快照的源码，可离线构建。
- `SHA256SUMS`：下载校验值。
- `INSTALL.zh-CN.md`：安装、升级与回退说明。
- `build-manifest.json`：源码提交、构建状态及应用文件哈希。

## 验证与限制

排序修复版通过 364 项 Rust 测试、8 项数据测试、30 组排序回归和 10 组翻译流程；禁止网络及文件写入的沙箱回放通过。稳定版发布前重新执行检查，并验证签名、架构与安装包内容。详见 [验证记录](https://github.com/songsongsongggg/LocalGloss/blob/v0.2.0/docs/validation-0.2.0.md)。

最低声明系统为 macOS 13，本机实际验收环境为 macOS 26.7、Apple Silicon；没有 Intel 安装包，尚未覆盖所有应用和系统版本。短简拼仍有歧义，旧静态词频不能替代完整的上下文模型；英文释义不保证适用于所有语境。

应用采用 ad-hoc 签名，尚未做 Developer ID 签名与公证，macOS 可能提示来源未验证。请按安装说明处理，不要关闭系统安全保护。稳定发布不代表完整隐私审计：跨应用、完整文件写入与持续网络观察仍待补充。普通文本框中的密码、密钥无法自动识别，敏感输入请切回系统输入源。

数据来源与独立许可见 [DATA-SOURCES.md](https://github.com/songsongsongggg/LocalGloss/blob/v0.2.0/DATA-SOURCES.md)。反馈请提供版本、系统、应用和虚构复现步骤，不上传真实输入历史或个人配置。
