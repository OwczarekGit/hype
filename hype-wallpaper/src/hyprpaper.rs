use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;

pub struct Hyprpaper;

impl Hyprpaper {
    pub fn set_wallpaper(&self, path: &PathBuf) {
        let monitors = Self::get_monitors();

        let _ = std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("unload")
            .arg("all")
            .stdout(Stdio::null())
            .spawn();

        let _ = std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("preload")
            .arg(path)
            .stdout(Stdio::null())
            .spawn();

        monitors.iter().for_each(|mon| {
            let _ = std::process::Command::new("hyprctl")
                .arg("hyprpaper")
                .arg("wallpaper")
                .arg(format!("{mon}, {}", &path.display()))
                .stdout(Stdio::null())
                .spawn();
        });
    }

    fn get_monitors() -> Vec<String> {
        let monitors = String::from_utf8(
            std::process::Command::new("hyprctl")
                .arg("monitors")
                .arg("-j")
                .output()
                .expect("'hyprctl monitors' failed.")
                .stdout,
        )
        .expect("Invalid format.");

        let monitors: Vec<HyprctlMonitor> =
            serde_json::from_str(&monitors).expect("Invalid format.");

        monitors.into_iter().map(|n| n.name).collect::<Vec<_>>()
    }

    pub fn save_wallpaper(&self, path: &Path) {
        let cfg = hyprpaper_config();
        let mut file = std::fs::File::create(cfg).unwrap();
        let mut content = String::new();
        content.push_str(format!("preload = {}\n", path.display()).as_str());

        Self::get_monitors().iter().for_each(|mon| {
            content.push_str(format!("wallpaper = {mon}, {}\n", path.display()).as_str())
        });

        let _ = file.write_all(content.as_bytes());
    }
}

fn hyprpaper_config() -> PathBuf {
    let home = env!("HOME").to_string();
    PathBuf::from(home + "/.config/hypr/hyprpaper.conf")
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyprctlMonitor {
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
