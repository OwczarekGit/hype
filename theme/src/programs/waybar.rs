use std::path::PathBuf;

use lib_theme::core::themeable::Themeable;

use super::config_directory;

pub struct Waybar;

impl Themeable for Waybar {
    fn path(&self) -> std::path::PathBuf {
        PathBuf::from(config_directory() + "/waybar/theme.css")
    }

    fn content(&self, theme: lib_theme::core::theme::Theme) -> String {
        theme.css_definitions()
    }
    
    fn run_post_apply_action(&self) {
        let _ = std::process::Command::new("pkill")
            .arg("-SIGUSR2")
            .arg("waybar")
            .spawn();
    }
}
