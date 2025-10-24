<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Changelog Automation

This project uses automated changelog generation with [git-cliff](https://git-cliff.org/).

## How It Works

### Automatic Updates

1. **On Every Commit to Main**:
   - GitHub Actions automatically updates the `[Unreleased]` section in `CHANGELOG.md`
   - See: `.github/workflows/changelog-update.yml`

2. **On Version Tag**:
   - When a tag like `v0.24.20` is pushed
   - GitHub Actions generates release notes from commits since last tag
   - Creates a GitHub Release with auto-generated notes
   - Updates `CHANGELOG.md` with the new version section
   - See: `.github/workflows/changelog.yml`

### Commit Format

For best results, use conventional commit format:

```
#<issue> <type>: <description>

Examples:
- #210 feat: add automatic changelog generation
- #291 fix: remove Apache-2.0 license references
- #232 test: add comprehensive tests for response mapping
```

**Supported types:**
- `feat`/`add` → Added section
- `fix` → Fixed section
- `doc` → Documentation section
- `perf` → Performance section
- `refactor` → Refactored section
- `test` → Testing section
- `chore` → Miscellaneous section
- `ci` → CI section
- `build` → Build section

### Configuration

All configuration is in `cliff.toml`:
- Commit parsing rules
- Section grouping
- Link generation
- Output format

### Manual Generation

Generate changelog manually:

```bash
# Full changelog
git cliff --output CHANGELOG.md

# Latest version only
git cliff --latest

# Unreleased changes
git cliff --unreleased

# Specific range
git cliff v0.24.18..v0.24.19
```

### GitHub Release Notes

Release notes are automatically generated from commits between tags:
- Grouped by type (Added, Fixed, etc.)
- Includes links to issues and PRs
- Shows contributors

Example: https://github.com/RAprogramm/masterror/releases

## Benefits

- **Consistency**: Same format across all versions
- **Automation**: No manual CHANGELOG maintenance
- **Visibility**: Clear history on GitHub Releases page
- **Links**: Automatic links to issues, PRs, and commits
- **Contributors**: Automatic contributor recognition
- **Keep a Changelog**: Follows [Keep a Changelog](https://keepachangelog.com/) format

## Development Workflow

1. Work on a feature branch
2. Use conventional commit format with issue numbers
3. Create PR (commit message format preserved)
4. Merge to main → `[Unreleased]` auto-updates
5. Create version tag → Release auto-created with changelog

No manual CHANGELOG editing needed!
