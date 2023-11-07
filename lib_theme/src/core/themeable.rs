use std::io::Write;
use std::path::PathBuf;

use super::theme::Theme;

pub trait Themeable {
    fn path(&self) -> PathBuf;
    fn content(&self, theme: Theme) -> String;
    fn apply(&self, theme: Theme) {
        let mut file = std::fs::File::create(self.path()).unwrap();
        let _ = file.write_all(self.content(theme).as_bytes());
    }
}
