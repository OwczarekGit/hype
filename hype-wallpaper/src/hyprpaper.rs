use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;

pub struct Hyprpaper {
    monitors: Vec<String>,
}

impl Default for Hyprpaper {
    fn default() -> Self {
        Self {
            monitors: vec![
                "DP-1".to_string(),
                "DP-2".to_string(),
                "HDMI-A-1".to_string(),
            ],
        }
    }
}

impl Hyprpaper {
    pub fn set_wallpaper(&self, path: &PathBuf) {
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

        self.monitors.iter().for_each(|mon| {
            let _ = std::process::Command::new("hyprctl")
                .arg("hyprpaper")
                .arg("wallpaper")
                .arg(format!("{mon}, {}", &path.display()))
                .stdout(Stdio::null())
                .spawn();
        });
    }

    pub fn save_wallpaper(&self, path: &Path) {
        let cfg = hyprpaper_config();
        let mut file = std::fs::File::create(cfg).unwrap();
        let mut content = String::new();
        content.push_str(format!("preload = {}\n", path.display()).as_str());

        self.monitors.iter().for_each(|mon| {
            content.push_str(format!("wallpaper = {mon}, {}\n", path.display()).as_str())
        });

        let _ = file.write_all(content.as_bytes());
    }
}

fn hyprpaper_config() -> PathBuf {
    let home = env!("HOME").to_string();
    PathBuf::from(home + "/.config/hypr/hyprpaper.conf")
}
