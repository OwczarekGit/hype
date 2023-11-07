use serde::{Deserialize, Serialize};

use super::palette::Palette;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Theme {
    name: String,
    palette: Palette,
}

impl Theme {
    pub fn new(name: String, palette: Palette) -> Self {
        Self { name, palette }
    }

    pub fn palette(&self) -> Palette {
        self.palette
    }
}
