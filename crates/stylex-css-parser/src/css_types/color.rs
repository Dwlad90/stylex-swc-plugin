/*!
CSS Color type parsing.

Handles all CSS color formats: named colors, hex, rgb, rgba, hsl, hsla, and modern color spaces.
Mirrors: packages/style-value-parser/src/css-types/color.js
*/

use crate::{
    token_parser::TokenParser,
    token_types::SimpleToken,
    css_types::Angle
};
use std::fmt::{self, Display};

/// List of all valid CSS named colors
/// This is a comprehensive list of CSS Level 4 named colors
const NAMED_COLORS: &[&str] = &[
    "aliceblue", "antiquewhite", "aqua", "aquamarine", "azure", "beige", "bisque", "black",
    "blanchedalmond", "blue", "blueviolet", "brown", "burlywood", "cadetblue", "chartreuse",
    "chocolate", "coral", "cornflowerblue", "cornsilk", "crimson", "cyan", "darkblue", "darkcyan",
    "darkgoldenrod", "darkgray", "darkgreen", "darkgrey", "darkkhaki", "darkmagenta",
    "darkolivegreen", "darkorange", "darkorchid", "darkred", "darksalmon", "darkseagreen",
    "darkslateblue", "darkslategray", "darkslategrey", "darkturquoise", "darkviolet", "deeppink",
    "deepskyblue", "dimgray", "dimgrey", "dodgerblue", "firebrick", "floralwhite", "forestgreen",
    "fuchsia", "gainsboro", "ghostwhite", "gold", "goldenrod", "gray", "grey", "green",
    "greenyellow", "honeydew", "hotpink", "indianred", "indigo", "ivory", "khaki", "lavender",
    "lavenderblush", "lawngreen", "lemonchiffon", "lightblue", "lightcoral", "lightcyan",
    "lightgoldenrodyellow", "lightgray", "lightgreen", "lightgrey", "lightpink", "lightsalmon",
    "lightseagreen", "lightskyblue", "lightslategray", "lightslategrey", "lightsteelblue",
    "lightyellow", "lime", "limegreen", "linen", "magenta", "maroon", "mediumaquamarine",
    "mediumblue", "mediumorchid", "mediumpurple", "mediumseagreen", "mediumslateblue",
    "mediumspringgreen", "mediumturquoise", "mediumvioletred", "midnightblue", "mintcream",
    "mistyrose", "moccasin", "navajowhite", "navy", "oldlace", "olive", "olivedrab", "orange",
    "orangered", "orchid", "palegoldenrod", "palegreen", "paleturquoise", "palevioletred",
    "papayawhip", "peachpuff", "peru", "pink", "plum", "powderblue", "purple", "rebeccapurple",
    "red", "rosybrown", "royalblue", "saddlebrown", "salmon", "sandybrown", "seagreen", "seashell",
    "sienna", "silver", "skyblue", "slateblue", "slategray", "slategrey", "snow", "springgreen",
    "steelblue", "tan", "teal", "thistle", "tomato", "turquoise", "violet", "wheat", "white",
    "whitesmoke", "yellow", "yellowgreen", "transparent", "currentcolor",
];

/// Base Color trait that all color types implement
pub trait ColorTrait {
    fn to_string(&self) -> String;
}

/// Main Color enum that encompasses all color types
/// Mirrors: Color class in color.js
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Named(NamedColor),
    Hash(HashColor),
    Rgb(RgbColor),
    Rgba(RgbaColor),
    Hsl(HslColor),
    Hsla(HslaColor),
    Lch(LchColor),
    Oklch(OklchColor),
    Oklab(OklabColor),
}

