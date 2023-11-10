use crate::core::rectangle::Rectangle;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::HyprctlError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: Vec<i64>,
    pub size: Vec<i64>,
    pub workspace: Workspace,
    pub floating: bool,
    pub monitor: i64,
    pub class: String,
    pub title: String,
    pub initial_class: String,
    pub initial_title: String,
    pub pid: i64,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: bool,
    pub fullscreen_mode: i64,
    pub fake_fullscreen: bool,
    pub grouped: Vec<Value>,
    pub swallowing: String,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: i64,
}

impl Client {
    pub fn get_all() -> Result<Vec<Client>, HyprctlError> {
        let out = std::process::Command::new("hyprctl")
            .arg("clients")
            .arg("-j")
            .output()?
            .stdout;

        let out = String::from_utf8(out)?;
        Ok(serde_json::from_str::<'_, Vec<Client>>(&out)?)
    }

    pub fn rect(&self) -> Rectangle {
        self.clone().into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: i64,
    pub name: String,
}
