<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.25.0] - 2025-10-29

### Added

- Integrate crates.io publishing into Auto Release
- Enforce dependency publish order in Auto Release
- Make Auto Release workflow idempotent
- Add license symlink

### Fixed

- Remove emojis from Auto Release workflow
- Grant write permissions to reusable CI in Release workflow
- Check crates.io version before publishing each package
- Move codecov.yml to correct location
- Update Codecov badge URLs to new format
- Match infra repo codecov configuration exactly
- Remove pip cache requirement from translation workflow
- Use GH_TOKEN for protected branch push in changelog workflows
- Rebase before push in Auto Release workflow

### Testing

- Test path
- Test path 2
- Trigger AI translation workflow

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci] (#262) (#262)
- Update CHANGELOG.md [skip ci]
- Bump version to 0.25.0 and remove Apache-2.0 license
- Update CHANGELOG.md [skip ci]
- Update CHANGELOG.md [skip ci]

## [0.24.19] - 2025-10-12

### Added

- Add explicit permissions to workflow jobs
- Add Codecov Test Analytics with organized structure

### Fixed

- Enable OIDC tokenless upload for Codecov v5
- Add id-token permission for Codecov OIDC in CI workflow
- Auto Release now tracks masterror package version
- Release workflow triggers on GitHub release creation
- Add permissions to checks job in Release workflow
- Professional Release workflow with robust event handling
- Use correct inputs reference in checkout
- Use github.event.inputs consistently
- Use num-cpus for nextest test-threads

### Documentation

- Add Codecov badge and coverage visualizations
- Add Codecov badge and coverage visualizations

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]

### CI

- Upgrade codecov action to v5
- Upgrade codecov action to v5
- Trigger CI run

## [0.24.18] - 2025-10-09

## [0.24.16] - 2025-10-05

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.24.17] - 2025-10-05

### Added

- Rultor
- Add reuse
- Add env to dependbot
- Add reuse
- Add anyhow-compatible error chain API
- Add comparative benchmarks vs thiserror/anyhow
- Add basic_usage example
- Add derive_error example
- Add structured_metadata example
- Add downcast API for anyhow parity
- Add simple .context() method for anyhow parity
- Add simple .context() method for anyhow parity
- Add redaction example

### Fixed

- **ci**: Declare benchmarks feature for benches

### Documentation

- Add binary size and compilation time metrics
- Update WHY_MIGRATE.md with new anyhow parity features

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]

### Merge

- Resolve README.md conflict from upstream

## [0.24.12] - 2025-09-30

### Testing

- Test 1

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.24.11] - 2025-09-30

### Fixed

- Gate provide shim based on error request support
- **ci**: Correct reusable workflow indentation

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **ci**: Extract cargo steps into composite actions

## [0.24.10] - 2025-09-30

### Miscellaneous

- Prepare 0.24.10 release
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]

## [0.24.8] - 2025-09-28

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.24.9] - 2025-09-28

### Added

- Restore AppError::with_context helper

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- Fix lint warning

## [0.21.1] - 2025-09-24

## [0.21.0] - 2025-09-24

### Added

- Add metadata container and richer app error
- Store shared sources and lazy backtraces
- 2 tips

### Documentation

- Add error-handling wiki
- Refresh readmes for expanded scope
- Rewrite README for 0.20 workspace

### Refactored

- Enrich converter errors with context metadata

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]

## [0.10.9] - 2025-09-21

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.11.0] - 2025-09-21

### Refactored

- Improve database constructor ergonomics

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.10.8] - 2025-09-21

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.10.7] - 2025-09-20

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.10.6] - 2025-09-20

### Testing

- Test 2

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.10.5] - 2025-09-20

### Testing

- Test 1

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]

## [0.10.4] - 2025-09-20

### Fixed

- Release action
- Release action

### Miscellaneous

- **readme**: Auto-refresh [skip ci]

## [0.9.0] - 2025-09-20

### Added

- Rust version
- **template**: Support implicit placeholders
- Target.md
- Derive AppError conversions
- Target.md
- Idea.md
- Expose telemetry via provide attribute

### Fixed

- Manifest
- **ci**: Allow cargo package dry run

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- Harden sqlx integration and add audit checks
- **ci**: Add cargo deny checks
- **readme**: Auto-refresh [skip ci]

## [0.5.0] - 2025-09-18

### Added

- **derive**: Support #[from] conversions
- **derive**: Add transparent error support

### Fixed

- **ci**: Grant permissions to reusable workflow

### Documentation

- Add release checklist to readme
- Regenerate readme comment
- Capture post-0.4 updates
- Emphasize readme sync requirement
- Prepare 0.5.0 release notes

### Testing

