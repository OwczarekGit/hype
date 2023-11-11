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
    pub fn create_config_file(cfg: impl Into<PathBuf>) -> Result<PathBuf, std::io::Error> {
        let mut config_file = config_dir();
        let cfg: PathBuf = cfg.into();
        config_file.push(cfg);
        if let Some(p) = config_file.parent() {
            std::fs::create_dir_all(p)?;
        }
        Ok(config_file)
    }

    pub fn create_config_file_in_hype(cfg: impl Into<PathBuf>) -> Result<PathBuf, std::io::Error> {
        let mut config_file = hype_config_dir();
        let cfg: PathBuf = cfg.into();
        config_file.push(cfg);
        if let Some(p) = config_file.parent() {
            std::fs::create_dir_all(p)?;
        }
        Ok(config_file)
    }
}
