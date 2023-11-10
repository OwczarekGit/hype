use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Screenshot { output: Option<PathBuf> },
    ScreenshotRect { output: Option<PathBuf> },
}