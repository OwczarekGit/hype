use lib_theme::core::themeable::Themeable;

use crate::create_config_dir_and_file;

pub struct Mako;

impl Themeable for Mako {
    fn path(&self) -> std::path::PathBuf {
        create_config_dir_and_file!("mako", "config")
    }

    fn content(&self, theme: &lib_theme::core::theme::Theme) -> String {
        format!(
            r#"background-color={}
text-color={}
default-timeout=5000

[urgency=low]
border-color={}

[urgency=normal]
border-color={}

[urgency=critical]
border-color={}
default-timeout=0
"#,
            theme.palette().background().hex_rgb(),
            theme.palette().foreground().hex_rgb(),
            theme.palette().black().hex_rgb(),
            theme.palette().bright_black().hex_rgb(),
            theme.palette().bright_red().hex_rgb(),
        )
    }

    fn run_post_apply_action(&self) {
        let _ = std::process::Command::new("makoctl").arg("reload").spawn();
    }
}
