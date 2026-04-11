/*!
CSS Resolution type parsing.

Handles resolution values with 'dpi' (dots per inch), 'dpcm' (dots per cm), and 'dppx' (dots per px) units.
*/

use stylex_macros::stylex_unreachable;

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
  pub fn new(value: f32, unit: impl Into<String>) -> Self {
    Self {
      value,
      unit: unit.into(),
    }
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
          stylex_unreachable!()
        }
      },
      Some("to_resolution"),
    )
  }
}

#[cfg(not(tarpaulin_include))]
impl Display for Resolution {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[cfg(test)]
#[path = "../tests/css_types/resolution_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/css_types/resolution_test.rs"]
mod resolution_test;
