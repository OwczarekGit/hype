use crate::core::rectangle::Rectangle;
use std::str::FromStr;

pub struct Slurp;

impl Slurp {
    pub fn select_rectangle(&self) -> Result<Rectangle, SlurpError> {
        let x = String::from_utf8(
            std::process::Command::new("slurp")
                .arg("-f")
                .arg("%x %y %w %h")
                .output()
                .map_err(|_| SlurpError::IoError)?
                .stdout,
        )
        .map_err(|_| SlurpError::InvalidEncoding)?;

        Ok(Rectangle::from_str(&x).map_err(|_| SlurpError::InvalidSlurpRectangle)?)
    }

    pub fn select_point(&self) -> Result<Rectangle, SlurpError> {
        let x = String::from_utf8(
            std::process::Command::new("slurp")
                .arg("-p")
                .arg("-f")
                .arg("%x %y %w %h")
                .output()
                .map_err(|_| SlurpError::IoError)?
                .stdout,
        )
        .map_err(|_| SlurpError::InvalidEncoding)?;

        Ok(Rectangle::from_str(&x).map_err(|_| SlurpError::InvalidSlurpRectangle)?)
    }
}

pub enum SlurpError {
    InvalidEncoding,
    InvalidFormat,
    InvalidSlurpRectangle,
    IoError,
}
