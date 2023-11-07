use std::sync::OnceLock;

use crate::core::{palette::Palette, theme::Theme};

static SOLARIZED_DARK: OnceLock<Theme> = OnceLock::new();

pub fn theme() -> Theme {
    SOLARIZED_DARK
        .get_or_init(|| {
            Theme::new(
                "Solarized Light".to_string(),
                Palette::new(
                    0xFDF6E300_u32,
                    0x586E7500_u32,
                    0x002B3600_u32,
                    0xDC322F00_u32,
                    0x85990000_u32,
                    0xB5890000_u32,
                    0x268BD200_u32,
                    0x6C71C400_u32,
                    0x2AA19800_u32,
                    0x93A1A100_u32,
                    0x657B8300_u32,
                    0xDC322F00_u32,
                    0x85990000_u32,
                    0xB5890000_u32,
                    0x268BD200_u32,
                    0x6C71C400_u32,
                    0x2AA19800_u32,
                    0xFDF6E300_u32,
                ),
            )
        })
        .clone()
}
