use std::string::FromUtf8Error;

use serde::{Deserialize, Serialize};

pub mod clients;
pub mod cursorpos;
pub mod monitors;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HyprctlError {
    CommandNotExecuted,
    InvalidCommandOutput,
    InvalidJsonFormat,
}

impl From<std::io::Error> for HyprctlError {
    fn from(_: std::io::Error) -> Self {
        Self::CommandNotExecuted
    }
}

impl From<FromUtf8Error> for HyprctlError {
    fn from(_: FromUtf8Error) -> Self {
        Self::InvalidCommandOutput
    }
}

impl From<serde_json::Error> for HyprctlError {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidJsonFormat
    }
}
