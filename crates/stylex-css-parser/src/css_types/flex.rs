/*!
CSS Flex type parsing.

Handles flex grid fraction values (e.g., 1fr, 2.5fr).
*/

use stylex_macros::stylex_unreachable;

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// CSS flex fraction unit (e.g., 1fr)
#[derive(Debug, Clone, PartialEq)]
pub struct Flex {
  pub fraction: f32,
}

impl Flex {
  /// Create a new Flex value
  pub fn new(fraction: f32) -> Self {
    Self { fraction }
  }

  /// Check if a fraction value is valid for flex
  /// Flex values must be non-negative
  pub fn is_valid_fraction(fraction: f32) -> bool {
    fraction >= 0.0
  }

  /// Parser for flex fraction values
  pub fn parser() -> TokenParser<Flex> {
    TokenParser::<SimpleToken>::token(
      SimpleToken::Dimension {
        value: 0.0,
        unit: String::new(),
      },
      Some("Dimension"),
    )
    .where_fn(
      |token| {
        if let SimpleToken::Dimension { value, unit } = token {
          unit == "fr" && *value >= 0.0
        } else {
          false
        }
      },
      Some("valid_fr_unit"),
    )
    .map(
      |token| {
        if let SimpleToken::Dimension { value, unit: _ } = token {
          Flex::new(value as f32)
        } else {
          stylex_unreachable!()
        }
      },
      Some("to_flex"),
    )
  }
}

#[cfg(not(tarpaulin_include))]
impl Display for Flex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}fr", self.fraction)
  }
}

#[cfg(test)]
#[path = "../tests/css_types/flex_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/css_types/flex_test.rs"]
mod flex_test;
