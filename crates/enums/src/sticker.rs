//! Sticker enumerations.

use std::fmt;

/// The type of a sticker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum StickerType {
    /// A sticker from one of Discord's standard sticker packs.
    #[default]
    Standard = 1,
    /// A sticker uploaded to a guild.
    Guild = 2,
}

impl StickerType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Standard),
            2 => Some(Self::Guild),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Guild => "guild",
        }
    }
}

impl fmt::Display for StickerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The file format of a sticker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum StickerFormatType {
    /// A static PNG image.
    #[default]
    Png = 1,
    /// An animated PNG image.
    Apng = 2,
    /// A Lottie animation.
    Lottie = 3,
    /// An animated GIF image.
    Gif = 4,
}

impl StickerFormatType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Png),
            2 => Some(Self::Apng),
            3 => Some(Self::Lottie),
            4 => Some(Self::Gif),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Apng => "apng",
            Self::Lottie => "lottie",
            Self::Gif => "gif",
        }
    }

    /// Returns the file extension used by sticker CDN URLs.
    #[must_use]
    pub const fn file_extension(self) -> &'static str {
        match self {
            Self::Png | Self::Apng => "png",
            Self::Lottie => "json",
            Self::Gif => "gif",
        }
    }

    /// Returns `true` if the sticker uses the Lottie format.
    #[must_use]
    pub const fn is_lottie(self) -> bool {
        matches!(self, Self::Lottie)
    }
}

impl fmt::Display for StickerFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{StickerFormatType, StickerType};

    #[test]
    fn sticker_type_round_trips() {
        assert_eq!(StickerType::from_value(1), Some(StickerType::Standard));
        assert_eq!(StickerType::from_value(2), Some(StickerType::Guild));
        assert_eq!(StickerType::from_value(0), None);
        assert_eq!(StickerType::from_value(3), None);
        assert_eq!(StickerType::default(), StickerType::Standard);
        assert_eq!(StickerType::Guild.to_string(), "guild");
    }

    #[test]
    fn sticker_format_type_round_trips() {
        let values = [
            (1, StickerFormatType::Png),
            (2, StickerFormatType::Apng),
            (3, StickerFormatType::Lottie),
            (4, StickerFormatType::Gif),
        ];

        for (value, format) in values {
            assert_eq!(StickerFormatType::from_value(value), Some(format));
            assert_eq!(format.value(), value);
        }

        assert_eq!(StickerFormatType::from_value(5), None);
        assert_eq!(StickerFormatType::default(), StickerFormatType::Png);
    }

    #[test]
    fn format_extensions() {
        assert_eq!(StickerFormatType::Png.file_extension(), "png");
        assert_eq!(StickerFormatType::Apng.file_extension(), "png");
        assert_eq!(StickerFormatType::Lottie.file_extension(), "json");
        assert_eq!(StickerFormatType::Gif.file_extension(), "gif");
        assert!(StickerFormatType::Lottie.is_lottie());
        assert!(!StickerFormatType::Gif.is_lottie());
        assert_eq!(StickerFormatType::Gif.to_string(), "gif");
    }
}
