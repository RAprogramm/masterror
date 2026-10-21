#!/usr/bin/env bash
# Derive the MSRV in the documentation from the [workspace.package] section of
# Cargo.toml, the single source of truth.
#
# Usage:
#   scripts/sync-msrv.sh          rewrite the documentation
#   scripts/sync-msrv.sh --print  print the MSRV

set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

manifest="$(awk '
  /^\[workspace\.package\]/ { section = 1; next }
  /^\[/ { section = 0 }
  section && /^rust-version/ { gsub(/["[:space:]]/, "", $3); print $3; exit }
' Cargo.toml)"

if ! printf '%s' "$manifest" | grep -Eq '^[0-9]+(\.[0-9]+){0,2}$'; then
  echo "the MSRV in the [workspace.package] section of Cargo.toml is not a version: $manifest" >&2
  exit 1
fi

if [ "${1:-}" = "--print" ]; then
  printf '%s\n' "$manifest"
  exit 0
fi

git -c core.quotePath=false ls-files '*.md' \
  | grep -v -e '^README\.template\.md$' -e '^SECURITY\.md$' \
  | while IFS= read -r document; do
      sed -i -E \
        -e "s/(MSRV[^0-9]{0,24})[0-9]+\.[0-9]+/\1${manifest}/g" \
        -e "s/(rust-version = \")[0-9]+\.[0-9]+(\")/\1${manifest}\2/g" \
        -e "s/(targets Rust )[0-9]+\.[0-9]+/\1${manifest}/g" \
        "$document"
    done

git diff --stat
