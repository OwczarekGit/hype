use lib_hype::core::dirs::ConfigDirectory;
use lib_hype::theme::core::themeable::Themeable;

pub struct Hyprland;

impl Themeable for Hyprland {
    fn path(&self) -> std::path::PathBuf {
        ConfigDirectory::create_config_file("hypr/theme.conf").expect("No io access")
    }

    fn content(&self, theme: &lib_hype::theme::core::theme::Theme) -> String {
        format!(
            r#"# # Remember to source this file in your hyprland.conf
$background = {}
$foreground = {}
$black = {}
$red = {}
$green = {}
$yellow = {}
$blue = {}
$magenta = {}
$cyan = {}
$white = {}
$bblack = {}
$bred = {}
$bgreen = {}
$byellow = {}
$bblue = {}
$bmagenta = {}
$bcyan = {}
$bwhite = {}"#,
            theme.palette().background().hyprland_rgba(),
            theme.palette().foreground().hyprland_rgba(),
            theme.palette().black().hyprland_rgba(),
            theme.palette().red().hyprland_rgba(),
            theme.palette().green().hyprland_rgba(),
            theme.palette().yellow().hyprland_rgba(),
            theme.palette().blue().hyprland_rgba(),
            theme.palette().magenta().hyprland_rgba(),
            theme.palette().cyan().hyprland_rgba(),
            theme.palette().white().hyprland_rgba(),
            theme.palette().bright_black().hyprland_rgba(),
            theme.palette().bright_red().hyprland_rgba(),
            theme.palette().bright_green().hyprland_rgba(),
            theme.palette().bright_yellow().hyprland_rgba(),
            theme.palette().bright_blue().hyprland_rgba(),
            theme.palette().bright_magenta().hyprland_rgba(),
            theme.palette().bright_cyan().hyprland_rgba(),
            theme.palette().bright_white().hyprland_rgba(),
        )
    }
}
