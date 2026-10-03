# LocalGloss 发布流程

## 发布门槛

源码使用干净的公开 checkout；本机原型与旧开发历史不直接推送。测试版用不可变 `v<版本>` 标签和 GitHub Pre-release，稳定版须记录实际使用验收和剩余验证范围。v0.2.0 经维护者本机日常输入验收并明确授权发布；跨应用与完整动态隐私验收尚未完成，按本次限定范围发布，不将稳定渠道等同于全面安全审计。

本次仅发布 arm64。源码、数据快照、依赖锁定文件和构建脚本随版本保留；构建不自动安装、不关闭应用、不修改系统输入源。

## 构建和打包

在 Apple Silicon Mac 上，先按 README 获取工具与依赖，完成 CI 同等检查，再提交代码：

```sh
python3 scripts/prepare-release-data.py
bash scripts/dev.sh bundle
python3 scripts/package-release.py dist/<时间戳>/LocalGloss.app target/release-v<版本>
```

打包器检查源码干净、版本一致、源码提交与应用构建记录一致、文件哈希、arm64 架构和代码签名，拒绝将带本机用户路径的可执行文件发布。生成可直接解压的应用 ZIP、包含锁定依赖的源码归档、安装说明、构建清单及 SHA256SUMS。

源码归档包含 `vendor/` 和 `.cargo/config.toml`，已有 Rust 与 Apple 开发工具时可离线构建；系统 SDK 和编译器由构建者自行取得。数据转换完全离线。`SOURCE-COMMIT` 标识构建版本。

## 发布前与发布后

1. 校验下载包解压后的签名与内容，确认许可证和说明随包保留。
2. 对同一提交检查 GitHub CI；如服务不可用，明确标记未通过远端 CI，不假装通过。
3. 创建不可变标签；测试版发布 Pre-release，经明确验收授权的稳定版发布普通 Release；附件使用打包器生成的精确清单。
4. 发布后重新下载附件，校验 SHA-256；核对 tag、commit、平台和 prerelease 状态。
5. 发现问题发新版本，不覆盖旧标签或静默替换旧附件。

开发签名尚未公证，必须在下载页面清楚说明。发布说明须区分用户实际使用反馈、自动检查和未完成项，后续继续补全跨应用与隐私动态验收。

## Homebrew 渠道

仓库的 `Casks/` 作为自定义 tap；使用显式仓库 URL，无需额外 GitHub 仓库。`localgloss` 只指向稳定版，`localgloss@alpha` 指向最新公开测试版。发布并重新下载校验附件后，才更新对应 cask 的 version 与 sha256；此提交可以晚于应用标签，避免包哈希与源码包互相引用。

Cask 使用 `input_method` artifact，不执行第三方安装脚本，不绕过隔离属性，不关闭应用，不删除 settings.json。用独立的 `--input-methoddir` 测试安装与卸载，避免碰个人输入法；发布后验证 `brew info`、SHA-256 与应用文件清单。
