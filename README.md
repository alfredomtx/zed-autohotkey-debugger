# AutoHotkey Debugger

Debug AutoHotkey v1 scripts in Zed using the Debug Adapter Protocol.

## Features

- Set breakpoints (line and conditional)
- Step over, into, and out
- Inspect local and global variables
- View call stack
- Watch expressions

## Requirements

- **Windows** (AutoHotkey is Windows-only)
- **AutoHotkey v1** installed (v1.1.37 or later)

## Installation

1. Open Zed
2. Open the Extensions panel (`Ctrl+Shift+X`)
3. Search for "AutoHotkey Debugger"
4. Click Install

## Usage

**Tip**: Ask AI to do it for you!

Create a `.zed/debug.json` file in your project:

```json
[
  {
    "label": "Debug Current Script",
    "adapter": "autohotkey",
    "request": "launch",
    "program": "$ZED_FILE",
    "stopOnEntry": true
  }
]
```

Then open any `.ahk` file and start debugging with `F5` or via the Debug panel.

### Using a specific script

```json
[
  {
    "label": "Debug Main Script",
    "adapter": "autohotkey",
    "request": "launch",
    "program": "${workspaceFolder}/main.ahk",
    "stopOnEntry": false
  }
]
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `program` | string | *required* | Path to the `.ahk` script to debug |
| `runtime` | string | bundled | Path to `AutoHotkey.exe` (uses bundled runtime by default) |
| `stopOnEntry` | boolean | `true` | Stop at the first line of the script |
| `args` | array | `[]` | Command-line arguments passed to the script |

### Variables

- `$ZED_FILE` - Path to the currently open file
- `${workspaceFolder}` - Path to the project root

## Credits

This extension wraps [helsmy/autohotkey-debug-adapter](https://github.com/helsmy/autohotkey-debug-adapter), which provides the underlying Debug Adapter Protocol implementation.

## License

MIT
