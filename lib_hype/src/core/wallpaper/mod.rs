pub mod hyprpaper;
pub mod swww;

pub trait WallpaperDaemon {
    fn set_wallpaper(&self, path: &std::path::Path);
}
