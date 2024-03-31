use std::{fmt::Display, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[arg(long, short, default_value_t = WallpaperBackend::Swww)]
    pub backend: WallpaperBackend,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, ValueEnum)]
pub enum WallpaperBackend {
    #[default]
    Swww,
    Hyprpaper,
}

impl Display for WallpaperBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallpaperBackend::Swww => write!(f, "swww"),
            WallpaperBackend::Hyprpaper => write!(f, "hyprpaper"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Subcommand)]
pub enum Command {
    Collection {
        #[clap(subcommand)]
        collection_command: CollectionCommand,
    },
    Wallpaper {
        #[clap(subcommand)]
        wallpaper_command: WallpaperCommand,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Subcommand)]
pub enum WallpaperCommand {
    Set {
        collection: String,
        file: PathBuf,
    },
    Random {
        collection: String,
        #[arg(short)]
        save: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Subcommand)]
pub enum CollectionCommand {
    Show,
    List {
        name: String,
    },
    Create {
        name: String,
    },
    Add {
        collection: String,
        file: Vec<PathBuf>,
    },
}
