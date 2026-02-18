# Changelog

All notable changes to DiskDoctor will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/vineethkrishnan/diskdoc/compare/diskdoc-v0.1.0...diskdoc-v0.2.0) (2026-02-18)


### Features

* initialize DiskDoctor with hexagonal architecture and analyzers ([4cf41c8](https://github.com/vineethkrishnan/diskdoc/commit/4cf41c84a42f34bfa7492c56358a3451c59c4f05))


### Bug Fixes

* add release-please config and update to non-deprecated action ([#9](https://github.com/vineethkrishnan/diskdoc/issues/9)) ([970b37b](https://github.com/vineethkrishnan/diskdoc/commit/970b37b4274b94704e589db6019ed74d17f99f76))
* upgrade cargo-deny-action to v2 and update deny.toml config ([#8](https://github.com/vineethkrishnan/diskdoc/issues/8)) ([2c56aac](https://github.com/vineethkrishnan/diskdoc/commit/2c56aacba3d801163cecde6c7be77209ce016374))

## [0.1.0] - 2026-02-18

### Added
- Initial release of DiskDoctor (dru).
- Interactive TUI using Ratatui.
- Hexagonal Architecture implementation.
- Support for Docker, Logs, and Cache analysis.
- Safe cleanup with confirmation workflow.
- Pre-commit hooks for linting and testing.
- Comprehensive Getting Started guide.
- GitHub templates for issues and pull requests.

### Changed
- Refactored core logic into Domain and Application layers.
- Moved UI to Interface layer.
- Improved directory scanning performance with `jwalk`.