impl Color {
    /// Parser for all color formats
    /// Mirrors: Color.parser
    pub fn parser() -> TokenParser<Color> {
        TokenParser::one_of(vec![
            NamedColor::parser().map(Color::Named, Some("named")),
            HashColor::parser().map(Color::Hash, Some("hash")),
            RgbColor::parser().map(Color::Rgb, Some("rgb")),
            RgbaColor::parser().map(Color::Rgba, Some("rgba")),
            HslColor::parser().map(Color::Hsl, Some("hsl")),
            HslaColor::parser().map(Color::Hsla, Some("hsla")),
            LchColor::parser().map(Color::Lch, Some("lch")),
            OklchColor::parser().map(Color::Oklch, Some("oklch")),
            OklabColor::parser().map(Color::Oklab, Some("oklab")),
        ])
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Named(named) => named.fmt(f),
            Color::Hash(hash) => hash.fmt(f),
            Color::Rgb(rgb) => rgb.fmt(f),
            Color::Rgba(rgba) => rgba.fmt(f),
            Color::Hsl(hsl) => hsl.fmt(f),
            Color::Hsla(hsla) => hsla.fmt(f),
            Color::Lch(lch) => lch.fmt(f),
            Color::Oklch(oklch) => oklch.fmt(f),
            Color::Oklab(oklab) => oklab.fmt(f),
        }
    }
}

/// Named color like "red", "blue", etc.
/// Mirrors: NamedColor class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedColor {
    pub value: String,
}

impl NamedColor {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /// Check if a string is a valid named color
    pub fn is_valid_named_color(name: &str) -> bool {
        NAMED_COLORS.contains(&name.to_lowercase().as_str())
    }

    /// Parser for named colors
    /// Mirrors: NamedColor.parser
    pub fn parser() -> TokenParser<NamedColor> {
        TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .map(
                |token| {
                    if let SimpleToken::Ident(value) = token {
                        value
                    } else {
                        unreachable!()
                    }
                },
                Some("extract_ident"),
            )
            .where_fn(
                |value| Self::is_valid_named_color(value),
                Some("valid_named_color"),
            )
            .map(NamedColor::new, Some("to_named_color"))
    }
}

impl Display for NamedColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Hex color like #FF0000, #F00, #FF0000FF
/// Mirrors: HashColor class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashColor {
    pub value: String, // Hex value without the #
}

impl HashColor {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /// Validate hex color format
    pub fn is_valid_hex(value: &str) -> bool {
        let valid_lengths = [3, 6, 8]; // 3-digit, 6-digit, 8-digit (with alpha)
        valid_lengths.contains(&value.len()) && value.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Get red component (0-255)
    pub fn r(&self) -> u8 {
        match self.value.len() {
            3 => {
                // 3-digit hex: #RGB -> expand to #RRGGBB
                let r_char = self.value.chars().next().unwrap();
                let expanded = format!("{}{}", r_char, r_char);
                u8::from_str_radix(&expanded, 16).unwrap_or(0)
            }
            6 | 8 => {
                // 6-digit or 8-digit hex
                u8::from_str_radix(&self.value[0..2], 16).unwrap_or(0)
            }
            _ => 0,
        }
    }

    /// Get green component (0-255)
    pub fn g(&self) -> u8 {
        match self.value.len() {
            3 => {
                let g_char = self.value.chars().nth(1).unwrap();
                let expanded = format!("{}{}", g_char, g_char);
                u8::from_str_radix(&expanded, 16).unwrap_or(0)
            }
            6 | 8 => u8::from_str_radix(&self.value[2..4], 16).unwrap_or(0),
            _ => 0,
        }
    }

    /// Get blue component (0-255)
    pub fn b(&self) -> u8 {
        match self.value.len() {
            3 => {
                let b_char = self.value.chars().nth(2).unwrap();
                let expanded = format!("{}{}", b_char, b_char);
                u8::from_str_radix(&expanded, 16).unwrap_or(0)
            }
            6 | 8 => u8::from_str_radix(&self.value[4..6], 16).unwrap_or(0),
            _ => 0,
        }
    }

    /// Get alpha component (0.0-1.0)
    pub fn a(&self) -> f32 {
        if self.value.len() == 8 {
            let alpha_hex = &self.value[6..8];
            u8::from_str_radix(alpha_hex, 16).unwrap_or(255) as f32 / 255.0
        } else {
            1.0
        }
    }

    /// Parser for hex colors
    /// Mirrors: HashColor.parser
    pub fn parser() -> TokenParser<HashColor> {
        TokenParser::<SimpleToken>::token(SimpleToken::Hash(String::new()), Some("Hash"))
            .map(
                |token| {
                    if let SimpleToken::Hash(value) = token {
                        value
                    } else {
                        unreachable!()
                    }
                },
                Some("extract_hash"),
            )
            .where_fn(
                |value| Self::is_valid_hex(value),
                Some("valid_hex"),
            )
            .map(HashColor::new, Some("to_hash_color"))
    }
}

impl Display for HashColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.value)
    }
}

