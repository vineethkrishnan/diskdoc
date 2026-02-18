# Changelog

All notable changes to DiskDoctor will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
