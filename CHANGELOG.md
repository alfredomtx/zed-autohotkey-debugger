# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-26

### Added

- Initial release of the AutoHotkey Debugger extension for Zed
- Debug Adapter Protocol (DAP) integration using [helsmy/autohotkey-debug-adapter](https://github.com/helsmy/autohotkey-debug-adapter)
- Automatic download and caching of debug adapter from GitHub releases
- Support for launch request type with breakpoint debugging
- JSON schema for debug configuration autocomplete in `.zed/debug.json`
- Validation for required files (AutoHotkey.exe, debugAdapter.ahk) before debug sessions
- Stdio-based communication between Zed and the debug adapter
