#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
#
# SPDX-License-Identifier: MIT

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOKS_DIR="$REPO_ROOT/.git/hooks"
PRE_COMMIT_SCRIPT="$REPO_ROOT/.github/scripts/pre-commit"

if [ ! -f "$PRE_COMMIT_SCRIPT" ]; then
  echo "Error: pre-commit script not found at $PRE_COMMIT_SCRIPT"
  exit 1
fi

echo "Installing pre-commit hook..."
cp "$PRE_COMMIT_SCRIPT" "$HOOKS_DIR/pre-commit"
chmod +x "$HOOKS_DIR/pre-commit"

echo "✅ Pre-commit hook installed successfully!"
echo ""
echo "The hook will run the following checks before each commit:"
echo "  - rustfmt (nightly)"
echo "  - REUSE compliance"
echo "  - clippy (all features, all targets)"
echo "  - tests (all features)"
echo "  - cargo audit"
echo "  - cargo deny (advisories, bans, licenses, sources)"
echo "  - README.md generation"
