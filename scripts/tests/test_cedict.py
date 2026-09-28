"""数据转换边界；只使用小型虚构测试数据。"""
import importlib.util
import pathlib
import sys
import tempfile
import unittest
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from cedict import convert


class ConversionTests(unittest.TestCase):
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
