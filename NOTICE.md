# 来源与修改说明

LocalGloss 包含青简的离线核心代码，来源为：

- 上游项目：https://github.com/qingjian-team/qingjian
- 基准提交：`f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5`
- 保留的模块：`qingjian-core`、`qingjian-dictionary`、`qingjian-translate`、`qingjian-format`。
- 上游代码与本项目修改遵循 GPL-3.0-or-later，完整许可见 `LICENSE`。

LocalGloss contributors 在 2026-09-26 至 2026-09-28 新增离线适配层、macOS 输入法外壳、设置应用、测试和构建脚本，并缩小默认构建依赖范围、关闭运行日志。原模块的版权与来源注释予以保留。

LocalGloss 是独立派生项目，不代表青简官方版本；未使用青简产品 logo。仓库使用独立的公开源码快照，原始开发历史可在上述固定上游提交查阅。

Rust 第三方依赖版本列于 `Cargo.lock`，各依赖按自身许可提供。数据来源及其许可说明见 `DATA-SOURCES.md`；不得将根目录的代码许可证理解为所有数据来源的统一许可。

2026-09-28 的公开测试版新增 CC-CEDICT 数据转换、依赖许可归档、CI 与版本发布脚本。数据快照与派生 TSV 采用 CC BY-SA 4.0，保留 MDBG、CC-CEDICT 社区与原 CEDICT 作者署名；详见 DATA-SOURCES.md。
