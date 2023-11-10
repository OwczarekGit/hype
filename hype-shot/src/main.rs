use std::path::PathBuf;

use arguments::Arguments;
use chrono::{Datelike, Local, Timelike};
use clap::Parser;
use lib_hype::{slurp::Slurp, grim::Grim};

mod arguments;

fn main() {
    let args = Arguments::parse();
    let slurp = Slurp;

    match args.command {
        arguments::Command::ScreenshotRect { output } => {
            let path = resolve_output_path(output);
            let rect = slurp.select_rectangle().expect("Rectangle to be selected");
            if !rect.is_zero_size() {
                let _ = Grim::screenshot_rect(rect, &path);
            }
        }
        arguments::Command::Screenshot { output } => todo!(),
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
