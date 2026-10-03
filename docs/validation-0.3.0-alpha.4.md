# LocalGloss v0.3.0-alpha.4 验证记录

本次仅改图标、发布包门禁与安装渠道，不改变输入与设置行为。Apple Silicon、macOS 13+；应用仍采用 ad-hoc 签名，未公证。

## 自动验证

- Rust fmt/clippy 与 366 项 workspace 测试通过。
- 8 项 Python 数据测试、Swift 设置模型、草稿与保存冲突测试通过。
- 固定虚构配置完成 Swift 保存、Rust 解析与中文/完整译词提交的集成验证；不读取个人设置。
- 隐私结构检查通过：66 个运行时依赖、22 个输入法/引擎源码文件，无新增网络或剪贴板接口。
- 181 组固定候选：170 组首选命中、179 组前三命中、181 组前九命中，零退化。
- 原创图标从 AppKit 生成十种尺寸并转换为 ICNS；反向转换成功，1024px 预览已检查。

## 打包与安装范围

发布包新增主程序与设置应用的图标、版本和非 Preview 身份校验。Homebrew 使用固定版本、SHA-256 和 input_method artifact，默认当前用户输入法目录；不附带 Test/Preview 开发工具、不主动退出应用、不删除设置。

CI、最终发布包及 Homebrew 安装验证的实际结果在发布说明和后续记录中补充；本文件不将尚未执行的检查标记为通过。真实 IMK、密码框、跨应用和完整动态隐私验收尚未完成，保持 alpha 渠道。

## 本地发布包与 Homebrew 验证（2026-10-04）

- 发布 ZIP 的 SHA-256、ad-hoc 签名、build 11 与主程序/设置图标通过；解压后 150 个文件匹配构建清单，未包含 Test/Preview 应用。
- 稳定版 cask 使用 GitHub v0.2.0 ZIP，在独立目录完成下载、安装、版本检查与卸载。
- alpha cask 使用待发布 ZIP 的本地 URL，在独立目录完成 SHA-256、安装、150 个文件匹配与卸载；稳定/alpha 同时安装被拒绝。
- 使用 `--input-methoddir` 隔离目标，没有替换当前用户输入法，也没有读取或写入个人设置。测试 tap 在验证后移除。
- 新增的 cask 将使用 GitHub 固定版本 URL，最终发布后的在线下载与远端 CI 尚待执行。

## 发布与本机应用（2026-10-04）

- 应用源码提交 `e0fff877abcff1400198136f031078f9403c87bd` 的 [GitHub CI](https://github.com/songsongsongggg/LocalGloss/actions/runs/37146456748) 全部通过。
- [v0.3.0-alpha.4](https://github.com/songsongsongggg/LocalGloss/releases/tag/v0.3.0-alpha.4) 已发布为 Pre-release，稳定版仍为 v0.2.0。五项附件重新下载后校验通过。
- 公开 tap 下载与安装通过；本机由 Homebrew 安装 alpha.4 build 11，150 个文件匹配公开构建清单，主程序与设置应用图标存在且签名有效。
- 用户明确批准后，正常退出旧输入法、Test 与 Preview；旧 alpha.3 保留独立备份，未读取或修改个人设置。
- 新版本从当前用户输入法目录启动，输入源注册、启用、选择成功；不再运行 Test/Preview GUI。安装期间剪贴板变化计数未变化，未读取剪贴板内容。
- 本次完成安装与进程验证，未将其视为真实键盘、密码框、跨应用或完整动态隐私验收。
