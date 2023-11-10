use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use super::monitors::Monitor;
use std::io::Write;

pub struct Hyprpaper;

impl Hyprpaper {
    pub fn unload_all() {
        let _ = std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("unload")
            .arg("all")
            .stdout(Stdio::null())
            .spawn();
    }

    pub fn preload(path: &Path) {
        let _ = std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("preload")
            .arg(path)
            .stdout(Stdio::null())
            .spawn();
    }

    pub fn wallpaper(monitors: &[Monitor], path: &Path) {
        monitors.iter().for_each(|mon| {
            let _ = std::process::Command::new("hyprctl")
                .arg("hyprpaper")
                .arg("wallpaper")
                .arg(format!("{}, {}", mon.name, &path.display()))
                .stdout(Stdio::null())
                .spawn();
        });
    }
    
    pub fn set_wallpaper(path: &Path) {
        let monitors = Monitor::get_all()
            .expect("The list of monitors.");
        Self::unload_all();
        Self::preload(path);
        Self::wallpaper(&monitors, path);
    }

    pub fn save_wallpaper(path: &Path) {
        // Create config directory.
        let cfg_dir = default_config_directory();
        let _ = std::fs::create_dir_all(cfg_dir);

        // Create config file.
        let cfg = default_config_file();
        let mut file = std::fs::File::create(cfg).expect("The file to be accessible.");

        let mut content = String::new();
        content.push_str(format!("preload = {}\n", path.display()).as_str());

        Monitor::get_all()
            .expect("The list of monitors.")
            .iter()
            .for_each(|mon| {
                content.push_str(format!("wallpaper = {}, {}\n", mon.name, path.display()).as_str())
            });

        let _ = file.write_all(content.as_bytes());
    }
}

pub fn default_config_directory() -> PathBuf {
    let home = env!("HOME").to_string();
    PathBuf::from(home + "/.config/hypr")
}

pub fn default_config_file() -> PathBuf {
    let mut path = default_config_directory();
    path.push("hyprpaper.conf");
    path
}
