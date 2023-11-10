use serde::{Deserialize, Serialize};

use crate::core::rectangle::Rectangle;

use super::HyprctlError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: i64,
    pub height: i64,
    pub refresh_rate: f64,
    pub x: i64,
    pub y: i64,
    pub active_workspace: ActiveWorkspace,
    pub special_workspace: SpecialWorkspace,
    pub reserved: Vec<i64>,
    pub scale: f64,
    pub transform: i64,
    pub focused: bool,
    pub dpms_status: bool,
    pub vrr: bool,
    pub actively_tearing: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveWorkspace {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialWorkspace {
    pub id: i64,
    pub name: String,
}

impl Monitor {
    pub fn get_all() -> Result<Vec<Monitor>, HyprctlError> {
        let out = std::process::Command::new("hyprctl")
            .arg("monitors")
            .arg("-j")
            .output()?
            .stdout;

        let out = String::from_utf8(out)?;
        Ok(serde_json::from_str::<'_, Vec<Monitor>>(&out)?)
    }

    pub fn rect(&self) -> Rectangle {
        self.clone().into()
    }
}
