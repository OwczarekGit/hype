use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct TerminalColors {
    bg: Color,
    fg: Color,

    black: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    white: Color,

    bblack: Color,
    bred: Color,
    bgreen: Color,
    byellow: Color,
    bblue: Color,
    bmagenta: Color,
    bcyan: Color,
    bwhite: Color,
}

impl TerminalColors {
    pub fn new(
        background: impl Into<Color>,
        foreground: impl Into<Color>,

        black: impl Into<Color>,
        red: impl Into<Color>,
        green: impl Into<Color>,
        yellow: impl Into<Color>,
        blue: impl Into<Color>,
        magenta: impl Into<Color>,
        cyan: impl Into<Color>,
        white: impl Into<Color>,

        bblack: impl Into<Color>,
        bred: impl Into<Color>,
        bgreen: impl Into<Color>,
        byellow: impl Into<Color>,
        bblue: impl Into<Color>,
        bmagenta: impl Into<Color>,
        bcyan: impl Into<Color>,
        bwhite: impl Into<Color>,
    ) -> Self {
        Self {
            bg: background.into(),
            fg: foreground.into(),

            black: black.into(),
            red: red.into(),
            green: green.into(),
            yellow: yellow.into(),
            blue: blue.into(),
            magenta: magenta.into(),
            cyan: cyan.into(),
            white: white.into(),

            bblack: bblack.into(),
            bred: bred.into(),
            bgreen: bgreen.into(),
            byellow: byellow.into(),
            bblue: bblue.into(),
            bmagenta: bmagenta.into(),
            bcyan: bcyan.into(),
            bwhite: bwhite.into(),
        }
    }
}

impl TerminalColors {
    pub fn background(&self) -> Color {
        self.bg
    }

    pub fn foreground(&self) -> Color {
        self.fg
    }

    pub fn black(&self) -> Color {
        self.black
    }

    pub fn red(&self) -> Color {
        self.red
    }

    pub fn green(&self) -> Color {
        self.green
    }

    pub fn yellow(&self) -> Color {
        self.yellow
    }

    pub fn blue(&self) -> Color {
        self.blue
    }

    pub fn magenta(&self) -> Color {
        self.magenta
    }

    pub fn cyan(&self) -> Color {
        self.cyan
    }

    pub fn white(&self) -> Color {
        self.white
    }

    pub fn bright_black(&self) -> Color {
        self.bblack
    }

    pub fn bright_red(&self) -> Color {
        self.bred
    }

    pub fn bright_green(&self) -> Color {
        self.bgreen
    }

    pub fn bright_yellow(&self) -> Color {
        self.byellow
    }

    pub fn bright_blue(&self) -> Color {
        self.bblue
    }

    pub fn bright_magenta(&self) -> Color {
        self.bmagenta
    }

    pub fn bright_cyan(&self) -> Color {
        self.bcyan
    }

    pub fn bright_white(&self) -> Color {
        self.bwhite
    }
}
