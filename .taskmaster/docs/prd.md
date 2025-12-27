# Overview

A Zed editor debugger extension that enables debugging AutoHotkey v1 scripts. Wraps the [helsmy/autohotkey-debug-adapter](https://github.com/helsmy/autohotkey-debug-adapter) to provide DAP (Debug Adapter Protocol) integration with Zed's built-in debugger.

**Problem:** Zed lacks debugging support for AutoHotkey scripts, limiting its usefulness for AHK developers.

**Solution:** A Rust-based Zed extension that downloads and manages the helsmy debug adapter binary, enabling breakpoints, stepping, and variable inspection for AHK v1 scripts.

**Target users:** AutoHotkey v1 developers using Zed editor on Windows.

# Core Features

## 1. Debug Adapter Integration
- Downloads `debugAdapter.exe` from helsmy's GitHub releases
- Caches the binary locally for fast startup
- Handles version checking and updates

## 2. Launch Configuration
- Supports launching AHK scripts with debugging enabled
- Configurable AHK runtime path (AutoHotkey.exe location)
- Configurable DBGp port (default: 9005)
- Script arguments passthrough

## 3. Debugging Capabilities (provided by helsmy adapter)
- Breakpoints (line, conditional)
- Step over/into/out
- Call stack inspection
- Variable inspection (local/global)
- Watch expressions

# User Experience

## User Persona
Windows developer using AutoHotkey v1 for automation scripts, wanting IDE-level debugging instead of MsgBox debugging.

## Key User Flow
1. Install extension from Zed extension panel
2. Create `debug.json` in project:
   ```json
   {
     "program": "${workspaceFolder}/script.ahk",
     "runtime": "C:\\Program Files\\AutoHotkey\\AutoHotkey.exe"
   }
   ```
3. Set breakpoints in `.ahk` file
4. Press F5 to start debugging
5. Use debug toolbar for stepping/inspection

## UI/UX Considerations
- JSON schema provides autocomplete in `debug.json`
- Clear error messages when AHK runtime not found
- Status indicator during adapter download

# Technical Architecture

## System Components

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   Zed Editor    │────▶│  Extension       │────▶│ debugAdapter.exe│
│   (DAP Client)  │     │  (Rust/WASM)     │     │ (helsmy)        │
└─────────────────┘     └──────────────────┘     └────────┬────────┘
                                                          │ DBGp
                                                          ▼
                                                 ┌─────────────────┐
                                                 │ AutoHotkey.exe  │
                                                 │ (v1 runtime)    │
                                                 └─────────────────┘
```

## Required Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust project config, depends on `zed_extension_api` |
| `extension.toml` | Zed extension manifest, registers debug adapter |
| `src/lib.rs` | Rust implementation of Extension trait |
| `debug_adapter_schemas/autohotkey.json` | JSON schema for debug config |

## Key Rust Implementation

```rust
impl zed::Extension for AutoHotkeyDebugger {
    fn get_dap_binary(...) -> Result<DebugAdapterBinary, String>
    fn dap_request_kind(...) -> Result<DapRequestKind, String>
}
```

## APIs and Integrations
- **zed_extension_api**: Rust crate for Zed extension development
- **GitHub Releases API**: Download debugAdapter.exe from helsmy repo
- **DBGp Protocol**: Debug communication (handled by helsmy adapter)

## Infrastructure Requirements
- Windows only (AHK limitation)
- AutoHotkey v1 installed on user's system
- Internet connection for initial adapter download

# Development Roadmap

## Phase 1: MVP (Core Debugging)
- Initialize repo structure (Cargo.toml, extension.toml)
- Implement `get_dap_binary()` - download adapter from GitHub releases
- Implement `dap_request_kind()` - support launch mode
- Create JSON schema for basic config (program, runtime, port)
- Test locally with dev extension installation
- Document installation and usage in README

## Phase 2: Polish
- Add binary caching (avoid re-download on every session)
- Add version checking for adapter updates
- Improve error messages (AHK not found, port in use, download failed)
- Add attach mode support (debug already-running scripts)

## Phase 3: Publish
- Submit to Zed extension registry
- Add link from tree-sitter-autohotkey README

# Logical Dependency Chain

1. **Repo structure** → Basic Cargo.toml, extension.toml with debug adapter registration
2. **Binary download** → get_dap_binary() fetches from GitHub releases
3. **Launch config** → JSON schema + dap_request_kind() returns Launch
4. **Local testing** → Install as dev extension, debug a simple AHK script
5. **Documentation** → README with prerequisites and setup instructions
6. **Publishing** → Submit to Zed extension registry

# Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| helsmy adapter stops working | Fork if needed, adapter is self-contained binary |
| Zed extension API changes | Pin zed_extension_api version in Cargo.toml |
| GitHub rate limits on download | Cache binary after first download |
| User has wrong AHK version | Clear error message pointing to AHK v1 download |
| DBGp port conflict | Make port configurable, document common conflicts |

# Appendix

## References
- [Zed Debugger Extensions Docs](https://zed.dev/docs/extensions/debugger-extensions)
- [helsmy/autohotkey-debug-adapter](https://github.com/helsmy/autohotkey-debug-adapter)
- [zed_extension_api](https://docs.rs/zed_extension_api)
- [DBGp Protocol Specification](https://xdebug.org/docs/dbgp)
- [AutoHotkey Debug Clients](https://www.autohotkey.com/docs/v1/AHKL_DBGPClients.htm)

## helsmy Adapter Details
- Binary: `debugAdapter.exe` (Windows x64)
- Default port: 9005 (configurable)
- Supports: AHK v1.1.37+, v2.0.11+ (we target v1 only)
- GitHub releases contain pre-compiled binary
