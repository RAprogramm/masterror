#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
#
# SPDX-License-Identifier: MIT

set -euo pipefail

CRATES="masterror-template masterror-derive masterror"

local_version() {
  local crate=$1
  cargo metadata --no-deps --format-version=1 |
    jq -r --arg n "$crate" '.packages[] | select(.name == $n and .source == null) | .version'
}

published_version() {
  local crate=$1 response version
  if ! response=$(curl -sSf -A "masterror-ci/2.0" "https://crates.io/api/v1/crates/$crate"); then
    echo "::error::$crate: crates.io is not reachable" >&2
    return 1
  fi
  version=$(printf '%s' "$response" | jq -r '.crate.max_version // empty')
  if [ -z "$version" ]; then
    echo "::error::$crate: crates.io returned no published version" >&2
    return 1
  fi
  printf '%s' "$version"
}

version_gt() {
  local newer=$1 older=$2
  [ "$newer" != "$older" ] &&
    [ "$(printf '%s\n%s\n' "$older" "$newer" | sort -V | head -n1)" = "$older" ]
}

compare_versions() {
  local publish=false crate local_ver published_ver
  for crate in $CRATES; do
    local_ver=$(local_version "$crate")
    if [ -z "$local_ver" ] || [ "$local_ver" = "null" ]; then
      echo "::error::$crate: version not found in the workspace manifests" >&2
      return 1
    fi
    published_ver=$(published_version "$crate") || return 1
    echo "$crate: local=$local_ver published=$published_ver" >&2
    if [ "$local_ver" = "$published_ver" ]; then
      echo "::notice::$crate $published_ver is already published" >&2
      continue
    fi
    if ! version_gt "$local_ver" "$published_ver"; then
      echo "::error::$crate: repository version $local_ver is older than the published $published_ver" >&2
      return 1
    fi
    publish=true
  done
  printf '%s' "$publish"
}

if [ "${BASH_SOURCE[0]}" = "$0" ]; then
  compare_versions
  printf '\n'
fi
