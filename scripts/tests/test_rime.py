"""Rime 固定词表转换边界；使用虚构条目，不读取个人输入。"""
import contextlib
import importlib.util
import io
import pathlib
import shutil
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from rime import compose_candidates, parse_rime

HEADER = '# Rime dictionary\n---\nname: pinyin_simp\n...\n'


class RimeTests(unittest.TestCase):
    def test_duplicate_uses_highest_weight_and_zero_has_minimum(self):
        words, counts = parse_rime(HEADER + '测试\tce shi\t9\n测试\tce shi\t3\n字\tzi\t0\n')
        self.assertEqual(words, {('测试', 'ce shi'): 9, ('字', 'zi'): 1})
        self.assertEqual(counts['rime_entries'], 2)

    def test_missing_header_is_rejected(self):
        for marker in ['# Rime dictionary\n', 'name: pinyin_simp\n', '...\n']:
            with self.assertRaises(ValueError):
                parse_rime(HEADER.replace(marker, '') + '字\tzi\t9\n')

    def test_malformed_rows_and_weights_are_rejected(self):
        for row in ['字\tzi', '字\tzi\t1\textra', '字\tzi\t-1', '字\tzi\t５',
                    '字\tzi\t4294967296', '字\tzi\tNaN']:
            with self.assertRaises(ValueError):
                parse_rime(HEADER + row)

    def test_unsupported_entries_do_not_enter_dictionary(self):
        words, counts = parse_rime(HEADER + 'abc\tabc\t9\n测试\tce\t9\n字\tzi4\t9\n')
        self.assertFalse(words)
        self.assertEqual(counts['rime_skipped'], 3)

    def test_wrong_readings_are_not_restored_by_fallback(self):
        words, counts = parse_rime(HEADER + '都是\tdu shi\t500\n都是\tdou shi\t900\n')
        result, added = compose_candidates(words, '都是\tdu shi\t99\n都是\tdou shi\t1\n测试\tce shi\t50000\n')
        self.assertNotIn('都是\tdu shi\t', result)
        self.assertIn('都是\tdou shi\t900\n', result)
        self.assertIn('测试\tce shi\t1\n', result)
        self.assertEqual(counts['rime_corrected'], 1)
        self.assertEqual(added, 1)

    def test_source_checksum_failure_leaves_no_partial_output(self):
        root = pathlib.Path(__file__).resolve().parents[2]
        spec = importlib.util.spec_from_file_location('rime_release_data', root / 'scripts/prepare-release-data.py')
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        with tempfile.TemporaryDirectory() as directory:
            fixture = pathlib.Path(directory)
            for name in ['cedict/cedict-ts.txt.gz', 'localgloss/common-phrases.tsv']:
                destination = fixture / 'assets' / name
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(root / 'assets' / name, destination)
            archive = fixture / 'assets/rime/pinyin_simp.dict.yaml.gz'
            archive.parent.mkdir(parents=True)
            archive.write_bytes(b'corrupt')
            with self.assertRaisesRegex(ValueError, 'Rime archive checksum mismatch'):
                with contextlib.redirect_stdout(io.StringIO()):
                    module.prepare(fixture)
            self.assertFalse((fixture / 'assets/cedict/generated').exists())