- Test readme
- Enforce AppResult alias usage
- Test readme 2
- Test ci 1

### Miscellaneous

- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]
- **readme**: Auto-refresh [skip ci]

### Build

- Align readme generator toml version

## [0.4.0] - 2025-09-16

### Added

- Add frontend console logging support

## [0.3.5] - 2025-09-12

### Added

- **telegram**: Expand validation error coverage
- Convert telegram init validation errors
- Add teloxide error mapping

## [0.3.3] - 2025-09-11

### Added

- Classify serde_json errors
- Add generic details helper
- Add duration-based retry helper
- Log error code

### Documentation

- Clarify case-insensitive helpers
- Add 0.3.3 release notes
- Record recent changes
- Explain config error mapping
- Sync changelog with recent mappings

### Refactored

- Streamline status code conversion

### Testing

- Add integration conversion tests
- Check service fallback for unknown turnkey message
- **actix**: Validate headers for AppError

## [0.3.2] - 2025-09-08

### Added

- Turnkey
- Turnkey

## [0.3.1] - 2025-08-25

### Added

- Release script

### Documentation

- Update changelog for 0.3.1 (IntoResponse for AppError)
- Update changelog for 0.3.1 (IntoResponse for AppError)

### Masterror

- Add IntoResponse for AppError under axum feature

## [0.3.0] - 2025-08-24

### Added

- Introduce AppCode and new ErrorResponse fields

### Fixed

- Convert test

## [0.2.1] - 2025-08-20

### Miscellaneous

- **release**: Prepare v0.2.0

## [0.2.0] - 2025-08-20

### Added

- Add help task in makefile
- Opencollective link
- **actix**: AppError as ResponseError; ErrorResponse as Responder; docs/readme
- **actix**: AppError as ResponseError; ErrorResponse as Responder; docs/readme

## [0.1.1] - 2025-08-12

### Added

- README.md
- License
- Makefile
- Add pre commit

### Fixed

- Fix
- License name
- Repo link
- Fix pre commit
- Fix pre commit
- Fix ci 1
- Fix ci 2
- Fix ci 3
- Funding

### Testing

- Auto publish

[0.25.0]: https://github.com/RAprogramm/masterror/compare/v0.24.19...v0.25.0
[0.24.19]: https://github.com/RAprogramm/masterror/compare/v0.24.18...v0.24.19
[0.24.18]: https://github.com/RAprogramm/masterror/compare/v0.24.16...v0.24.18
[0.24.16]: https://github.com/RAprogramm/masterror/compare/v0.24.17...v0.24.16
[0.24.17]: https://github.com/RAprogramm/masterror/compare/v0.24.12...v0.24.17
[0.24.12]: https://github.com/RAprogramm/masterror/compare/v0.24.11...v0.24.12
[0.24.11]: https://github.com/RAprogramm/masterror/compare/v0.24.10...v0.24.11
[0.24.10]: https://github.com/RAprogramm/masterror/compare/v0.24.8...v0.24.10
[0.24.8]: https://github.com/RAprogramm/masterror/compare/v0.24.9...v0.24.8
[0.24.9]: https://github.com/RAprogramm/masterror/compare/v0.21.1...v0.24.9
[0.21.1]: https://github.com/RAprogramm/masterror/compare/v0.21.0...v0.21.1
[0.21.0]: https://github.com/RAprogramm/masterror/compare/v0.10.9...v0.21.0
[0.10.9]: https://github.com/RAprogramm/masterror/compare/v0.11.0...v0.10.9
[0.11.0]: https://github.com/RAprogramm/masterror/compare/v0.10.8...v0.11.0
[0.10.8]: https://github.com/RAprogramm/masterror/compare/v0.10.7...v0.10.8
[0.10.7]: https://github.com/RAprogramm/masterror/compare/v0.10.6...v0.10.7
[0.10.6]: https://github.com/RAprogramm/masterror/compare/v0.10.5...v0.10.6
[0.10.5]: https://github.com/RAprogramm/masterror/compare/v0.10.4...v0.10.5
[0.10.4]: https://github.com/RAprogramm/masterror/compare/v0.9.0...v0.10.4
[0.9.0]: https://github.com/RAprogramm/masterror/compare/v0.5.0...v0.9.0
[0.5.0]: https://github.com/RAprogramm/masterror/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/RAprogramm/masterror/compare/v0.3.5...v0.4.0
[0.3.5]: https://github.com/RAprogramm/masterror/compare/v0.3.3...v0.3.5
[0.3.3]: https://github.com/RAprogramm/masterror/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/RAprogramm/masterror/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/RAprogramm/masterror/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/RAprogramm/masterror/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/RAprogramm/masterror/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/RAprogramm/masterror/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.1.1

