/*!
CSS Color type parsing.

Handles all CSS color formats: named colors, hex, rgb, rgba, hsl, hsla, and modern color spaces.
*/

use crate::{
  CssParseError,
  css_types::{Angle, Percentage},
  token_parser::{TokenParser, tokens},
  token_types::{SimpleToken, TokenList},
};

use std::fmt::{self, Display};

/// Parses numbers in range 0-255 for RGB color channels
fn rgb_number_parser() -> TokenParser<u8> {
  tokens::number()
    .map(
      |token| {
        if let SimpleToken::Number(v) = token {
          v
        } else {
          0.0
        }
      },
      Some("extract_number"),
    )
    .where_fn(|v| *v >= 0.0 && *v <= 255.0, Some("0..255"))
    .map(|v| v as u8, Some("to_u8"))
}

fn alpha_as_number() -> TokenParser<f32> {
  crate::css_types::alpha_value::alpha_as_number()
}

/// Function name parser helper
fn function_parser(name: &'static str) -> TokenParser<()> {
  tokens::function()
    .where_fn(
      move |token| {
        if let SimpleToken::Function(fn_name) = token {
          fn_name == name
        } else {
          false
        }
      },
      Some(&format!("{}_function", name)),
    )
    .map(|_| (), Some(&format!("{}_fn", name)))
}

pub struct AdvancedColorParsers;

impl AdvancedColorParsers {
  /// Implements: TokenParser.sequence(fn, r, comma, g, comma, b, closeParen).map(([_fn, r, _c, g, _c2, b, _cp]) => ...)
  pub fn rgb_comma_full() -> TokenParser<(u8, u8, u8)> {
    function_parser("rgb")
      .flat_map(|_| rgb_number_parser(), Some("r"))
      .flat_map(
        |r| {
          tokens::comma().flat_map(
            move |_| rgb_number_parser().map(move |g| (r, g), Some("rg")),
            Some("comma1"),
          )
        },
        Some("rg_step"),
      )
      .flat_map(
        |(r, g)| {
          tokens::comma().flat_map(
            move |_| rgb_number_parser().map(move |b| (r, g, b), Some("rgb")),
            Some("comma2"),
          )
        },
        Some("rgb_step"),
      )
      .flat_map(
        |(r, g, b)| tokens::close_paren().map(move |_| (r, g, b), Some("final_rgb")),
        Some("close"),
      )
  }

  pub fn rgb_space_full() -> TokenParser<(u8, u8, u8)> {
    function_parser("rgb")
      .flat_map(
        |_| {
          rgb_number_parser()
            .separated_by(tokens::whitespace())
            .one_or_more()
            .where_fn(|vals| vals.len() >= 3, Some("has_3_values"))
        },
        Some("rgb_values"),
      )
      .flat_map(
        |vals| tokens::close_paren().map(move |_| (vals[0], vals[1], vals[2]), Some("final_rgb")),
        Some("close"),
      )
  }

  /// [_fn, r, _comma, g, _comma2, b, _comma3, a, _closeParen] => new Rgba(r, g, b, a)
  pub fn rgba_comma_full() -> TokenParser<(u8, u8, u8, f32)> {
    function_parser("rgba")
      .flat_map(|_| rgb_number_parser(), Some("r"))
      .flat_map(
        |r| {
          tokens::comma().flat_map(
            move |_| rgb_number_parser().map(move |g| (r, g), Some("rg")),
            Some("comma1"),
          )
        },
        Some("rg_step"),
      )
      .flat_map(
        |(r, g)| {
          tokens::comma().flat_map(
            move |_| rgb_number_parser().map(move |b| (r, g, b), Some("rgb")),
            Some("comma2"),
          )
        },
        Some("rgb_step"),
      )
      .flat_map(
        |(r, g, b)| {
          tokens::comma().flat_map(
            move |_| alpha_as_number().map(move |a| (r, g, b, a), Some("rgba")),
            Some("comma3"),
          )
        },
        Some("rgba_step"),
      )
      .flat_map(
        |(r, g, b, a)| tokens::close_paren().map(move |_| (r, g, b, a), Some("final_rgba")),
        Some("close"),
      )
  }

  /// Uses rgb() function with space-separated values and slash for alpha: rgb(r g b / a)
  /// [_fn, _preSpace, r, _space, g, _space2, b, _slash, a, _postSpace, _closeParen] => new Rgba(r, g, b, a)
  pub fn rgba_space_slash_full() -> TokenParser<(u8, u8, u8, f32)> {
    function_parser("rgb") // Note: rgb function, not rgba!
      .flat_map(
        |_| {
          tokens::whitespace()
            .optional()
            .flat_map(move |_| rgb_number_parser(), Some("r"))
        },
        Some("optional_space_r"),
      )
      .flat_map(
        |r| {
          tokens::whitespace().flat_map(
            move |_| rgb_number_parser().map(move |g| (r, g), Some("rg")),
            Some("space1"),
          )
        },
        Some("rg_step"),
      )
      .flat_map(
        |(r, g)| {
          tokens::whitespace().flat_map(
            move |_| rgb_number_parser().map(move |b| (r, g, b), Some("rgb")),
            Some("space2"),
          )
        },
        Some("rgb_step"),
      )
      .flat_map(
        |(r, g, b)| {
          // Parse slash surrounded by optional whitespace
          tokens::whitespace().optional().flat_map(
            move |_| {
              tokens::delim('/').flat_map(
                move |_| {
                  tokens::whitespace().optional().flat_map(
                    move |_| alpha_as_number().map(move |a| (r, g, b, a), Some("rgba")),
                    Some("alpha"),
                  )
                },
                Some("slash"),
              )
            },
            Some("slash_alpha"),
          )
        },
        Some("rgba_step"),
      )
      .flat_map(
        |(r, g, b, a)| {
          tokens::whitespace().optional().flat_map(
            move |_| tokens::close_paren().map(move |_| (r, g, b, a), Some("final_rgba")),
            Some("close"),
          )
        },
        Some("final_close"),
      )
  }