/// RGB color: rgb(255, 0, 0)
/// Mirrors: Rgb class
#[derive(Debug, Clone, PartialEq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parser for RGB colors
    pub fn parser() -> TokenParser<RgbColor> {
        // number channel 0..=255
        let number_channel = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .map(|t| if let SimpleToken::Number(v) = t { v } else { 0.0 }, Some("to_f64"))
            .where_fn(|v| *v >= 0.0 && *v <= 255.0, Some("0..255"))
            .map(|v| v as u8, Some("to_u8"));

        let percent_channel = TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
            .map(|t| if let SimpleToken::Percentage(v) = t { v } else { 0.0 }, Some("to_f64"))
            .where_fn(|v| *v >= 0.0 && *v <= 100.0, Some("0..100%"))
            .map(|v| ((v * 255.0) / 100.0).round() as u8, Some("pct_to_u8"));

        let channel = TokenParser::one_of(vec![number_channel.clone(), percent_channel.clone()]);

        // rgb(<n>,<n>,<n>) comma syntax
        let comma_args: TokenParser<Vec<u8>> = TokenParser::<u8>::sequence(vec![channel.clone(), channel.clone(), channel.clone()]);

        // rgb function start and )
        let fn_rgb: TokenParser<String> = TokenParser::<String>::fn_name("rgb");
        let close_paren_rgb1 = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
        let close_paren_rgb2 = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        // comma separated variant: rgb <args> ) with commas handled by sequence using simple approach
        let comma_parser = fn_rgb
            .flat_map(move |_| {
                // naive: allow optional whitespace and commas by consuming delimiters/whitespace between numbers
                // we approximate by parsing three numbers sequentially and relying on upstream separation
                comma_args.clone()
            }, Some("args"))
            .flat_map(move |vals| {
                let cp = close_paren_rgb1.clone();
                cp.map(move |_| vals.clone(), Some(")"))
            }, Some("close"))
            .map(|vals| RgbColor::new(vals[0], vals[1], vals[2]), Some("to_rgb"));

        // space separated variant: rgb <c> <c> <c> )
        let space_args: TokenParser<Vec<u8>> = TokenParser::<u8>::sequence(vec![channel.clone(), channel.clone(), channel.clone()]);
        let space_parser = TokenParser::<String>::fn_name("rgb")
            .flat_map(move |_| space_args.clone(), Some("space_args"))
            .flat_map(move |vals| {
                let cp = close_paren_rgb2.clone();
                cp.map(move |_| vals.clone(), Some(")"))
            }, Some("space_close"))
            .map(|vals| RgbColor::new(vals[0], vals[1], vals[2]), Some("to_rgb_space"));

        TokenParser::one_of(vec![comma_parser, space_parser])
    }
}

impl Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}

