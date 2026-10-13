"""在临时目录检查规则安装、内容保留和异常处理，不修改实际安装文件。"""
import shutil
import stat
import subprocess
import tempfile
import unittest
from pathlib import Path


class RuleInstallerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="rule-installer-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        (self.root / "scripts").mkdir()
        (self.root / "agentmd").mkdir()
        self.script = self.root / "scripts/setup-rule.sh"
        shutil.copyfile(Path(__file__).resolve().parents[1] / "scripts/setup-rule.sh", self.script)
        self.source = self.root / "agentmd/global.md"
        self.source.write_bytes("# 通用规则\n\n使用中文。\n".encode())
        self.target = self.root / "installed/AGENTS.md"

    def run_install(self, rule="global", agent=None, destination=None, success=True):
        result = subprocess.run(
            ["bash", str(self.script), rule] + ([agent] if agent else [])
            + ["--destination", str(destination or self.target)],
            cwd=self.root.parent, capture_output=True, text=True,
        )
        self.assertEqual(result.returncode == 0, success, result.stdout + result.stderr)
        return result

    def write_target(self, content):
        self.target.parent.mkdir(parents=True, exist_ok=True)
        self.target.write_bytes(content)

    def test_new_install_and_noop(self):
        self.run_install()
        content = self.target.read_bytes()
        self.assertIn(self.source.read_bytes(), content)
        self.assertEqual(content.count(b": start -->"), 1)
        before = self.target.stat().st_mtime_ns
        result = self.run_install()
        self.assertIn("Unchanged", result.stdout)
        self.assertEqual(self.target.stat().st_mtime_ns, before)

    def test_identical_legacy_file_is_adopted(self):
        self.write_target(self.source.read_bytes())
        self.run_install()
        self.assertEqual(self.target.read_bytes().count(self.source.read_bytes()), 1)

    def test_update_preserves_surrounding_bytes_and_mode(self):
        self.run_install()
        before, after = b"prefix\r\n\n", b"\n\nother rules\n\n"
        self.write_target(before + self.target.read_bytes() + after)
        self.target.chmod(0o640)
        self.source.write_bytes(b"changed without final newline")
        self.run_install()
        result = self.target.read_bytes()
        self.assertTrue(result.startswith(before))
        self.assertTrue(result.endswith(after))
        self.assertIn(b"changed without final newline\n<!--", result)
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o640)

    def test_unmarked_content_is_preserved_on_install_and_update(self):
        original = b"existing personal instructions\r\n\n"
        for rule in ("global", "workspace"):
            with self.subTest(rule=rule):
                destination = self.root / rule / "AGENTS.md"
                destination.parent.mkdir()
                destination.write_bytes(original)
                destination.chmod(0o600)
                source = self.root / "agentmd" / f"{rule}.md"
                source.write_bytes(b"first version\n")
                self.run_install(rule=rule, destination=destination)
                source.write_bytes(b"updated version\n")
                self.run_install(rule=rule, destination=destination)
                installed = destination.read_bytes()
                start = f"<!-- Personal Agent Skills: {rule}: start -->".encode()
                self.assertEqual(installed.split(start)[0], original)
                self.assertEqual(installed.count(start), 1)
                self.assertIn(source.read_bytes(), installed)
                self.assertNotIn(b"first version", installed)
                self.assertEqual(stat.S_IMODE(destination.stat().st_mode), 0o600)
                result = self.run_install(rule=rule, destination=destination)
                self.assertIn("Unchanged", result.stdout)

    def test_independent_rule_appends_and_updates_without_touching_global(self):
        self.run_install()
        original = self.target.read_bytes()
        rule = self.root / "agentmd/english.md"
        rule.write_bytes(b"first version\n")
        self.run_install(rule="english")
        rule.write_bytes(b"second version\n")
        self.run_install(rule="english")
        result = self.target.read_bytes()
        self.assertTrue(result.startswith(original))
        self.assertNotIn(b"first version", result)
        self.assertEqual(result.count(b"english: start"), 1)

    def test_malformed_markers_leave_destination_unchanged(self):
        start = b"<!-- Personal Agent Skills: global: start -->"
        end = b"<!-- Personal Agent Skills: global: end -->"
        nested = b"<!-- Personal Agent Skills: other: start -->"
        cases = [start, end, end + b"\n" + start,
                 start + b"\n" + start + b"\n" + end,
                 start + b"\n" + end + b"\n" + end,
                 b"prefix " + start + b"\n" + end,
                 start + b"\n" + end + b" suffix",
                 start + b"\n" + nested + b"\n" + end]
        for content in cases:
            with self.subTest(content=content):
                self.write_target(content)
                self.run_install(success=False)
                self.assertEqual(self.target.read_bytes(), content)

    def test_symlink_and_referent_permissions_are_preserved(self):
        self.target.parent.mkdir()
        referent = self.root / "actual.md"
        referent.write_bytes(self.source.read_bytes())
        referent.chmod(0o600)
        self.target.symlink_to(referent)
        self.run_install()
        self.assertTrue(self.target.is_symlink())
        self.assertIn(b"global: start", referent.read_bytes())
        self.assertEqual(stat.S_IMODE(referent.stat().st_mode), 0o600)

    def test_broken_symlink_is_rejected(self):
        self.target.parent.mkdir()
        referent = self.root / "missing.md"
        self.target.symlink_to(referent)
        self.run_install(success=False)
        self.assertTrue(self.target.is_symlink())
        self.assertFalse(referent.exists())

    def test_explicit_codex_argument_matches_default(self):
        self.run_install()
        original = self.target.read_bytes()
        result = self.run_install(agent="codex")
        self.assertIn("Unchanged", result.stdout)
        self.assertEqual(self.target.read_bytes(), original)

    def test_invalid_inputs_leave_existing_file_unchanged(self):
        original = b"untouched\n"
        self.write_target(original)
        for rule, agent in (("../global", "codex"), ("missing", "codex"), ("global", "unknown")):
            self.run_install(rule=rule, agent=agent, success=False)
            self.assertEqual(self.target.read_bytes(), original)
        for content in (b"", b"  \n", b"<!-- Personal Agent Skills: global: start -->\n"):
            self.source.write_bytes(content)
            self.run_install(success=False)
            self.assertEqual(self.target.read_bytes(), original)

    def test_source_cannot_be_destination(self):
        original = self.source.read_bytes()
        self.run_install(destination=self.source, success=False)
        self.assertEqual(self.source.read_bytes(), original)


if __name__ == "__main__":
    unittest.main()
