/*!
CSS Alpha value parsing.

Handles alpha values for colors - numbers (0.0-1.0) and percentages (0%-100%).
*/

use stylex_macros::stylex_unreachable;

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// Alpha value for CSS colors
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue {
  pub value: f32, // 0.0 to 1.0
}

impl AlphaValue {
  /// Create a new AlphaValue
  pub fn new(value: f32) -> Self {
    Self { value }
  }

  /// Parser for alpha values
  pub fn parser() -> TokenParser<AlphaValue> {
    TokenParser::one_of(vec![
      // Percentage: v[4].signCharacter === '-' ? -1 : 1) * v[4].value) / 100
      TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage")).map(
        |token| {
          if let SimpleToken::Percentage(value) = token {
            // Handle sign and convert to alpha value (0.0-1.0)
            // cssparser stores percentage as unit_value (already converted: 50% = 0.50)

            AlphaValue::new(value as f32)
          } else {
            stylex_unreachable!()
          }
        },
        Some("percentage_to_alpha"),
      ),
      // Number: (v[4].signCharacter === '-' ? -1 : 1) * v[4].value
      TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number")).map(
        |token| {
          if let SimpleToken::Number(value) = token {
            // Handle sign and use directly as alpha value
            AlphaValue::new(value as f32)
          } else {
            stylex_unreachable!()
          }
        },
        Some("number_to_alpha"),
      ),
    ])
  }
}

#[cfg(not(tarpaulin_include))]
impl Display for AlphaValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

/// Helper function to get alpha as number for color parsing
pub fn alpha_as_number() -> TokenParser<f32> {
  AlphaValue::parser().map(|alpha| alpha.value, Some("alpha_to_f32"))
}

#[cfg(test)]
#[path = "../tests/css_types/alpha_value_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/css_types/alpha_value_test.rs"]
mod alpha_value_test;
