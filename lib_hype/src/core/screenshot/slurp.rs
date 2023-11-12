use crate::core::rectangle::Rectangle;
use crate::theme::core::theme::Theme;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Slurp {
    theme: Option<Theme>,
}

impl Slurp {
    pub fn with_theme(theme: Theme) -> Self {
        Self { theme: Some(theme) }
    }
}

impl Slurp {
    pub fn select_rectangle(&self) -> Result<Rectangle, SlurpError> {
        let mut cmd = std::process::Command::new("slurp");
        cmd.arg("-f");
        cmd.arg("%x %y %w %h");

        if let Some(theme) = &self.theme {
            cmd.arg("-b");
            cmd.arg(theme.palette().background().with_alpha(0.25).hex_rgba());
            cmd.arg("-c");
            cmd.arg(theme.palette().bright_red().with_alpha(0.75).hex_rgba());
        }

        let cmd = cmd.output().map_err(|_| SlurpError::IoError)?.stdout;

        let text = String::from_utf8(cmd).map_err(|_| SlurpError::InvalidEncoding)?;

        Rectangle::from_str(&text).map_err(|_| SlurpError::InvalidSlurpRectangle)
    }

    pub fn select_point(&self) -> Result<Rectangle, SlurpError> {
        let x = String::from_utf8(
            std::process::Command::new("slurp")
                .arg("-p")
                .arg("-b")
                .arg("00000000")
                .arg("-f")
                .arg("%x %y %w %h")
                .output()
                .map_err(|_| SlurpError::IoError)?
                .stdout,
        )
        .map_err(|_| SlurpError::InvalidEncoding)?;

        Rectangle::from_str(&x).map_err(|_| SlurpError::InvalidSlurpRectangle)
    }
}

#[derive(Debug, Clone)]
pub enum SlurpError {
    InvalidEncoding,
    InvalidFormat,
    InvalidSlurpRectangle,
    IoError,
}
