# v0.3.0-alpha.2 验证记录

日期：2026-10-03（北京时间）。本版为 A 阶段修正版，build 9；已构建，未安装、提交或发布。现用 alpha.1 build 8 的 148 个文件重新匹配原构建清单。

## 修改与证据

- 修正 Tab 译词关闭后仍显示可提交译词的问题。TermPreview 读取草稿选项；隔离 GUI 自动测试确认关闭选项后，编辑 kaifa 的预览显示「译词快捷键已关闭；以目标应用的 Tab 行为为准」。
- 将固定高度标签改为可滚动只读文本预览。240 Unicode 标量译词完整保留通过模型测试；实际最长文本滚动尚未完成 GUI 验收。
- 补测另一个保存者持有目录锁的情形：立即拒绝保存且原文件字节不变。
- 格式、Clippy、366 项 Rust、8 项 Python、Swift 设置及草稿测试、静态隐私检查均通过。181 组包内词表回放无排序退化。
- 完整包通过签名检查，148 个文件与清单匹配。签名仍为 ad-hoc，未公证。

## 交付与限制

完整包：`dist/20261002T190936Z/LocalGloss.app`（目录采用 UTC 时间）；隔离预览：`target/preview-0.3.0-alpha.2/LocalGlossPreview.app`。日志：`target/validation-0.3.0-alpha.2/`。

专用实际输入测试窗口连接返回 timeoutReached；隔离预览的取消操作之后，观察返回 ScreenCaptureKit -3812。未用这些工具错误判定产品失败，也未把真实按键、密码框、跨应用或完整动态隐私验收标为通过。本轮未访问个人设置或剪贴板内容，未退出现用输入法。后续继续对已安装版本做实际输入验收，alpha.2 替换须取得当次退出与安装确认。

## 公开发布结果（2026-10-03，北京时间）

- 已发布 GitHub Pre-release v0.3.0-alpha.2，发布时间 2026-10-03 03:51:43 +08:00；对应公开源码提交 ac6636a4d55de328c057a48f185a30550e55be56。
- 精确提交 CI 全部成功：https://github.com/songsongsongggg/LocalGloss/actions/runs/37056515854 。
- 从干净公开 checkout 重建应用、生成应用 ZIP 与含依赖源码归档；ZIP 解压签名和 148 个文件哈希通过，源码归档解压后 cargo check --offline --locked --workspace 通过。
- 五个 GitHub 附件重新下载后 SHA-256 与本地原件逐一相同；API 确认 draft=false、prerelease=true，最新稳定版仍为 v0.2.0。
- 发布页：https://github.com/songsongsongggg/LocalGloss/releases/tag/v0.3.0-alpha.2 。仅发布测试渠道，前文未完成的动态验收限制仍有效。本机未替换，仍为 alpha.1。
- 发布证据与下载回验位于 target/public-source/target/release-validation-030a2 和 target/public-source/target/download-verify-030a2。后续方案见 design/localgloss-next-iteration.md。
