
use lib_theme::core::themeable::Themeable;

use crate::create_config_dir_and_file;


pub struct Waybar;

impl Themeable for Waybar {
    fn path(&self) -> std::path::PathBuf {
        create_config_dir_and_file!("waybar", "theme.css")
    }

    fn content(&self, theme: &lib_theme::core::theme::Theme) -> String {
        theme.gtk_css_definitions()
    }

    fn run_post_apply_action(&self) {
        let _ = std::process::Command::new("pkill")
            .arg("-SIGUSR2")
            .arg("waybar")
            .spawn();
    }
}
