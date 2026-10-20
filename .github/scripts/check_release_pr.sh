#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
#
# SPDX-License-Identifier: MIT

set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
source "$SCRIPT_DIR/compare_versions.sh"

BASE=${1:-}
CRATES="masterror-template masterror-derive masterror"

if [ -z "$BASE" ]; then
  echo "::error::the base commit is required" >&2
  exit 1
fi

if ! git cat-file -e "$BASE^{commit}" 2>/dev/null; then
  echo "::error::base commit $BASE is not available" >&2
  exit 1
fi

MERGE_BASE=$(git merge-base "$BASE" HEAD)

crate_manifest() {
  case "$1" in
    masterror) echo "Cargo.toml" ;;
    *) echo "$1/Cargo.toml" ;;
  esac
}

crate_root() {
  case "$1" in
    masterror) echo "." ;;
    *) echo "$1" ;;
  esac
}

include_patterns() {
  python3 - "$1" <<'PY'
import sys
import tomllib

with open(sys.argv[1], "rb") as manifest:
    data = tomllib.load(manifest)
for pattern in data.get("package", {}).get("include", []):
    print(pattern)
PY
}

matches_include() {
  local file=$1 pattern=$2
  if [[ "$pattern" == *"/**" ]]; then
    local prefix=${pattern%/**}
    [[ "$file" == "$prefix"/* ]]
  elif [[ "$pattern" == */* ]]; then
    [[ "$file" == "$pattern" ]]
  else
    [[ "$(basename "$file")" == "$pattern" ]]
  fi
}

changed_files() {
  local crate=$1 file
  local -a patterns=()
  while IFS= read -r pattern; do
    patterns+=("$pattern")
  done < <(include_patterns "$(crate_manifest "$crate")")

  while IFS= read -r file; do
    [ -n "$file" ] || continue
    if [ ${#patterns[@]} -eq 0 ]; then
      case "$file" in
        "$(crate_root "$crate")"/*) printf '%s\n' "$file" ;;
        *) ;;
      esac
      continue
    fi
    for pattern in "${patterns[@]}"; do
      if matches_include "$file" "$pattern"; then
        printf '%s\n' "$file"
        break
      fi
    done
  done < <(git diff --name-only "$MERGE_BASE" HEAD)
}

status=0

printf '| Crate | Published | In this PR | Changed files | Action |\n'
printf '|---|---|---|---|---|\n'

for crate in $CRATES; do
  published=$(published_version "$crate") || exit 1
  local_ver=$(local_version "$crate")
  mapfile -t files < <(changed_files "$crate")

  if [ "$local_ver" = "$published" ]; then
    if [ ${#files[@]} -gt 0 ]; then
      action="changes are not released, the version must be bumped"
      status=1
    else
      action="no changes, nothing to publish"
    fi
  elif version_gt "$local_ver" "$published"; then
    action="publish $local_ver"
  else
    action="the version is behind the published one, the repository is out of sync"
    status=1
  fi

  if [ ${#files[@]} -gt 0 ]; then
    changed=$(printf '%s<br>' "${files[@]}")
  else
    changed="none"
  fi

  printf '| %s | %s | %s | %s | %s |\n' \
    "$crate" "$published" "$local_ver" "$changed" "$action"
done

exit "$status"
