use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    PathBuf::from(env!("HOME"))
}

pub fn config_dir() -> PathBuf {
    let mut home = home_dir();
    home.push(".config");
    home
}

pub fn hype_config_dir() -> PathBuf {
    let mut cfg = config_dir();
    cfg.push("hype");
    cfg
}

pub struct ConfigDirectory;

impl ConfigDirectory {
    pub fn get_create_hype_config_dir() -> Result<PathBuf, std::io::Error> {
        let dir = hype_config_dir();
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }
    
    pub fn get_create_config_subdir(path: impl Into<PathBuf>) -> Result<PathBuf, std::io::Error> {
        let mut dir = config_dir();
        dir.push(path.into());
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}