mod arguments;
mod collection;

use arguments::Arguments;
use clap::Parser;
use collection::Collection;
use lib_hype::core::wallpaper::{hyprpaper::Hyprpaper, swww::Swww, WallpaperDaemon};
use std::path::PathBuf;

fn default_config_directory() -> PathBuf {
    let home = env!("HOME");
    PathBuf::from(format!("{home}/.config/hype/wallpaper"))
}

fn default_config_file() -> PathBuf {
    let mut dir = default_config_directory();
    dir.push("config.toml");
    dir
}

fn main() -> Result<(), String> {
    let args = Arguments::parse();

    let config = match args.config {
        None => {
            let file = default_config_file();
            if std::path::Path::exists(&file) {
                file
            } else {
                let default_dir = default_config_directory();
                std::fs::create_dir_all(default_dir)
                    .map_err(|_| "Could not create default config directory.")?;

                let col = Collection::default();
                col.save(&file)
                    .map_err(|_| "Could not save default config file.")?;

                file
            }
        }
        Some(ref cfg) => cfg.to_owned(),
    };

    let mut col = Collection::from_file(&config)?;

    match args.command {
        arguments::Command::Collection { collection_command } => match collection_command {
            arguments::CollectionCommand::Show => {
                col.get_collections().iter().for_each(|c| println!("{c}"));
            }
            arguments::CollectionCommand::List { name } => {
                match col.list_items_in_collection(&name) {
                    None => println!("Collection {name} not found."),
                    Some(entries) => {
                        entries.iter().for_each(|e| println!("{e}"));
                    }
                }
            }
            arguments::CollectionCommand::Create { name } => {
                col.create_collection(&name);
                if col.save(&config).is_ok() {
                    println!("Collection {name} created.");
                }
            }
            arguments::CollectionCommand::Add { collection, file } => {
                col.add_to_collection(&collection, file.clone());

                if col.save(&config).is_ok() {
                    println!("{} files has been added to {collection}.", file.len());
                }
            }
        },
        arguments::Command::Wallpaper { wallpaper_command } => match wallpaper_command {
            arguments::WallpaperCommand::Set { collection, file } => {
                let path = col.set_wallpaper(&collection, &file)?;
                match args.backend {
                    arguments::WallpaperBackend::Swww => Swww.set_wallpaper(&path),
                    arguments::WallpaperBackend::Hyprpaper => Hyprpaper(false).set_wallpaper(&path),
                }
            }
            arguments::WallpaperCommand::Random { collection, save } => {
                let wall = col.random_from_collection(&collection)?;
                match args.backend {
                    arguments::WallpaperBackend::Swww => Swww.set_wallpaper(&wall),
                    arguments::WallpaperBackend::Hyprpaper => Hyprpaper(save).set_wallpaper(&wall),
                }
            }
        },
    }

    Ok(())
}
