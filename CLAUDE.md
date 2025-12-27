# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Check code compiles (run this frequently)
cargo check

# Build the extension WASM
cargo build --release

# The compiled extension.wasm is at target/wasm32-wasip1/release/autohotkey_debugger.wasm
```

## Testing the Extension

No automated tests. Manual testing in Zed:

1. `cmd+shift+p` → "zed: install dev extension"
2. Select the project folder
3. Open an `.ahk` file and use the debugger

## Architecture

This is a **Zed editor debug extension** written in Rust that compiles to WASM. It wraps [helsmy/autohotkey-debug-adapter](https://github.com/helsmy/autohotkey-debug-adapter) to provide DAP integration.

### Communication Model

The debug adapter (`debugAdapter.ahk`) communicates via **stdio** (stdin/stdout), NOT TCP. This is why `DebugAdapterBinary.connection` is set to `None`.

### Extension Flow

```
Zed Editor                    Extension (WASM)                   Debug Adapter
    │                              │                                   │
    ├─ get_dap_binary() ──────────►│                                   │
    │                              ├─ ensure_adapter_installed()       │
    │                              │   (downloads .vsix from GitHub)   │
    │                              ├─ build_binary()                   │
    │◄─ DebugAdapterBinary ────────┤   (returns AHK.exe + script path) │
    │                              │                                   │
    ├─ Launches process ──────────────────────────────────────────────►│
    │   (AutoHotkey.exe debugAdapter.ahk)                              │
    │                                                                  │
    │◄─────────────────── DAP over stdio ─────────────────────────────►│
```

### Key Implementation Details

- **Binary download**: Downloads `.vsix` from GitHub releases, extracts to `autohotkey/autohotkey_{version}/`
- **Version caching**: Uses `OnceLock<String>` to cache version after first fetch
- **Fallback**: If GitHub fetch fails, uses any previously downloaded version

## Key Files

| File | Purpose |
|------|---------|
| `src/lib.rs` | Main extension implementation, implements `zed::Extension` trait |
| `extension.toml` | Zed extension manifest, registers the `autohotkey` debug adapter |
| `debug_adapter_schemas/autohotkey.json` | JSON schema for debug configuration autocomplete |

## Debug Configuration

Users create `.zed/debug.json` in their project:

```json
[
  {
    "label": "Debug Current AHK File",
    "adapter": "autohotkey",
    "request": "launch",
    "program": "$ZED_FILE",
    "stopOnEntry": true
  }
]
```

The `$ZED_FILE` variable resolves to the currently open file.