  pub fn hsl_comma_full() -> TokenParser<(Angle, Percentage, Percentage)> {
    function_parser("hsl")
      .flat_map(|_| Angle::parser(), Some("h"))
      .flat_map(
        |h| {
          tokens::comma().flat_map(
            {
              let h = h.clone();
              move |_| {
                Percentage::parser().map(
                  {
                    let value = h.clone();
                    move |s| (value.clone(), s)
                  },
                  Some("hs"),
                )
              }
            },
            Some("comma1"),
          )
        },
        Some("h_step"),
      )
      .flat_map(
        |(h, s)| {
          tokens::comma().flat_map(
            {
              let h = h.clone();
              let s = s.clone();
              move |_| {
                Percentage::parser().map(
                  {
                    let h_value = h.clone();
                    let s_value = s.clone();
                    move |l| (h_value.clone(), s_value.clone(), l)
                  },
                  Some("hsl"),
                )
              }
            },
            Some("comma2"),
          )
        },
        Some("s_step"),
      )
      .flat_map(
        |(h, s, l)| {
          tokens::close_paren().map(
            move |_| (h.clone(), s.clone(), l.clone()),
            Some("final_hsl"),
          )
        },
        Some("close"),
      )
  }

  pub fn hsla_comma_full() -> TokenParser<(Angle, Percentage, Percentage, f32)> {
    function_parser("hsla")
      .flat_map(|_| Angle::parser(), Some("h"))
      .flat_map(
        |h| {
          tokens::comma().flat_map(
            {
              let h = h.clone();
              move |_| {
                Percentage::parser().map(
                  {
                    let value = h.clone();
                    move |s| (value.clone(), s)
                  },
                  Some("hs"),
                )
              }
            },
            Some("comma1"),
          )
        },
        Some("h_step"),
      )
      .flat_map(
        |(h, s)| {
          tokens::comma().flat_map(
            {
              let h = h.clone();
              let s = s.clone();
              move |_| {
                Percentage::parser().map(
                  {
                    let h_value = h.clone();
                    let s_value = s.clone();
                    move |l| (h_value.clone(), s_value.clone(), l)
                  },
                  Some("hsl"),
                )
              }
            },
            Some("comma2"),
          )
        },
        Some("s_step"),
      )
      .flat_map(
        |(h, s, l)| {
          tokens::comma().flat_map(
            {
              let h = h.clone();
              let s = s.clone();
              let l = l.clone();
              move |_| {
                alpha_as_number().map(
                  {
                    let h_value = h.clone();
                    let s_value = s.clone();
                    let l_value = l.clone();
                    move |a| (h_value.clone(), s_value.clone(), l_value.clone(), a)
                  },
                  Some("hsla"),
                )
              }
            },
            Some("comma3"),
          )
        },
        Some("l_step"),
      )
      .flat_map(
        |(h, s, l, a)| {
          tokens::close_paren().map(
            move |_| (h.clone(), s.clone(), l.clone(), a),
            Some("final_hsla"),
          )
        },
        Some("close"),
      )
  }

  // NOTE: HSLA space/slash parser temporarily removed due to complex ownership issues
  // This will be addressed in a future enhancement phase
  // The comma parser already provides full functionality for most use cases
}

/// List of all valid CSS named colors
/// This is a comprehensive list of CSS Level 4 named colors
const NAMED_COLORS: &[&str] = &[
  "aliceblue",
  "antiquewhite",
  "aqua",
  "aquamarine",
  "azure",
  "beige",
  "bisque",
  "black",
  "blanchedalmond",
  "blue",
  "blueviolet",
  "brown",
  "burlywood",
  "cadetblue",
  "chartreuse",
  "chocolate",
  "coral",
  "cornflowerblue",
  "cornsilk",
  "crimson",
  "cyan",
  "darkblue",
  "darkcyan",
  "darkgoldenrod",
  "darkgray",
  "darkgreen",
  "darkgrey",
  "darkkhaki",
  "darkmagenta",
  "darkolivegreen",
  "darkorange",
  "darkorchid",
  "darkred",
  "darksalmon",
  "darkseagreen",
  "darkslateblue",
  "darkslategray",
  "darkslategrey",
  "darkturquoise",
  "darkviolet",
  "deeppink",
  "deepskyblue",
  "dimgray",
  "dimgrey",
  "dodgerblue",
  "firebrick",
  "floralwhite",
  "forestgreen",
  "fuchsia",
  "gainsboro",
  "ghostwhite",
  "gold",
  "goldenrod",
  "gray",
  "grey",
  "green",
  "greenyellow",
  "honeydew",
  "hotpink",
  "indianred",
  "indigo",
  "ivory",
  "khaki",
  "lavender",
  "lavenderblush",
  "lawngreen",
  "lemonchiffon",
  "lightblue",
  "lightcoral",
  "lightcyan",
  "lightgoldenrodyellow",
  "lightgray",
  "lightgreen",
  "lightgrey",
  "lightpink",
  "lightsalmon",
  "lightseagreen",
  "lightskyblue",
  "lightslategray",
  "lightslategrey",
  "lightsteelblue",
  "lightyellow",
  "lime",
  "limegreen",
  "linen",
  "magenta",
  "maroon",
  "mediumaquamarine",
  "mediumblue",
  "mediumorchid",
  "mediumpurple",
  "mediumseagreen",
  "mediumslateblue",
  "mediumspringgreen",
  "mediumturquoise",
  "mediumvioletred",
  "midnightblue",
  "mintcream",
  "mistyrose",
  "moccasin",
  "navajowhite",
  "navy",
  "oldlace",
  "olive",
  "olivedrab",
  "orange",
  "orangered",
  "orchid",
  "palegoldenrod",
  "palegreen",
  "paleturquoise",
  "palevioletred",
  "papayawhip",
  "peachpuff",
  "peru",
  "pink",
  "plum",
  "powderblue",
  "purple",
  "rebeccapurple",
  "red",
  "rosybrown",
  "royalblue",
  "saddlebrown",
  "salmon",
  "sandybrown",
  "seagreen",
  "seashell",
  "sienna",
  "silver",
  "skyblue",
  "slateblue",
  "slategray",
  "slategrey",
  "snow",
  "springgreen",
  "steelblue",
  "tan",
  "teal",
  "thistle",
  "tomato",
  "turquoise",
  "violet",
  "wheat",
  "white",
  "whitesmoke",
  "yellow",
  "yellowgreen",
  "transparent",
  "currentcolor",
];

