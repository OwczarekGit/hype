pub trait Screenshoter {
    fn screenshot_rect(
        &self,
        rectangle: impl Into<(i64, i64, i64, i64)>,
        path: &std::path::Path,
    ) -> Result<(), std::io::Error>;

    fn screenshot_all(&self, path: &std::path::Path) -> Result<(), std::io::Error>;
}
