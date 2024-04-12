use args::Args;
use clap::Parser;
use lib_hype::hyprland::hyprctl::zoom::Zoom;

mod args;

fn main() {
    match Args::parse() {
        Args::Reset => Zoom::set(1.0),
        Args::Get => println!("{}", Zoom::get()),
        Args::Set { value } => Zoom::set(value),
        Args::ZoomIn { delta } => Zoom::zoom_in(delta),
        Args::ZoomOut { delta } => Zoom::zoom_out(delta),
    }
}