/// Base Color trait that all color types implement
pub trait ColorTrait {
  fn to_string(&self) -> String;
}

/// Main Color enum that encompasses all color types
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
  Named(NamedColor),
  Hash(HashColor),
  Rgb(Rgb),
  Rgba(Rgba),
  Hsl(Hsl),
  Hsla(Hsla),
  Lch(Lch),
  Oklch(Oklch),
  Oklab(Oklab),
}

impl Color {
  pub fn parse() -> TokenParser<Color> {
    TokenParser::one_of(vec![
      NamedColor::parse().map(Color::Named, Some("named")),
      HashColor::parse().map(Color::Hash, Some("hash")),
      Rgb::parse().map(Color::Rgb, Some("rgb")),
      Rgba::parse().map(Color::Rgba, Some("rgba")),
      Hsl::parse().map(Color::Hsl, Some("hsl")),
      Hsla::parse().map(Color::Hsla, Some("hsla")),
      Lch::parse().map(Color::Lch, Some("lch")),
      Oklch::parse().map(Color::Oklch, Some("oklch")),
      Oklab::parse().map(Color::Oklab, Some("oklab")),
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
  pub fn parse() -> TokenParser<NamedColor> {
    tokens::ident()
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
  /// IMPORTANT: Implements CORRECT CSS behavior for 3-digit hex expansion
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
  /// IMPORTANT: Implements CORRECT CSS behavior for 3-digit hex expansion
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
  /// IMPORTANT: Implements CORRECT CSS behavior for 3-digit hex expansion
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
  pub fn parse() -> TokenParser<HashColor> {
    tokens::hash()
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
      .where_fn(|value| Self::is_valid_hex(value), Some("valid_hex"))
      .map(HashColor::new, Some("to_hash_color"))
  }
}

impl Display for HashColor {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "#{}", self.value)
  }
}

/// RGB color: rgb(255, 0, 0)
#[derive(Debug, Clone, PartialEq)]
pub struct Rgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Rgb {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }

  pub fn parse() -> TokenParser<Rgb> {
    TokenParser::one_of(vec![Self::comma_parser(), Self::space_parser()])
  }

  /// Parse comma-separated RGB: rgb(255, 0, 0)
  fn comma_parser() -> TokenParser<Rgb> {
    TokenParser::new(
      |tokens| {
        // Expect Function("rgb")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected RGB function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "rgb" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'rgb' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse r value
        let r = Self::parse_rgb_number_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse g value
        let g = Self::parse_rgb_number_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse b value
        let b = Self::parse_rgb_number_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Rgb::new(r, g, b))
      },
      "rgb_comma_parser",
    )
  }

  /// Parse space-separated RGB: rgb(255 0 0)
  fn space_parser() -> TokenParser<Rgb> {
    TokenParser::new(
      |tokens| {
        // Expect Function("rgb")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected RGB function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "rgb" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'rgb' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse r value
        let r = Self::parse_rgb_number_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse g value
        let g = Self::parse_rgb_number_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse b value
        let b = Self::parse_rgb_number_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Rgb::new(r, g, b))
      },
      "rgb_space_parser",
    )
  }

  /// Helper: Parse RGB number token (0-255)
  fn parse_rgb_number_token(tokens: &mut TokenList) -> Result<u8, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected number token".to_string(),
      })?;

    if let SimpleToken::Number(value) = token {
      if (0.0..=255.0).contains(&value) {
        Ok(value as u8)
      } else {
        Err(CssParseError::ParseError {
          message: format!("RGB number must be 0-255, got {}", value),
        })
      }
    } else {
      Err(CssParseError::ParseError {
        message: format!("Expected Number token, got {:?}", token),
      })
    }
  }

  fn consume_comma_with_optional_whitespace(tokens: &mut TokenList) -> Result<(), CssParseError> {
    // Skip optional whitespace before comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    // Expect comma
    let comma_token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected comma".to_string(),
      })?;

    if !matches!(comma_token, SimpleToken::Comma) {
      return Err(CssParseError::ParseError {
        message: format!("Expected ',' token, got {:?}", comma_token),
      });
    }

    // Skip optional whitespace after comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    Ok(())
  }
}

impl Display for Rgb {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "rgb({},{},{})", self.r, self.g, self.b)
  }
}

/// RGBA color: rgba(255, 0, 0, 0.5)
#[derive(Debug, Clone, PartialEq)]
pub struct Rgba {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: f32,
}

