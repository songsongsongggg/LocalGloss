"""数据转换边界；只使用小型虚构测试数据。"""
import importlib.util
import pathlib
import sys
import tempfile
import unittest
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from cedict import convert, parse_frequencies


class ConversionTests(unittest.TestCase):
    def test_supplement_fills_missing_phrase_without_replacing_source(self):
        source = '斗士 斗士 [dou4 shi4] /warrior/\n'
        supplement = '都是\tdou shi\t20000\tall are\n斗士\tdou shi\t99999\twrong gloss\n'
        dictionary, glossary, counts = convert(source, {'斗士': 500}, supplement)
        self.assertIn('都是\tdou shi\t20000\n', dictionary)
        self.assertIn('斗士\tdou shi\t500\n', dictionary)
        self.assertIn('斗士\twarrior\n', glossary)
        self.assertNotIn('wrong gloss', glossary)
        self.assertEqual(counts['supplement_added_entries'], 1)

    def test_supplement_validation(self):
        for text in ['都是\tdou\t20000\tall are', '都是\tdou shi\t0\tall are',
                     '都是\tdou shi\t20000\t', '都是\tdou shi\t20000\tall|are',
                     '都是\tdou shi\t20000\tall are\n' * 2]:
            with self.assertRaises(ValueError):
                convert('', {}, text)

    def test_static_frequency_beats_unicode_order_and_unknown_fallback(self):
        source = '亟需 亟需 [ji2 xu1] /urgently need/\n繼續 继续 [ji4 xu4] /continue/\n幾希 几希 [ji1 xi1] /very little/\n'
        dictionary, _, counts = convert(source, parse_frequencies('继续 14690 v\n亟需 52 v\n'))
        self.assertIn('继续\tji xu\t14690\n', dictionary)
        self.assertIn('亟需\tji xu\t52\n', dictionary)
        self.assertIn('几希\tji xi\t1\n', dictionary)
        self.assertEqual(counts['frequency_matched_entries'], 2)

    def test_rare_reading_does_not_inherit_full_word_frequency(self):
        dictionary, _, _ = convert('見 见 [jian4] /see/\n見 见 [xian4] /appear/\n', {'见': 10000})
        self.assertIn('见\tjian\t10000\n', dictionary)
        self.assertIn('见\txian\t100\n', dictionary)

    def test_frequency_validation_and_duplicate_policy(self):
        self.assertEqual(parse_frequencies('词 5 n\n词 3 n\n字 0 n\n'), {'词': 5, '字': 1})
        for invalid in ['词 -1 n', '词 4294967296 n', '词 x n', '词 5', '词 ５ n']:
            with self.assertRaises(ValueError):
                parse_frequencies(invalid)

    def test_polyphonic_dedup_and_reference_separator(self):
        source = '重 重 [zhong4] /heavy/CL:個|个[ge4]/\n重 重 [chong2] /again/heavy/\n'
        dictionary, glossary, counts = convert(source)
        self.assertIn('重\tzhong\t', dictionary)
        self.assertIn('重\tchong\t', dictionary)
        self.assertIn('重\theavy\tagain\n', glossary)
        self.assertNotIn('CL:', glossary)
        self.assertEqual(counts['glossary_words'], 1)
        _, referenced, _ = convert('詞 词 [ci2] /see 詞|词[ci2]/\n')
        self.assertNotIn('|', referenced)

    def test_all_senses_fit_two_slot_engine(self):
        _, glossary, _ = convert('詞 词 [ci2] /word/term/expression/\n')
        self.assertIn('词\tword\tterm; expression\n', glossary)

    def test_umlaut_and_unsupported_entry(self):
        dictionary, _, counts = convert('綠 绿 [lu:4] /green/\nA A [A] /letter A/\n')
        self.assertIn('绿\tlv\t', dictionary)
        self.assertEqual(counts['skipped_entries'], 1)

    def test_invalid_data_is_not_silently_accepted(self):
        with self.assertRaises(ValueError):
            convert('broken row')

    def test_checksum_failure_writes_no_generated_files(self):
        path = pathlib.Path(__file__).resolve().parents[1] / 'prepare-release-data.py'
        spec = importlib.util.spec_from_file_location('release_data', path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / 'assets/cedict').mkdir(parents=True)
            (root / 'assets/cedict/cedict-ts.txt.gz').write_bytes(b'corrupt')
            with self.assertRaises(ValueError):
                module.prepare(root)
            self.assertFalse((root / 'assets/cedict/generated').exists())