/// RGBA color: rgba(255, 0, 0, 0.5)
/// Mirrors: Rgba class
#[derive(Debug, Clone, PartialEq)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Parser for RGBA colors
    pub fn parser() -> TokenParser<RgbaColor> {
        let number_channel = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .map(|t| if let SimpleToken::Number(v) = t { v } else { 0.0 }, Some("to_f64"))
            .where_fn(|v| *v >= 0.0 && *v <= 255.0, Some("0..255"))
            .map(|v| v as u8, Some("to_u8"));
        let percent_channel = TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
            .map(|t| if let SimpleToken::Percentage(v) = t { v } else { 0.0 }, Some("to_f64"))
            .where_fn(|v| *v >= 0.0 && *v <= 100.0, Some("0..100%"))
            .map(|v| ((v * 255.0) / 100.0).round() as u8, Some("pct_to_u8"));
        let channel = TokenParser::one_of(vec![number_channel.clone(), percent_channel.clone()]);

        let alpha_number = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .map(|t| if let SimpleToken::Number(v) = t { v as f32 } else { 0.0 }, Some("to_f32"));
        let alpha_percent = TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
            .map(|t| if let SimpleToken::Percentage(v) = t { (v as f32)/100.0 } else { 0.0 }, Some("pct_to_alpha"));
        let alpha_value = TokenParser::one_of(vec![alpha_number.clone(), alpha_percent.clone()]);
        let alpha_value_for_rgba = alpha_value.clone();

        let fn_rgba: TokenParser<String> = TokenParser::<String>::fn_name("rgba");

        // rgba(<c>,<c>,<c>,<a>) simple variant (alpha supports percent or number)
        let args: TokenParser<Vec<u8>> = TokenParser::<u8>::sequence(vec![channel.clone(), channel.clone(), channel.clone()]);
        // Parse: rgba(<n>,<n>,<n>,<a>)
        let rgba_parser = fn_rgba
            .flat_map(move |_| {
                let rgb = args.clone();
                rgb
            }, Some("rgb_args"))
            .flat_map(move |rgb_vals| {
                let rgb_clone = rgb_vals.clone();
                alpha_value_for_rgba.clone().map(move |a| (rgb_clone.clone(), a), Some("pair_alpha"))
            }, Some("alpha"))
            .flat_map(move |(rgb_vals, a)| {
                let cp = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
                cp.map(move |_| (rgb_vals.clone(), a), Some(")"))
            }, Some("close"))
            .map(|(rgb_vals, a)| RgbaColor::new(rgb_vals[0], rgb_vals[1], rgb_vals[2], a), Some("to_rgba"));

        // Modern rgb space syntax with slash alpha: rgb r g b / a
        let fn_rgb_space = TokenParser::<String>::fn_name("rgb");
        let space_args: TokenParser<Vec<u8>> = TokenParser::<u8>::sequence(vec![channel.clone(), channel.clone(), channel.clone()]);
        let slash = TokenParser::<SimpleToken>::token(SimpleToken::Delim('/'), Some("Slash"));
        let close_paren2 = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
        let alpha_value_for_rgb_space = alpha_value.clone();

        let rgb_slash_alpha = fn_rgb_space
            .flat_map(move |_| space_args.clone(), Some("space_args"))
            .flat_map(move |rgb_vals| {
                let rgb_clone_outer = rgb_vals.clone();
                let slash_local = slash.clone();
                let alpha_local = alpha_value_for_rgb_space.clone();
                slash_local.flat_map(move |_| {
                    let rgb_clone_inner = rgb_clone_outer.clone();
                    alpha_local.clone().map(move |a| (rgb_clone_inner.clone(), a), Some("pair"))
                }, Some("alpha"))
            }, Some("after_slash"))
            .flat_map(move |(rgb_vals, a)| {
                let cp = close_paren2.clone();
                cp.map(move |_| (rgb_vals.clone(), a), Some(")"))
            }, Some("close"))
            .map(|(rgb_vals, a)| RgbaColor::new(rgb_vals[0], rgb_vals[1], rgb_vals[2], a), Some("to_rgba_space"));

        TokenParser::one_of(vec![rgba_parser, rgb_slash_alpha])
    }
}

impl Display for RgbaColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

/// HSL color: hsl(360, 100%, 50%)
/// Mirrors: Hsl class
#[derive(Debug, Clone, PartialEq)]
pub struct HslColor {
    pub h: f32, // hue (0-360)
    pub s: f32, // saturation percentage (0-100)
    pub l: f32, // lightness percentage (0-100)
}

impl HslColor {
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l }
    }

    /// Parser for HSL colors
    pub fn parser() -> TokenParser<HslColor> {
        // Angle: deg/rad/turn or bare 0 -> deg
        let dim_angle = TokenParser::<SimpleToken>::token(
            SimpleToken::Dimension { value: 0.0, unit: String::new() },
            Some("Dimension")
        )
        .where_fn(|t| {
            if let SimpleToken::Dimension { unit, .. } = t {
                matches!(unit.as_str(), "deg" | "rad" | "turn")
            } else { false }
        }, Some("angle_unit"))
        .map(|t| {
            if let SimpleToken::Dimension { value, unit } = t {
                match unit.as_str() {
                    "deg" => value as f32,
                    "rad" => (value as f32) * 180.0 / std::f32::consts::PI,
                    _ => (value as f32) * 360.0, // turn
                }
            } else { 0.0 }
        }, Some("to_deg"));

        let zero_angle = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .where_fn(|t| matches!(t, SimpleToken::Number(v) if *v == 0.0), Some("zero"))
            .map(|_| 0.0_f32, Some("zero_deg"));

        let angle_number = TokenParser::one_of(vec![dim_angle, zero_angle]);

        // Percentage is SimpleToken::Percentage(value)
        let percent = TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
            .map(|t| if let SimpleToken::Percentage(v) = t { v as f32 } else { 0.0 }, Some("to_f32"));

        let fn_hsl: TokenParser<String> = TokenParser::<String>::fn_name("hsl");
        let close_paren = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        let args: TokenParser<Vec<f32>> = TokenParser::<f32>::sequence(vec![angle_number.clone(), percent.clone(), percent.clone()]);

        fn_hsl
            .flat_map(move |_| args.clone(), Some("args"))
            .flat_map(move |vals| close_paren.map(move |_| vals.clone(), Some(")")), Some("close"))
            .map(|vals| HslColor::new(vals[0], vals[1], vals[2]), Some("to_hsl"))
    }
}