impl Rgba {
  pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
    Self { r, g, b, a }
  }

  pub fn parse() -> TokenParser<Rgba> {
    TokenParser::one_of(vec![Self::comma_parser(), Self::space_slash_parser()])
  }

  /// Parse comma-separated RGBA: rgba(255, 0, 0, 0.5)
  fn comma_parser() -> TokenParser<Rgba> {
    TokenParser::new(
      |tokens| {
        // Expect Function("rgba")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected RGBA function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "rgba" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'rgba' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse r value
        let r = Self::parse_rgba_number_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse g value
        let g = Self::parse_rgba_number_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse b value
        let b = Self::parse_rgba_number_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse alpha value
        let a = Self::parse_alpha_value_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Rgba::new(r, g, b, a))
      },
      "rgba_comma_parser",
    )
  }

  /// Parse space-separated RGBA with slash: rgb(255 0 0 / 0.5)
  fn space_slash_parser() -> TokenParser<Rgba> {
    TokenParser::new(
      |tokens| {
        // Expect Function("rgb") - note: uses rgb, not rgba!
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected RGB function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "rgb" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'rgb' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse r value
        let r = Self::parse_rgba_number_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse g value
        let g = Self::parse_rgba_number_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse b value
        let b = Self::parse_rgba_number_token(tokens)?;

        // Expect whitespace before slash (optional)
        if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect slash
        let slash_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected slash".to_string(),
          })?;
        if !matches!(slash_token, SimpleToken::Delim('/')) {
          return Err(CssParseError::ParseError {
            message: format!("Expected '/' token, got {:?}", slash_token),
          });
        }

        // Expect whitespace after slash (optional)
        if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse alpha value
        let a = Self::parse_alpha_value_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Rgba::new(r, g, b, a))
      },
      "rgba_space_slash_parser",
    )
  }

  /// Helper: Parse RGBA number token (0-255) - same as RGB
  fn parse_rgba_number_token(tokens: &mut TokenList) -> Result<u8, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected number token".to_string(),
      })?;

    if let SimpleToken::Number(value) = token {
      if (0.0..=255.0).contains(&value) {
        Ok(value as u8)
      } else {
        Err(CssParseError::ParseError {
          message: format!("RGBA number must be 0-255, got {}", value),
        })
      }
    } else {
      Err(CssParseError::ParseError {
        message: format!("Expected Number token, got {:?}", token),
      })
    }
  }

  /// Helper: Parse alpha value token (0.0-1.0 or 0%-100%)
  fn parse_alpha_value_token(tokens: &mut TokenList) -> Result<f32, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected alpha value token".to_string(),
      })?;

    match token {
      SimpleToken::Number(value) => {
        if (0.0..=1.0).contains(&value) {
          Ok(value as f32)
        } else {
          Err(CssParseError::ParseError {
            message: format!("Alpha number must be 0.0-1.0, got {}", value),
          })
        }
      }
      SimpleToken::Percentage(value) => {
        // cssparser stores percentage as already converted (0.50 for 50%)
        // so we don't need to divide by 100
        if (0.0..=1.0).contains(&value) {
          Ok(value as f32)
        } else {
          Err(CssParseError::ParseError {
            message: format!(
              "Alpha percentage must be 0%-100% (stored as 0.0-1.0), got {}",
              value
            ),
          })
        }
      }
      _ => Err(CssParseError::ParseError {
        message: format!(
          "Expected Number or Percentage token for alpha, got {:?}",
          token
        ),
      }),
    }
  }

  fn consume_comma_with_optional_whitespace(tokens: &mut TokenList) -> Result<(), CssParseError> {
    // Skip optional whitespace before comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    // Expect comma
    let comma_token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected comma".to_string(),
      })?;

    if !matches!(comma_token, SimpleToken::Comma) {
      return Err(CssParseError::ParseError {
        message: format!("Expected ',' token, got {:?}", comma_token),
      });
    }

    // Skip optional whitespace after comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    Ok(())
  }
}

impl Display for Rgba {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.a)
  }
}

/// HSL color: hsl(360deg, 100%, 50%)
#[derive(Debug, Clone, PartialEq)]
pub struct Hsl {
  pub h: Angle,      // hue angle (0-360deg)
  pub s: Percentage, // saturation percentage (0-100%)
  pub l: Percentage, // lightness percentage (0-100%)
}

impl Hsl {
  pub fn new(h: Angle, s: Percentage, l: Percentage) -> Self {
    Self { h, s, l }
  }

  /// Convenience constructor for backward compatibility with tests
  /// Creates Hsl from primitive f32 values
  pub fn from_primitives(h: f32, s: f32, l: f32) -> Self {
    Self {
      h: Angle::new(h, "deg".to_string()),
      s: Percentage::new(s),
      l: Percentage::new(l),
    }
  }

  pub fn parse() -> TokenParser<Hsl> {
    TokenParser::one_of(vec![Self::comma_parser(), Self::space_parser()])
  }

  /// Parse comma-separated HSL: hsl(360deg, 100%, 50%)
  fn comma_parser() -> TokenParser<Hsl> {
    TokenParser::new(
      |tokens| {
        // Expect Function("hsl")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected HSL function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "hsl" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'hsl' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse hue value (angle or number)
        let h = Self::parse_hsl_hue_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse saturation percentage
        let s = Self::parse_hsl_percentage_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse lightness percentage
        let l = Self::parse_hsl_percentage_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Hsl::new(h, s, l))
      },
      "hsl_comma_parser",
    )
  }

  /// Parse space-separated HSL: hsl(360deg 100% 50%)
  fn space_parser() -> TokenParser<Hsl> {
    TokenParser::new(
      |tokens| {
        // Expect Function("hsl")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected HSL function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "hsl" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'hsl' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse hue value (angle or number)
        let h = Self::parse_hsl_hue_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse saturation percentage
        let s = Self::parse_hsl_percentage_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse lightness percentage
        let l = Self::parse_hsl_percentage_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Hsl::new(h, s, l))
      },
      "hsl_space_parser",
    )
  }

  /// Helper: Parse HSL hue token (angle or number treated as degrees)
  fn parse_hsl_hue_token(tokens: &mut TokenList) -> Result<Angle, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected hue value token".to_string(),
      })?;

    match token {
      SimpleToken::Dimension { value, unit } => {
        // Parse as angle with unit
        if Angle::is_valid_unit(&unit) {
          Ok(Angle::new(value as f32, unit))
        } else {
          Err(CssParseError::ParseError {
            message: format!("Invalid angle unit: {}", unit),
          })
        }
      }
      SimpleToken::Number(value) => {
        // Treat numbers as degrees (CSS standard for HSL)
        Ok(Angle::new(value as f32, "deg".to_string()))
      }
      _ => Err(CssParseError::ParseError {
        message: format!(
          "Expected Number or Dimension token for hue, got {:?}",
          token
        ),
      }),
    }
  }

  /// Helper: Parse HSL percentage token
  fn parse_hsl_percentage_token(tokens: &mut TokenList) -> Result<Percentage, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected percentage token".to_string(),
      })?;

    if let SimpleToken::Percentage(value) = token {
      // cssparser stores percentage as already converted (0.5 for 50%)
      // Convert to our format (50.0 for 50%)
      Ok(Percentage::new((value as f32) * 100.0))
    } else {
      Err(CssParseError::ParseError {
        message: format!("Expected Percentage token, got {:?}", token),
      })
    }
  }

  fn consume_comma_with_optional_whitespace(tokens: &mut TokenList) -> Result<(), CssParseError> {
    // Skip optional whitespace before comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    // Expect comma
    let comma_token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected comma".to_string(),
      })?;

    if !matches!(comma_token, SimpleToken::Comma) {
      return Err(CssParseError::ParseError {
        message: format!("Expected ',' token, got {:?}", comma_token),
      });
    }

    // Skip optional whitespace after comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    Ok(())
  }
}

