use std::path::PathBuf;

use lib_theme::core::themeable::Themeable;

use super::config_directory;

pub struct Swaylock;

impl Themeable for Swaylock {
    fn path(&self) -> std::path::PathBuf {
        PathBuf::from(config_directory() + "/swaylock/config")
    }

    fn content(&self, theme: &lib_theme::core::theme::Theme) -> String {
        let p = theme.palette();
        format!(
            r#"ignore-empty-password
color={}
inside-color={}
key-hl-color={}
bs-hl-color={}
separator-color={}
inside-clear-color={}
ring-color={}
inside-ver-color={}
ring-ver-color={}
inside-wrong-color={}
ring-wrong-color={}"#,
            p.bg().raw_hex_rgb(),
            p.bg().raw_hex_rgb(),
            p.c3().raw_hex_rgb(),
            p.c1().raw_hex_rgb(),
            p.c1().raw_hex_rgb(),
            p.c2().raw_hex_rgb(),
            p.c0().raw_hex_rgb(),
            p.c4().raw_hex_rgb(),
            p.c4().raw_hex_rgb(),
            p.c1().raw_hex_rgb(),
            p.c1().raw_hex_rgb(),
        )
    }
}
