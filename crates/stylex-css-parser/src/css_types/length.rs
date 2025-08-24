/*!
CSS Length type parsing.

Handles all CSS length units including font-relative, viewport-relative,
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// Font-relative length units
pub const UNITS_BASED_ON_FONT: &[&str] = &["ch", "em", "ex", "ic", "lh", "rem", "rlh"];

pub const UNITS_BASED_ON_VIEWPORT: &[&str] = &[
  "vh", "svh", "lvh", "dvh", "vw", "svw", "lvw", "dvw", "vmin", "svmin", "lvmin", "dvmin", "vmax",
  "svmax", "lvmax", "dvmax",
];

/// Container-relative length units
pub const UNITS_BASED_ON_CONTAINER: &[&str] = &["cqw", "cqi", "cqh", "cqb", "cqmin", "cqmax"];

/// Absolute length units
pub const UNITS_BASED_ON_ABSOLUTE_UNITS: &[&str] = &["px", "cm", "mm", "in", "pt"];

/// CSS Length value with unit
#[derive(Debug, Clone, PartialEq)]
pub struct Length {
  pub value: f32,
  pub unit: String,
}

impl Length {
  /// Create a new Length value
  pub fn new(value: f32, unit: String) -> Self {
    Self { value, unit }
  }

  /// All valid length units
  pub fn units() -> Vec<&'static str> {
    let mut units = Vec::new();
    units.extend_from_slice(UNITS_BASED_ON_FONT);
    units.extend_from_slice(UNITS_BASED_ON_VIEWPORT);
    units.extend_from_slice(UNITS_BASED_ON_CONTAINER);
    units.extend_from_slice(UNITS_BASED_ON_ABSOLUTE_UNITS);
    units
  }

  /// Check if a unit is a valid length unit
  pub fn is_valid_unit(unit: &str) -> bool {
    Self::units().contains(&unit)
  }

  /// Parser for CSS length values
  pub fn parser() -> TokenParser<Length> {
    // Parser for dimension tokens with valid length units
    let dimension_parser = TokenParser::<SimpleToken>::token(
      SimpleToken::Dimension {
        value: 0.0,
        unit: String::new(),
      },
      Some("Dimension"),
    )
    .map(
      |token| {
        if let SimpleToken::Dimension { value, unit } = token {
          Some((value as f32, unit))
        } else {
          None
        }
      },
      Some("extract_dimension"),
    )
    .where_fn(
      |opt| {
        if let Some((_, unit)) = opt {
          Self::is_valid_unit(unit)
        } else {
          false
        }
      },
      Some("valid_length_unit"),
    )
    .map(
      |opt| {
        let (value, unit) = opt.unwrap();
        Length::new(value, unit)
      },
      Some("to_length"),
    );

    // Parser for zero without unit (special case for lengths)
    let zero_parser = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
      .where_fn(
        |token| {
          if let SimpleToken::Number(value) = token {
            *value == 0.0
          } else {
            false
          }
        },
        Some("zero_value"),
      )
      .map(|_| Length::new(0.0, String::new()), Some("zero_length"));

    // Combine both parsers
    TokenParser::one_of(vec![dimension_parser, zero_parser])
  }
}

impl Display for Length {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_length_creation() {
    let len = Length::new(16.0, "px".to_string());
    assert_eq!(len.value, 16.0);
    assert_eq!(len.unit, "px");
  }

  #[test]
  fn test_length_display() {
    let len = Length::new(16.0, "px".to_string());
    assert_eq!(len.to_string(), "16px");

    let zero_len = Length::new(0.0, String::new());
    assert_eq!(zero_len.to_string(), "0");
  }

  #[test]
  fn test_valid_units() {
    // Font-relative units
    assert!(Length::is_valid_unit("em"));
    assert!(Length::is_valid_unit("rem"));
    assert!(Length::is_valid_unit("ch"));

    assert!(Length::is_valid_unit("vh"));
    assert!(Length::is_valid_unit("vw"));
    assert!(Length::is_valid_unit("vmin"));

    // Container units
    assert!(Length::is_valid_unit("cqw"));
    assert!(Length::is_valid_unit("cqh"));

    // Absolute units
    assert!(Length::is_valid_unit("px"));
    assert!(Length::is_valid_unit("cm"));
    assert!(Length::is_valid_unit("in"));

    // Invalid units
    assert!(!Length::is_valid_unit("invalid"));
    assert!(!Length::is_valid_unit("deg"));
    assert!(!Length::is_valid_unit("s"));
  }

  #[test]
  fn test_units_constants() {
    assert!(UNITS_BASED_ON_FONT.contains(&"em"));
    assert!(UNITS_BASED_ON_FONT.contains(&"rem"));

    assert!(UNITS_BASED_ON_VIEWPORT.contains(&"vh"));
    assert!(UNITS_BASED_ON_VIEWPORT.contains(&"vw"));

    assert!(UNITS_BASED_ON_CONTAINER.contains(&"cqw"));

    assert!(UNITS_BASED_ON_ABSOLUTE_UNITS.contains(&"px"));
    assert!(UNITS_BASED_ON_ABSOLUTE_UNITS.contains(&"cm"));
  }

  #[test]
  fn test_length_parser_creation() {
    // Basic test that parser can be created
    let _parser = Length::parser();
  }

  #[test]
  fn test_all_units_included() {
    let all_units = Length::units();

    // Should include all categories
    assert!(all_units.len() > 20); // We have many units
    assert!(all_units.contains(&"px"));
    assert!(all_units.contains(&"em"));
    assert!(all_units.contains(&"vh"));
    assert!(all_units.contains(&"cqw"));
  }
}
