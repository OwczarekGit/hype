use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct Palette {
    bg: Color,
    fg: Color,
    c0: Color,
    c1: Color,
    c2: Color,
    c3: Color,
    c4: Color,
    c5: Color,
    c6: Color,
    c7: Color,
    c8: Color,
    c9: Color,
    c10: Color,
    c11: Color,
    c12: Color,
    c13: Color,
    c14: Color,
    c15: Color,
}

impl Palette {
    pub fn new(
        bg: impl Into<Color>,
        fg: impl Into<Color>,
        c0: impl Into<Color>,
        c1: impl Into<Color>,
        c2: impl Into<Color>,
        c3: impl Into<Color>,
        c4: impl Into<Color>,
        c5: impl Into<Color>,
        c6: impl Into<Color>,
        c7: impl Into<Color>,
        c8: impl Into<Color>,
        c9: impl Into<Color>,
        c10: impl Into<Color>,
        c11: impl Into<Color>,
        c12: impl Into<Color>,
        c13: impl Into<Color>,
        c14: impl Into<Color>,
        c15: impl Into<Color>,
    ) -> Self {
        Self {
            bg: bg.into(),
            fg: fg.into(),
            c0: c0.into(),
            c1: c1.into(),
            c2: c2.into(),
            c3: c3.into(),
            c4: c4.into(),
            c5: c5.into(),
            c6: c6.into(),
            c7: c7.into(),
            c8: c8.into(),
            c9: c9.into(),
            c10: c10.into(),
            c11: c11.into(),
            c12: c12.into(),
            c13: c13.into(),
            c14: c14.into(),
            c15: c15.into(),
        }
    }
}

impl Palette {
    pub fn bg(&self) -> Color {
        self.bg
    }

    pub fn fg(&self) -> Color {
        self.fg
    }

    pub fn c0(&self) -> Color {
        self.c0
    }

    pub fn c1(&self) -> Color {
        self.c1
    }

    pub fn c2(&self) -> Color {
        self.c2
    }

    pub fn c3(&self) -> Color {
        self.c3
    }

    pub fn c4(&self) -> Color {
        self.c4
    }

    pub fn c5(&self) -> Color {
        self.c5
    }

    pub fn c6(&self) -> Color {
        self.c6
    }

    pub fn c7(&self) -> Color {
        self.c7
    }

    pub fn c8(&self) -> Color {
        self.c8
    }

    pub fn c9(&self) -> Color {
        self.c9
    }

    pub fn c10(&self) -> Color {
        self.c10
    }

    pub fn c11(&self) -> Color {
        self.c11
    }

    pub fn c12(&self) -> Color {
        self.c12
    }

    pub fn c13(&self) -> Color {
        self.c13
    }

    pub fn c14(&self) -> Color {
        self.c14
    }

    pub fn c15(&self) -> Color {
        self.c15
    }
}
