use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::hyprland::hyprctl::clients::Client;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Rectangle(i64, i64, i64, i64);

impl Rectangle {
    pub fn get_dimensions(&self) -> Option<(i64, i64, i64, i64)> {
        match self.is_zero_size() {
            true => None,
            false => Some((self.0, self.1, self.2, self.3)),
        }
    }

    pub fn is_zero_size(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0
    }
}

impl From<Client> for Rectangle {
    fn from(v: Client) -> Self {
        Self(
            *v.at.first().unwrap_or(&0),
            *v.at.get(1).unwrap_or(&0),
            *v.size.first().unwrap_or(&0),
            *v.size.get(1).unwrap_or(&0),
        )
    }
}

impl FromStr for Rectangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let p1 = split.next().ok_or(())?;
        let p2 = split.next().ok_or(())?;
        let p3 = split.next().ok_or(())?;
        let p4 = split.next().ok_or(())?;

        Ok(Self(
            p1.parse().map_err(|_| ())?,
            p2.parse().map_err(|_| ())?,
            p3.parse().map_err(|_| ())?,
            p4.parse().map_err(|_| ())?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use std::str::FromStr;

    #[test]
    fn rect_from_correct_string() {
        let s = "200 30 400 300";
        let rect = Rectangle::from_str(s);
        assert!(rect.is_ok());
        let rect = rect.unwrap();
        assert_eq!(rect.0, 200);
        assert_eq!(rect.1, 30);
        assert_eq!(rect.2, 400);
        assert_eq!(rect.3, 300);
    }

    #[test]
    fn rect_from_incorrect_string() {
        let s = "200 30 400";
        let rect = Rectangle::from_str(s);
        assert!(rect.is_err());
    }
}
