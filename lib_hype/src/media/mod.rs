use std::fmt::Display;

pub mod wpctl;

#[derive(Debug)]
pub enum WpCtlError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    MissingVolume,
    ParseFloat,
}

impl From<std::io::Error> for WpCtlError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for WpCtlError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl Display for WpCtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}

impl From<WpCtlError> for String {
    fn from(value: WpCtlError) -> Self {
        value.to_string()
    }
}
