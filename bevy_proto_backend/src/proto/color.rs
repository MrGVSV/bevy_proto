use bevy::reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy::render::color::Color;
use serde::{Deserialize, Serialize};

/// A stand-in for [`Color`] that is easier to use in prototype files.
///
/// This enum contains all the variants of `Color` as well
/// as all of its associated color constants, such as [`Red`] and [`AliceBlue`].
///
/// [`Red`]: ProtoColor::Red
/// [`AliceBlue`]: ProtoColor::AliceBlue
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[reflect(Default, PartialEq, Serialize, Deserialize)]
pub enum ProtoColor {
    /// sRGBA color
    Rgba {
        /// Red channel. [0.0, 1.0]
        red: f32,
        /// Green channel. [0.0, 1.0]
        green: f32,
        /// Blue channel. [0.0, 1.0]
        blue: f32,
        /// Alpha channel. [0.0, 1.0]
        alpha: f32,
    },
    /// RGBA color in the Linear sRGB colorspace (often colloquially referred to as "linear", "RGB", or "linear RGB").
    RgbaLinear {
        /// Red channel. [0.0, 1.0]
        red: f32,
        /// Green channel. [0.0, 1.0]
        green: f32,
        /// Blue channel. [0.0, 1.0]
        blue: f32,
        /// Alpha channel. [0.0, 1.0]
        alpha: f32,
    },
    /// HSL (hue, saturation, lightness) color with an alpha channel
    Hsla {
        /// Hue channel. [0.0, 360.0]
        hue: f32,
        /// Saturation channel. [0.0, 1.0]
        saturation: f32,
        /// Lightness channel. [0.0, 1.0]
        lightness: f32,
        /// Alpha channel. [0.0, 1.0]
        alpha: f32,
    },
    /// LCH(ab) (lightness, chroma, hue) color with an alpha channel
    Lcha {
        /// Lightness channel. [0.0, 1.5]
        lightness: f32,
        /// Chroma channel. [0.0, 1.5]
        chroma: f32,
        /// Hue channel. [0.0, 360.0]
        hue: f32,
        /// Alpha channel. [0.0, 1.0]
        alpha: f32,
    },
    /// <div style="background-color:rgb(94%, 97%, 100%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    AliceBlue,
    /// <div style="background-color:rgb(98%, 92%, 84%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    AntiqueWhite,
    /// <div style="background-color:rgb(49%, 100%, 83%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Aquamarine,
    /// <div style="background-color:rgb(94%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Azure,
    /// <div style="background-color:rgb(96%, 96%, 86%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Beige,
    /// <div style="background-color:rgb(100%, 89%, 77%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Bisque,
    /// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Black,
    /// <div style="background-color:rgb(0%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Blue,
    /// <div style="background-color:rgb(86%, 8%, 24%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Crimson,
    /// <div style="background-color:rgb(0%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Cyan,
    /// <div style="background-color:rgb(25%, 25%, 25%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    DarkGray,
    /// <div style="background-color:rgb(0%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    DarkGreen,
    /// <div style="background-color:rgb(100%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Fuchsia,
    /// <div style="background-color:rgb(100%, 84%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Gold,
    /// <div style="background-color:rgb(50%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Gray,
    /// <div style="background-color:rgb(0%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Green,
    /// <div style="background-color:rgb(28%, 0%, 51%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Indigo,
    /// <div style="background-color:rgb(20%, 80%, 20%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    LimeGreen,
    /// <div style="background-color:rgb(50%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Maroon,
    /// <div style="background-color:rgb(10%, 10%, 44%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    MidnightBlue,
    /// <div style="background-color:rgb(0%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Navy,
    /// <div style="background-color:rgba(0%, 0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    None,
    /// <div style="background-color:rgb(50%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Olive,
    /// <div style="background-color:rgb(100%, 65%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Orange,
    /// <div style="background-color:rgb(100%, 27%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    OrangeRed,
    /// <div style="background-color:rgb(100%, 8%, 57%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Pink,
    /// <div style="background-color:rgb(50%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Purple,
    /// <div style="background-color:rgb(100%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Red,
    /// <div style="background-color:rgb(98%, 50%, 45%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Salmon,
    /// <div style="background-color:rgb(18%, 55%, 34%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    SeaGreen,
    /// <div style="background-color:rgb(75%, 75%, 75%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Silver,
    /// <div style="background-color:rgb(0%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Teal,
    /// <div style="background-color:rgb(100%, 39%, 28%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Tomato,
    /// <div style="background-color:rgb(25%, 88%, 82%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Turquoise,
    /// <div style="background-color:rgb(93%, 51%, 93%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Violet,
    /// <div style="background-color:rgb(100%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    White,
    /// <div style="background-color:rgb(100%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    Yellow,
    /// <div style="background-color:rgb(60%, 80%, 20%); width: 10px; padding: 10px; border: 1px solid;" ></div>
    YellowGreen,
}

