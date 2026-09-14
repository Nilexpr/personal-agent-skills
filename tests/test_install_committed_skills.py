import importlib.util
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("installer", ROOT / "scripts/install-committed-skills.py")
installer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(installer)


class InstallTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="skills-install-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.repo = self.base / "repo"
        self.repo.mkdir()
        self.installed = self.base / "installed"
        self.installed.mkdir()
        self.calls = []
        self.git("init", "-q", "--initial-branch=main")
        self.git("config", "core.hooksPath", "/dev/null")
        self.git("config", "user.name", "Test")
        self.git("config", "user.email", "test@example.invalid")
        self.skill = self.repo / "skills/example"
        self.skill.mkdir(parents=True)
        (self.skill / "SKILL.md").write_text("---\nname: example\ndescription: Test\n---\nCommitted\n")
        (self.skill / "example.txt").write_text("original\n")
        self.commit()

    def git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.repo), *args], stderr=subprocess.PIPE)

    def commit(self):
        self.git("add", "--all")
        self.git("commit", "-qm", "Test change")
        return self.git("rev-parse", "HEAD").decode().strip()

    def copy_runner(self, command, **kwargs):
        self.assertEqual(command[:6], ["timeout", "120s", "npx", "--yes", "skills@1.5.22", "add"])
        self.assertEqual(command[-5:], ["--global", "--agent", "codex", "--copy", "--yes"])
        source = Path(command[6])
        names = command[command.index("--skill") + 1:command.index("--global")]
        self.calls.append(names)
        for name in names:
            destination = self.installed / name
            if destination.exists():
                shutil.rmtree(destination)
            shutil.copytree(source / "skills" / name, destination)
        return subprocess.CompletedProcess(command, 0, "", "")

    def test_frozen_commit_excludes_dirty_and_untracked_work(self):
        commit = self.git("rev-parse", "HEAD").decode().strip()
        (self.skill / "example.txt").write_text("unfinished\n")
        (self.skill / "unfinished.txt").write_text("draft\n")
        installer.install(self.repo, commit, self.installed, self.copy_runner)
        self.assertEqual((self.installed / "example/example.txt").read_text(), "original\n")
        self.assertFalse((self.installed / "example/unfinished.txt").exists())
        self.assertFalse((self.installed / "example").is_symlink())

    def test_updates_resources_removes_stale_files_preserves_other_skills(self):
        shutil.copytree(self.skill, self.installed / "example")
        other = self.installed / "third-party"
        other.mkdir()
        (other / "SKILL.md").write_text("Keep this skill\n")
        (self.skill / "example.txt").unlink()
        (self.skill / "new.txt").write_text("new resource\n")
        self.commit()
        self.assertEqual(installer.install(self.repo, "HEAD", self.installed, self.copy_runner), ["example"])
        self.assertFalse((self.installed / "example/example.txt").exists())
        self.assertEqual((self.installed / "example/new.txt").read_text(), "new resource\n")
        self.assertEqual((other / "SKILL.md").read_text(), "Keep this skill\n")
        self.assertEqual(installer.install(self.repo, "HEAD", self.installed, self.copy_runner), [])
        self.assertEqual(self.calls, [["example"]])

    def test_installer_failure_is_reported(self):
        def fail(command, **kwargs):
            return subprocess.CompletedProcess(command, 7, "", "installation failed")
        with self.assertRaisesRegex(RuntimeError, "安装失败"):
            installer.install(self.repo, "HEAD", self.installed, fail)

    def test_success_without_matching_files_is_rejected(self):
        def incomplete(command, **kwargs):
            return subprocess.CompletedProcess(command, 0, "", "")
        with self.assertRaisesRegex(RuntimeError, "不一致"):
            installer.install(self.repo, "HEAD", self.installed, incomplete)

    def test_symlink_in_snapshot_is_rejected_before_install(self):
        (self.skill / "link.txt").symlink_to("example.txt")
        self.commit()
        with self.assertRaisesRegex(RuntimeError, "链接或特殊文件"):
            installer.install(self.repo, "HEAD", self.installed, self.copy_runner)
        self.assertEqual(self.calls, [])

    def test_push_failure_still_runs_local_install(self):
        expected = "https://github.com/Nilexpr/personal-agent-skills.git"
        self.git("remote", "add", "origin", expected)
        self.git("config", "url." + (self.base / "missing.git").as_uri() + ".insteadOf", expected)
        binaries = self.base / "bin"
        binaries.mkdir()
        marker = self.base / "install-arguments"
        python = binaries / "python3"
        python.write_text('#!/bin/sh\nprintf "%s\\n" "$@" > "$INSTALL_MARKER"\n')
        python.chmod(0o755)
        result = subprocess.run(
            [str(ROOT / ".githooks/post-commit")], cwd=self.repo,
            env={**os.environ, "PATH": str(binaries) + os.pathsep + os.environ["PATH"], "INSTALL_MARKER": str(marker)},
            text=True, capture_output=True,
        )
        self.assertEqual(result.returncode, 1)
        self.assertIn("继续更新本机技能", result.stderr)
        arguments = marker.read_text().splitlines()
        self.assertEqual(arguments[0], str(self.repo / "scripts/install-committed-skills.py"))
        self.assertEqual(arguments[1], self.git("rev-parse", "HEAD").decode().strip())


if __name__ == "__main__":
    unittest.main()
