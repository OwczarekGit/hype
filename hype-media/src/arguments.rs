use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Volume {
        #[clap(subcommand)]
        volume_command: VolumeCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum VolumeCommand {
    Get,
    Set { volume: u32 },
    Increase { volume: Option<u32> },
    Decrease { volume: Option<u32> },
}
