use serde::{Deserialize, Serialize};

use super::HyprctlError;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CursorPos {
    pub x: i32,
    pub y: i32,
}

impl CursorPos {
    pub fn current() -> Result<Self, HyprctlError> {
        let out = std::process::Command::new("hyprctl")
            .arg("cursorpos")
            .arg("-j")
            .output()?
            .stdout;

        let out_str = String::from_utf8(out)?;

        serde_json::from_str(&out_str)?
    }
}
