//! Discord colours.
//! Discord stores colours as a 24-bit RGB integer.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Represents a Discord colour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Colour(u32);

/// American spelling of [`Colour`].
pub type Color = Colour;

impl Colour {
    /// The default colour (`#000000`).
    pub const DEFAULT: Self = Self(0);

    /// Teal (`#1ABC9C`).
    pub const TEAL: Self = Self(0x1ABC9C);
    /// Dark teal (`#11806A`).
    pub const DARK_TEAL: Self = Self(0x11806A);
    /// Brand green (`#57F287`).
    pub const BRAND_GREEN: Self = Self(0x57F287);
    /// Green (`#2ECC71`).
    pub const GREEN: Self = Self(0x2ECC71);
    /// Dark green (`#1F8B4C`).
    pub const DARK_GREEN: Self = Self(0x1F8B4C);
    /// Blue (`#3498DB`).
    pub const BLUE: Self = Self(0x3498DB);
    /// Dark blue (`#206694`).
    pub const DARK_BLUE: Self = Self(0x206694);
    /// Purple (`#9B59B6`).
    pub const PURPLE: Self = Self(0x9B59B6);
    /// Dark purple (`#71368A`).
    pub const DARK_PURPLE: Self = Self(0x71368A);
    /// Magenta (`#E91E63`).
    pub const MAGENTA: Self = Self(0xE91E63);
    /// Dark magenta (`#AD1457`).
    pub const DARK_MAGENTA: Self = Self(0xAD1457);
    /// Gold (`#F1C40F`).
    pub const GOLD: Self = Self(0xF1C40F);
    /// Dark gold (`#C27C0E`).
    pub const DARK_GOLD: Self = Self(0xC27C0E);
    /// Orange (`#E67E22`).
    pub const ORANGE: Self = Self(0xE67E22);
    /// Dark orange (`#A84300`).
    pub const DARK_ORANGE: Self = Self(0xA84300);
    /// Brand red (`#ED4245`).
    pub const BRAND_RED: Self = Self(0xED4245);
    /// Red (`#E74C3C`).
    pub const RED: Self = Self(0xE74C3C);
    /// Dark red (`#992D22`).
    pub const DARK_RED: Self = Self(0x992D22);
    /// Lighter grey (`#95A5A6`).
    pub const LIGHTER_GREY: Self = Self(0x95A5A6);
    /// Lighter gray, alias of [`Colour::LIGHTER_GREY`].
    pub const LIGHTER_GRAY: Self = Self::LIGHTER_GREY;
    /// Dark grey (`#607D8B`).
    pub const DARK_GREY: Self = Self(0x607D8B);
    /// Dark gray, alias of [`Colour::DARK_GREY`].
    pub const DARK_GRAY: Self = Self::DARK_GREY;
    /// Light grey (`#979C9F`).
    pub const LIGHT_GREY: Self = Self(0x979C9F);
    /// Light gray, alias of [`Colour::LIGHT_GREY`].
    pub const LIGHT_GRAY: Self = Self::LIGHT_GREY;
    /// Darker grey (`#546E7A`).
    pub const DARKER_GREY: Self = Self(0x546E7A);
    /// Darker gray, alias of [`Colour::DARKER_GREY`].
    pub const DARKER_GRAY: Self = Self::DARKER_GREY;
    /// Old blurple (`#7289DA`).
    pub const OG_BLURPLE: Self = Self(0x7289DA);
    /// Blurple (`#5865F2`).
    pub const BLURPLE: Self = Self(0x5865F2);
    /// Greyple (`#99AAB5`).
    pub const GREYPLE: Self = Self(0x99AAB5);
    /// Ash theme (`#2E2E34`).
    pub const ASH_THEME: Self = Self(0x2E2E34);
    /// Dark theme (`#1A1A1E`).
    pub const DARK_THEME: Self = Self(0x1A1A1E);
    /// Onyx theme (`#070709`).
    pub const ONYX_THEME: Self = Self(0x070709);
    /// Light theme (`#FBFBFB`).
    pub const LIGHT_THEME: Self = Self(0xFBFBFB);
    /// Fuchsia (`#EB459E`).
    pub const FUCHSIA: Self = Self(0xEB459E);
    /// Yellow (`#FEE75C`).
    pub const YELLOW: Self = Self(0xFEE75C);
    /// Ash embed (`#37373E`).
    pub const ASH_EMBED: Self = Self(0x37373E);
    /// Dark embed (`#242429`).
    pub const DARK_EMBED: Self = Self(0x242429);
    /// Onyx embed (`#131416`).
    pub const ONYX_EMBED: Self = Self(0x131416);
    /// Light embed (`#FFFFFF`).
    pub const LIGHT_EMBED: Self = Self(0xFFFFFF);
    /// Pink (`#EB459F`).
    pub const PINK: Self = Self(0xEB459F);

    /// Creates a colour from its raw 24-bit value.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw 24-bit value.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Returns the red component.
    #[must_use]
    pub const fn r(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Returns the green component.
    #[must_use]
    pub const fn g(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Returns the blue component.
    #[must_use]
    pub const fn b(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns the `(red, green, blue)` components.
    #[must_use]
    pub const fn to_rgb(self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }

    /// Builds a colour from its red, green and blue components.
    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | b as u32)
    }

    /// Returns the default colour (`#000000`).
    #[must_use]
    pub const fn default_colour() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:06x}", self.0)
    }
}

impl From<u32> for Colour {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Colour> for u32 {
    fn from(colour: Colour) -> Self {
        colour.value()
    }
}

impl FromStr for Colour {
    type Err = ColourError;

