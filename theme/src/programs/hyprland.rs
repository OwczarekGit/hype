use std::path::PathBuf;

use lib_theme::core::themeable::Themeable;

pub struct Hyprland;

impl Themeable for Hyprland {
    fn path(&self) -> std::path::PathBuf {
        let home = env!("HOME");
        PathBuf::from(format!("{home}/.config/hypr/theme.conf"))
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
            theme.palette().bg().hex_rgb(),
            theme.palette().fg().hex_rgb(),
            theme.palette().c0().hex_rgb(),
            theme.palette().c1().hex_rgb(),
            theme.palette().c2().hex_rgb(),
            theme.palette().c3().hex_rgb(),
            theme.palette().c4().hex_rgb(),
            theme.palette().c5().hex_rgb(),
            theme.palette().c6().hex_rgb(),
            theme.palette().c7().hex_rgb(),
            theme.palette().c8().hex_rgb(),
            theme.palette().c9().hex_rgb(),
            theme.palette().c10().hex_rgb(),
            theme.palette().c11().hex_rgb(),
            theme.palette().c12().hex_rgb(),
            theme.palette().c13().hex_rgb(),
            theme.palette().c14().hex_rgb(),
            theme.palette().c15().hex_rgb(),
        )
    }
}
