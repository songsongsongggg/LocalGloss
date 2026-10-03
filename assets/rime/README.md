# Rime 简体拼音候选数据

来源：[rime/rime-pinyin-simp](https://github.com/rime/rime-pinyin-simp/tree/0c6861ef7420ee780270ca6d993d18d4101049d0)，固定提交 `0c6861ef7420ee780270ca6d993d18d4101049d0`，提交时间 2024-12-29T09:47:36Z。只采用词表数据，不安装 Rime 程序、不执行上游脚本。

上游 AUTHORS 标明 `pinyin_simp.dict.yaml` 派生自 Android Pinyin IME，采用 Apache License 2.0；保留原文 `LICENSE.txt`、`AUTHORS.txt`。Gong Chen 的署名对应输入方案文件，本项目不使用该方案文件。

- 压缩归档 SHA-256：`1420536dec32cb4c7b070bdbdbc07f7a72f342ff1c22b7945f120cac0d5e6cae`。
- 解压原文 SHA-256：`e341598343a0f0f2035bb1aafc34a7f3bb7887deeecb3f60796262aaa2983e6b`。
- LICENSE SHA-256：`cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`。
- AUTHORS SHA-256：`f4cff0fcbca4668ac449c24a53be547e162bc60cce63fdc5d5906801a452edc4`。

2026-10-04 的 LocalGloss 修改：将固定词表转为无声调 TSV，保留原权重，重复读音取最大权重，零权重取 1；跳过非纯汉字或字音数量不匹配项；过滤「都是、都有、都要」的 `du` 误读，保留正确的 `dou` 读音。词表转换使用标准库，不执行 YAML。

Rime 是主要中文候选源；CC-CEDICT 和 LocalGloss 表只补缺失词条，权重为 1，不覆盖 Rime。英文释义来自 CC-CEDICT 及项目补充表；没有释义也可提交中文。组合文件按来源分别保留 Apache-2.0 与 CC BY-SA 4.0 声明，不把 Rime 数据统一改为 CC BY-SA。构建清单记录源哈希、数量和过滤规则。

执行 `python3 scripts/prepare-release-data.py` 完全离线生成。应用包内已有生成表，输入期间不联网、不抓取词库、不读取个人输入历史，也没有新增自动学习。
