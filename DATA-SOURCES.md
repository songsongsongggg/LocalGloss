# 数据来源

输入法运行时需要 `assets/lexicon/dict.tsv` 和 `assets/glossary/glossary-en.tsv`。公开源码仓库不内置这两个完整数据文件，测试使用代码中的虚构小词表。

数据准备脚本仅在传入 `--download` 后访问 GitHub，固定来源为青简提交 `f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5`，下载后校验 SHA-256。已有文件不匹配时拒绝覆盖。这个步骤不会发送输入内容或读取个人设置。

| 文件 | SHA-256 |
| --- | --- |
| `dict.tsv` | `7787d6d219674dd51297652456c55ea2d49a65b5e571d2309a62add0ec2d2279` |
| `glossary-en.tsv` | `7b9676979aa227bde7a2d541354a59b73a04f33acd36ef5d6c433ce789b81625` |

[上游字词库说明](https://github.com/qingjian-team/qingjian/blob/f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5/assets/lexicon/README.md) 明确指出整个数据包不采用单一许可证。来源包含规范字转录、通用词和 THUOCL 领域词；THUOCL 的 MIT 声明随本仓库保留。本项目尚未完成所有来源的逐项再分发核验，因此首个公开版本只发布源码，不附带完整词库安装包。

[上游释义说明](https://github.com/qingjian-team/qingjian/blob/f7abaefcb1a3aeaca5c01692941a64a7b1f43eb5/assets/glossary/README.md) 将英文表标为 GPL-3.0-or-later，并说明它是离线批量生成的机器释义，可能存在误译。LocalGloss 在输入时只查询现成表，不调用生成模型。

`assets/lexicon/README.md` 与 `assets/glossary/README.md` 保留上游原文，其中部分相对链接指向未包含的上游资源；完整内容请使用上面的固定提交链接。
