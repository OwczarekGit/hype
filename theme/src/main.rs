mod programs;
use lib_theme::core::themeable::Themeable;

fn main() {
    let theme = lib_theme::themes::rose_pine_moon::theme();
    programs::alacritty::Alacritty.apply(theme.clone());
    programs::wofi::Wofi.apply(theme.clone());
    programs::hyprland::Hyprland.apply(theme.clone());
    programs::waybar::Waybar.apply(theme.clone());
}
