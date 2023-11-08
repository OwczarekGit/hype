use std::path::PathBuf;
use lib_theme::core::themeable::Themeable;
use super::config_directory;

pub struct Alacritty;

impl Themeable for Alacritty {
    fn path(&self) -> PathBuf {
        PathBuf::from(config_directory() + "/alacritty/theme.yml")
    }

    fn content(&self, theme: &lib_theme::core::theme::Theme) -> String {
        format!(
            r#"# Remember to import this file in your alacritty.yml
colors:
    primary:
        background: '{}'
        foreground: '{}'
    normal:
        black: '{}'
        red: '{}'
        green: '{}'
        yellow: '{}'
        blue: '{}'
        magenta: '{}'
        cyan: '{}'
        white: '{}'
    bright:
        black: '{}'
        red: '{}'
        green: '{}'
        yellow: '{}'
        blue: '{}'
        magenta: '{}'
        cyan: '{}'
        white: '{}'"#,
            theme.palette().bg().hex_rgb(),
            theme.palette().fg().hex_rgb(),
            theme.palette().bg().hex_rgb(),
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
