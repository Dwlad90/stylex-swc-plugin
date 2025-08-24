/*!
CSS Resolution type parsing.

Handles resolution values with 'dpi' (dots per inch), 'dpcm' (dots per cm), and 'dppx' (dots per px) units.
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// Valid resolution units
pub const RESOLUTION_UNITS: &[&str] = &["dpi", "dpcm", "dppx"];

/// CSS Resolution value with unit
#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
  pub value: f32,
  pub unit: String, // "dpi", "dpcm", or "dppx"
}

impl Resolution {
  /// Create a new Resolution value
  pub fn new(value: f32, unit: String) -> Self {
    Self { value, unit }
  }

  /// All valid resolution units
  pub fn units() -> &'static [&'static str] {
    RESOLUTION_UNITS
  }

  /// Check if a unit is a valid resolution unit
  pub fn is_valid_unit(unit: &str) -> bool {
    RESOLUTION_UNITS.contains(&unit)
  }

  /// Parser for CSS resolution values
  pub fn parser() -> TokenParser<Resolution> {
    TokenParser::<SimpleToken>::token(
      SimpleToken::Dimension {
        value: 0.0,
        unit: String::new(),
      },
      Some("Dimension"),
    )
    .where_fn(
      |token| {
        if let SimpleToken::Dimension { unit, .. } = token {
          Self::is_valid_unit(unit)
        } else {
          false
        }
      },
      Some("valid_resolution_unit"),
    )
    .map(
      |token| {
        if let SimpleToken::Dimension { value, unit } = token {
          Resolution::new(value as f32, unit)
        } else {
          unreachable!()
        }
      },
      Some("to_resolution"),
    )
  }
}

impl Display for Resolution {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resolution_creation() {
    let res = Resolution::new(96.0, "dpi".to_string());
    assert_eq!(res.value, 96.0);
    assert_eq!(res.unit, "dpi");
  }

  #[test]
  fn test_resolution_display() {
    let dpi = Resolution::new(96.0, "dpi".to_string());
    assert_eq!(dpi.to_string(), "96dpi");

    let dpcm = Resolution::new(38.0, "dpcm".to_string());
    assert_eq!(dpcm.to_string(), "38dpcm");

    let dppx = Resolution::new(2.0, "dppx".to_string());
    assert_eq!(dppx.to_string(), "2dppx");
  }

  #[test]
  fn test_valid_resolution_units() {
    assert!(Resolution::is_valid_unit("dpi"));
    assert!(Resolution::is_valid_unit("dpcm"));
    assert!(Resolution::is_valid_unit("dppx"));

    // Invalid units
    assert!(!Resolution::is_valid_unit("px"));
    assert!(!Resolution::is_valid_unit("s"));
    assert!(!Resolution::is_valid_unit("Hz"));
    assert!(!Resolution::is_valid_unit("deg"));
  }

  #[test]
  fn test_resolution_units_constant() {
    let units = Resolution::units();
    assert_eq!(units.len(), 3);
    assert!(units.contains(&"dpi"));
    assert!(units.contains(&"dpcm"));
    assert!(units.contains(&"dppx"));
  }

  #[test]
  fn test_resolution_parser_creation() {
    // Basic test that parser can be created
    let _parser = Resolution::parser();
  }
}
