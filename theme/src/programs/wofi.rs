use lib_theme::core::themeable::Themeable;
use std::path::PathBuf;

use super::config_directory;

pub struct Wofi;

impl Themeable for Wofi {
    fn path(&self) -> std::path::PathBuf {
        PathBuf::from(config_directory() + "/wofi/theme.css")
    }

    fn content(&self, theme: &lib_theme::core::theme::Theme) -> String {
        theme.gtk_css_definitions()
    }
}
