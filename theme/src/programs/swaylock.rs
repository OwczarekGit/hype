use lib_hype::core::dirs::ConfigDirectory;
use lib_hype::theme::core::themeable::Themeable;

pub struct Swaylock;

impl Themeable for Swaylock {
    fn path(&self) -> std::path::PathBuf {
        ConfigDirectory::create_config_file("swaylock/config").expect("No io access")
    }

    fn content(&self, theme: &lib_hype::theme::core::theme::Theme) -> String {
        let p = theme.palette();
        format!(
            r#"ignore-empty-password
color={}
inside-color={}
key-hl-color={}
bs-hl-color={}
separator-color={}
inside-clear-color={}
ring-color={}
inside-ver-color={}
ring-ver-color={}
inside-wrong-color={}
ring-wrong-color={}"#,
            p.background().raw_hex_rgb(),
            p.background().raw_hex_rgb(),
            p.yellow().raw_hex_rgb(),
            p.red().raw_hex_rgb(),
            p.red().raw_hex_rgb(),
            p.green().raw_hex_rgb(),
            p.black().raw_hex_rgb(),
            p.blue().raw_hex_rgb(),
            p.blue().raw_hex_rgb(),
            p.red().raw_hex_rgb(),
            p.red().raw_hex_rgb(),
        )
    }
}
