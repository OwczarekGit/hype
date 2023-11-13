use crate::core::rectangle::Rectangle;

pub trait Selection {
    fn select_rectangle(&self) -> Result<Rectangle, Error>;
    fn select_point(&self) -> Result<Rectangle, Error>;
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidEncoding,
    InvalidFormat,
    InvalidSlurpRectangle,
    IoError,
}
