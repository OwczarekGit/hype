use std::path::PathBuf;

use arguments::Arguments;
use chrono::{Datelike, Local, Timelike};
use clap::Parser;
use lib_hype::core::dirs::hype_config_dir;
use lib_hype::theme::core::theme::Theme;
use lib_hype::core::screenshot::screenshoter::Screenshoter;
use lib_hype::{
    core::{
        dirs::THEMES_CONFIG_FILE,
        notification::{Notification, Urgency},
        rectangle::Rectangle,
        screenshot::{grim::Grim, slurp::Slurp},
    },
    hyprland::hyprctl::monitors::Monitor,
};
mod arguments;

fn main() {
    let args = Arguments::parse();
    let slurp = get_slurp().unwrap_or_default();
    dbg!(&slurp);
    let grim = Grim;

    match args.command {
        arguments::Command::Selection { output } => {
            let path = resolve_output_path(output);
            let rect = slurp.select_rectangle().expect("Rectangle to be selected");
            if !rect.is_zero_size() && grim.screenshot_rect(rect, &path).is_ok() {
                Notification::send("Screenshot saved", path.to_str().unwrap(), Urgency::Low);
            }
        }
        arguments::Command::Monitor { output } => {
            let Ok(p) = slurp.select_point() else {
                return;
            };

            let mon = Monitor::get_all().expect("The list of monitors.");
            let monitor = mon
                .into_iter()
                .find(|m| p.inside(&Rectangle::from(m.clone())));

            if let Some(monitor) = monitor {
                let path = resolve_output_path(output);
                if grim.screenshot_rect(Rectangle::from(monitor), &path).is_ok() {
                    Notification::send("Screenshot saved", path.to_str().unwrap(), Urgency::Low);
                }
            }
        }
    }
}

pub fn resolve_output_path(path: Option<PathBuf>) -> PathBuf {
    match path {
        None => {
            let mut dir = default_screenshot_directory();
            dir.push(default_file_name("screenshot"));
            dir
        }
        Some(path) => path,
    }
}

pub fn default_screenshot_directory() -> PathBuf {
    let home = env!("HOME");
    let dir = PathBuf::from(format!("{home}/Pictures/screenshots"));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

pub fn default_file_name(suf: &str) -> String {
    let now = Local::now();
    format!(
        "{:02}-{:02}-{:04}_{:02}-{:02}-{:02}_{}.png",
        now.day(),
        now.month(),
        now.year(),
        now.hour(),
        now.minute(),
        now.second(),
        suf,
    )
}

pub fn get_slurp() -> Option<Slurp> {
    let mut path = hype_config_dir();
    path.push(THEMES_CONFIG_FILE);
    let text = std::fs::read_to_string(path).ok()?;
    Some(Slurp::with_theme(toml::from_str::<Theme>(&text).ok()?))
}
