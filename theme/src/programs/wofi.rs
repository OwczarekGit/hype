use lib_theme::core::themeable::Themeable;

use crate::create_config_dir_and_file;

pub struct Wofi;

impl Themeable for Wofi {
    fn path(&self) -> std::path::PathBuf {
        create_config_dir_and_file!("wofi", "theme.css")
    }

    fn content(&self, theme: &lib_theme::core::theme::Theme) -> String {
        theme.gtk_css_definitions()
    }
}
