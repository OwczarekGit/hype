use std::process::{Command, Stdio};

use super::WallpaperDaemon;

pub struct Swww;

impl WallpaperDaemon for Swww {
    fn set_wallpaper(&self, path: &std::path::Path) {
        Command::new("swww")
            .args(["img", path.to_str().expect("Invalid path")])
            .stdout(Stdio::null())
            .spawn()
            .expect("Swww not found or swww-daemon not running");
    }
}
