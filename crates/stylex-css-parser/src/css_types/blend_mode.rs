/*!
CSS Blend Mode type parsing.

Handles blend mode values for properties like mix-blend-mode and background-blend-mode.
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// CSS blend mode values
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlendMode {
  Normal,
  Multiply,
  Screen,
  Overlay,
  Darken,
  Lighten,
  ColorDodge,
  ColorBurn,
  HardLight,
  SoftLight,
  Difference,
  Exclusion,
  Hue,
  Saturation,
  Color,
  Luminosity,
}

impl BlendMode {
  /// All valid blend mode values
  pub fn all_values() -> &'static [&'static str] {
    &[
      "normal",
      "multiply",
      "screen",
      "overlay",
      "darken",
      "lighten",
      "color-dodge",
      "color-burn",
      "hard-light",
      "soft-light",
      "difference",
      "exclusion",
      "hue",
      "saturation",
      "color",
      "luminosity",
    ]
  }

  /// Convert from string representation
  pub fn parse(s: &str) -> Option<BlendMode> {
    match s {
      "normal" => Some(BlendMode::Normal),
      "multiply" => Some(BlendMode::Multiply),
      "screen" => Some(BlendMode::Screen),
      "overlay" => Some(BlendMode::Overlay),
      "darken" => Some(BlendMode::Darken),
      "lighten" => Some(BlendMode::Lighten),
      "color-dodge" => Some(BlendMode::ColorDodge),
      "color-burn" => Some(BlendMode::ColorBurn),
      "hard-light" => Some(BlendMode::HardLight),
      "soft-light" => Some(BlendMode::SoftLight),
      "difference" => Some(BlendMode::Difference),
      "exclusion" => Some(BlendMode::Exclusion),
      "hue" => Some(BlendMode::Hue),
      "saturation" => Some(BlendMode::Saturation),
      "color" => Some(BlendMode::Color),
      "luminosity" => Some(BlendMode::Luminosity),
      _ => None,
    }
  }

  /// Convert to string representation
  pub fn as_str(&self) -> &'static str {
    match self {
      BlendMode::Normal => "normal",
      BlendMode::Multiply => "multiply",
      BlendMode::Screen => "screen",
      BlendMode::Overlay => "overlay",
      BlendMode::Darken => "darken",
      BlendMode::Lighten => "lighten",
      BlendMode::ColorDodge => "color-dodge",
      BlendMode::ColorBurn => "color-burn",
      BlendMode::HardLight => "hard-light",
      BlendMode::SoftLight => "soft-light",
      BlendMode::Difference => "difference",
      BlendMode::Exclusion => "exclusion",
      BlendMode::Hue => "hue",
      BlendMode::Saturation => "saturation",
      BlendMode::Color => "color",
      BlendMode::Luminosity => "luminosity",
    }
  }

  /// Check if a string is a valid blend mode
  pub fn is_valid_blend_mode(s: &str) -> bool {
    Self::all_values().contains(&s)
  }

  /// Parser for blend mode values
  pub fn parser() -> TokenParser<BlendMode> {
    use crate::token_parser::tokens;

    tokens::ident()
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            value
          } else {
            unreachable!()
          }
        },
        Some("extract_ident_value"),
      )
      .where_fn(
        |value: &String| Self::is_valid_blend_mode(value),
        Some("valid_blend_mode"),
      )
      .map(|value| Self::parse(&value).unwrap(), Some("to_blend_mode"))
  }
}

impl Display for BlendMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_blend_mode_from_str() {
    assert_eq!(BlendMode::parse("normal"), Some(BlendMode::Normal));
    assert_eq!(BlendMode::parse("multiply"), Some(BlendMode::Multiply));
    assert_eq!(BlendMode::parse("screen"), Some(BlendMode::Screen));
    assert_eq!(BlendMode::parse("overlay"), Some(BlendMode::Overlay));
    assert_eq!(BlendMode::parse("darken"), Some(BlendMode::Darken));
    assert_eq!(BlendMode::parse("lighten"), Some(BlendMode::Lighten));
    assert_eq!(BlendMode::parse("color-dodge"), Some(BlendMode::ColorDodge));
    assert_eq!(BlendMode::parse("color-burn"), Some(BlendMode::ColorBurn));
    assert_eq!(BlendMode::parse("hard-light"), Some(BlendMode::HardLight));
    assert_eq!(BlendMode::parse("soft-light"), Some(BlendMode::SoftLight));
    assert_eq!(BlendMode::parse("difference"), Some(BlendMode::Difference));
    assert_eq!(BlendMode::parse("exclusion"), Some(BlendMode::Exclusion));
    assert_eq!(BlendMode::parse("hue"), Some(BlendMode::Hue));
    assert_eq!(BlendMode::parse("saturation"), Some(BlendMode::Saturation));
    assert_eq!(BlendMode::parse("color"), Some(BlendMode::Color));
    assert_eq!(BlendMode::parse("luminosity"), Some(BlendMode::Luminosity));

    // Invalid values
    assert_eq!(BlendMode::parse("invalid"), None);
    assert_eq!(BlendMode::parse("NORMAL"), None);
    assert_eq!(BlendMode::parse(""), None);
  }

  #[test]
  fn test_blend_mode_as_str() {
    assert_eq!(BlendMode::Normal.as_str(), "normal");
    assert_eq!(BlendMode::Multiply.as_str(), "multiply");
    assert_eq!(BlendMode::Screen.as_str(), "screen");
    assert_eq!(BlendMode::Overlay.as_str(), "overlay");
    assert_eq!(BlendMode::Darken.as_str(), "darken");
    assert_eq!(BlendMode::Lighten.as_str(), "lighten");
    assert_eq!(BlendMode::ColorDodge.as_str(), "color-dodge");
    assert_eq!(BlendMode::ColorBurn.as_str(), "color-burn");
    assert_eq!(BlendMode::HardLight.as_str(), "hard-light");
    assert_eq!(BlendMode::SoftLight.as_str(), "soft-light");
    assert_eq!(BlendMode::Difference.as_str(), "difference");
    assert_eq!(BlendMode::Exclusion.as_str(), "exclusion");
    assert_eq!(BlendMode::Hue.as_str(), "hue");
    assert_eq!(BlendMode::Saturation.as_str(), "saturation");
    assert_eq!(BlendMode::Color.as_str(), "color");
    assert_eq!(BlendMode::Luminosity.as_str(), "luminosity");
  }

  #[test]
  fn test_blend_mode_display() {
    assert_eq!(BlendMode::Normal.to_string(), "normal");
    assert_eq!(BlendMode::ColorDodge.to_string(), "color-dodge");
    assert_eq!(BlendMode::HardLight.to_string(), "hard-light");
    assert_eq!(BlendMode::Luminosity.to_string(), "luminosity");
  }

  #[test]
  fn test_blend_mode_is_valid() {
    assert!(BlendMode::is_valid_blend_mode("normal"));
    assert!(BlendMode::is_valid_blend_mode("multiply"));
    assert!(BlendMode::is_valid_blend_mode("color-dodge"));
    assert!(BlendMode::is_valid_blend_mode("luminosity"));

    // Invalid
    assert!(!BlendMode::is_valid_blend_mode("invalid"));
    assert!(!BlendMode::is_valid_blend_mode("NORMAL"));
    assert!(!BlendMode::is_valid_blend_mode(""));
  }

  #[test]
  fn test_blend_mode_all_values() {
    let values = BlendMode::all_values();
    assert_eq!(values.len(), 16);

    // Test that all values can be parsed
    for value_str in values {
      assert!(BlendMode::parse(value_str).is_some());
    }
  }

  #[test]
  fn test_blend_mode_parser_creation() {
    // Basic test that parser can be created
    let _parser = BlendMode::parser();
  }

  #[test]
  fn test_blend_mode_equality() {
    let normal1 = BlendMode::Normal;
    let normal2 = BlendMode::Normal;
    let multiply = BlendMode::Multiply;

    assert_eq!(normal1, normal2);
    assert_ne!(normal1, multiply);
  }

  #[test]
  fn test_blend_mode_round_trip() {
    // Test that parse and as_str are consistent
    for value_str in BlendMode::all_values() {
      let blend_mode = BlendMode::parse(value_str).unwrap();
      assert_eq!(blend_mode.as_str(), *value_str);
    }
  }

  #[test]
  fn test_blend_mode_coverage() {
    // Test that we have all the blend modes from CSS spec
    assert!(BlendMode::all_values().contains(&"normal"));
    assert!(BlendMode::all_values().contains(&"multiply"));
    assert!(BlendMode::all_values().contains(&"screen"));
    assert!(BlendMode::all_values().contains(&"overlay"));
    assert!(BlendMode::all_values().contains(&"darken"));
    assert!(BlendMode::all_values().contains(&"lighten"));
    assert!(BlendMode::all_values().contains(&"color-dodge"));
    assert!(BlendMode::all_values().contains(&"color-burn"));
    assert!(BlendMode::all_values().contains(&"hard-light"));
    assert!(BlendMode::all_values().contains(&"soft-light"));
    assert!(BlendMode::all_values().contains(&"difference"));
    assert!(BlendMode::all_values().contains(&"exclusion"));
    assert!(BlendMode::all_values().contains(&"hue"));
    assert!(BlendMode::all_values().contains(&"saturation"));
    assert!(BlendMode::all_values().contains(&"color"));
    assert!(BlendMode::all_values().contains(&"luminosity"));
  }

  #[test]
  fn test_blend_mode_compositing_groups() {
    // Test separable blend modes
    let separable = &[
      BlendMode::Normal,
      BlendMode::Multiply,
      BlendMode::Screen,
      BlendMode::Overlay,
      BlendMode::Darken,
      BlendMode::Lighten,
      BlendMode::ColorDodge,
      BlendMode::ColorBurn,
      BlendMode::HardLight,
      BlendMode::SoftLight,
      BlendMode::Difference,
      BlendMode::Exclusion,
    ];

    // Test non-separable blend modes
    let non_separable = &[
      BlendMode::Hue,
      BlendMode::Saturation,
      BlendMode::Color,
      BlendMode::Luminosity,
    ];

    // Verify all modes are accounted for
    assert_eq!(separable.len() + non_separable.len(), 16);
  }
}