impl Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({},{}%,{}%)", self.h, self.s, self.l)
    }
}

/// HSLA color: hsla(360, 100%, 50%, 0.5)
/// Mirrors: Hsla class
#[derive(Debug, Clone, PartialEq)]
pub struct HslaColor {
    pub h: f32, // hue (0-360)
    pub s: f32, // saturation percentage (0-100)
    pub l: f32, // lightness percentage (0-100)
    pub a: f32, // alpha (0.0-1.0)
}

impl HslaColor {
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self { h, s, l, a }
    }

    /// Parser for HSLA colors
    pub fn parser() -> TokenParser<HslaColor> {
        let angle_number = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .map(|t| if let SimpleToken::Number(v) = t { v as f32 } else { 0.0 }, Some("to_f32"));

        let percent = TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
            .map(|t| if let SimpleToken::Percentage(v) = t { v as f32 } else { 0.0 }, Some("to_f32"));

        // alpha may be number 0-1 or percentage
        let alpha_num = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .map(|t| if let SimpleToken::Number(v) = t { v as f32 } else { 0.0 }, Some("to_f32"));
        let alpha_pct = TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
            .map(|t| if let SimpleToken::Percentage(v) = t { (v as f32) / 100.0 } else { 0.0 }, Some("pct_to_alpha"));
        let alpha = TokenParser::one_of(vec![alpha_num, alpha_pct]);

        let fn_hsla: TokenParser<String> = TokenParser::<String>::fn_name("hsla");
        let close_paren = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        let args: TokenParser<Vec<f32>> = TokenParser::<f32>::sequence(vec![
            angle_number.clone(),
            percent.clone(),
            percent.clone(),
            alpha.clone(),
        ]);

        fn_hsla
            .flat_map(move |_| args.clone(), Some("args"))
            .flat_map(move |vals| close_paren.map(move |_| vals.clone(), Some(")")), Some("close"))
            .map(|vals| HslaColor::new(vals[0], vals[1], vals[2], vals[3]), Some("to_hsla"))
    }
}

impl Display for HslaColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsla({},{}%,{}%,{})", self.h, self.s, self.l, self.a)
    }
}

/// LCH color: lch(50% 100 270deg)
/// Mirrors: Lch class
#[derive(Debug, Clone, PartialEq)]
pub struct LchColor {
    pub l: f32,    // lightness (0-100)
    pub c: f32,    // chroma (0-150)
    pub h: Angle,  // hue angle
    pub alpha: Option<f32>, // alpha (0-1)
}

impl LchColor {
    pub fn new(l: f32, c: f32, h: Angle, alpha: Option<f32>) -> Self {
        Self { l, c, h, alpha }
    }

    /// Parser for LCH colors (simplified implementation)
    /// Mirrors: Lch.parser
    pub fn parser() -> TokenParser<LchColor> {
        // For now, implement a basic version that handles the test case: lch(50% 100 270deg)
        let fn_lch = TokenParser::<String>::fn_name("lch");
        let close_paren = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        // Parse lightness as percentage
        let lightness = TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
            .map(|t| if let SimpleToken::Percentage(v) = t { v as f32 } else { 0.0 }, Some("l_percent"));

        // Parse chroma as number
        let chroma = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .map(|t| if let SimpleToken::Number(v) = t { v as f32 } else { 0.0 }, Some("chroma"));

        // Parse hue as angle
        let hue = Angle::parser();

        // Combine: lch(<l> <c> <h>)
        fn_lch
            .flat_map(move |_| lightness.clone(), Some("lightness"))
            .flat_map(move |l| {
                let l_clone = l.clone();
                chroma.clone().map(move |c| (l_clone, c), Some("lc_pair"))
            }, Some("l_step"))
            .flat_map(move |(l, c)| {
                let l_clone = l.clone();
                let c_clone = c.clone();
                hue.clone().map(move |h| (l_clone, c_clone, h), Some("lch_triple"))
            }, Some("c_step"))
            .flat_map(move |(l, c, h)| {
                close_paren.clone().map(move |_| LchColor::new(l, c, h.clone(), None), Some("to_lch"))
            }, Some("close"))
    }
}

