use arguments::Arguments;
use arguments::VolumeCommand;
use clap::Parser;
use lib_hype::media::wpctl::{WpCtl, VOLUME_DEFAULT_STEP};

mod arguments;

fn main() -> Result<(), String> {
    let args = Arguments::parse();

    match args.command {
        arguments::Command::Volume { volume_command } => match volume_command {
            VolumeCommand::Get => println!("{}", WpCtl::get_volume()?),
            VolumeCommand::Set { volume } => WpCtl::set_volume(volume)?,
            VolumeCommand::Increase { volume } => {
                WpCtl::increase_volume(volume.unwrap_or(VOLUME_DEFAULT_STEP))?;
            }
            VolumeCommand::Decrease { volume } => {
                WpCtl::decrease_volume(volume.unwrap_or(VOLUME_DEFAULT_STEP))?;
            }
        },
    }
    Ok(())
}
