use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
    #[arg(long)]
    pub config: Option<PathBuf>,
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
