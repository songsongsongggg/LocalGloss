# 第三方依赖声明

发布脚本根据 Cargo.lock 对应的实际运行依赖收集许可文件与包信息，随应用放入 `Contents/Resources/notices/dependencies`。包含运行依赖的完整源代码也在 Release 的 `source-with-dependencies.tar.gz` 中提供。

部分 crate 归档未包含独立许可正文，补充材料如下：

- objc2 及框架封装：上游 LICENSE.md，分别取自 `madsmtm/objc2` 的 `7b1abfd750a2cacaea71d6a56ecfb83cb7de560b`、`8852b424193ca41602281b3d7540d7c8ed51e49a` 和 `8d214f5477365ffcbcbb7de058c86ed9a518efb7`；声明 MIT 或多选许可，分发选择 MIT，并保留原许可说明。作者信息以各包元数据及源码为准。
- ferrous-opencc 0.4.0：包 manifest 声明 Apache-2.0，代码来自 `apoint123/ferrous-opencc` 的 `bd557afbf2fd07f0bc55371159d7006629dfd935`；保留作者和仓库信息，并附 Apache-2.0 正文。
- OpenCC 数据：补充 [OpenCC ver.1.1.9 许可正文](https://github.com/BYVoid/OpenCC/blob/ver.1.1.9/LICENSE)。此处版本仅指许可文本来源；随包数据的准确内容以 ferrous-opencc 的源码归档为准。
- Apache-2.0 标准正文取自本次 Rust 工具链所含许可证；MIT 正文附作者定位说明，不改变上游的许可授予。

系统 AppKit、InputMethodKit 和 Swift/Rust 工具链不由本项目重新授予许可。构建者仍须遵循其开发工具许可。
