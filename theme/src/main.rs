mod programs;
use lib_theme::core::themeable::Themeable;

fn main() {
    let theme = lib_theme::themes::catppuccin_frappe::theme();
    programs::alacritty::Alacritty.apply(&theme);
    programs::wofi::Wofi.apply(&theme);
    programs::hyprland::Hyprland.apply(&theme);
    programs::waybar::Waybar.apply(&theme);
    programs::swaylock::Swaylock.apply(&theme);
    programs::mako::Mako.apply(&theme);
}
