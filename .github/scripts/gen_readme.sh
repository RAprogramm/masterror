#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
#
# SPDX-License-Identifier: MIT

set -euo pipefail

# Deterministic env for local runs too
export TZ="${TZ:-UTC}"
export LC_ALL="${LC_ALL:-C.UTF-8}"
export NO_COLOR="${NO_COLOR:-1}"
export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-never}"
export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-0}"

# Allow forcing toolchain via TOOLCHAIN env (e.g. "+1.78.0")
TOOLCHAIN="${TOOLCHAIN:-}"

# If you use cargo-readme, prefer it
if command -v cargo-readme >/dev/null 2>&1; then
  cargo ${TOOLCHAIN} readme > README.md
  exit 0
fi

# If you have your own generator, call it here instead.
# Examples (uncomment the one you actually use):
# cargo ${TOOLCHAIN} xtask readme
# cargo ${TOOLCHAIN} run -p readme-gen
# cargo ${TOOLCHAIN} run --bin readme_gen
# cargo ${TOOLCHAIN} readme > README.md

# Fallback: no-op to keep CI green if README is static
touch README.md

