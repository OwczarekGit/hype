pub mod rose_pine_moon;
pub mod solarized_dark;
pub mod solarized_light;
pub mod catppuccin_frappe;

#[macro_export]
macro_rules! define_theme {
    (
        $name:expr,
        $variant:expr,
        $bg:expr,
        $fg:expr,
        $c0:expr,
        $c1:expr,
        $c2:expr,
        $c3:expr,
        $c4:expr,
        $c5:expr,
        $c6:expr,
        $c7:expr,
        $c8:expr,
        $c9:expr,
        $c10:expr,
        $c11:expr,
        $c12:expr,
        $c13:expr,
        $c14:expr,
        $c15:expr,
    ) => {
        pub fn theme() -> $crate::core::theme::Theme {
            static THEME: std::sync::OnceLock<$crate::core::theme::Theme> =
                std::sync::OnceLock::new();
            THEME
                .get_or_init(|| {
                    $crate::core::theme::Theme::new(
                        $name.to_string(),
                        $variant,
                        $crate::core::terminal_colors::TerminalColors::new(
                            $bg, $fg, $c0, $c1, $c2, $c3, $c4, $c5, $c6, $c7, $c8, $c9, $c10, $c11,
                            $c12, $c13, $c14, $c15,
                        ),
                    )
                })
                .clone()
        }
    };
}
