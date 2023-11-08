use serde::{Deserialize, Serialize};

use super::palette::Palette;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Theme {
    name: String,
    variant: Variant,
    palette: Palette,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Variant {
    Dark,
    Light,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Dark
    }
}

impl Theme {
    pub fn new(name: String, variant: Variant, palette: Palette) -> Self {
        Self { name, variant, palette }
    }

    pub fn palette(&self) -> Palette {
        self.palette
    }

    pub fn css_definitions(&self) -> String {
        let colors = self.palette();
        format!(
            r#"/* Remember to import this file in your style.css */
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
            colors.bg().hex_rgb(),
            colors.fg().hex_rgb(),
            colors.c0().hex_rgb(),
            colors.c1().hex_rgb(),
            colors.c2().hex_rgb(),
            colors.c3().hex_rgb(),
            colors.c4().hex_rgb(),
            colors.c5().hex_rgb(),
            colors.c6().hex_rgb(),
            colors.c7().hex_rgb(),
            colors.c8().hex_rgb(),
            colors.c9().hex_rgb(),
            colors.c10().hex_rgb(),
            colors.c11().hex_rgb(),
            colors.c12().hex_rgb(),
            colors.c13().hex_rgb(),
            colors.c14().hex_rgb(),
            colors.c15().hex_rgb(),
        )
    }
}
