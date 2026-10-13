#!/usr/bin/env bash

set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
[ "$#" -eq 0 ] || { echo "用法：bash scripts/package-skills.sh" >&2; exit 1; }
name="personal-agent-skills-$(date '+%Y%m%d-%H%M%S')-$$"
output="$repo_dir/$name.zip"
temp_dir="$(mktemp -d)"
package_dir="$temp_dir/$name"

cleanup() {
  rm -rf "$temp_dir"
}
trap cleanup EXIT

mkdir -p "$package_dir"

# 明确列出项目文件，避免带入本地配置、会话或旧压缩包。
for entry in README.md AGENTS.md DECISIONS.md .gitignore skills agentmd goals scripts; do
  cp -R "$repo_dir/$entry" "$package_dir/$entry"
done

if [ -d "$repo_dir/references" ]; then
  cp -R "$repo_dir/references" "$package_dir/references"
fi

(
  cd "$temp_dir"
  /usr/bin/zip -q -r -y -X "$temp_dir/$name.zip" "$name" \
    -x '*/.DS_Store' '*/__pycache__/*' '*.pyc' '*.zip' \
       '*/target/' '*/target/*' '*/.dependencies/' '*/.dependencies/*'
)

/usr/bin/unzip -tq "$temp_dir/$name.zip" >/dev/null
mv "$temp_dir/$name.zip" "$output"
echo "已生成：${output}"
