use lib_hype::core::dirs::ConfigDirectory;
use lib_hype::theme::core::themeable::Themeable;
use std::path::PathBuf;

pub struct Alacritty;

impl Themeable for Alacritty {
    fn path(&self) -> PathBuf {
        ConfigDirectory::create_config_file("alacritty/theme.yml").expect("No io access")
    }

    fn content(&self, theme: &lib_hype::theme::core::theme::Theme) -> String {
        format!(
            r#"# Remember to import this file in your alacritty.yml
colors:
    primary:
        background: '{}'
        foreground: '{}'
    normal:
        black: '{}'
        red: '{}'
        green: '{}'
        yellow: '{}'
        blue: '{}'
        magenta: '{}'
        cyan: '{}'
        white: '{}'
    bright:
        black: '{}'
        red: '{}'
        green: '{}'
        yellow: '{}'
        blue: '{}'
        magenta: '{}'
        cyan: '{}'
        white: '{}'"#,
            theme.palette().background().hex_rgb(),
            theme.palette().foreground().hex_rgb(),
            theme.palette().black().hex_rgb(),
            theme.palette().red().hex_rgb(),
            theme.palette().green().hex_rgb(),
            theme.palette().yellow().hex_rgb(),
            theme.palette().blue().hex_rgb(),
            theme.palette().magenta().hex_rgb(),
            theme.palette().cyan().hex_rgb(),
            theme.palette().white().hex_rgb(),
            theme.palette().bright_black().hex_rgb(),
            theme.palette().bright_red().hex_rgb(),
            theme.palette().bright_green().hex_rgb(),
            theme.palette().bright_yellow().hex_rgb(),
            theme.palette().bright_blue().hex_rgb(),
            theme.palette().bright_magenta().hex_rgb(),
            theme.palette().bright_cyan().hex_rgb(),
            theme.palette().bright_white().hex_rgb(),
        )
    }
}
