#!/usr/bin/env python3
"""把 CC-CEDICT 转为只读 TSV；转换结果按 CC BY-SA 4.0 分发。"""
import re

ENTRY = re.compile(r"^(\S+) (\S+) \[([^\]]+)\] /(.*)/$")
HAN = re.compile(r"^[\u3400-\u4dbf\u4e00-\u9fff\U00020000-\U0002ffff]+$")
# 手工指定常用项优先级，不代表语料词频，不来自用户输入。
COMMON = "的 是 我 你 他 她 它 们 在 有 不 了 和 人 一 个 好 中 国 时 十 事 为 这 那 来 去 吗 呢 上 下 大 小 多 少 开发 咖啡 你好 谢谢 再见 今天 明天 工作 学习 输入 翻译 隐私 安全 电脑 软件 设置 测试 语言 中国 中文 英文 我们 可以 使用 本地".split()
PRIORITY = {word: 1_000_000 - index * 1000 for index, word in enumerate(COMMON)}


def convert(text):
    words, senses, skipped = {}, {}, 0
    for number, raw in enumerate(text.splitlines(), 1):
        if not raw or raw.startswith('#'):
            continue
        match = ENTRY.fullmatch(raw)
        if not match:
            raise ValueError(f'Malformed CC-CEDICT entry at line {number}')
        _, word, reading, definitions = match.groups()
        syllables = [re.sub('[1-5]', '', part.lower().replace('u:', 'v').replace('ü', 'v')) for part in reading.split()]
        syllables = ['er' if part == 'r' else part for part in syllables]
        if not HAN.fullmatch(word) or len(word) != len(syllables) or not all(re.fullmatch('[a-z]+', part) for part in syllables):
            skipped += 1
            continue
        glosses = []
        for sense in definitions.split('/'):
            if not sense or sense.startswith('CL:'):
                continue
            # 英文表的竖线表示读音分隔符；保留交叉引用文字但换为斜线。
            sense = sense.replace('|', '/').replace('\t', ' ').strip()
            if sense:
                glosses.append(sense)
        if not glosses:
            skipped += 1
            continue
        key = (word, ' '.join(syllables))
        words[key] = PRIORITY.get(word, 1000)
        for sense in glosses:
            if sense not in senses.setdefault(word, []):
                senses[word].append(sense)
    header = '# Adapted from CC-CEDICT / MDBG contributors; CC BY-SA 4.0. See ../LICENSE.txt.\n'
    dictionary = header + ''.join(f'{word}\t{pinyin}\t{weight}\n' for (word, pinyin), weight in sorted(words.items()))
    # 引擎上限两条；第一条单列，其余合并为第二条，使详情仍保留全部释义。
    glossary = header + ''.join(word + '\t' + '\t'.join(glosses[:1] + (['; '.join(glosses[1:])] if len(glosses) > 1 else [])) + '\n' for word, glosses in sorted(senses.items()))
    return dictionary, glossary, {'dictionary_entries': len(words), 'glossary_words': len(senses), 'skipped_entries': skipped}