impl From<ProtoColor> for Color {
    fn from(value: ProtoColor) -> Self {
        match value {
            ProtoColor::Rgba {
                red,
                green,
                blue,
                alpha,
            } => Self::Rgba {
                red,
                green,
                blue,
                alpha,
            },
            ProtoColor::RgbaLinear {
                red,
                green,
                blue,
                alpha,
            } => Self::RgbaLinear {
                red,
                green,
                blue,
                alpha,
            },
            ProtoColor::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => Self::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            },
            ProtoColor::Lcha {
                lightness,
                chroma,
                hue,
                alpha,
            } => Self::Lcha {
                lightness,
                chroma,
                hue,
                alpha,
            },
            ProtoColor::AliceBlue => Self::ALICE_BLUE,
            ProtoColor::AntiqueWhite => Self::ANTIQUE_WHITE,
            ProtoColor::Aquamarine => Self::AQUAMARINE,
            ProtoColor::Azure => Self::AZURE,
            ProtoColor::Beige => Self::BEIGE,
            ProtoColor::Bisque => Self::BISQUE,
            ProtoColor::Black => Self::BLACK,
            ProtoColor::Blue => Self::BLUE,
            ProtoColor::Crimson => Self::CRIMSON,
            ProtoColor::Cyan => Self::CYAN,
            ProtoColor::DarkGray => Self::DARK_GRAY,
            ProtoColor::DarkGreen => Self::DARK_GREEN,
            ProtoColor::Fuchsia => Self::FUCHSIA,
            ProtoColor::Gold => Self::GOLD,
            ProtoColor::Gray => Self::GRAY,
            ProtoColor::Green => Self::GREEN,
            ProtoColor::Indigo => Self::INDIGO,
            ProtoColor::LimeGreen => Self::LIME_GREEN,
            ProtoColor::Maroon => Self::MAROON,
            ProtoColor::MidnightBlue => Self::MIDNIGHT_BLUE,
            ProtoColor::Navy => Self::NAVY,
            ProtoColor::None => Self::NONE,
            ProtoColor::Olive => Self::OLIVE,
            ProtoColor::Orange => Self::ORANGE,
            ProtoColor::OrangeRed => Self::ORANGE_RED,
            ProtoColor::Pink => Self::PINK,
            ProtoColor::Purple => Self::PURPLE,
            ProtoColor::Red => Self::RED,
            ProtoColor::Salmon => Self::SALMON,
            ProtoColor::SeaGreen => Self::SEA_GREEN,
            ProtoColor::Silver => Self::SILVER,
            ProtoColor::Teal => Self::TEAL,
            ProtoColor::Tomato => Self::TOMATO,
            ProtoColor::Turquoise => Self::TURQUOISE,
            ProtoColor::Violet => Self::VIOLET,
            ProtoColor::White => Self::WHITE,
            ProtoColor::Yellow => Self::YELLOW,
            ProtoColor::YellowGreen => Self::YELLOW_GREEN,
        }
    }
}

impl From<Color> for ProtoColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgba {
                red,
                green,
                blue,
                alpha,
            } => Self::Rgba {
                red,
                green,
                blue,
                alpha,
            },
            Color::RgbaLinear {
                red,
                green,
                blue,
                alpha,
            } => Self::RgbaLinear {
                red,
                green,
                blue,
                alpha,
            },
            Color::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => Self::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            },
            Color::Lcha {
                lightness,
                chroma,
                hue,
                alpha,
            } => Self::Lcha {
                lightness,
                chroma,
                hue,
                alpha,
            },
        }
    }
}

impl Default for ProtoColor {
    fn default() -> Self {
        Self::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }
}
