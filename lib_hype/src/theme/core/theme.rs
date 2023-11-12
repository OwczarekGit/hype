use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};

use super::terminal_colors::TerminalColors;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Theme {
    name: String,
    variant: Variant,
    palette: TerminalColors,
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
    pub fn new(name: String, variant: Variant, palette: TerminalColors) -> Self {
        Self {
            name,
            variant,
            palette,
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), ThemeError> {
        let mut file = std::fs::File::create(path)?;
        let parsed = toml::to_string_pretty(self)?;
        file.write_all(parsed.as_bytes())?;
        Ok(())
    }

    pub fn palette(&self) -> TerminalColors {
        self.palette
    }

    pub fn gtk_css_definitions(&self) -> String {
        let colors = self.palette();
        format!(
            r#"/* Remember to import this file in your style.css */
@define-color background {};
@define-color foreground {};
@define-color black {};
@define-color red {};
@define-color green {};
@define-color yellow {};
@define-color blue {};
@define-color magenta {};
@define-color cyan {};
@define-color white {};
@define-color bblack {};
@define-color bred {};
@define-color bgreen {};
@define-color byellow {};
@define-color bblue {};
@define-color bmagenta {};
@define-color bcyan {};
@define-color bwhite {};"#,
            colors.background().hex_rgb(),
            colors.foreground().hex_rgb(),
            colors.black().hex_rgb(),
            colors.red().hex_rgb(),
            colors.green().hex_rgb(),
            colors.yellow().hex_rgb(),
            colors.blue().hex_rgb(),
            colors.magenta().hex_rgb(),
            colors.cyan().hex_rgb(),
            colors.white().hex_rgb(),
            colors.bright_black().hex_rgb(),
            colors.bright_red().hex_rgb(),
            colors.bright_green().hex_rgb(),
            colors.bright_yellow().hex_rgb(),
            colors.bright_blue().hex_rgb(),
            colors.bright_magenta().hex_rgb(),
            colors.bright_cyan().hex_rgb(),
            colors.bright_white().hex_rgb(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThemeError {
    IoError,
    ParseError,
}

impl From<toml::ser::Error> for ThemeError {
    fn from(_: toml::ser::Error) -> Self {
        Self::ParseError
    }
}

impl From<std::io::Error> for ThemeError {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}
