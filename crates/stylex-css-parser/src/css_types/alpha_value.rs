/*!
CSS Alpha value parsing.

Handles alpha values for colors - numbers (0.0-1.0) and percentages (0%-100%).
*/

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
            unreachable!()
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
            unreachable!()
          }
        },
        Some("number_to_alpha"),
      ),
    ])
  }
}

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
mod tests {
  use super::*;

  #[test]
  fn test_alpha_value_creation() {
    let alpha1 = AlphaValue::new(0.5);
    let alpha2 = AlphaValue::new(1.0);
    let alpha3 = AlphaValue::new(0.0);

    assert_eq!(alpha1.value, 0.5);
    assert_eq!(alpha2.value, 1.0);
    assert_eq!(alpha3.value, 0.0);
  }

  #[test]
  fn test_alpha_display() {
    let alpha = AlphaValue::new(0.75);
    assert_eq!(alpha.to_string(), "0.75");
  }

  #[test]
  fn test_parser_creation() {
    // Test that the parser can be created
    let _parser = AlphaValue::parser();
    let _alpha_as_num = alpha_as_number();
  }

  #[test]
  fn test_alpha_value_equality() {
    let alpha1 = AlphaValue::new(0.5);
    let alpha2 = AlphaValue::new(0.5);
    let alpha3 = AlphaValue::new(0.75);

    assert_eq!(alpha1, alpha2);
    assert_ne!(alpha1, alpha3);
  }
}
