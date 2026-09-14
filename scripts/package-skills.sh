#!/usr/bin/env bash

set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
name="personal-agent-skills-$(date '+%Y%m%d-%H%M%S')"
output="$repo_dir/$name.zip"
temp_dir="$(mktemp -d)"
package_dir="$temp_dir/$name"

cleanup() {
  rm -rf "$temp_dir"
}
trap cleanup EXIT

mkdir -p "$package_dir/skills"

skill_count=0
for skill_file in "$repo_dir"/skills/*/SKILL.md; do
  [ -f "$skill_file" ] || continue
  skill_dir="$(dirname "$skill_file")"
  cp -R "$skill_dir" "$package_dir/skills/"
  skill_count=$((skill_count + 1))
done

[ "$skill_count" -gt 0 ] || { echo "没有找到可打包的 skill" >&2; exit 1; }

branch="$(git -C "$repo_dir" branch --show-current)"
git -C "$package_dir" init --quiet --template= --initial-branch="${branch:-main}"

(
  cd "$temp_dir"
  /usr/bin/zip -q -r -y -X "$output" "$name"
)

/usr/bin/unzip -tq "$output" >/dev/null
echo "已生成：${output}（${skill_count} 个 skill）"
