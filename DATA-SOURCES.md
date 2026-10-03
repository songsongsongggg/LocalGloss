# 发布数据来源

默认使用 **CC-CEDICT**，完全替代旧原型中混合来源的词库与机器释义表。代码许可是 GPL-3.0-or-later；CC-CEDICT 及其派生表单独采用 CC BY-SA 4.0；新增静态词频来源 jieba 采用 MIT，保留其声明。

来源、日期、哈希、转换规则及署名见 [CC-CEDICT 说明](assets/cedict/README.md)。MDBG 的 [官方下载页](https://www.mdbg.net/chinese/dictionary?page=cedict) 与归档头部均标明 CC BY-SA 4.0。仓库保留 2026-09-27T12:50:23Z 的原始压缩快照、许可链接及转换脚本；派生 TSV 与原数据按同一许可分发。

执行 `python3 scripts/prepare-release-data.py` 即可离线生成发布数据；不联网、不读取个人设置、不修改旧词库。数据归档和生成表都校验 SHA-256。应用 ZIP 内已包含生成表，终端用户无需执行转换。

CC-CEDICT 并非输入法语料词频库。0.2.0 起采用固定的 [jieba 词频快照](assets/jieba/README.md) 改善同类候选排序，替代 beta.1 的手工优先词和统一权重。该数据与转换均随源码提供，构建过程保持离线，不采集个人输入来训练排序。英语释义可能不适合直接用于每一个语境；词形合并也可能合并不同读音的词义。

## 常用表达补充

下一测试版增加 [LocalGloss 常用表达表](assets/localgloss/README.md)，只补 CC-CEDICT 缺失短语。由本项目贡献者编写，采用 CC BY-SA 4.0；优先沿用 jieba 已有词频，缺失时使用明确标注的固定优先级，不冒充真实语料频次。本次新增 18 个词形；生成清单记录补充文件哈希和数量。

## 保留的上游历史说明

以下只解释旧原型来源，不构成当前发布版本的数据清单。`scripts/prepare-data.py` 保留作上游复现入口，默认构建不调用它。

### 旧原型数据来源

输入法运行时需要 `assets/lexicon/dict.tsv` 和 `assets/glossary/glossary-en.tsv`。旧原型文件不随当前版本发布，测试使用代码中的虚构小词表。

数据准备脚本仅在传入 `--download` 后访问 GitHub，固定来源为青简提交 `f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5`，下载后校验 SHA-256。已有文件不匹配时拒绝覆盖。这个步骤不会发送输入内容或读取个人设置。

| 文件 | SHA-256 |
| --- | --- |
| `dict.tsv` | `7787d6d219674dd51297652456c55ea2d49a65b5e571d2309a62add0ec2d2279` |
| `glossary-en.tsv` | `7b9676979aa227bde7a2d541354a59b73a04f33acd36ef5d6c433ce789b81625` |

[上游字词库说明](https://github.com/qingjian-team/qingjian/blob/f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5/assets/lexicon/README.md) 明确指出整个数据包不采用单一许可证。来源包含规范字转录、通用词和 THUOCL 领域词；THUOCL 的 MIT 声明随本仓库保留。本项目尚未完成所有来源的逐项再分发核验，因此这份旧词表不进入当前下载包。

[上游释义说明](https://github.com/qingjian-team/qingjian/blob/f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5/assets/glossary/README.md) 将英文表标为 GPL-3.0-or-later，并说明它是离线批量生成的机器释义，可能存在误译。LocalGloss 在输入时只查询现成表，不调用生成模型。

`assets/lexicon/README.md` 与 `assets/glossary/README.md` 保留上游原文，其中部分相对链接指向未包含的上游资源；完整内容请使用上面的固定提交链接。
