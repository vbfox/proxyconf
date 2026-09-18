# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/vbfox/proxyconf/compare/v0.2.0...v0.2.1) - 2019-05-05

### Added

- Display information from environment variables (`HTTP_PROXY`, `HTTPS_PROXY`, `NO_PROXY`)

### Fixed

- Handle errors more nicely, especially when modifying WinHTTP as non-admin
- Report a specific error when the command line is incorrect

### Other

- Move to the `failure` crate for error handling
- Update to Rust 2018 edition

## [0.2.0](https://github.com/vbfox/proxyconf/releases/tag/v0.2.0) - 2018-07-08

First version doing something useful enough to be released.

### Added

- Handle both serialization forms of the modern Internet Explorer settings at the same time
- Handle the `WOW6432Node` registry key in modern mode
- Make writing to the modern registry keys configurable

### Changed

- Treat the Internet Explorer configuration as a single entity in commands
- Simpler command line interface
- Better `show` output

### Fixed

- Don't try to write to `WOW6432Node` for the current user

## [0.1.2](https://crates.io/crates/proxyconf/0.1.2) - 2018-06-28

### Added

- Read WinHTTP configuration
- WinHTTP command line

### Fixed

- Various WinHTTP bugfixes

### Other

- Update dependencies

## [0.1.1](https://crates.io/crates/proxyconf/0.1.1) - 2018-06-26

### Added

- Start of legacy Internet Explorer settings support
- Module documentation

### Changed

- Move the Internet Explorer configuration to its own module, with modern settings in a separate folder

## [0.1.0](https://crates.io/crates/proxyconf/0.1.0) - 2018-05-28

Initial release.

### Added

- Read and write the modern Internet Explorer proxy settings from the registry
- Command line tool with `show` and `set` subcommands (`no-proxy`, `auto-detect`, `setup-script`, `proxy`)
- Library crate exposing the configuration parsing and serialization
