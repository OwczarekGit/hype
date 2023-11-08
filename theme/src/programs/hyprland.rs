use std::path::PathBuf;

use lib_theme::core::themeable::Themeable;

use super::config_directory;

pub struct Hyprland;

impl Themeable for Hyprland {
    fn path(&self) -> std::path::PathBuf {
        PathBuf::from(config_directory() + "/hypr/theme.conf")
    }

    fn content(&self, theme: lib_theme::core::theme::Theme) -> String {
        format!(
            r#"## Remember to source this file in your hyprland.conf
$bg = {}
$fg = {}
$c0 = {}
$c1 = {}
$c2 = {}
$c3 = {}
$c4 = {}
$c5 = {}
$c6 = {}
$c7 = {}
$c8 = {}
$c9 = {}
$c10 = {}
$c11 = {}
$c12 = {}
$c13 = {}
$c14 = {}
$c15 = {}
"#,
            theme.palette().bg().hyprland_rgba(),
            theme.palette().fg().hyprland_rgba(),
            theme.palette().c0().hyprland_rgba(),
            theme.palette().c1().hyprland_rgba(),
            theme.palette().c2().hyprland_rgba(),
            theme.palette().c3().hyprland_rgba(),
            theme.palette().c4().hyprland_rgba(),
            theme.palette().c5().hyprland_rgba(),
            theme.palette().c6().hyprland_rgba(),
            theme.palette().c7().hyprland_rgba(),
            theme.palette().c8().hyprland_rgba(),
            theme.palette().c9().hyprland_rgba(),
            theme.palette().c10().hyprland_rgba(),
            theme.palette().c11().hyprland_rgba(),
            theme.palette().c12().hyprland_rgba(),
            theme.palette().c13().hyprland_rgba(),
            theme.palette().c14().hyprland_rgba(),
            theme.palette().c15().hyprland_rgba(),
        )
    }
}
