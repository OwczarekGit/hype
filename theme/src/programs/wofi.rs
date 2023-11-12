use lib_hype::core::dirs::ConfigDirectory;
use lib_hype::theme::core::themeable::Themeable;

pub struct Wofi;

impl Themeable for Wofi {
    fn path(&self) -> std::path::PathBuf {
        ConfigDirectory::create_config_file("wofi/theme.css").expect("No io access")
    }

    fn content(&self, theme: &lib_hype::theme::core::theme::Theme) -> String {
        theme.gtk_css_definitions()
    }
}