impl Display for Hsl {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let h_value = self.h.value; // Extract the numeric value without unit
    write!(f, "hsl({},{},{})", h_value, self.s, self.l)
  }
}

/// HSLA color: hsla(360deg, 100%, 50%, 0.5)
#[derive(Debug, Clone, PartialEq)]
pub struct Hsla {
  pub h: Angle,      // hue angle (0-360deg)
  pub s: Percentage, // saturation percentage (0-100%)
  pub l: Percentage, // lightness percentage (0-100%)
  pub a: f32,        // alpha (0.0-1.0)
}

impl Hsla {
  pub fn new(h: Angle, s: Percentage, l: Percentage, a: f32) -> Self {
    Self { h, s, l, a }
  }

  /// Convenience constructor for backward compatibility with tests
  /// Creates Hsla from primitive f32 values
  pub fn from_primitives(h: f32, s: f32, l: f32, a: f32) -> Self {
    Self {
      h: Angle::new(h, "deg".to_string()),
      s: Percentage::new(s),
      l: Percentage::new(l),
      a,
    }
  }

  pub fn parse() -> TokenParser<Hsla> {
    TokenParser::one_of(vec![Self::comma_parser(), Self::space_slash_parser()])
  }

  /// Parse comma-separated HSLA: hsla(360deg, 100%, 50%, 0.5)
  fn comma_parser() -> TokenParser<Hsla> {
    TokenParser::new(
      |tokens| {
        // Expect Function("hsla")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected HSLA function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "hsla" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'hsla' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse hue value (angle or number)
        let h = Self::parse_hsla_hue_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse saturation percentage
        let s = Self::parse_hsla_percentage_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse lightness percentage
        let l = Self::parse_hsla_percentage_token(tokens)?;

        // Expect comma (with optional whitespace)
        Self::consume_comma_with_optional_whitespace(tokens)?;

        // Parse alpha value
        let a = Self::parse_hsla_alpha_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Hsla::new(h, s, l, a))
      },
      "hsla_comma_parser",
    )
  }

  /// Parse space-separated HSLA with slash: hsl(360deg 100% 50% / 0.5)
  fn space_slash_parser() -> TokenParser<Hsla> {
    TokenParser::new(
      |tokens| {
        // Expect Function("hsl") - note: hsl, not hsla!
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected HSL function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "hsl" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'hsl' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse hue value (angle or number)
        let h = Self::parse_hsla_hue_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse saturation percentage
        let s = Self::parse_hsla_percentage_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse lightness percentage
        let l = Self::parse_hsla_percentage_token(tokens)?;

        // Expect whitespace before slash (optional)
        if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect slash
        let slash_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected slash".to_string(),
          })?;
        if !matches!(slash_token, SimpleToken::Delim('/')) {
          return Err(CssParseError::ParseError {
            message: format!("Expected '/' token, got {:?}", slash_token),
          });
        }

        // Expect whitespace after slash (optional)
        if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse alpha value
        let a = Self::parse_hsla_alpha_token(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Hsla::new(h, s, l, a))
      },
      "hsla_space_slash_parser",
    )
  }

  /// Helper: Parse HSLA hue token (angle or number treated as degrees)
  fn parse_hsla_hue_token(tokens: &mut TokenList) -> Result<Angle, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected hue value token".to_string(),
      })?;

    match token {
      SimpleToken::Dimension { value, unit } => {
        // Parse as angle with unit
        if Angle::is_valid_unit(&unit) {
          Ok(Angle::new(value as f32, unit))
        } else {
          Err(CssParseError::ParseError {
            message: format!("Invalid angle unit: {}", unit),
          })
        }
      }
      SimpleToken::Number(value) => {
        // Treat numbers as degrees (CSS standard for HSL/HSLA)
        Ok(Angle::new(value as f32, "deg".to_string()))
      }
      _ => Err(CssParseError::ParseError {
        message: format!(
          "Expected Number or Dimension token for hue, got {:?}",
          token
        ),
      }),
    }
  }

  /// Helper: Parse HSLA percentage token
  fn parse_hsla_percentage_token(tokens: &mut TokenList) -> Result<Percentage, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected percentage token".to_string(),
      })?;

    if let SimpleToken::Percentage(value) = token {
      // cssparser stores percentage as already converted (0.5 for 50%)
      // Convert to our format (50.0 for 50%)
      Ok(Percentage::new((value as f32) * 100.0))
    } else {
      Err(CssParseError::ParseError {
        message: format!("Expected Percentage token, got {:?}", token),
      })
    }
  }

  /// Helper: Parse HSLA alpha value token (0.0-1.0 or 0%-100%)
  fn parse_hsla_alpha_token(tokens: &mut TokenList) -> Result<f32, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected alpha value token".to_string(),
      })?;

    match token {
      SimpleToken::Number(value) => {
        if (0.0..=1.0).contains(&value) {
          Ok(value as f32)
        } else {
          Err(CssParseError::ParseError {
            message: format!("Alpha number must be 0.0-1.0, got {}", value),
          })
        }
      }
      SimpleToken::Percentage(value) => {
        // cssparser stores percentage as already converted (0.50 for 50%)
        if (0.0..=1.0).contains(&value) {
          Ok(value as f32)
        } else {
          Err(CssParseError::ParseError {
            message: format!(
              "Alpha percentage must be 0%-100% (stored as 0.0-1.0), got {}",
              value
            ),
          })
        }
      }
      _ => Err(CssParseError::ParseError {
        message: format!(
          "Expected Number or Percentage token for alpha, got {:?}",
          token
        ),
      }),
    }
  }

  fn consume_comma_with_optional_whitespace(tokens: &mut TokenList) -> Result<(), CssParseError> {
    // Skip optional whitespace before comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    // Expect comma
    let comma_token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected comma".to_string(),
      })?;

    if !matches!(comma_token, SimpleToken::Comma) {
      return Err(CssParseError::ParseError {
        message: format!("Expected ',' token, got {:?}", comma_token),
      });
    }

    // Skip optional whitespace after comma
    while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    Ok(())
  }
}

