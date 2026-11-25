//! RGBA colors: parsing, formatting, and contrast math.

use std::fmt;
use std::str::FromStr;

use anyhow::{Context, bail};
use serde::{Deserialize, Deserializer, de};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub const fn with_alpha(self, a: u8) -> Color {
        Color { a, ..self }
    }

    /// `#RRGGBBAA`, uppercase (Zed's convention).
    pub fn hex_rgba(self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    /// `#RRGGBB`, uppercase; alpha is dropped.
    pub fn hex_rgb(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// WCAG 2.x relative luminance (alpha ignored).
    pub fn luminance(self) -> f64 {
        fn channel(c: u8) -> f64 {
            let c = f64::from(c) / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * channel(self.r) + 0.7152 * channel(self.g) + 0.0722 * channel(self.b)
    }

    /// WCAG 2.x contrast ratio, in `1.0..=21.0`.
    pub fn contrast(self, other: Color) -> f64 {
        let (a, b) = (self.luminance(), other.luminance());
        let (hi, lo) = if a > b { (a, b) } else { (b, a) };
        (hi + 0.05) / (lo + 0.05)
    }
}

impl FromStr for Color {
    type Err = anyhow::Error;

    /// Accepts `#RRGGBB` or `#RRGGBBAA` (case-insensitive).
    fn from_str(s: &str) -> anyhow::Result<Color> {
        let Some(hex) = s.strip_prefix('#') else {
            bail!("expected a hex color like #RRGGBB, got {s:?}");
        };
        if !matches!(hex.len(), 6 | 8) || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            bail!("expected #RRGGBB or #RRGGBBAA, got {s:?}");
        }
        let byte = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).expect("validated hex");
        Ok(Color {
            r: byte(0),
            g: byte(2),
            b: byte(4),
            a: if hex.len() == 8 { byte(6) } else { 0xFF },
        })
    }
}

/// Parse an alpha suffix (`"80"`) as a hex byte.
pub fn parse_alpha(s: &str) -> anyhow::Result<u8> {
    if s.len() != 2 {
        bail!("alpha must be two hex digits (00-FF), got {s:?}");
    }
    u8::from_str_radix(s, 16)
        .with_context(|| format!("alpha must be two hex digits (00-FF), got {s:?}"))
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hex_rgba())
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Color, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(|e: anyhow::Error| de::Error::custom(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_formats() {
        let c: Color = "#88c0d0".parse().unwrap();
        assert_eq!(
            c,
            Color {
                r: 0x88,
                g: 0xC0,
                b: 0xD0,
                a: 0xFF
            }
        );
        assert_eq!(c.hex_rgba(), "#88C0D0FF");
        assert_eq!(c.hex_rgb(), "#88C0D0");
        assert_eq!("#445A7780".parse::<Color>().unwrap().a, 0x80);
    }

    #[test]
    fn rejects_malformed() {
        for bad in ["88C0D0", "#88C0D", "#88C0D0F", "#GGGGGG", "#88C0D0FF00"] {
            assert!(bad.parse::<Color>().is_err(), "{bad} should be rejected");
        }
    }

    #[test]
    fn contrast_extremes() {
        let black: Color = "#000000".parse().unwrap();
        let white: Color = "#FFFFFF".parse().unwrap();
        assert!((black.contrast(white) - 21.0).abs() < 1e-9);
        assert!((white.contrast(white) - 1.0).abs() < 1e-9);
    }
}
