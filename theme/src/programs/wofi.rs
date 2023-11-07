use lib_theme::core::themeable::Themeable;
use std::path::PathBuf;

pub struct Wofi;

impl Themeable for Wofi {
    fn path(&self) -> std::path::PathBuf {
        let home = env!("HOME");
        PathBuf::from(format!("{home}/.config/wofi/theme.css"))
    }

    fn content(&self, theme: lib_theme::core::theme::Theme) -> String {
        format!(
            r#"/* Remember to include this file in your style.css */
@define-color bg {};
@define-color fg {};
@define-color c0 {};
@define-color c1 {};
@define-color c2 {};
@define-color c3 {};
@define-color c4 {};
@define-color c5 {};
@define-color c6 {};
@define-color c7 {};
@define-color c8 {};
@define-color c9 {};
@define-color c10 {};
@define-color c11 {};
@define-color c12 {};
@define-color c13 {};
@define-color c14 {};
@define-color c15 {};"#,
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
