use crate::theme::core::theme::Theme;
use crate::{core::rectangle::Rectangle, hyprland::hyprctl::monitors::Monitor};
use std::str::FromStr;

use super::selection::{self, Selection};

#[derive(Debug, Default)]
pub struct Slurp {
    theme: Option<Theme>,
}

impl Slurp {
    pub fn with_theme(theme: Theme) -> Self {
        Self { theme: Some(theme) }
    }
}

impl Selection for Slurp {
    fn select_rectangle(&self) -> Result<Rectangle, selection::Error> {
        let mut cmd = std::process::Command::new("slurp");
        cmd.arg("-f");
        cmd.arg("%x %y %w %h");

        if let Some(theme) = &self.theme {
            cmd.arg("-b");
            cmd.arg(theme.palette().background().with_alpha(0.25).hex_rgba());
            cmd.arg("-c");
            cmd.arg(theme.palette().bright_red().with_alpha(0.75).hex_rgba());
        }

        let cmd = cmd.output().map_err(|_| selection::Error::IoError)?.stdout;

        let text = String::from_utf8(cmd).map_err(|_| selection::Error::InvalidEncoding)?;

        Rectangle::from_str(&text).map_err(|_| selection::Error::InvalidSlurpRectangle)
    }

    fn select_point(&self) -> Result<Rectangle, selection::Error> {
        let x = String::from_utf8(
            std::process::Command::new("slurp")
                .arg("-p")
                .arg("-b")
                .arg("00000000")
                .arg("-f")
                .arg("%x %y %w %h")
                .output()
                .map_err(|_| selection::Error::IoError)?
                .stdout,
        )
        .map_err(|_| selection::Error::InvalidEncoding)?;

        Rectangle::from_str(&x).map_err(|_| selection::Error::InvalidSlurpRectangle)
    }

    fn select_monitor(&self, monitors: Vec<Monitor>) -> Result<Monitor, selection::Error> {
        let point = self.select_point()?;
        monitors
            .into_iter()
            .find(|m| point.inside(&Rectangle::from(m.clone())))
            .ok_or(selection::Error::MonitorNotFound)
    }
}
