use std::env;

pub mod alacritty;
pub mod hyprland;
pub mod waybar;
pub mod wofi;

pub fn config_directory() -> String {
    let home = env::var("HOME").expect("HOME env variable not set.");
    format!("{home}/.config")
}
