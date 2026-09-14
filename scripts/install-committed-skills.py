#!/usr/bin/env python3
import hashlib
import io
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import tempfile


def git(repo, *args):
    return subprocess.check_output(["git", "-C", str(repo), *args])


def fingerprint(directory):
    if directory.is_symlink() or not directory.is_dir():
        return None
    result = {}
    for path in sorted(directory.rglob("*")):
        if path.is_symlink():
            return None
        if path.is_file():
            result[str(path.relative_to(directory))] = hashlib.sha256(
                path.read_bytes()
            ).hexdigest()
    return result


def install(repo, revision, installed_root, runner=subprocess.run):
    commit = git(repo, "rev-parse", "--verify", revision + "^{commit}").decode().strip()
    paths = git(repo, "ls-tree", "-r", "--name-only", "-z", commit, "--", "skills")
    if not paths:
        return []

    with tempfile.TemporaryDirectory(prefix="personal-skills-install-") as temporary:
        source = Path(temporary)
        archive = git(repo, "archive", commit, "--", "skills")
        with tarfile.open(fileobj=io.BytesIO(archive)) as bundle:
            if any(not (member.isfile() or member.isdir()) for member in bundle):
                raise RuntimeError("已提交的 skills 中含链接或特殊文件，无法验证独立安装副本")
            bundle.extractall(source, filter="data")

        skills = {}
        for definition in sorted((source / "skills").glob("*/SKILL.md")):
            name = definition.parent.name
            text = definition.read_text()
            frontmatter = text.split("---", 2)
            if not text.startswith("---\n") or len(frontmatter) != 3:
                raise RuntimeError(f"{name}: SKILL.md 缺少 frontmatter")
            match = re.search(r"^name:\s*(['\"]?)([a-z0-9-]+)\1\s*$", frontmatter[1], re.M)
            if not match or match[2] != name:
                raise RuntimeError(f"{name}: 技能名称必须与目录名一致")
            skills[name] = fingerprint(definition.parent)

        changed = [
            name for name, expected in skills.items()
            if fingerprint(installed_root / name) != expected
        ]
        if changed:
            command = [
                "timeout", "120s", "npx", "--yes", "skills@1.5.22", "add", str(source),
                "--skill", *changed, "--global", "--agent", "codex", "--copy", "--yes",
            ]
            result = runner(
                command, cwd=source, env={**os.environ, "DISABLE_TELEMETRY": "1", "CI": "1"},
                text=True, capture_output=True,
            )
            if result.returncode:
                details = (result.stdout + result.stderr)[-4000:]
                raise RuntimeError(f"npx skills 安装失败（{result.returncode}）：\n{details}")

        for name, expected in skills.items():
            if fingerprint(installed_root / name) != expected:
                raise RuntimeError(f"{name}: 安装副本与提交 {commit[:7]} 不一致")
        if changed:
            print(f"已安装并验证：{', '.join(changed)}（提交 {commit[:7]}）")
        else:
            print(f"本机 {len(skills)} 个技能已与提交 {commit[:7]} 一致，无需重装。")
        return changed


if __name__ == "__main__":
    try:
        repository = Path(git(Path.cwd(), "rev-parse", "--show-toplevel").decode().strip())
        revision = sys.argv[1] if len(sys.argv) == 2 else "HEAD"
        install(repository, revision, Path.home() / ".agents" / "skills")
    except (OSError, RuntimeError, subprocess.CalledProcessError, tarfile.TarError) as error:
        print(f"本机技能安装失败：{error}", file=sys.stderr)
        sys.exit(1)
