use core::panic;

use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Color(u32);

impl Color {
    pub fn r(&self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn g(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn b(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn a(&self) -> u8 {
        self.0 as u8
    }

    pub fn raw_hex_rgb(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r(), self.g(), self.b())
    }

    pub fn hex_rgb(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r(), self.g(), self.b())
    }

    pub fn hex_rgba(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }

    pub fn hyprland_rgba(&self) -> String {
        format!(
            "rgba({:02X}{:02X}{:02X}{:02X})",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        let combined =
            ((value.0 as u32) << 24) | ((value.1 as u32) << 16) | ((value.2 as u32) << 8) | 0xFF;

        Self(combined)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        let combined = ((value.0 as u32) << 24)
            | ((value.1 as u32) << 16)
            | ((value.2 as u32) << 8)
            | (value.3 as u32);

        Self(combined)
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        let combined = ((value as u32) << 24)
            | ((value as u32) << 16)
            | ((value as u32) << 8)
            | (value as u32);

        Self(combined)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let combined =
            (value & 0xFF000000) | (value & 0x00FF0000) | (value & 0x0000FF00) | (value & 0xFF);

        Self(combined)
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Self {
        let value = value as u32;
        let combined =
            (value & 0xFF000000) | (value & 0x00FF0000) | (value & 0x0000FF00) | (value & 0xFF);

        Self(combined)
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let mut v = value.replace('#', "");

        match v.len() {
            8 => {}
            6 => v += "FF",
            _ => panic!("Invalid hex string '{}'.", v),
        };

        u32::from_str_radix(&v, 16)
            .expect("Invalid hex code")
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_r() {
        let c = Color(0xABCDEF00);
        assert_eq!(c.r(), 0xAB)
    }

    #[test]
    fn get_g() {
        let c = Color(0xABCDEF00);
        assert_eq!(c.g(), 0xCD)
    }

    #[test]
    fn get_b() {
        let c = Color(0xABCDEF00);
        assert_eq!(c.b(), 0xEF)
    }

    #[test]
    fn get_a() {
        let c = Color(0xABCDEFCB);
        assert_eq!(c.a(), 0xCB)
    }

    #[test]
    fn from_touple_3x_u8() {
        let t = (255, 128, 64);
        let c: Color = t.into();

        assert_eq!(c.r(), t.0);
        assert_eq!(c.g(), t.1);
        assert_eq!(c.b(), t.2);
        assert_eq!(c.a(), 255);
    }

    #[test]
    fn from_touple_4x_u8() {
        let t = (255, 128, 64, 32);
        let c: Color = t.into();

        assert_eq!(c.r(), t.0);
        assert_eq!(c.g(), t.1);
        assert_eq!(c.b(), t.2);
        assert_eq!(c.a(), t.3);
    }

    #[test]
    fn from_u8_grayscale() {
        let t = 123;
        let c: Color = t.into();

        assert_eq!(c.r(), t);
        assert_eq!(c.g(), t);
        assert_eq!(c.b(), t);
        assert_eq!(c.a(), t);
    }

    #[test]
    fn from_u32() {
        let c: Color = 0xABCDEF22_u32.into();
        assert_eq!(c.r(), 0xAB);
        assert_eq!(c.g(), 0xCD);
        assert_eq!(c.b(), 0xEF);
        assert_eq!(c.a(), 0x22);
    }

    #[test]
    fn get_hex_rgb() {
        let c: Color = 0xABFFCD00_u32.into();
        assert_eq!(c.hex_rgb(), "#ABFFCD")
    }

    #[test]
    fn get_hex_rgba() {
        let c: Color = 0xABFFCD44_u32.into();
        assert_eq!(c.hex_rgba(), "#ABFFCD44")
    }

    #[test]
    fn from_string_with_hash_in_front() {
        let c: Color = "#ABCDEF".into();
        assert_eq!(c.r(), 0xAB);
        assert_eq!(c.g(), 0xCD);
        assert_eq!(c.b(), 0xEF);
        assert_eq!(c.a(), 0xFF);
    }

    #[test]
    fn from_string_with_hash_in_front_with_alpha() {
        let c: Color = "#ABCDEF44".into();
        assert_eq!(c.r(), 0xAB);
        assert_eq!(c.g(), 0xCD);
        assert_eq!(c.b(), 0xEF);
        assert_eq!(c.a(), 0x44);
    }

    #[test]
    fn from_string_without_hash_in_front() {
        let c: Color = "FDAACC".into();
        assert_eq!(c.r(), 0xFD);
        assert_eq!(c.g(), 0xAA);
        assert_eq!(c.b(), 0xCC);
        assert_eq!(c.a(), 0xFF);
    }

    #[test]
    fn from_string_without_hash_in_front_with_alpha() {
        let c: Color = "FDAACC22".into();
        assert_eq!(c.r(), 0xFD);
        assert_eq!(c.g(), 0xAA);
        assert_eq!(c.b(), 0xCC);
        assert_eq!(c.a(), 0x22);
    }
}
