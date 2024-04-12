use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomStatus {
    option: String,
    float: f32,
    set: bool,
}

pub struct Zoom;

impl Zoom {
    pub fn get() -> f32 {
        let res = Command::new("hyprctl")
            .arg("getoption")
            .arg("misc:cursor_zoom_factor")
            .arg("-j")
            .output()
            .expect("Hyprctl error")
            .stdout;

        let s = String::from_utf8(res).expect("Invalid output");

        let zoom: ZoomStatus = serde_json::from_str(&s).expect("Invalid structure.");
        zoom.float
    }

    pub fn set(value: f32) {
        Command::new("hyprctl")
            .arg("keyword")
            .arg("misc:cursor_zoom_factor")
            .arg(value.max(1.0).to_string())
            .stdout(Stdio::null())
            .spawn()
            .expect("Hyprctl error");
    }

    pub fn zoom_in(delta: f32) {
        let zoom = Zoom::get();
        Zoom::set(zoom + delta)
    }

    pub fn zoom_out(delta: f32) {
        let zoom = Zoom::get();
        Zoom::set(zoom - delta)
    }
}
