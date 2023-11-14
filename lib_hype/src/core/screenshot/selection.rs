use crate::{core::rectangle::Rectangle, hyprland::hyprctl::monitors::Monitor};

pub trait Selection {
    fn select_rectangle(&self) -> Result<Rectangle, Error>;
    fn select_point(&self) -> Result<Rectangle, Error>;
    fn select_monitor(&self, monitors: Vec<Monitor>) -> Result<Monitor, Error>;
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidEncoding,
    InvalidFormat,
    InvalidSlurpRectangle,
    IoError,
    MonitorNotFound,
}
