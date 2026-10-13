#!/usr/bin/env bash
# Adapted from https://github.com/tw93/Waza/blob/main/scripts/setup-rule.sh
# Copyright (c) 2026 Tw93. MIT license.
# 首次安装追加标记区块；后续仅更新该区块，保留其余正文。
# 安装前检查新旧规则语义；可明确消除的冲突先修订仓库源文件，不能确定时交由用户判断。
set -euo pipefail

usage() {
  echo "Usage: bash scripts/setup-rule.sh <rule-name> [codex] [--destination <file>]"
}
if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi
if (( $# < 1 )); then
  usage >&2
  exit 1
fi
rule_name="$1"
shift
if [[ "${1:-}" == "codex" ]]; then
  shift
fi
destination_override=""
if (( $# > 0 )); then
  if (( $# != 2 )) || [[ "$1" != "--destination" ]] || [[ -z "$2" ]]; then
    usage >&2
    exit 1
  fi
  destination_override="$2"
fi
if [[ ! "$rule_name" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
  echo "Error: rule-name must contain lowercase letters, digits, and single internal hyphens." >&2
  exit 1
fi
command -v python3 >/dev/null 2>&1 || { echo "Error: python3 is required." >&2; exit 1; }
script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

python3 - "$script_dir/../agentmd/$rule_name.md" "$rule_name" "$destination_override" <<'PY'
import os
import stat
import sys
import tempfile
from pathlib import Path


def install():
    source_path = Path(sys.argv[1]).resolve(strict=True)
    rule, override = sys.argv[2:]
    source = source_path.read_bytes()
    source_text = source.decode("utf-8")
    if not source_text.strip():
        raise ValueError("Rule source is empty.")
    if "<!-- Personal Agent Skills:" in source_text:
        raise ValueError("Rule source must not contain installer markers.")
    codex_directory = Path(os.environ.get("CODEX_HOME", str(Path.home() / ".codex")))
    target = Path(override).expanduser() if override else codex_directory / "AGENTS.md"
    # Resolve symlinks before replacing, preserving the user's link itself.
    destination = target.resolve(strict=True) if target.is_symlink() else target.resolve()
    if destination == source_path:
        raise ValueError("Destination must differ from the rule source.")
    existing = destination.read_bytes() if destination.exists() else b""
    start = f"<!-- Personal Agent Skills: {rule}: start -->".encode()
    end = f"<!-- Personal Agent Skills: {rule}: end -->".encode()
    block = start + b"\n" + source + (b"" if source.endswith(b"\n") else b"\n") + end + b"\n"
    start_count, end_count = existing.count(start), existing.count(end)
    if start_count or end_count:
        if start_count != 1 or end_count != 1:
            raise ValueError("Missing or duplicate block markers; destination left unchanged.")
        first, last = existing.index(start), existing.index(end)
        if first >= last:
            raise ValueError("Block markers are out of order; destination left unchanged.")
        for position, marker in ((first, start), (last, end)):
            following = existing[position + len(marker):]
            if (position and existing[position - 1:position] != b"\n") or (following and not following.startswith(b"\n")):
                raise ValueError("Block markers must occupy separate lines.")
        if b"<!-- Personal Agent Skills:" in existing[first + len(start):last]:
            raise ValueError("Nested rule blocks; destination left unchanged.")
        tail = last + len(end)
        if existing[tail:tail + 1] == b"\n":
            tail += 1
        result = existing[:first] + block + existing[tail:]
    elif existing == source or not existing:
        # Adopt the identical pre-existing file without duplicating its rules.
        result = block
    else:
        separator = b"\n" * (2 - min(2, len(existing) - len(existing.rstrip(b"\n"))))
        result = existing + separator + block
    if result == existing:
        print(f"Unchanged: {target}")
        return
    mode = stat.S_IMODE(destination.stat().st_mode) if destination.exists() else 0o644
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(prefix=f".{destination.name}.", dir=destination.parent)
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(result)
            handle.flush()
            os.fsync(handle.fileno())
        os.chmod(temporary, mode)
        os.replace(temporary, destination)
    finally:
        temporary.unlink(missing_ok=True)
    print(f"Installed {rule} for Codex: {target}")


try:
    install()
except (OSError, ValueError) as error:
    print(f"Error: {error}", file=sys.stderr)
    sys.exit(1)
PY
