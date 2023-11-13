use std::path::Path;

use super::screenshoter::Screenshoter;

pub struct Grim;

impl Screenshoter for Grim {
    fn screenshot_rect(
        &self,
        rect: impl Into<(i64, i64, i64, i64)>,
        output: &Path,
    ) -> Result<(), std::io::Error> {
        let rect = rect.into();

        std::process::Command::new("grim")
            .arg("-g")
            .arg(format!("{},{} {}x{}", rect.0, rect.1, rect.2, rect.3))
            .arg(output)
            .spawn()?;

        Ok(())
    }
}