impl Display for Hsla {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let h_value = self.h.value; // Extract the numeric value without unit
    write!(f, "hsla({},{},{},{})", h_value, self.s, self.l, self.a)
  }
}

/// LCH color: lch(50% 100 270deg) or lch(50 100 270)
#[derive(Debug, Clone, PartialEq)]
pub struct Lch {
  pub l: f32,             // lightness (0-100) - can be percentage, number, or 'none'
  pub c: f32,             // chroma (0-150) - can be percentage or number
  pub h: LchHue,          // hue - can be angle or number
  pub alpha: Option<f32>, // alpha (0-1) - optional with slash syntax
}

#[derive(Debug, Clone, PartialEq)]
pub enum LchHue {
  Angle(Angle),
  Number(f32),
}

impl LchHue {
  pub fn from_angle(angle: Angle) -> Self {
    LchHue::Angle(angle)
  }

  pub fn from_number(number: f32) -> Self {
    LchHue::Number(number)
  }
}

impl std::fmt::Display for LchHue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LchHue::Angle(angle) => write!(f, "{}", angle),
      LchHue::Number(number) => write!(f, "{}", number),
    }
  }
}

impl Lch {
  pub fn new(l: f32, c: f32, h: LchHue, alpha: Option<f32>) -> Self {
    Self { l, c, h, alpha }
  }

  pub fn new_with_angle(l: f32, c: f32, h: Angle, alpha: Option<f32>) -> Self {
    Self::new(l, c, LchHue::Angle(h), alpha)
  }

  pub fn new_with_number(l: f32, c: f32, h: f32, alpha: Option<f32>) -> Self {
    Self::new(l, c, LchHue::Number(h), alpha)
  }

  /// Parses lch(lightness chroma hue) and lch(lightness chroma hue / alpha) formats
  pub fn parse() -> TokenParser<Lch> {
    TokenParser::new(
      |tokens| {
        // Expect Function("lch")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected LCH function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "lch" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'lch' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse lightness: percentage or number
        let l = Self::parse_lch_lightness_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse chroma: number
        let c = Self::parse_lch_chroma_token(tokens)?;

        // Expect whitespace
        let whitespace_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace".to_string(),
          })?;
        if !matches!(whitespace_token, SimpleToken::Whitespace) {
          return Err(CssParseError::ParseError {
            message: format!("Expected whitespace token, got {:?}", whitespace_token),
          });
        }

        // Parse hue: angle or number
        let h = Self::parse_lch_hue_token(tokens)?;

        // Parse optional alpha: / <alpha-value>
        let alpha = Self::parse_optional_alpha(tokens)?;

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')' token, got {:?}", close_token),
          });
        }

        Ok(Lch::new(l, c, h, alpha))
      },
      "lch_parser",
    )
  }

  /// Parse optional alpha: / <alpha-value>
  fn parse_optional_alpha(
    input: &mut crate::token_types::TokenList,
  ) -> Result<Option<f32>, CssParseError> {
    // Check if there's a slash for alpha
    let checkpoint = input.current_index;

    // Skip optional whitespace
    if let Ok(Some(SimpleToken::Whitespace)) = input.peek() {
      input.consume_next_token()?;
    }

    match input.peek()? {
      Some(SimpleToken::Delim('/')) => {
        input.consume_next_token()?; // consume '/'

        // Skip optional whitespace after slash
        if let Ok(Some(SimpleToken::Whitespace)) = input.peek() {
          input.consume_next_token()?;
        }

        // Parse alpha value using enhanced alpha parser
        let alpha_parser = crate::css_types::alpha_value::alpha_as_number();
        match alpha_parser.run.as_ref()(input) {
          Ok(alpha) => Ok(Some(alpha)),
          Err(e) => Err(e),
        }
      }
      _ => {
        // No alpha, rewind to checkpoint
        input.set_current_index(checkpoint);
        Ok(None)
      }
    }
  }

  /// Helper: Parse LCH lightness token (percentage or number)
  fn parse_lch_lightness_token(tokens: &mut TokenList) -> Result<f32, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected lightness value token".to_string(),
      })?;

    match token {
      SimpleToken::Percentage(value) => {
        // Convert percentage to value (50% = 50.0)
        Ok((value as f32) * 100.0)
      }
      SimpleToken::Number(value) => Ok(value as f32),
      _ => Err(CssParseError::ParseError {
        message: format!(
          "Expected Number or Percentage token for lightness, got {:?}",
          token
        ),
      }),
    }
  }

  /// Helper: Parse LCH chroma token (number)
  fn parse_lch_chroma_token(tokens: &mut TokenList) -> Result<f32, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected chroma value token".to_string(),
      })?;

    if let SimpleToken::Number(value) = token {
      Ok(value as f32)
    } else {
      Err(CssParseError::ParseError {
        message: format!("Expected Number token for chroma, got {:?}", token),
      })
    }
  }

  /// Helper: Parse LCH hue token (angle or number)
  fn parse_lch_hue_token(tokens: &mut TokenList) -> Result<LchHue, CssParseError> {
    let token = tokens
      .consume_next_token()?
      .ok_or(CssParseError::ParseError {
        message: "Expected hue value token".to_string(),
      })?;

    match token {
      SimpleToken::Dimension { value, unit } => {
        // Parse as angle with unit
        if crate::css_types::Angle::is_valid_unit(&unit) {
          let angle = crate::css_types::Angle::new(value as f32, unit);
          Ok(LchHue::Angle(angle))
        } else {
          Err(CssParseError::ParseError {
            message: format!("Invalid angle unit: {}", unit),
          })
        }
      }
      SimpleToken::Number(value) => {
        // Treat numbers as plain numbers (not degrees)
        Ok(LchHue::Number(value as f32))
      }
      _ => Err(CssParseError::ParseError {
        message: format!(
          "Expected Number or Dimension token for hue, got {:?}",
          token
        ),
      }),
    }
  }
}