impl Display for LchColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.alpha {
            Some(alpha) => write!(f, "lch({} {} {} / {})", self.l, self.c, self.h, alpha),
            None => write!(f, "lch({} {} {})", self.l, self.c, self.h),
        }
    }
}

/// OKLCH color: oklch(0.5 0.1 270deg)
/// Mirrors: Oklch class
#[derive(Debug, Clone, PartialEq)]
pub struct OklchColor {
    pub l: f32,    // lightness (0-1)
    pub c: f32,    // chroma (0-0.4)
    pub h: Angle,  // hue angle
    pub alpha: Option<f32>, // alpha (0-1)
}

impl OklchColor {
    pub fn new(l: f32, c: f32, h: Angle, alpha: Option<f32>) -> Self {
        Self { l, c, h, alpha }
    }

    /// Parser for OKLCH colors (simplified implementation)
    /// Mirrors: Oklch.parser
    pub fn parser() -> TokenParser<OklchColor> {
        // For now, return a never parser as a placeholder
        // TODO: Implement full OKLCH parsing when needed
        TokenParser::never()
    }
}

impl Display for OklchColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.alpha {
            Some(alpha) => write!(f, "oklch({} {} {} / {})", self.l, self.c, self.h, alpha),
            None => write!(f, "oklch({} {} {})", self.l, self.c, self.h),
        }
    }
}

/// OKLAB color: oklab(0.5 0.1 0.1)
/// Mirrors: Oklab class
#[derive(Debug, Clone, PartialEq)]
pub struct OklabColor {
    pub l: f32,    // lightness (0-1)
    pub a: f32,    // green-red (-0.4 to 0.4)
    pub b: f32,    // blue-yellow (-0.4 to 0.4)
    pub alpha: Option<f32>, // alpha (0-1)
}

impl OklabColor {
    pub fn new(l: f32, a: f32, b: f32, alpha: Option<f32>) -> Self {
        Self { l, a, b, alpha }
    }

    /// Parser for OKLAB colors (simplified implementation)
    /// Mirrors: Oklab.parser
    pub fn parser() -> TokenParser<OklabColor> {
        // For now, return a never parser as a placeholder
        // TODO: Implement full OKLAB parsing when needed
        TokenParser::never()
    }
}

impl Display for OklabColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.alpha {
            Some(alpha) => write!(f, "oklab({} {} {} / {})", self.l, self.a, self.b, alpha),
            None => write!(f, "oklab({} {} {})", self.l, self.a, self.b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_color_validation() {
        assert!(NamedColor::is_valid_named_color("red"));
        assert!(NamedColor::is_valid_named_color("blue"));
        assert!(NamedColor::is_valid_named_color("transparent"));
        assert!(NamedColor::is_valid_named_color("currentcolor"));

        // Case insensitive
        assert!(NamedColor::is_valid_named_color("RED"));
        assert!(NamedColor::is_valid_named_color("Blue"));

        // Invalid
        assert!(!NamedColor::is_valid_named_color("notacolor"));
        assert!(!NamedColor::is_valid_named_color(""));
    }

    #[test]
    fn test_named_color_display() {
        let color = NamedColor::new("red".to_string());
        assert_eq!(color.to_string(), "red");
    }

    #[test]
    fn test_hex_color_validation() {
        // Valid formats
        assert!(HashColor::is_valid_hex("F00")); // 3-digit
        assert!(HashColor::is_valid_hex("FF0000")); // 6-digit
        assert!(HashColor::is_valid_hex("FF0000FF")); // 8-digit with alpha
        assert!(HashColor::is_valid_hex("123abc")); // lowercase

        // Invalid formats
        assert!(!HashColor::is_valid_hex("GG0000")); // invalid hex
        assert!(!HashColor::is_valid_hex("FF00")); // wrong length
        assert!(!HashColor::is_valid_hex("")); // empty
    }

    #[test]
    fn test_hex_color_rgb_extraction() {
        // 6-digit hex
        let color = HashColor::new("FF0000".to_string());
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 0);
        assert_eq!(color.b(), 0);
        assert_eq!(color.a(), 1.0);

        // 3-digit hex
        let short_color = HashColor::new("F0A".to_string());
        assert_eq!(short_color.r(), 255); // F -> FF
        assert_eq!(short_color.g(), 0);   // 0 -> 00
        assert_eq!(short_color.b(), 170); // A -> AA

        // 8-digit with alpha
        let alpha_color = HashColor::new("FF000080".to_string());
        assert!((alpha_color.a() - 0.5).abs() < 0.01); // 80 hex = 128 dec ≈ 0.5 alpha
    }

