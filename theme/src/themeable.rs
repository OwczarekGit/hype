use std::path::PathBuf;
use lib_theme::core::theme::Theme;
use std::io::Write;

pub trait Themeable {
    fn path(&self) -> PathBuf;
    fn content(&self, theme: Theme) -> String;
    fn apply(&self, theme: Theme) {
        let mut file = std::fs::File::create(self.path()).unwrap();
        let _ = file.write_all(self.content(theme).as_bytes());
    }
}