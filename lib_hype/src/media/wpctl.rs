use super::WpCtlError;
use std::process::Command;

pub const VOLUME_MAX: u32 = 150;
pub const VOLUME_MIN: u32 = 0;
pub const VOLUME_DEFAULT_STEP: u32 = 3;

pub struct WpCtl;

impl WpCtl {
    pub fn mute_output() -> Result<(), WpCtlError> {
        let mut cmd = Command::new("wpctl");
        cmd.arg("set-mute");
        cmd.arg("@DEFAULT_SINK@");
        cmd.arg("1");
        let _ = cmd.output()?;
        Ok(())
    }

    pub fn unmute_output() -> Result<(), WpCtlError> {
        let mut cmd = Command::new("wpctl");
        cmd.arg("set-mute");
        cmd.arg("@DEFAULT_SINK@");
        cmd.arg("0");
        let _ = cmd.output()?;
        Ok(())
    }

    pub fn toggle_mute_output() -> Result<(), WpCtlError> {
        let mut cmd = Command::new("wpctl");
        cmd.arg("set-mute");
        cmd.arg("@DEFAULT_SINK@");
        cmd.arg("toggle");
        let _ = cmd.output()?;
        Ok(())
    }

    pub fn increase_volume(value: u32) -> Result<(), WpCtlError> {
        let mut cmd = Command::new("wpctl");
        cmd.arg("set-volume");
        cmd.arg("@DEFAULT_SINK@");
        cmd.arg(&format!("{}%+", value.clamp(VOLUME_MIN, VOLUME_MAX)));
        let _ = cmd.output()?;
        Ok(())
    }

    pub fn decrease_volume(value: u32) -> Result<(), WpCtlError> {
        let mut cmd = Command::new("wpctl");
        cmd.arg("set-volume");
        cmd.arg("@DEFAULT_SINK@");
        cmd.arg(&format!("{}%-", value.clamp(VOLUME_MIN, VOLUME_MAX)));
        let _ = cmd.output()?;
        Ok(())
    }

    pub fn set_volume(value: u32) -> Result<(), WpCtlError> {
        let mut cmd = Command::new("wpctl");
        cmd.arg("set-volume");
        cmd.arg("@DEFAULT_SINK@");
        cmd.arg(&format!("{}%", value.clamp(VOLUME_MIN, VOLUME_MAX)));
        let _ = cmd.output()?;
        Ok(())
    }

    pub fn get_volume() -> Result<f32, WpCtlError> {
        let mut cmd = Command::new("wpctl");
        cmd.arg("get-volume");
        cmd.arg("@DEFAULT_SINK@");

        let cmd = cmd.output()?.stdout;
        let text = String::from_utf8(cmd)?;

        let mut split = text.split(' ');
        let _ = split.next();
        let volume: f32 = split
            .next()
            .ok_or(WpCtlError::MissingVolume)?
            .trim()
            .parse()
            .map_err(|_| WpCtlError::ParseFloat)?;

        Ok(volume)
    }
}