impl Display for Lch {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.alpha {
      Some(alpha) => write!(f, "lch({} {} {} / {})", self.l, self.c, self.h, alpha),
      None => write!(f, "lch({} {} {})", self.l, self.c, self.h),
    }
  }
}

/// OKLCH color: oklch(0.5 0.1 270deg)
#[derive(Debug, Clone, PartialEq)]
pub struct Oklch {
  pub l: f32,             // lightness (0-1)
  pub c: f32,             // chroma (0-0.4)
  pub h: Angle,           // hue angle
  pub alpha: Option<f32>, // alpha (0-1)
}

impl Oklch {
  pub fn new(l: f32, c: f32, h: Angle, alpha: Option<f32>) -> Self {
    Self { l, c, h, alpha }
  }

  /// Parser for OKLCH colors
  /// - Lightness/Chroma: numbers, 'none' keyword (maps to 0)
  /// - Hue: angles or numbers (numbers * 360 -> angle conversion)
  /// - Alpha: optional with slash syntax
  pub fn parse() -> TokenParser<Oklch> {
    TokenParser::new(
      move |input| {
        // Parse 'oklch(' function start
        match input.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "oklch" => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected oklch() function".to_string(),
            });
          }
        }

        // Parse lightness (l): number | 'none'
        let l = Self::parse_oklch_lc_value(input)?;

        // Parse whitespace
        match input.consume_next_token()? {
          Some(SimpleToken::Whitespace) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected whitespace after lightness".to_string(),
            });
          }
        }

        // Parse chroma (c): number | 'none'
        let c = Self::parse_oklch_lc_value(input)?;

        // Parse whitespace
        match input.consume_next_token()? {
          Some(SimpleToken::Whitespace) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected whitespace after chroma".to_string(),
            });
          }
        }

        // Parse hue (h): angle | number (number * 360 -> angle)
        let h = Self::parse_oklch_hue(input)?;

        // Parse optional alpha: / <alpha-value>
        let alpha = Self::parse_optional_alpha(input)?;

        // Parse closing paren
        match input.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected ) after oklch values".to_string(),
            });
          }
        }

        Ok(Oklch::new(l, c, h, alpha))
      },
      "oklch_parser",
    )
  }

  /// Parse OKLCH lightness/chroma value: number | 'none'
  fn parse_oklch_lc_value(input: &mut crate::token_types::TokenList) -> Result<f32, CssParseError> {
    match input.consume_next_token()? {
      Some(SimpleToken::Number(n)) => Ok(n as f32),
      Some(SimpleToken::Ident(keyword)) if keyword == "none" => Ok(0.0),
      _ => Err(CssParseError::ParseError {
        message: "Expected number or 'none'".to_string(),
      }),
    }
  }

  /// Parse OKLCH hue: angle | number (number * 360 -> angle conversion)
  fn parse_oklch_hue(input: &mut crate::token_types::TokenList) -> Result<Angle, CssParseError> {
    match input.consume_next_token()? {
      Some(SimpleToken::Dimension { value, unit }) => {
        // Try to parse as angle
        if Angle::is_valid_unit(&unit) {
          Ok(Angle::new(value as f32, unit))
        } else {
          Err(CssParseError::ParseError {
            message: format!("Invalid angle unit: {}", unit),
          })
        }
      }
      Some(SimpleToken::Number(n)) => Ok(Angle::new((n as f32) * 360.0, "deg".to_string())),
      _ => Err(CssParseError::ParseError {
        message: "Expected hue: angle or number".to_string(),
      }),
    }
  }

  /// Parse optional alpha: / <alpha-value>
  fn parse_optional_alpha(
    input: &mut crate::token_types::TokenList,
  ) -> Result<Option<f32>, CssParseError> {
    // Check if there's a slash for alpha
    let checkpoint = input.current_index;

    // Skip optional whitespace
    if let Ok(Some(SimpleToken::Whitespace)) = input.peek() {
      input.consume_next_token()?;
    }

    match input.peek()? {
      Some(SimpleToken::Delim('/')) => {
        input.consume_next_token()?; // consume '/'

        // Skip optional whitespace after slash
        if let Ok(Some(SimpleToken::Whitespace)) = input.peek() {
          input.consume_next_token()?;
        }

        // Parse alpha value using enhanced alpha parser
        let alpha_parser = crate::css_types::alpha_value::alpha_as_number();
        match alpha_parser.run.as_ref()(input) {
          Ok(alpha) => Ok(Some(alpha)),
          Err(e) => Err(e),
        }
      }
      _ => {
        // No alpha, rewind to checkpoint
        input.set_current_index(checkpoint);
        Ok(None)
      }
    }
  }
}

impl Display for Oklch {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.alpha {
      Some(alpha) => write!(f, "oklch({} {} {} / {})", self.l, self.c, self.h, alpha),
      None => write!(f, "oklch({} {} {})", self.l, self.c, self.h),
    }
  }
}

/// OKLAB color: oklab(0.5 0.1 0.1)
#[derive(Debug, Clone, PartialEq)]
pub struct Oklab {
  pub l: f32,             // lightness (0-1)
  pub a: f32,             // green-red (-0.4 to 0.4)
  pub b: f32,             // blue-yellow (-0.4 to 0.4)
  pub alpha: Option<f32>, // alpha (0-1)
}

impl Oklab {
  pub fn new(l: f32, a: f32, b: f32, alpha: Option<f32>) -> Self {
    Self { l, a, b, alpha }
  }

