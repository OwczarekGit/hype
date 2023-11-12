mod programs;
use lib_hype::core::dirs::ConfigDirectory;
use lib_hype::theme::core::themeable::Themeable;

fn main() {
    let theme = lib_hype::theme::themes::catppuccin_frappe::theme();
    programs::alacritty::Alacritty.apply(&theme);
    programs::wofi::Wofi.apply(&theme);
    programs::hyprland::Hyprland.apply(&theme);
    programs::waybar::Waybar.apply(&theme);
    programs::swaylock::Swaylock.apply(&theme);
    programs::mako::Mako.apply(&theme);

    if let Ok(path) = ConfigDirectory::create_config_file_in_hype("theme") {
        let _ = theme.save(&path);
    }
}