    #[test]
    fn test_hex_color_display() {
        let color = HashColor::new("FF0000".to_string());
        assert_eq!(color.to_string(), "#FF0000");
    }

    #[test]
    fn test_rgb_color_display() {
        let color = RgbColor::new(255, 0, 0);
        assert_eq!(color.to_string(), "rgb(255,0,0)");
    }

    #[test]
    fn test_rgba_color_display() {
        let color = RgbaColor::new(255, 0, 0, 0.5);
        assert_eq!(color.to_string(), "rgba(255,0,0,0.5)");
    }

    #[test]
    fn test_hsl_color_display() {
        let color = HslColor::new(360.0, 100.0, 50.0);
        assert_eq!(color.to_string(), "hsl(360,100%,50%)");
    }

    #[test]
    fn test_hsla_color_display() {
        let color = HslaColor::new(360.0, 100.0, 50.0, 0.8);
        assert_eq!(color.to_string(), "hsla(360,100%,50%,0.8)");
    }

    #[test]
    fn test_color_enum_display() {
        let named = Color::Named(NamedColor::new("red".to_string()));
        assert_eq!(named.to_string(), "red");

        let hash = Color::Hash(HashColor::new("FF0000".to_string()));
        assert_eq!(hash.to_string(), "#FF0000");

        let rgb = Color::Rgb(RgbColor::new(255, 0, 0));
        assert_eq!(rgb.to_string(), "rgb(255,0,0)");
    }

    #[test]
    fn test_color_parsers_creation() {
        // Basic test that parsers can be created
        let _named = NamedColor::parser();
        let _hash = HashColor::parser();
        let _rgb = RgbColor::parser();
        let _rgba = RgbaColor::parser();
        let _hsl = HslColor::parser();
        let _hsla = HslaColor::parser();
        let _lch = LchColor::parser();
        let _oklch = OklchColor::parser();
        let _oklab = OklabColor::parser();
        let _color = Color::parser();
    }

    #[test]
    fn test_lch_color_display() {
        let color = LchColor::new(50.0, 100.0, Angle::new(270.0, "deg".to_string()), None);
        assert_eq!(color.to_string(), "lch(50 100 270deg)");

        let color_with_alpha = LchColor::new(50.0, 100.0, Angle::new(270.0, "deg".to_string()), Some(0.8));
        assert_eq!(color_with_alpha.to_string(), "lch(50 100 270deg / 0.8)");
    }

    #[test]
    fn test_oklch_color_display() {
        let color = OklchColor::new(0.5, 0.1, Angle::new(270.0, "deg".to_string()), None);
        assert_eq!(color.to_string(), "oklch(0.5 0.1 270deg)");

        let color_with_alpha = OklchColor::new(0.5, 0.1, Angle::new(270.0, "deg".to_string()), Some(0.8));
        assert_eq!(color_with_alpha.to_string(), "oklch(0.5 0.1 270deg / 0.8)");
    }

    #[test]
    fn test_oklab_color_display() {
        let color = OklabColor::new(0.5, 0.1, 0.1, None);
        assert_eq!(color.to_string(), "oklab(0.5 0.1 0.1)");

        let color_with_alpha = OklabColor::new(0.5, 0.1, 0.1, Some(0.8));
        assert_eq!(color_with_alpha.to_string(), "oklab(0.5 0.1 0.1 / 0.8)");
    }
}
