#!/usr/bin/env bash
# Derive the MSRV and the crate version in the documentation from Cargo.toml,
# the single source of truth.
#
# Usage:
#   scripts/sync-docs.sh              rewrite the documentation
#   scripts/sync-docs.sh --print msrv
#   scripts/sync-docs.sh --print version

set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

read_manifest_value() {
  awk -v section_name="$1" -v key="$2" '
    $0 == "[" section_name "]" { section = 1; next }
    /^\[/ { section = 0 }
    section && $1 == key { gsub(/["[:space:]]/, "", $3); print $3; exit }
  ' Cargo.toml
}

msrv="$(read_manifest_value workspace.package rust-version)"
version="$(read_manifest_value package version)"

for value in "$msrv" "$version"; do
  if ! printf '%s' "$value" | grep -Eq '^[0-9]+(\.[0-9]+){0,2}$'; then
    echo "the value read from Cargo.toml is not a version: $value" >&2
    exit 1
  fi
done

if [ "${1:-}" = "--print" ]; then
  case "${2:-}" in
    msrv) printf '%s\n' "$msrv" ;;
    version) printf '%s\n' "$version" ;;
    *)
      echo "usage: scripts/sync-docs.sh --print [msrv|version]" >&2
      exit 1
    ;;
  esac
  exit 0
fi

git -c core.quotePath=false ls-files '*.md' \
  | grep -v -e '^README\.md$' -e '^README\.template\.md$' -e '^SECURITY\.md$' \
  | while IFS= read -r document; do
      sed -i -E \
        -e "s/(MSRV[^0-9]{0,24})[0-9]+\.[0-9]+/\1${msrv}/g" \
        -e "s/(rust-version = \")[0-9]+\.[0-9]+(\")/\1${msrv}\2/g" \
        -e "s/(targets Rust )[0-9]+\.[0-9]+/\1${msrv}/g" \
        -e "s/(masterror = \{ version = \")[0-9]+\.[0-9]+(\.[0-9]+)?(\")/\1${version}\3/g" \
        -e "s/(masterror = \")[0-9]+\.[0-9]+(\.[0-9]+)?(\")/\1${version}\3/g" \
        "$document"
    done

git diff --stat
