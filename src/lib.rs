use zed_extension_api as zed;

struct AutoHotkeyDebugger;

impl zed::Extension for AutoHotkeyDebugger {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(AutoHotkeyDebugger);
