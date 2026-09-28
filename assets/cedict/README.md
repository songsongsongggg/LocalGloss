# CC-CEDICT 发布数据

来源：[MDBG 官方下载](https://www.mdbg.net/chinese/dictionary?page=cedict)，维护者为 MDBG 与 CC-CEDICT 社区。原文件保留了 CEDICT 原作者 Paul Andrew Denisowski 的署名。

- 原始快照日期：2026-09-27T12:50:23Z；125,127 个源条目。
- 归档：`cedict-ts.txt.gz`。
- SHA-256：`05bb7cf923fd24cd636a703da2b0172d3b8686c3de28f613ac924e57ea44a95a`。
- 原始数据与派生 TSV 采用 [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)，详见 `LICENSE.txt`。不表示 MDBG 为本项目背书。

执行 `python3 scripts/prepare-release-data.py`，从已随源码提供的归档离线生成 `generated/`。转换选取简体汉字词条，去掉拼音声调、把 ü/u: 转为 v，去重并合并释义；跳过字音长度不匹配、混合拉丁字母等暂不支持条目。分类量词说明被过滤，交叉引用竖线改为斜线。首条释义单列，其余合并为第二条，供完整释义查看。

生成 121,256 个拼音词条、120,541 个有释义词形；完整原文仍在归档中。0.2.0 起按简体词形合并 [jieba 静态词频](../jieba/README.md)，未匹配词权重为 1，并对少数生僻读音降权。beta.1 的少量手工优先词与统一权重方案不再用于新构建。排序不使用个人输入历史；公开旧词频也不能替代完整的上下文输入模型。

派生表没有额外的使用限制，贡献也按 CC BY-SA 4.0 提供。代码采用独立的 GPL-3.0-or-later 许可，数据与代码许可不能混为一谈。
