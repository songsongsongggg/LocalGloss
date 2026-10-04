# LocalGloss 安装与回退

这是稳定版，仅提供 Apple Silicon（M1 及后续）macOS 13+ 构建。已完成自动检查及原生输入框、独立应用切换与标准密码框的限定自动验收；尚未逐一验证所有 macOS 版本与应用。Intel Mac 暂无安装包。

## 下载和校验

从 https://github.com/songsongsongggg/LocalGloss/releases/tag/v0.3.0 下载 `LocalGloss-v0.3.0-macOS-arm64.zip` 和 `SHA256SUMS`。

在终端进入下载目录，运行 `shasum -a 256 LocalGloss-v0.3.0-macOS-arm64.zip`，比较结果与 SHA256SUMS 中对应行。校验值用于检查文件是否一致；它不替代开发者身份签名。

下载包已内置开放词库，使用时不需要联网、模型账户或 API Key。解压后可看到 LocalGloss.app。

## Homebrew 安装

Homebrew 使用相同 GitHub 发布包和固定 SHA-256 校验值，默认安装到当前用户 `~/Library/Input Methods/`，不需要管理员权限。请先自行安装 Homebrew。

```sh
brew tap songsongsongggg/localgloss https://github.com/songsongsongggg/LocalGloss
# 稳定版 v0.3.0
brew install --cask songsongsongggg/localgloss/localgloss
# 或使用历史测试版（不能同时安装两个渠道）
brew install --cask songsongsongggg/localgloss/localgloss@alpha
```

当前测试版为 v0.3.0-alpha.5，下载 ZIP 也可手动安装。Homebrew 安装不替代系统输入源添加，不绕过 Gatekeeper。升级前保存工作、结束组合并切到系统输入源，备份应用，由使用者退出旧输入法后再执行 `brew upgrade --cask songsongsongggg/localgloss/localgloss`。

从手动安装转为 Homebrew 时，先完成上述备份与退出，再将旧应用移到备份目录；不要使用 `--force` 覆盖运行中的输入法。已有相同版本也可在确认文件一致后用 `--adopt` 接管。卸载使用对应的 `brew uninstall --cask`，不删除个人设置；从 alpha 切到稳定渠道时，先备份并正常退出旧输入法，再卸载 `localgloss@alpha`，安装 `localgloss`，不要使用强制覆盖。

应用仍未公证，首次下载可能被系统阻止；不要添加 `--no-quarantine`。

## 首次安装

1. 保存正在编辑的内容，保持系统输入法可用。
2. 在 Finder 按 Shift＋Command＋G，前往 `~/Library/Input Methods/`；若目录不存在，可在当前用户 Library 下创建 `Input Methods`。
3. 将 LocalGloss.app 复制到该目录。不要直接双击输入法主程序，它不是普通编辑器。
4. 打开「系统设置 → 键盘 → 文本输入 → 编辑 → ＋」，查找中文分类下的 LocalGloss / 本地译词并添加。
5. 切换到本地译词，先在非敏感文本框输入 `kaifa`。Space 应输入中文，Tab 应输入第一条英文释义。

应用为 ad-hoc 开发签名，尚无 Developer ID 签名和公证。macOS 可能阻止加载或提示来源未验证；不要关闭 Gatekeeper、批量清除隔离属性或运行来源不明的绕过脚本。确认来源与校验值后，可参考 [Apple 的官方说明](https://support.apple.com/102445) 人工决定是否允许；不能正常允许时，可按 README 从源码自行构建。

升级后若输入源启用或切换失败，可先保持系统拼音，在系统设置中仅移除并重新添加「本地译词」以刷新注册；不删除个人词条。

如果输入源暂未出现，不要反复复制安装。保存工作后，由使用者自行决定是否重新登录；项目脚本不会退出登录、重启或强制关闭应用。

## 从旧版升级

先结束拼音组合并切换系统输入法。备份 `~/Library/Input Methods/LocalGloss.app` 到独立目录，再由使用者安排旧输入法进程退出后替换。不要覆盖仍在运行的旧进程后判断升级成功。当前用户的 `settings.json` 与安装目录分开，升级不应删除个人设置。

## 设置、验证与回退

在输入源菜单打开「LocalGloss 设置与词条…」。设置只在点击保存时写入；无需开启辅助功能或输入监控权限。候选排序基于公开词典和固定优先级，与旧版不同。

先使用虚构文本检查中文、译词、F1、英文直通、焦点切换和密码框。发现异常时切回系统输入法，在 GitHub Issue 描述虚构复现，不上传密码或真实输入记录。

回退时先切换系统输入法、结束旧会话，再恢复备份应用。卸载只处理此精确应用路径，不要模糊删除其他输入法；个人设置是否删除由使用者决定。
