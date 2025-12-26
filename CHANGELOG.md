<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased](https://github.com/RAprogramm/masterror/compare/v0.26.0...HEAD)

### Dependencies

- **deps:** Bump uuid from 1.18.1 to 1.19.0 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#336](https://github.com/RAprogramm/masterror/pull/336)
- **deps:** Bump metrics from 0.24.2 to 0.24.3 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#338](https://github.com/RAprogramm/masterror/pull/338)
- **deps:** Bump criterion from 0.7.0 to 0.8.1 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#339](https://github.com/RAprogramm/masterror/pull/339)
- **deps:** Bump tracing from 0.1.41 to 0.1.43 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#337](https://github.com/RAprogramm/masterror/pull/337)
- **deps:** Bump redis from 0.32.7 to 1.0.0 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#335](https://github.com/RAprogramm/masterror/pull/335)
## [0.26.0](https://github.com/RAprogramm/masterror/releases/tag/v0.26.0) - 2025-11-25

### Added

- Replace telegram-webapp-sdk with init-data-rs by [@RAprogramm](https://github.com/RAprogramm) ([19e1798](https://github.com/RAprogramm/masterror/commit/19e1798c7838a458a75392e9835605357211d4b4))

### CI/CD

- Allow changelog and release on workflow_dispatch by [@RAprogramm](https://github.com/RAprogramm) ([befaa0a](https://github.com/RAprogramm/masterror/commit/befaa0aafc197b2d422267999ce864dbe137a5ee))
- Fix nextest junit path and artifact upload by [@RAprogramm](https://github.com/RAprogramm) ([64099b4](https://github.com/RAprogramm/masterror/commit/64099b404c7ad938eb7a4731a5b8f9abfa56900e))
- Fix version comparison in release job by [@RAprogramm](https://github.com/RAprogramm) ([e78a64d](https://github.com/RAprogramm/masterror/commit/e78a64d4e2bbf2cbad04fc48068cf49e3e3c4f17))
- Fix changelog conflict resolution by [@RAprogramm](https://github.com/RAprogramm) ([39970a5](https://github.com/RAprogramm/masterror/commit/39970a5e7457ee5d912e2eb681ccbceece1b9aab))
- Professional unified CI pipeline by [@RAprogramm](https://github.com/RAprogramm) ([535ade6](https://github.com/RAprogramm/masterror/commit/535ade61e5c90ce878c47793b93a4031488a29d4))
- Consolidate all workflows into single CI pipeline by [@RAprogramm](https://github.com/RAprogramm) ([a50ea4b](https://github.com/RAprogramm/masterror/commit/a50ea4b74c02235aef4fdbfeb9881738b4a42197))
- Fix changelog generation and improve robustness by [@RAprogramm](https://github.com/RAprogramm) ([ae83a14](https://github.com/RAprogramm/masterror/commit/ae83a14b3f07ef609c6eb0cd2462dab2ade42314))
- Consolidate changelog into main CI workflow by [@RAprogramm](https://github.com/RAprogramm) ([b58ebb6](https://github.com/RAprogramm/masterror/commit/b58ebb69645eadf9d33663edd80840341e032fa9))

### Dependencies

- **deps:** Bump syn from 2.0.107 to 2.0.111 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#333](https://github.com/RAprogramm/masterror/pull/333)
- **deps:** Bump trybuild from 1.0.112 to 1.0.114 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#332](https://github.com/RAprogramm/masterror/pull/332)
- **deps:** Bump http from 1.3.1 to 1.4.0 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#330](https://github.com/RAprogramm/masterror/pull/330)
- **deps:** Bump telegram-webapp-sdk from 0.2.15 to 0.3.0 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#331](https://github.com/RAprogramm/masterror/pull/331)
- **deps:** Bump config from 0.15.18 to 0.15.19 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#334](https://github.com/RAprogramm/masterror/pull/334)

### Fixed

- Resolve broken intra-doc link in builder.rs by [@RAprogramm](https://github.com/RAprogramm) ([d1d2e18](https://github.com/RAprogramm/masterror/commit/d1d2e180b5c4e8c0e6ff8010189fcf4535b764b2))
- Make auto-release workflow idempotent with version existence check by [@RAprogramm](https://github.com/RAprogramm) ([3e4a351](https://github.com/RAprogramm/masterror/commit/3e4a3511aff2604e3f0f1fb861ac6071cfcc88bd))
- Add rebase before push in CHANGELOG workflow to prevent race conditions by [@RAprogramm](https://github.com/RAprogramm) ([1e5db0d](https://github.com/RAprogramm/masterror/commit/1e5db0d24c9f7f19819159dd0bac57108537b4d1))

### Miscellaneous

- Ignore RUSTSEC-2025-0120 (json5 unmaintained) by [@RAprogramm](https://github.com/RAprogramm) ([3d2691a](https://github.com/RAprogramm/masterror/commit/3d2691a5a649c731a2c28b6a5229b3d3c12c1c01))

**Full Changelog**: [v0.25.1...v0.26.0](https://github.com/RAprogramm/masterror/compare/v0.25.1...v0.26.0)
## [0.25.1](https://github.com/RAprogramm/masterror/releases/tag/v0.25.1) - 2025-10-29

### Fixed

- Regenerate CHANGELOG.md with full release history [skip ci] by [@RAprogramm](https://github.com/RAprogramm) ([b6b9bb9](https://github.com/RAprogramm/masterror/commit/b6b9bb93117e296da14a8d0c85fe4fa7545b7050))
- Rebase before push in Auto Release workflow by [@RAprogramm](https://github.com/RAprogramm) ([8249bdb](https://github.com/RAprogramm/masterror/commit/8249bdbe9d31a1fb22b5e74f24946d4b6aa03a62))

**Full Changelog**: [v0.25.0...v0.25.1](https://github.com/RAprogramm/masterror/compare/v0.25.0...v0.25.1)
## [0.25.0](https://github.com/RAprogramm/masterror/releases/tag/v0.25.0) - 2025-10-29

### Added

- Add license symlink by [@RAprogramm](https://github.com/RAprogramm) ([bca4860](https://github.com/RAprogramm/masterror/commit/bca4860563d0f0cc56dcde5cc09ab97691bf4c30))
- Make Auto Release workflow idempotent by [@RAprogramm](https://github.com/RAprogramm) ([7e97b5a](https://github.com/RAprogramm/masterror/commit/7e97b5ada0ab1187c33b3dab98433fd4b7641304))
- Enforce dependency publish order in Auto Release by [@RAprogramm](https://github.com/RAprogramm) ([39b7ecf](https://github.com/RAprogramm/masterror/commit/39b7ecf5bd3ee19abd59867070da9ad62588ecce))
- Integrate crates.io publishing into Auto Release by [@RAprogramm](https://github.com/RAprogramm) ([409ad72](https://github.com/RAprogramm/masterror/commit/409ad721b0416ac409a6c75bfbc0273fea640fb4))

### Dependencies

- **deps:** Bump reqwest from 0.12.23 to 0.12.24 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#203](https://github.com/RAprogramm/masterror/pull/203)
- **deps:** Bump toml from 0.9.7 to 0.9.8 by [@dependabot[bot]](https://github.com/dependabot[bot]) in [#204](https://github.com/RAprogramm/masterror/pull/204)

### Fixed

- Use GH_TOKEN for protected branch push in changelog workflows by [@RAprogramm](https://github.com/RAprogramm) ([c05b63a](https://github.com/RAprogramm/masterror/commit/c05b63af69cf57d6a50f06655d90986eac3dbd54))
- Remove pip cache requirement from translation workflow by [@RAprogramm](https://github.com/RAprogramm) ([4a8acf4](https://github.com/RAprogramm/masterror/commit/4a8acf4bf22d56517d09c97c3c471968f5eae83e))
- Match infra repo codecov configuration exactly by [@RAprogramm](https://github.com/RAprogramm) ([3833d64](https://github.com/RAprogramm/masterror/commit/3833d64e82f643d74149a0600061cc049fef2626))
- Update Codecov badge URLs to new format by [@RAprogramm](https://github.com/RAprogramm) ([8de2b0c](https://github.com/RAprogramm/masterror/commit/8de2b0c2fdcf0102c81b4878cd5ea316c4923370))
- Move codecov.yml to correct location by [@RAprogramm](https://github.com/RAprogramm) ([7e72c37](https://github.com/RAprogramm/masterror/commit/7e72c377f35a8b69330c63e2776045c8cc48a9d0))
- Check crates.io version before publishing each package by [@RAprogramm](https://github.com/RAprogramm) ([2dce8c2](https://github.com/RAprogramm/masterror/commit/2dce8c28bb5113f3390ffecffc313c8a39450186))
- Grant write permissions to reusable CI in Release workflow by [@RAprogramm](https://github.com/RAprogramm) ([43f831a](https://github.com/RAprogramm/masterror/commit/43f831a55d5322f4caa673e0d56befe8ba2ecb80))
- Remove emojis from Auto Release workflow by [@RAprogramm](https://github.com/RAprogramm) ([1cbc7aa](https://github.com/RAprogramm/masterror/commit/1cbc7aa8cec829776351fa5f9c51108177596022))

### Miscellaneous

- Bump version to 0.25.0 and remove Apache-2.0 license by [@RAprogramm](https://github.com/RAprogramm) ([8afac0d](https://github.com/RAprogramm/masterror/commit/8afac0d97baef67c63af74985fc3505bd9c28753))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) in [#262](https://github.com/RAprogramm/masterror/pull/262)
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([4f79c12](https://github.com/RAprogramm/masterror/commit/4f79c127586d0d5929f0020a0e5f1353e3c7ec0c))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([101cbfb](https://github.com/RAprogramm/masterror/commit/101cbfba76477f2836c4c5c6bdc86b184aa576ae))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([9488941](https://github.com/RAprogramm/masterror/commit/9488941f7eff7f217c90feb1d306b2222305b817))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([602ba10](https://github.com/RAprogramm/masterror/commit/602ba10e78d09fbf4913790e964fb2d6cba1f983))

### Testing

- Trigger AI translation workflow by [@RAprogramm](https://github.com/RAprogramm) ([76beac6](https://github.com/RAprogramm/masterror/commit/76beac6b18c552589da3ac1ea2142b61dd037664))
- Test path 2 by [@RAprogramm](https://github.com/RAprogramm) ([1d0410b](https://github.com/RAprogramm/masterror/commit/1d0410b930d9c8356e622f0171918ca482394bd3))
- Test path by [@RAprogramm](https://github.com/RAprogramm) ([299f080](https://github.com/RAprogramm/masterror/commit/299f080d2ef7a892e1750a606456c7dd878a8818))

**Full Changelog**: [v0.24.19...v0.25.0](https://github.com/RAprogramm/masterror/compare/v0.24.19...v0.25.0)
## [0.24.19](https://github.com/RAprogramm/masterror/releases/tag/v0.24.19) - 2025-10-12

### Added

- Add Codecov Test Analytics with organized structure by [@RAprogramm](https://github.com/RAprogramm) ([824f9ad](https://github.com/RAprogramm/masterror/commit/824f9adb0dd03eb82703f70680202e3ce7b8e946))
- Add explicit permissions to workflow jobs by [@RAprogramm](https://github.com/RAprogramm) ([aa57b4d](https://github.com/RAprogramm/masterror/commit/aa57b4db21037a80763761393b0605fbbf010cb1))

### CI/CD

- Trigger CI run by [@RAprogramm](https://github.com/RAprogramm) ([ad7c2e9](https://github.com/RAprogramm/masterror/commit/ad7c2e9e57f638718910619ed4e242dc43e0d135))
- Upgrade codecov action to v5 by [@RAprogramm](https://github.com/RAprogramm) ([aa0c748](https://github.com/RAprogramm/masterror/commit/aa0c7487fcb2f58612b31c966551f979de901943))
- Upgrade codecov action to v5 by [@RAprogramm](https://github.com/RAprogramm) ([e7673e1](https://github.com/RAprogramm/masterror/commit/e7673e19b1aa8e98e5f0a70e4c62c1d928784059))

### Documentation

- Add Codecov badge and coverage visualizations by [@RAprogramm](https://github.com/RAprogramm) ([930c3c2](https://github.com/RAprogramm/masterror/commit/930c3c21d55339b52f3c917b44dfe5594ba1d9bd))
- Add Codecov badge and coverage visualizations by [@RAprogramm](https://github.com/RAprogramm) ([dcce5b0](https://github.com/RAprogramm/masterror/commit/dcce5b0b7aa1323b19b86b50161c68c8146af3ac))

### Fixed

- Use num-cpus for nextest test-threads by [@RAprogramm](https://github.com/RAprogramm) ([90cf239](https://github.com/RAprogramm/masterror/commit/90cf239e2d44653a3c8dbcaf53de6710cc53dcf1))
- Use github.event.inputs consistently by [@RAprogramm](https://github.com/RAprogramm) ([ca13784](https://github.com/RAprogramm/masterror/commit/ca13784f920a94d049af1a48609749cd9c7199dd))
- Use correct inputs reference in checkout by [@RAprogramm](https://github.com/RAprogramm) ([e0b8397](https://github.com/RAprogramm/masterror/commit/e0b839701e2680243acdc0a751c1774533284f37))
- Professional Release workflow with robust event handling by [@RAprogramm](https://github.com/RAprogramm) ([af05ae7](https://github.com/RAprogramm/masterror/commit/af05ae72283ab95c93cd08bf7177e12009f12183))
- Add permissions to checks job in Release workflow by [@RAprogramm](https://github.com/RAprogramm) ([5d55036](https://github.com/RAprogramm/masterror/commit/5d5503680f63019d683960be9b2b1418b7b092cf))
- Release workflow triggers on GitHub release creation by [@RAprogramm](https://github.com/RAprogramm) ([a8f37a7](https://github.com/RAprogramm/masterror/commit/a8f37a745264d1006512228068c7d6be314d277c))
- Auto Release now tracks masterror package version by [@RAprogramm](https://github.com/RAprogramm) ([a566039](https://github.com/RAprogramm/masterror/commit/a5660399467422449530bb1ff0fae2f7f7541115))
- Add id-token permission for Codecov OIDC in CI workflow by [@RAprogramm](https://github.com/RAprogramm) ([00fbc49](https://github.com/RAprogramm/masterror/commit/00fbc49543bb62b30e0592979398d9dc1076c49b))
- Enable OIDC tokenless upload for Codecov v5 by [@RAprogramm](https://github.com/RAprogramm) ([b16b5fb](https://github.com/RAprogramm/masterror/commit/b16b5fbb93637d07d417a56df56045325054d466))

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([9bfadff](https://github.com/RAprogramm/masterror/commit/9bfadfffeee3741cd68c95edfb180c51f63a52b4))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([b9f486f](https://github.com/RAprogramm/masterror/commit/b9f486faf6de4615e07d915f9c84644030de503f))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([082c89e](https://github.com/RAprogramm/masterror/commit/082c89e004eb06aa29dcab29deef4af9256f7628))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([2c1766a](https://github.com/RAprogramm/masterror/commit/2c1766a5c18e6242aacf08adb173dea7e80d1e53))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([86cbf7b](https://github.com/RAprogramm/masterror/commit/86cbf7b8b3eeadb30b25404603c312d76c5467dc))

**Full Changelog**: [v0.24.18...v0.24.19](https://github.com/RAprogramm/masterror/compare/v0.24.18...v0.24.19)
## [0.24.18](https://github.com/RAprogramm/masterror/releases/tag/v0.24.18) - 2025-10-09

### Dependencies

- **deps:** Bump telegram-webapp-sdk from 0.2.14 to 0.2.15 by [@dependabot[bot]](https://github.com/dependabot[bot]) ([eb828b8](https://github.com/RAprogramm/masterror/commit/eb828b85924f190872f3859d062251f4d4501a67))

**Full Changelog**: [v0.24.16...v0.24.18](https://github.com/RAprogramm/masterror/compare/v0.24.16...v0.24.18)
## [0.24.16](https://github.com/RAprogramm/masterror/releases/tag/v0.24.16) - 2025-10-05

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([34e15d2](https://github.com/RAprogramm/masterror/commit/34e15d2994a4796ff48dbd6256f751fb7f9e670d))

**Full Changelog**: [v0.24.17...v0.24.16](https://github.com/RAprogramm/masterror/compare/v0.24.17...v0.24.16)
## [0.24.17](https://github.com/RAprogramm/masterror/releases/tag/v0.24.17) - 2025-10-05

### Added

- Add redaction example by [@RAprogramm](https://github.com/RAprogramm) ([fbe0143](https://github.com/RAprogramm/masterror/commit/fbe01438ba35821d5fb70dd4870c9286343eb25b))
- Add simple .context() method for anyhow parity by [@RAprogramm](https://github.com/RAprogramm) ([6d2f7c0](https://github.com/RAprogramm/masterror/commit/6d2f7c0a4ffb47b30fbc837ec93fe5e2cb1cda37))
- Add simple .context() method for anyhow parity by [@RAprogramm](https://github.com/RAprogramm) ([cdce7cc](https://github.com/RAprogramm/masterror/commit/cdce7cc0c90231dd699287411064c1c522445495))
- Add downcast API for anyhow parity by [@RAprogramm](https://github.com/RAprogramm) ([c00b24a](https://github.com/RAprogramm/masterror/commit/c00b24a9d24e968e7f4c9dca9a284a952fa28511))
- Add structured_metadata example by [@RAprogramm](https://github.com/RAprogramm) ([b1bfe95](https://github.com/RAprogramm/masterror/commit/b1bfe95e0f1698a06d6343fdf45acfbdababd076))
- Add derive_error example by [@RAprogramm](https://github.com/RAprogramm) ([2c86671](https://github.com/RAprogramm/masterror/commit/2c86671f0d6335995347982c12a3bb4eb8679d0d))
- Add basic_usage example by [@RAprogramm](https://github.com/RAprogramm) ([48a498a](https://github.com/RAprogramm/masterror/commit/48a498a1dd0b9667493b1f8d0e8c41972e07a105))
- Add comparative benchmarks vs thiserror/anyhow by [@RAprogramm](https://github.com/RAprogramm) ([01d380d](https://github.com/RAprogramm/masterror/commit/01d380d0e2ccae99bb32854b7249fe55e5a4283e))
- Add anyhow-compatible error chain API by [@RAprogramm](https://github.com/RAprogramm) ([9160545](https://github.com/RAprogramm/masterror/commit/9160545a6d13d618a9f235096ca42a0434e3007a))
- Add reuse by [@RAprogramm](https://github.com/RAprogramm) ([ca59cd0](https://github.com/RAprogramm/masterror/commit/ca59cd0275444c6deb4a2db327006ad8fcaa51a8))
- Add env to dependbot by [@RAprogramm](https://github.com/RAprogramm) ([ace0552](https://github.com/RAprogramm/masterror/commit/ace05521af91fae1006f2758cb4af274d7e18cf0))
- Add reuse by [@RAprogramm](https://github.com/RAprogramm) ([d753cec](https://github.com/RAprogramm/masterror/commit/d753cecce78194f197048b0040e5b54bad049b72))
- Rultor by [@RAprogramm](https://github.com/RAprogramm) ([b0843ad](https://github.com/RAprogramm/masterror/commit/b0843adc3b8964a3829d9a4cc658ab0c7eea94b1))

### Documentation

- Update WHY_MIGRATE.md with new anyhow parity features by [@RAprogramm](https://github.com/RAprogramm) ([4e60933](https://github.com/RAprogramm/masterror/commit/4e609339c0106c554db23ae28de4cf357e4e1bcc))
- Add binary size and compilation time metrics by [@RAprogramm](https://github.com/RAprogramm) ([d47ae2f](https://github.com/RAprogramm/masterror/commit/d47ae2fded565ede9aadffa779f60656e81cbaf5))

### Fixed

- **ci:** Declare benchmarks feature for benches by [@RAprogramm](https://github.com/RAprogramm) ([0efa663](https://github.com/RAprogramm/masterror/commit/0efa6639b86773c8ccf9d905f02baf91c22f6565))

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([422aff8](https://github.com/RAprogramm/masterror/commit/422aff899b2b81f869bb1f5508eb27053654907d))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([1d38508](https://github.com/RAprogramm/masterror/commit/1d38508ab2c64af10277de12af0d7f4e6e222434))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([c532576](https://github.com/RAprogramm/masterror/commit/c532576a7f0145384157d89b08b35544c4b1bb6d))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([277122b](https://github.com/RAprogramm/masterror/commit/277122b7e95796f8e7f02bcfa55c42f6a545dfd2))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([6755136](https://github.com/RAprogramm/masterror/commit/6755136df4aeee2c0270998a8026634955d3baba))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([f31e568](https://github.com/RAprogramm/masterror/commit/f31e56865c1d2375f06e3f5fa22a18cd365aa98a))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([704a1af](https://github.com/RAprogramm/masterror/commit/704a1afe82b2c0305223aad17f5eb62349ee517e))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([8511527](https://github.com/RAprogramm/masterror/commit/8511527cfbc55533d27f64e17b5b0d262b88c144))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([817c85f](https://github.com/RAprogramm/masterror/commit/817c85ffd2fcadf5f33d7d0fb7303940364d1390))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([8c6cded](https://github.com/RAprogramm/masterror/commit/8c6cded0a2dfd45e1cb99ec7234b6ca5de114532))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([c5584d7](https://github.com/RAprogramm/masterror/commit/c5584d79a907fd588755afa56dd6a40b2c94b589))

### Merge

- Resolve README.md conflict from upstream by [@RAprogramm](https://github.com/RAprogramm) ([a3f0150](https://github.com/RAprogramm/masterror/commit/a3f0150978fa09d1394c9bdeb0bb0859321a4691))

**Full Changelog**: [v0.24.12...v0.24.17](https://github.com/RAprogramm/masterror/compare/v0.24.12...v0.24.17)
## [0.24.12](https://github.com/RAprogramm/masterror/releases/tag/v0.24.12) - 2025-09-30

### Fixed

- **ci:** Correct reusable workflow indentation by [@RAprogramm](https://github.com/RAprogramm) ([5475e6a](https://github.com/RAprogramm/masterror/commit/5475e6a45485c4a9c4984d1b1ba4dc7546b2cf92))
- Gate provide shim based on error request support by [@RAprogramm](https://github.com/RAprogramm) ([5a78b69](https://github.com/RAprogramm/masterror/commit/5a78b69e66df644a5cf99bae5782923ced37b644))

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([1e0595a](https://github.com/RAprogramm/masterror/commit/1e0595a0e3cddafe479ffed710dfe9360d8b5107))
- **ci:** Extract cargo steps into composite actions by [@RAprogramm](https://github.com/RAprogramm) ([a2c8d2c](https://github.com/RAprogramm/masterror/commit/a2c8d2c02e477d808a67dab40ec0b955ebeaf770))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([c4952f3](https://github.com/RAprogramm/masterror/commit/c4952f321ae10a4fd1b7c5929b4bde3f3148646b))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([732c71c](https://github.com/RAprogramm/masterror/commit/732c71ca48262411c37bb38da9fd31cb4aeed67d))

### Testing

- Test 1 by [@RAprogramm](https://github.com/RAprogramm) ([ce399a4](https://github.com/RAprogramm/masterror/commit/ce399a40cddf275361ec8c52080fe293ffefae33))

**Full Changelog**: [v0.24.10...v0.24.12](https://github.com/RAprogramm/masterror/compare/v0.24.10...v0.24.12)
## [0.24.10](https://github.com/RAprogramm/masterror/releases/tag/v0.24.10) - 2025-09-30

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([dd9f333](https://github.com/RAprogramm/masterror/commit/dd9f333ab943b33a34dea4b36b18dbd0532d5a64))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([3256d96](https://github.com/RAprogramm/masterror/commit/3256d9696e6b4e2717a32212def2b084b91ff64e))
- Prepare 0.24.10 release by [@RAprogramm](https://github.com/RAprogramm) ([951dd89](https://github.com/RAprogramm/masterror/commit/951dd89b587be61e759b5b64c983708072310592))

**Full Changelog**: [v0.24.8...v0.24.10](https://github.com/RAprogramm/masterror/compare/v0.24.8...v0.24.10)
## [0.24.8](https://github.com/RAprogramm/masterror/releases/tag/v0.24.8) - 2025-09-28

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([0d9c1df](https://github.com/RAprogramm/masterror/commit/0d9c1df5e1e07386706a1aef489562f152dd0527))

**Full Changelog**: [v0.24.9...v0.24.8](https://github.com/RAprogramm/masterror/compare/v0.24.9...v0.24.8)
## [0.24.9](https://github.com/RAprogramm/masterror/releases/tag/v0.24.9) - 2025-09-28

### Added

- Restore AppError::with_context helper by [@RAprogramm](https://github.com/RAprogramm) ([1924dc1](https://github.com/RAprogramm/masterror/commit/1924dc1bb7e5868216c59a14213ea9ed501edcdf))

### Miscellaneous

- Fix lint warning by [@RAprogramm](https://github.com/RAprogramm) ([8ba7832](https://github.com/RAprogramm/masterror/commit/8ba7832895240714dc3374a81851517b661deb8a))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([18ccdc8](https://github.com/RAprogramm/masterror/commit/18ccdc85d49f1f2ee71fc19ff6e6b3c82cb647df))

**Full Changelog**: [v0.21.1...v0.24.9](https://github.com/RAprogramm/masterror/compare/v0.21.1...v0.24.9)
## [0.21.1](https://github.com/RAprogramm/masterror/releases/tag/v0.21.1) - 2025-09-24

**Full Changelog**: [v0.21.0...v0.21.1](https://github.com/RAprogramm/masterror/compare/v0.21.0...v0.21.1)
## [0.21.0](https://github.com/RAprogramm/masterror/releases/tag/v0.21.0) - 2025-09-24

### Added

- 2 tips by [@RAprogramm](https://github.com/RAprogramm) ([a1d2edf](https://github.com/RAprogramm/masterror/commit/a1d2edf5bafa2a527d4d609c0df9f7a4b38f49b8))
- Store shared sources and lazy backtraces by [@RAprogramm](https://github.com/RAprogramm) ([1f162ca](https://github.com/RAprogramm/masterror/commit/1f162ca2fd5b40366535758726652da4e819963f))
- Add metadata container and richer app error by [@RAprogramm](https://github.com/RAprogramm) ([19dbce0](https://github.com/RAprogramm/masterror/commit/19dbce016abee3f4e49b8ed4059923e1ea0e3332))

### Documentation

- Rewrite README for 0.20 workspace by [@RAprogramm](https://github.com/RAprogramm) ([87984f7](https://github.com/RAprogramm/masterror/commit/87984f76530163d2746cdadb2b4c110b79c4527d))
- Refresh readmes for expanded scope by [@RAprogramm](https://github.com/RAprogramm) ([a351295](https://github.com/RAprogramm/masterror/commit/a3512958f4209414bcd09069a1f8bc58960e125a))
- Add error-handling wiki by [@RAprogramm](https://github.com/RAprogramm) ([4f539c9](https://github.com/RAprogramm/masterror/commit/4f539c97c830639a73f3bf41a3cfeeb6535860b8))

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([96b242b](https://github.com/RAprogramm/masterror/commit/96b242bc4f17ff3cb8f115e814b54db7d1303402))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([9bdfcc1](https://github.com/RAprogramm/masterror/commit/9bdfcc1e786d028fb766309a84daadda10526b5b))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([08e5c02](https://github.com/RAprogramm/masterror/commit/08e5c02aa6c7cc91febbf5397cf9271f5757783b))

### Refactored

- Enrich converter errors with context metadata by [@RAprogramm](https://github.com/RAprogramm) ([37b06fc](https://github.com/RAprogramm/masterror/commit/37b06fc69fa4257c8027d5f9e0ba6d4b1987fd93))

**Full Changelog**: [v0.10.9...v0.21.0](https://github.com/RAprogramm/masterror/compare/v0.10.9...v0.21.0)
## [0.10.9](https://github.com/RAprogramm/masterror/releases/tag/v0.10.9) - 2025-09-21

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([11b6b09](https://github.com/RAprogramm/masterror/commit/11b6b0913ae94ff81ea07c1c01c628dc4777282b))

**Full Changelog**: [v0.11.0...v0.10.9](https://github.com/RAprogramm/masterror/compare/v0.11.0...v0.10.9)
## [0.11.0](https://github.com/RAprogramm/masterror/releases/tag/v0.11.0) - 2025-09-21

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([da2d5c5](https://github.com/RAprogramm/masterror/commit/da2d5c5b31bec5cb2e57b554a523d39f94e88ea9))

### Refactored

- Improve database constructor ergonomics by [@RAprogramm](https://github.com/RAprogramm) ([78afc05](https://github.com/RAprogramm/masterror/commit/78afc050755079203b717d978c2ac836401a08bc))

**Full Changelog**: [v0.10.8...v0.11.0](https://github.com/RAprogramm/masterror/compare/v0.10.8...v0.11.0)
## [0.10.8](https://github.com/RAprogramm/masterror/releases/tag/v0.10.8) - 2025-09-21

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([bccb7df](https://github.com/RAprogramm/masterror/commit/bccb7dfbf97eb4d56b9233057945238673942198))

**Full Changelog**: [v0.10.7...v0.10.8](https://github.com/RAprogramm/masterror/compare/v0.10.7...v0.10.8)
## [0.10.7](https://github.com/RAprogramm/masterror/releases/tag/v0.10.7) - 2025-09-20

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([882912c](https://github.com/RAprogramm/masterror/commit/882912c4078ed15306b69946361dee724a2ee33a))

**Full Changelog**: [v0.10.6...v0.10.7](https://github.com/RAprogramm/masterror/compare/v0.10.6...v0.10.7)
## [0.10.6](https://github.com/RAprogramm/masterror/releases/tag/v0.10.6) - 2025-09-20

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([13329b7](https://github.com/RAprogramm/masterror/commit/13329b768dd289d909e5ef69a9c4f303d23a4108))

### Testing

- Test 2 by [@RAprogramm](https://github.com/RAprogramm) ([61e2844](https://github.com/RAprogramm/masterror/commit/61e2844d56b4d701610dffb53e6303734f1ca45d))

**Full Changelog**: [v0.10.5...v0.10.6](https://github.com/RAprogramm/masterror/compare/v0.10.5...v0.10.6)
## [0.10.5](https://github.com/RAprogramm/masterror/releases/tag/v0.10.5) - 2025-09-20

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([c209547](https://github.com/RAprogramm/masterror/commit/c209547a6eb658db2d90d7d82866430f03ac67b6))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([d5495a4](https://github.com/RAprogramm/masterror/commit/d5495a462f35b853b6e4aded1e0efba0483a3ab0))

### Testing

- Test 1 by [@RAprogramm](https://github.com/RAprogramm) ([1ddc6d0](https://github.com/RAprogramm/masterror/commit/1ddc6d0742f152de531f66657af6d0b079427f6e))

**Full Changelog**: [v0.10.4...v0.10.5](https://github.com/RAprogramm/masterror/compare/v0.10.4...v0.10.5)
## [0.10.4](https://github.com/RAprogramm/masterror/releases/tag/v0.10.4) - 2025-09-20

### Fixed

- Release action by [@RAprogramm](https://github.com/RAprogramm) ([4bd1bb0](https://github.com/RAprogramm/masterror/commit/4bd1bb052217a5bd6dd93bd95f968cf49428bf20))
- Release action by [@RAprogramm](https://github.com/RAprogramm) ([ff4720c](https://github.com/RAprogramm/masterror/commit/ff4720c81a376f9e4f990c6c568edde48607501f))

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([8eda8e1](https://github.com/RAprogramm/masterror/commit/8eda8e1c6aba61f314e19817ef28bffa463d5bc4))

**Full Changelog**: [v0.9.0...v0.10.4](https://github.com/RAprogramm/masterror/compare/v0.9.0...v0.10.4)
## [0.9.0](https://github.com/RAprogramm/masterror/releases/tag/v0.9.0) - 2025-09-20

### Added

- Expose telemetry via provide attribute by [@RAprogramm](https://github.com/RAprogramm) ([f4f2066](https://github.com/RAprogramm/masterror/commit/f4f206692bebaac5b46ed54a64997fb1f7e18395))
- Idea.md by [@RAprogramm](https://github.com/RAprogramm) ([35eec48](https://github.com/RAprogramm/masterror/commit/35eec4875617939775edc0a4ae85a56cbafb43a6))
- Target.md by [@RAprogramm](https://github.com/RAprogramm) ([996ca62](https://github.com/RAprogramm/masterror/commit/996ca62d0fa1095ea7d24743d167dafbc3eccf6f))
- Derive AppError conversions by [@RAprogramm](https://github.com/RAprogramm) ([ff3e9cd](https://github.com/RAprogramm/masterror/commit/ff3e9cd0f9246455a35bc4eaa0612397a9904f76))
- Target.md by [@RAprogramm](https://github.com/RAprogramm) ([e93a883](https://github.com/RAprogramm/masterror/commit/e93a8832ef87d65438a6d4b83557a0f0710c9cce))
- **template:** Support implicit placeholders by [@RAprogramm](https://github.com/RAprogramm) ([6b2e174](https://github.com/RAprogramm/masterror/commit/6b2e1745ee6a8d93dd310f449ec7ebfccd07dbc5))
- Rust version by [@RAprogramm](https://github.com/RAprogramm) ([f727655](https://github.com/RAprogramm/masterror/commit/f7276555f221553174fa3d2cc1e5ce0ad8763b9a))

### Fixed

- **ci:** Allow cargo package dry run by [@RAprogramm](https://github.com/RAprogramm) ([7297536](https://github.com/RAprogramm/masterror/commit/7297536b6abd7cf4f016658f86f818468fcb139b))
- Manifest by [@RAprogramm](https://github.com/RAprogramm) ([4216f11](https://github.com/RAprogramm/masterror/commit/4216f1178679c7355a8aebb196acabdb31099cc9))

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([9e727d1](https://github.com/RAprogramm/masterror/commit/9e727d141716a6b57c1e5b98175635a37250227e))
- **ci:** Add cargo deny checks by [@RAprogramm](https://github.com/RAprogramm) ([3d6df54](https://github.com/RAprogramm/masterror/commit/3d6df5425781834feae9d9fd63313e11cd533ac1))
- Harden sqlx integration and add audit checks by [@RAprogramm](https://github.com/RAprogramm) ([b9cbf7f](https://github.com/RAprogramm/masterror/commit/b9cbf7ffa383578e055a6c6318a81633aa23d3e9))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([fb4ec5d](https://github.com/RAprogramm/masterror/commit/fb4ec5d7cd06a01b6f8922e39eee49a6c538e05e))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([579c097](https://github.com/RAprogramm/masterror/commit/579c097ed77ede4bcc5c6e52cc6d35dbd6c4c276))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([824654c](https://github.com/RAprogramm/masterror/commit/824654c03417b1833ea55d740bb6114fd6f6e690))

**Full Changelog**: [v0.5.0...v0.9.0](https://github.com/RAprogramm/masterror/compare/v0.5.0...v0.9.0)
## [0.5.0](https://github.com/RAprogramm/masterror/releases/tag/v0.5.0) - 2025-09-18

### Added

- **derive:** Add transparent error support by [@RAprogramm](https://github.com/RAprogramm) ([ae6476a](https://github.com/RAprogramm/masterror/commit/ae6476afd0f6c68052cbd1942d40355324412f03))
- **derive:** Support #[from] conversions by [@RAprogramm](https://github.com/RAprogramm) ([9b5d351](https://github.com/RAprogramm/masterror/commit/9b5d35140d5cc7263706167790babfdc1a060712))

### Build

- Align readme generator toml version by [@RAprogramm](https://github.com/RAprogramm) ([22c8a76](https://github.com/RAprogramm/masterror/commit/22c8a76571325282e7c2dda67f991c739be40cc4))

### Documentation

- Prepare 0.5.0 release notes by [@RAprogramm](https://github.com/RAprogramm) ([2777baf](https://github.com/RAprogramm/masterror/commit/2777baf972fdd28506ddca6d8b3f22e377041735))
- Emphasize readme sync requirement by [@RAprogramm](https://github.com/RAprogramm) ([1d25200](https://github.com/RAprogramm/masterror/commit/1d252005248faca89586b28ad81dbf584423a8ad))
- Capture post-0.4 updates by [@RAprogramm](https://github.com/RAprogramm) ([899b32c](https://github.com/RAprogramm/masterror/commit/899b32c9755f305afe3ead446d33132c75ed831e))
- Regenerate readme comment by [@RAprogramm](https://github.com/RAprogramm) ([ec58e9b](https://github.com/RAprogramm/masterror/commit/ec58e9ba751d5d6f7e5a41feb46ed55510470452))
- Add release checklist to readme by [@RAprogramm](https://github.com/RAprogramm) ([097f197](https://github.com/RAprogramm/masterror/commit/097f1971cf583cccdfe19dd9d79bb1d3209146de))

### Fixed

- **ci:** Grant permissions to reusable workflow by [@RAprogramm](https://github.com/RAprogramm) ([56aa81a](https://github.com/RAprogramm/masterror/commit/56aa81a1f31ee35ebf1412c249c06849282c16be))

### Miscellaneous

- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([52f709b](https://github.com/RAprogramm/masterror/commit/52f709be605bd73d1943a3213aa2a0ebc3429ffd))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([df8029e](https://github.com/RAprogramm/masterror/commit/df8029ec37451f0092bab2e7406df5e71408787f))
- **readme:** Auto-refresh [skip ci] by [@github-actions[bot]](https://github.com/github-actions[bot]) ([d3666a3](https://github.com/RAprogramm/masterror/commit/d3666a342e47d4879183430642082fc18a48bca1))

### Testing

- Test ci 1 by [@RAprogramm](https://github.com/RAprogramm) ([501eafe](https://github.com/RAprogramm/masterror/commit/501eafeb0abe04497dd8b4f37269bb197f3a7ee7))
- Test readme 2 by [@RAprogramm](https://github.com/RAprogramm) ([9b269d9](https://github.com/RAprogramm/masterror/commit/9b269d96b7c5aef44bb56e259b576be9d487677c))
- Enforce AppResult alias usage by [@RAprogramm](https://github.com/RAprogramm) ([595576d](https://github.com/RAprogramm/masterror/commit/595576d9a781fe2f34f778831e12f9830151a6f1))
- Test readme by [@RAprogramm](https://github.com/RAprogramm) ([0ce3f45](https://github.com/RAprogramm/masterror/commit/0ce3f45524537d0bd4caa5c4860a40ae8577a263))

**Full Changelog**: [v0.4.0...v0.5.0](https://github.com/RAprogramm/masterror/compare/v0.4.0...v0.5.0)
## [0.4.0](https://github.com/RAprogramm/masterror/releases/tag/v0.4.0) - 2025-09-16

### Added

- Add frontend console logging support by [@RAprogramm](https://github.com/RAprogramm) ([e0b932e](https://github.com/RAprogramm/masterror/commit/e0b932e830ac716418df0870bf1a8c72c308922b))

**Full Changelog**: [v0.3.5...v0.4.0](https://github.com/RAprogramm/masterror/compare/v0.3.5...v0.4.0)
## [0.3.5](https://github.com/RAprogramm/masterror/releases/tag/v0.3.5) - 2025-09-12

### Added

- Add teloxide error mapping by [@RAprogramm](https://github.com/RAprogramm) ([40d05cb](https://github.com/RAprogramm/masterror/commit/40d05cb2412205968df23af334a20506cd2fe861))
- Convert telegram init validation errors by [@RAprogramm](https://github.com/RAprogramm) ([b039563](https://github.com/RAprogramm/masterror/commit/b0395639e9e7ddf315423a9d23927c6a3c1ae55c))
- **telegram:** Expand validation error coverage by [@RAprogramm](https://github.com/RAprogramm) ([451f423](https://github.com/RAprogramm/masterror/commit/451f4236990026db22775402a098d0ebea92d1f0))

**Full Changelog**: [v0.3.3...v0.3.5](https://github.com/RAprogramm/masterror/compare/v0.3.3...v0.3.5)
## [0.3.3](https://github.com/RAprogramm/masterror/releases/tag/v0.3.3) - 2025-09-11

### Added

- Log error code by [@RAprogramm](https://github.com/RAprogramm) ([0f612db](https://github.com/RAprogramm/masterror/commit/0f612dbad755f07d65df56cd09a03b3cd4ca0fa6))
- Add duration-based retry helper by [@RAprogramm](https://github.com/RAprogramm) ([db9059e](https://github.com/RAprogramm/masterror/commit/db9059e0819c9477f561424fb7f22a2b35219d76))
- Add generic details helper by [@RAprogramm](https://github.com/RAprogramm) ([071ed77](https://github.com/RAprogramm/masterror/commit/071ed770f4836567892f338a369b04a628c39587))
- Classify serde_json errors by [@RAprogramm](https://github.com/RAprogramm) ([e0ddfe5](https://github.com/RAprogramm/masterror/commit/e0ddfe573c2df047e1c4e3c4d609e579cbb67423))

### Documentation

- Sync changelog with recent mappings by [@RAprogramm](https://github.com/RAprogramm) ([6ee010c](https://github.com/RAprogramm/masterror/commit/6ee010cb8793bd605f316700e1388edd61067898))
- Explain config error mapping by [@RAprogramm](https://github.com/RAprogramm) ([41cc79d](https://github.com/RAprogramm/masterror/commit/41cc79db2ac330deb1204c27e873b08294ea8a3d))
- Record recent changes by [@RAprogramm](https://github.com/RAprogramm) ([9a33acb](https://github.com/RAprogramm/masterror/commit/9a33acb55f876195826c8301121600aa84a5a181))
- Add 0.3.3 release notes by [@RAprogramm](https://github.com/RAprogramm) ([3de88fd](https://github.com/RAprogramm/masterror/commit/3de88fd09146c704ae50b62a9327d5d401ca7918))
- Clarify case-insensitive helpers by [@RAprogramm](https://github.com/RAprogramm) ([ae8751c](https://github.com/RAprogramm/masterror/commit/ae8751c1dea4cdb83f99c68f2e309b889689a44a))

### Refactored

- Streamline status code conversion by [@RAprogramm](https://github.com/RAprogramm) ([8938d54](https://github.com/RAprogramm/masterror/commit/8938d543c1fc232b3b9100bccc4f9f3cdd068874))

### Testing

- **actix:** Validate headers for AppError by [@RAprogramm](https://github.com/RAprogramm) ([5b32252](https://github.com/RAprogramm/masterror/commit/5b3225263650126bb1b9ef30183af79fcc04491c))
- Check service fallback for unknown turnkey message by [@RAprogramm](https://github.com/RAprogramm) ([91fec65](https://github.com/RAprogramm/masterror/commit/91fec659d4c015a30b51d613cb266cdf9f4d59a9))
- Add integration conversion tests by [@RAprogramm](https://github.com/RAprogramm) ([087d6d3](https://github.com/RAprogramm/masterror/commit/087d6d3ff07bde42cd78ebf9ba51f55ef672da9c))

**Full Changelog**: [v0.3.2...v0.3.3](https://github.com/RAprogramm/masterror/compare/v0.3.2...v0.3.3)
## [0.3.2](https://github.com/RAprogramm/masterror/releases/tag/v0.3.2) - 2025-09-08

### Added

- Turnkey by [@RAprogramm](https://github.com/RAprogramm) ([10acc80](https://github.com/RAprogramm/masterror/commit/10acc80847a83dd51482243870e55054078d15e2))
- Turnkey by [@RAprogramm](https://github.com/RAprogramm) ([4fac768](https://github.com/RAprogramm/masterror/commit/4fac7685102516e7ef5888fb6c4dc75c22a263fc))

**Full Changelog**: [v0.3.1...v0.3.2](https://github.com/RAprogramm/masterror/compare/v0.3.1...v0.3.2)
## [0.3.1](https://github.com/RAprogramm/masterror/releases/tag/v0.3.1) - 2025-08-25

### Added

- Release script by [@RAprogramm](https://github.com/RAprogramm) ([86c10d6](https://github.com/RAprogramm/masterror/commit/86c10d62839c3c1dba46e2cae6dec4d7299d7717))

### Documentation

- Update changelog for 0.3.1 (IntoResponse for AppError) by [@RAprogramm](https://github.com/RAprogramm) ([282e716](https://github.com/RAprogramm/masterror/commit/282e7161f8375524a37197b3681ff70e083c722a))
- Update changelog for 0.3.1 (IntoResponse for AppError) by [@RAprogramm](https://github.com/RAprogramm) ([c29da57](https://github.com/RAprogramm/masterror/commit/c29da573c53294f80b19b1f0bbd58901015fba30))

### Masterror

- Add IntoResponse for AppError under axum feature by [@RAprogramm](https://github.com/RAprogramm) ([77dbd82](https://github.com/RAprogramm/masterror/commit/77dbd8244e23d245ab5bba6610249cb8bb8f5396))

**Full Changelog**: [v0.3.0...v0.3.1](https://github.com/RAprogramm/masterror/compare/v0.3.0...v0.3.1)
## [0.3.0](https://github.com/RAprogramm/masterror/releases/tag/v0.3.0) - 2025-08-24

### Added

- Introduce AppCode and new ErrorResponse fields by [@RAprogramm](https://github.com/RAprogramm) ([2c25477](https://github.com/RAprogramm/masterror/commit/2c254777cdd93b18702fd36d0efa4b020e6d32ac))

### Fixed

- Convert test by [@RAprogramm](https://github.com/RAprogramm) ([2af88e9](https://github.com/RAprogramm/masterror/commit/2af88e931f53e48ed080fb839ed57ad9c82709f1))

**Full Changelog**: [v0.2.1...v0.3.0](https://github.com/RAprogramm/masterror/compare/v0.2.1...v0.3.0)
## [0.2.1](https://github.com/RAprogramm/masterror/releases/tag/v0.2.1) - 2025-08-20

**Full Changelog**: [v0.2.0...v0.2.1](https://github.com/RAprogramm/masterror/compare/v0.2.0...v0.2.1)
## [0.2.0](https://github.com/RAprogramm/masterror/releases/tag/v0.2.0) - 2025-08-20

### Added

- **actix:** AppError as ResponseError; ErrorResponse as Responder; docs/readme by [@RAprogramm](https://github.com/RAprogramm) ([96638e3](https://github.com/RAprogramm/masterror/commit/96638e30f7bcf3cd9db5ea338ba7c9af0710be65))
- **actix:** AppError as ResponseError; ErrorResponse as Responder; docs/readme by [@RAprogramm](https://github.com/RAprogramm) ([357862c](https://github.com/RAprogramm/masterror/commit/357862c46edf95ca8b090726b51eff887b77e49f))
- Opencollective link by [@RAprogramm](https://github.com/RAprogramm) ([b1b8be7](https://github.com/RAprogramm/masterror/commit/b1b8be7ee9d66078d2fe5a830f514bcddf5a598f))
- Add help task in makefile by [@RAprogramm](https://github.com/RAprogramm) ([8e09703](https://github.com/RAprogramm/masterror/commit/8e09703896dfc022b9546617e3536703b12fe62d))

**Full Changelog**: [v0.1.1...v0.2.0](https://github.com/RAprogramm/masterror/compare/v0.1.1...v0.2.0)
## [0.1.1](https://github.com/RAprogramm/masterror/releases/tag/v0.1.1) - 2025-08-12

### Added

- Add pre commit by [@RAprogramm](https://github.com/RAprogramm) ([c5ab3c0](https://github.com/RAprogramm/masterror/commit/c5ab3c075438fbc519fa3c8b5759eac759a56c7a))
- Makefile by [@RAprogramm](https://github.com/RAprogramm) ([db84bfd](https://github.com/RAprogramm/masterror/commit/db84bfd906876936bdf02d400301ad8984e45259))
- License by [@RAprogramm](https://github.com/RAprogramm) ([f7da152](https://github.com/RAprogramm/masterror/commit/f7da1525840068df7fe32c400d587cbd2f654649))
- README.md by [@RAprogramm](https://github.com/RAprogramm) ([fb6c8ea](https://github.com/RAprogramm/masterror/commit/fb6c8ea9d1566c688de551dc6b58a211f6ca9f27))

### Fixed

- Funding by [@RAprogramm](https://github.com/RAprogramm) ([c003434](https://github.com/RAprogramm/masterror/commit/c003434400aa05fddb602aaf00cfb5ae227e2513))
- Fix ci 3 by [@RAprogramm](https://github.com/RAprogramm) ([4b4ff00](https://github.com/RAprogramm/masterror/commit/4b4ff00b8fd4de0e7635bb35f92c06a1d148d617))
- Fix ci 2 by [@RAprogramm](https://github.com/RAprogramm) ([d4286fb](https://github.com/RAprogramm/masterror/commit/d4286fb38deb4e191920129a0f2479b0eadac91b))
- Fix ci 1 by [@RAprogramm](https://github.com/RAprogramm) ([10ef875](https://github.com/RAprogramm/masterror/commit/10ef875fbd77f83a42539ffe102a11b238ab1e14))
- Fix pre commit by [@RAprogramm](https://github.com/RAprogramm) ([2b31c15](https://github.com/RAprogramm/masterror/commit/2b31c158c21fe903c34208f7bf048666cb5a9fda))
- Fix pre commit by [@RAprogramm](https://github.com/RAprogramm) ([c23611a](https://github.com/RAprogramm/masterror/commit/c23611aed0c6e18dbc607daa05d80cb5dc2a7e1a))
- Repo link by [@RAprogramm](https://github.com/RAprogramm) ([f1cbd51](https://github.com/RAprogramm/masterror/commit/f1cbd51e037b7a50b92410fe8b22880ade71eae4))
- License name by [@RAprogramm](https://github.com/RAprogramm) ([9671e7a](https://github.com/RAprogramm/masterror/commit/9671e7a6d38570c65a2734ec07fe9427d284974c))
- Fix by [@RAprogramm](https://github.com/RAprogramm) ([716b8a8](https://github.com/RAprogramm/masterror/commit/716b8a8a0c48d855c96ffdb5c7c1d63264406b39))

### Testing

- Auto publish by [@RAprogramm](https://github.com/RAprogramm) ([f55f0e1](https://github.com/RAprogramm/masterror/commit/f55f0e1c59e06238f5fd89660a4251f83c2028b7))

**Full Changelog**: [...v0.1.1](https://github.com/RAprogramm/masterror/compare/...v0.1.1)