    /// Parses `0x<hex>`, `#<hex>`, `0x#<hex>`, or `rgb(r, g, b)`.
    /// Hex values may use the three-digit shortcut. RGB components may be
    /// 0-255 or percentages, like CSS.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(ColourError::Empty);
        }

        if let Some(hex) = value.strip_prefix('#') {
            return parse_hex(hex);
        }

        if let Some(rest) = value.strip_prefix("0x") {
            let hex = rest.strip_prefix('#').unwrap_or(rest);
            return parse_hex(hex);
        }

        if value.to_ascii_lowercase().starts_with("rgb") {
            return parse_rgb(value);
        }

        Err(ColourError::Invalid)
    }
}

/// Error returned when a string cannot be parsed as a colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColourError {
    /// The input was empty.
    Empty,
    /// The input was not a supported colour format.
    Invalid,
}

impl fmt::Display for ColourError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("unknown colour format given"),
            Self::Invalid => f.write_str("invalid colour"),
        }
    }
}

impl Error for ColourError {}

fn parse_hex(value: &str) -> Result<Colour, ColourError> {
    let expanded = if value.len() == 3 {
        value
            .chars()
            .flat_map(|character| [character, character])
            .collect()
    } else {
        value.to_owned()
    };

    let raw = u32::from_str_radix(&expanded, 16).map_err(|_| ColourError::Invalid)?;
    if raw > 0xFF_FFFF {
        return Err(ColourError::Invalid);
    }

    Ok(Colour(raw))
}

fn parse_rgb(value: &str) -> Result<Colour, ColourError> {
    let lower = value.to_ascii_lowercase();
    let inner = lower
        .strip_prefix("rgb")
        .and_then(|rest| rest.trim().strip_prefix('('))
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or(ColourError::Invalid)?;

    let components: Vec<&str> = inner.split(',').map(str::trim).collect();
    if components.len() != 3 {
        return Err(ColourError::Invalid);
    }

    let r = parse_component(components[0])?;
    let g = parse_component(components[1])?;
    let b = parse_component(components[2])?;

    Ok(Colour::from_rgb(r, g, b))
}

fn parse_component(value: &str) -> Result<u8, ColourError> {
    if let Some(percent) = value.strip_suffix('%') {
        let percent: f64 = percent.parse().map_err(|_| ColourError::Invalid)?;
        if !(0.0..=100.0).contains(&percent) {
            return Err(ColourError::Invalid);
        }
        Ok((255.0 * (percent / 100.0)).round() as u8)
    } else {
        let number: u16 = value.parse().map_err(|_| ColourError::Invalid)?;
        if number > 255 {
            return Err(ColourError::Invalid);
        }
        Ok(number as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::{Colour, ColourError};

    #[test]
    fn components_and_hex_display() {
        let colour = Colour::from_rgb(0x12, 0x34, 0x56);

        assert_eq!(colour.value(), 0x12_3456);
        assert_eq!(colour.r(), 0x12);
        assert_eq!(colour.g(), 0x34);
        assert_eq!(colour.b(), 0x56);
        assert_eq!(colour.to_rgb(), (0x12, 0x34, 0x56));
        assert_eq!(colour.to_string(), "#123456");
        assert_eq!(u32::from(colour), 0x12_3456);
        assert_eq!(Colour::from(0x12_3456), colour);
    }

    #[test]
    fn parses_hex_formats() {
        assert_eq!(
            "#123456".parse::<Colour>(),
            Ok(Colour::from_rgb(0x12, 0x34, 0x56))
        );
        assert_eq!(
            "0x123456".parse::<Colour>(),
            Ok(Colour::from_rgb(0x12, 0x34, 0x56))
        );
        assert_eq!(
            "0x#123456".parse::<Colour>(),
            Ok(Colour::from_rgb(0x12, 0x34, 0x56))
        );
        assert_eq!(
            "#fff".parse::<Colour>(),
            Ok(Colour::from_rgb(255, 255, 255))
        );
        assert_eq!("".parse::<Colour>(), Err(ColourError::Empty));
        assert_eq!("nope".parse::<Colour>(), Err(ColourError::Invalid));
        assert_eq!("#1000000".parse::<Colour>(), Err(ColourError::Invalid));
    }

    #[test]
    fn parses_rgb_formats() {
        assert_eq!(
            "rgb(18, 52, 86)".parse::<Colour>(),
            Ok(Colour::from_rgb(18, 52, 86))
        );
        assert_eq!(
            "RGB(100%, 0%, 50%)".parse::<Colour>(),
            Ok(Colour::from_rgb(255, 0, 128))
        );
        assert_eq!(
            "rgb(256, 0, 0)".parse::<Colour>(),
            Err(ColourError::Invalid)
        );
        assert_eq!("rgb(1, 2)".parse::<Colour>(), Err(ColourError::Invalid));
    }

    #[test]
    fn named_colours_match_discord_py() {
        assert_eq!(Colour::DEFAULT.value(), 0);
        assert_eq!(Colour::BLURPLE.value(), 0x5865F2);
        assert_eq!(Colour::BRAND_GREEN.value(), 0x57F287);
        assert_eq!(Colour::PINK.value(), 0xEB459F);
        assert_eq!(Colour::DARK_GRAY, Colour::DARK_GREY);
        assert_eq!(Colour::default(), Colour::DEFAULT);
        assert_eq!(Colour::default_colour(), Colour::DEFAULT);
    }
}
