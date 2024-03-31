use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

use crate::hyprland::hyprctl::monitors::Monitor;

use super::WallpaperDaemon;

pub struct Hyprpaper(pub bool);

impl Hyprpaper {
    pub fn unload_all(&self) {
        std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("unload")
            .arg("all")
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to unload hyprpaper.");
    }

    pub fn preload(&self, path: &Path) {
        std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("preload")
            .arg(path)
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to preload hyprpaper.");
    }

    pub fn wallpaper(&self, monitors: &[Monitor], path: &Path) {
        monitors.iter().for_each(|mon| {
            std::process::Command::new("hyprctl")
                .arg("hyprpaper")
                .arg("wallpaper")
                .arg(format!("{}, {}", mon.name, &path.display()))
                .stdout(Stdio::null())
                .spawn()
                .expect("Failed to set wallpaper hyprpaper.");
        });
    }

    fn save_wallpaper(&self, path: &Path) {
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

impl WallpaperDaemon for Hyprpaper {
    fn set_wallpaper(&self, path: &std::path::Path) {
        let monitors = Monitor::get_all().expect("The list of monitors.");
        self.unload_all();
        self.preload(path);
        self.wallpaper(&monitors, path);

        if self.0 {
            self.save_wallpaper(path);
        }
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
