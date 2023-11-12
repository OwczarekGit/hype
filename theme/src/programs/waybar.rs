use lib_hype::core::dirs::ConfigDirectory;
use lib_hype::theme::core::themeable::Themeable;

pub struct Waybar;

impl Themeable for Waybar {
    fn path(&self) -> std::path::PathBuf {
        ConfigDirectory::create_config_file("waybar/theme.css").expect("No io access")
    }

    fn content(&self, theme: &lib_hype::theme::core::theme::Theme) -> String {
        theme.gtk_css_definitions()
    }

    fn run_post_apply_action(&self) {
        let _ = std::process::Command::new("pkill")
            .arg("-SIGUSR2")
            .arg("waybar")
            .spawn();
    }
}
