use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{process::Stdio, io::Write, path::{PathBuf, Path}};

use crate::{hyprpaper_config, Variant};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFile {
    wallpapers_dark: PathBuf,
    wallpapers_light: PathBuf,
    monitors: Vec<String>,
}

impl ConfigFile {
    pub fn get_dark(&self) -> Result<Vec<PathBuf>, String> {
        let Ok(paths) = std::fs::read_dir(&self.wallpapers_dark) else {
            return Err(format!(
                "Could not read directory: '{}'",
                &self.wallpapers_dark.display()
            ));
        };

        Ok(paths
            .into_iter()
            .filter_map(|entry| entry.map(|e| e.path()).ok())
            .collect::<Vec<_>>())
    }

    pub fn get_light(&self) -> Result<Vec<PathBuf>, String> {
        let Ok(paths) = std::fs::read_dir(&self.wallpapers_light) else {
            return Err(format!(
                "Could not read directory: '{}'",
                &self.wallpapers_dark.display()
            ));
        };

        Ok(paths
            .into_iter()
            .filter_map(|entry| entry.map(|e| e.path()).ok())
            .collect::<Vec<_>>())
    }
    
    pub fn random_dark(&self) -> Option<PathBuf> {
        let mut rng = rand::thread_rng();
        let walls = self.get_dark().ok()?;
        walls.choose(&mut rng).cloned()
    }

    pub fn random_light(&self) -> Option<PathBuf> {
        let mut rng = rand::thread_rng();
        let walls = self.get_light().ok()?;
        walls.choose(&mut rng).cloned()
    }
    
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
        
        self.monitors
            .iter()
            .for_each(|mon| {
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
        content.push_str(format!("preload = {}\n",path.display()).as_str());

        self.monitors
            .iter()
            .for_each(|mon| content.push_str(format!("wallpaper = {mon}, {}\n", path.display()).as_str()));

        let _ = file.write_all(content.as_bytes());
    }
    
    pub fn find_by_name(&self, variant: Variant, path: &Path) -> Option<PathBuf> {
        let walls = match variant {
            Variant::Dark => self.get_dark().ok(),
            Variant::Light => self.get_light().ok(),
        }?;
        
        walls
            .iter()
            .find(|p| {
                return p.file_name() == Some(path.as_os_str())
            }).cloned()
    }
}
