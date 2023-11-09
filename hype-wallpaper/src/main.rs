use std::path::PathBuf;

use clap::{Parser, ValueEnum, Subcommand};
use config_file::ConfigFile;

mod config_file;

fn main() -> Result<(), String> {
    let args = Config::parse();

    let config_dir = config_directory();
    let _ = std::fs::create_dir_all(&config_dir);
    let mut config_file = config_dir.clone();
    config_file.push("config.toml");

    let Some(cfg) = read_config(config_file.clone()) else {
        return Err(format!(
            "Config file invalid or not found in: '{}'",
            config_file.display()
        ));
    };
    
    match args.command {
        Command::Random { variant, save } => {
            let wall = match variant {
                Variant::Dark => cfg.random_dark(),
                Variant::Light => cfg.random_light()
            }.ok_or("No wallpapers found.".to_string())?;
            cfg.set_wallpaper(&wall);
            if save {
                cfg.save_wallpaper(&wall);
            }
        },
        Command::List{variant} => {
            let walls = match variant {
                Variant::Dark => cfg.get_dark().ok(),
                Variant::Light => cfg.get_light().ok(),
            }.ok_or("No wallpapers found.")?;
            walls
                .iter()
                .filter_map(|w| w.file_name())
                .for_each(|w| println!("{}", w.to_str().unwrap_or("")) );
        }
        Command::Set { variant, path , save } => {
            match cfg.find_by_name(variant, &path) {
                Some(path) => {
                    cfg.set_wallpaper(&path);
                    if save {
                        cfg.save_wallpaper(&path);
                    }
                },
                None => {
                    println!("Wallpaper not found: '{}'", path.display())
                }
            }
        }
    }
    
    Ok(())
}

#[derive(Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, ValueEnum, Subcommand)]
pub enum Variant {
    Dark,
    Light,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Random{
        variant: Variant,
        #[arg(short, long, default_value_t = false)]
        save: bool,
    },
    List{
        variant: Variant
    },
    Set{
        variant: Variant,
        path: PathBuf,
        #[arg(short, long, default_value_t = false)]
        save: bool,
    },
}

fn config_directory() -> PathBuf {
    let home = env!("HOME").to_owned();
    PathBuf::from(home + "/.config/hype/wallpaper")
}

fn hyprpaper_config() -> PathBuf {
    let home = env!("HOME").to_owned();
    PathBuf::from(home + "/.config/hypr/hyprpaper.conf")
}

fn read_config(path: PathBuf) -> Option<ConfigFile> {
    let content = std::fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