  /// Parser for OKLAB colors
  /// - L/A/B values: numbers, 'none' keyword (maps to 0)
  /// - Alpha: optional with slash syntax
  pub fn parse() -> TokenParser<Oklab> {
    TokenParser::new(
      move |input| {
        // Parse 'oklab(' function start
        match input.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "oklab" => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected oklab() function".to_string(),
            });
          }
        }

        // Parse lightness (l): number | 'none'
        let l = Self::parse_oklab_lab_value(input)?;

        // Parse whitespace
        match input.consume_next_token()? {
          Some(SimpleToken::Whitespace) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected whitespace after lightness".to_string(),
            });
          }
        }

        // Parse a component (green-red): number | 'none'
        let a = Self::parse_oklab_lab_value(input)?;

        // Parse whitespace
        match input.consume_next_token()? {
          Some(SimpleToken::Whitespace) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected whitespace after a component".to_string(),
            });
          }
        }

        // Parse b component (blue-yellow): number | 'none'
        let b = Self::parse_oklab_lab_value(input)?;

        // Parse optional alpha: / <alpha-value>
        let alpha = Self::parse_optional_alpha(input)?;

        // Parse closing paren
        match input.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected ) after oklab values".to_string(),
            });
          }
        }

        Ok(Oklab::new(l, a, b, alpha))
      },
      "oklab_parser",
    )
  }

  /// Parse OKLAB l/a/b value: number | 'none'
  fn parse_oklab_lab_value(
    input: &mut crate::token_types::TokenList,
  ) -> Result<f32, CssParseError> {
    match input.consume_next_token()? {
      Some(SimpleToken::Number(n)) => Ok(n as f32),
      Some(SimpleToken::Ident(keyword)) if keyword == "none" => Ok(0.0),
      _ => Err(CssParseError::ParseError {
        message: "Expected number or 'none'".to_string(),
      }),
    }
  }

  /// Parse optional alpha: / <alpha-value>
  fn parse_optional_alpha(
    input: &mut crate::token_types::TokenList,
  ) -> Result<Option<f32>, CssParseError> {
    // Check if there's a slash for alpha
    let checkpoint = input.current_index;

    // Skip optional whitespace
    if let Ok(Some(SimpleToken::Whitespace)) = input.peek() {
      input.consume_next_token()?;
    }

    match input.peek()? {
      Some(SimpleToken::Delim('/')) => {
        input.consume_next_token()?; // consume '/'

        // Skip optional whitespace after slash
        if let Ok(Some(SimpleToken::Whitespace)) = input.peek() {
          input.consume_next_token()?;
        }

        // Parse alpha value using enhanced alpha parser
        let alpha_parser = crate::css_types::alpha_value::alpha_as_number();
        match alpha_parser.run.as_ref()(input) {
          Ok(alpha) => Ok(Some(alpha)),
          Err(e) => Err(e),
        }
      }
      _ => {
        // No alpha, rewind to checkpoint
        input.set_current_index(checkpoint);
        Ok(None)
      }
    }
  }
}

impl Display for Oklab {
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
    assert_eq!(short_color.g(), 0); // 0 -> 00
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
    let color = Rgb::new(255, 0, 0);
    assert_eq!(color.to_string(), "rgb(255,0,0)");
  }

  #[test]
  fn test_rgba_color_display() {
    let color = Rgba::new(255, 0, 0, 0.5);
    assert_eq!(color.to_string(), "rgba(255,0,0,0.5)");
  }

  #[test]
  fn test_hsl_color_display() {
    let color = Hsl::from_primitives(360.0, 100.0, 50.0);
    assert_eq!(color.to_string(), "hsl(360,100%,50%)");
  }

  #[test]
  fn test_hsla_color_display() {
    let color = Hsla::from_primitives(360.0, 100.0, 50.0, 0.8);
    assert_eq!(color.to_string(), "hsla(360,100%,50%,0.8)");
  }

  #[test]
  fn test_color_enum_display() {
    let named = Color::Named(NamedColor::new("red".to_string()));
    assert_eq!(named.to_string(), "red");

    let hash = Color::Hash(HashColor::new("FF0000".to_string()));
    assert_eq!(hash.to_string(), "#FF0000");

    let rgb = Color::Rgb(Rgb::new(255, 0, 0));
    assert_eq!(rgb.to_string(), "rgb(255,0,0)");
  }

  #[test]
  fn test_color_parsers_creation() {
    // Basic test that parsers can be created
    let _named = NamedColor::parse();
    let _hash = HashColor::parse();
    let _rgb = Rgb::parse();
    let _rgba = Rgba::parse();
    let _hsl = Hsl::parse();
    let _hsla = Hsla::parse();
    let _lch = Lch::parse();
    let _oklch = Oklch::parse();
    let _oklab = Oklab::parse();
    let _color = Color::parse();
  }

  #[test]
  fn test_lch_color_display() {
    let color = Lch::new_with_angle(50.0, 100.0, Angle::new(270.0, "deg".to_string()), None);
    assert_eq!(color.to_string(), "lch(50 100 270deg)");

    let color_with_alpha =
      Lch::new_with_angle(50.0, 100.0, Angle::new(270.0, "deg".to_string()), Some(0.8));
    assert_eq!(color_with_alpha.to_string(), "lch(50 100 270deg / 0.8)");
  }

  #[test]
  fn test_oklch_color_display() {
    let color = Oklch::new(0.5, 0.1, Angle::new(270.0, "deg".to_string()), None);
    assert_eq!(color.to_string(), "oklch(0.5 0.1 270deg)");

    let color_with_alpha = Oklch::new(0.5, 0.1, Angle::new(270.0, "deg".to_string()), Some(0.8));
    assert_eq!(color_with_alpha.to_string(), "oklch(0.5 0.1 270deg / 0.8)");
  }

  #[test]
  fn test_oklab_color_display() {
    let color = Oklab::new(0.5, 0.1, 0.1, None);
    assert_eq!(color.to_string(), "oklab(0.5 0.1 0.1)");

    let color_with_alpha = Oklab::new(0.5, 0.1, 0.1, Some(0.8));
    assert_eq!(color_with_alpha.to_string(), "oklab(0.5 0.1 0.1 / 0.8)");
  }
}
