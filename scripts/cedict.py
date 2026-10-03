#!/usr/bin/env python3
"""把 CC-CEDICT 转为只读 TSV；转换结果按 CC BY-SA 4.0 分发。"""
import re

ENTRY = re.compile(r"^(\S+) (\S+) \[([^\]]+)\] /(.*)/$")
HAN = re.compile(r"^[\u3400-\u4dbf\u4e00-\u9fff\U00020000-\U0002ffff]+$")
# jieba 只有词形词频。以下生僻、姓氏或限于复词的读音不应继承常用读音的全部权重。
# 这是公开、固定的排序修正（1/100），不是按读音统计的真实词频。
RARE_READINGS = {('见', 'xian'), ('听', 'yin'), ('区', 'ou'), ('行', 'heng'),
                 ('什', 'shi'), ('的', 'di'), ('说', 'shui')}

def parse_frequencies(text):
    """读取固定 jieba 词频；不推断未收录词、不借用个人输入。"""
    frequencies = {}
    for number, raw in enumerate(text.splitlines(), 1):
        fields = raw.split()
        if len(fields) != 3 or not fields[1].isascii() or not fields[1].isdigit():
            raise ValueError(f'Malformed frequency entry at line {number}')
        word, count, _ = fields
        count = int(count)
        if count > 0xffffffff:
            raise ValueError(f'Frequency out of range at line {number}')
        # 同词多次出现时取最大值；零频词保留最低权重。
        frequencies[word] = max(frequencies.get(word, 1), count)
    return frequencies


def convert(text, frequencies=None, supplement=''):
    frequencies = frequencies or {}
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
        weight = frequencies.get(word, 1)
        if key in RARE_READINGS:
            weight = max(1, weight // 100)
        words[key] = weight
        for sense in glosses:
            if sense not in senses.setdefault(word, []):
                senses[word].append(sense)
    added = 0
    seen = set()
    for number, row in enumerate(supplement.splitlines(), 1):
        if not row or row.startswith('#'):
            continue
        fields = row.split('\t')
        if len(fields) != 4:
            raise ValueError(f'Malformed supplement at line {number}')
        word, pinyin, weight, gloss = fields
        syllables = pinyin.split(' ')
        if (not HAN.fullmatch(word) or len(word) != len(syllables)
                or not all(re.fullmatch('[a-z]+', part) for part in syllables)
                or not weight.isascii() or not weight.isdigit()
                or not 1 <= int(weight) <= 0xffffffff
                or not gloss.strip() or '|' in gloss):
            raise ValueError(f'Invalid supplement at line {number}')
        key = (word, pinyin)
        if key in seen:
            raise ValueError(f'Duplicate supplement at line {number}')
        seen.add(key)
        # 已有词典的读音和释义保留，只补缺失短语。
        if key not in words:
            words[key] = frequencies.get(word, int(weight))
            senses.setdefault(word, [gloss.strip()])
            added += 1
    header = '# CC-CEDICT / MDBG and LocalGloss contributors; CC BY-SA 4.0. Static weights from jieba / Sun Junyi; MIT; supplemental fallback weights are curated, not corpus counts. See bundled notices.\n'
    if not frequencies:
        header = '# CC-CEDICT / MDBG and LocalGloss contributors; CC BY-SA 4.0. See bundled notices.\n'
    dictionary = header + ''.join(f'{word}\t{pinyin}\t{weight}\n' for (word, pinyin), weight in sorted(words.items()))
    # 引擎上限两条；第一条单列，其余合并为第二条，使详情仍保留全部释义。
    glossary = header + ''.join(word + '\t' + '\t'.join(glosses[:1] + (['; '.join(glosses[1:])] if len(glosses) > 1 else [])) + '\n' for word, glosses in sorted(senses.items()))
    return dictionary, glossary, {'dictionary_entries': len(words), 'glossary_words': len(senses), 'skipped_entries': skipped, 'supplement_added_entries': added,
                                 'frequency_matched_entries': sum(word in frequencies for word, _ in words)}
