# 发布数据来源

alpha.5 起以 **Rime pinyin-simp** 提供主要中文候选及静态权重，以 **CC-CEDICT** 提供英文释义和低权重中文补充。代码是 GPL-3.0-or-later；Rime 数据为 Apache-2.0；CC-CEDICT 和项目补充数据为 CC BY-SA 4.0，各来源保留独立声明。

固定提交、归档哈希、Android Pinyin IME 来源署名和本项目过滤规则见 [Rime 数据说明](assets/rime/README.md)。转换后 Rime 有 65,121 个拼音条目；CC-CEDICT / 项目表补充 82,127 个缺失条目，统一权重 1，不覆盖 Rime。合计 147,248 个拼音条目。

[CC-CEDICT 说明](assets/cedict/README.md) 保留 MDBG、CC-CEDICT 社区及原作者署名。项目 [常用表达表](assets/localgloss/README.md) 补 18 个缺失词形，保留原有释义；当前共 120,559 个有英文释义的词形。中文候选不要求具备译词；没有释义时仍能提交中文，不联网补译。

执行 `python3 scripts/prepare-release-data.py` 从仓库内固定归档离线生成；不读取个人配置或输入历史。脚本验证源归档 SHA-256，拒绝覆盖不同的生成表，清单记录来源、数量与输出哈希。应用 ZIP 内包含生成表，使用者无需联网获取词库。

历史 v0.2.0 至 alpha.4 使用 CC-CEDICT + [jieba 静态词频](assets/jieba/README.md)，jieba 为 MIT；快照和转换能力保留用于复现。alpha.5 默认排序不再合并 jieba 权重，也不增加 Rime 运行库。公开静态词频不能替代上下文语言模型，仍可能出现同音词排序差异。

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
