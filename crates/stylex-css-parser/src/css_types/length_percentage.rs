/*!
CSS Length-Percentage union type parsing.

Handles values that can be either length or percentage values.
*/

use crate::{
  css_types::{Length, Percentage, calc::Calc},
  token_parser::TokenParser,
};
use std::fmt::{self, Display};

/// Union type for length or percentage values
#[derive(Debug, Clone, PartialEq)]
pub enum LengthPercentage {
  Length(Length),
  Percentage(Percentage),
  Calc(Calc),
}

impl LengthPercentage {
  /// Parser for length or percentage values
  pub fn parser() -> TokenParser<LengthPercentage> {
    TokenParser::one_of(vec![
      Calc::parser().map(LengthPercentage::Calc, Some("calc")),
      Percentage::parser().map(LengthPercentage::Percentage, Some("percentage")),
      Length::parser().map(LengthPercentage::Length, Some("length")),
    ])
  }

  /// Check if this is a length value
  pub fn is_length(&self) -> bool {
    matches!(self, LengthPercentage::Length(_))
  }

  /// Check if this is a percentage value
  pub fn is_percentage(&self) -> bool {
    matches!(self, LengthPercentage::Percentage(_))
  }

  /// Check if this is a calc value
  pub fn is_calc(&self) -> bool {
    matches!(self, LengthPercentage::Calc(_))
  }

  /// Get the length value if this is a length, None otherwise
  pub fn as_length(&self) -> Option<&Length> {
    match self {
      LengthPercentage::Length(length) => Some(length),
      _ => None,
    }
  }

  /// Get the percentage value if this is a percentage, None otherwise
  pub fn as_percentage(&self) -> Option<&Percentage> {
    match self {
      LengthPercentage::Percentage(percentage) => Some(percentage),
      _ => None,
    }
  }

  /// Get the calc value if this is a calc, None otherwise
  pub fn as_calc(&self) -> Option<&Calc> {
    match self {
      LengthPercentage::Calc(calc) => Some(calc),
      _ => None,
    }
  }
}

impl Display for LengthPercentage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LengthPercentage::Length(length) => length.fmt(f),
      LengthPercentage::Percentage(percentage) => percentage.fmt(f),
      LengthPercentage::Calc(calc) => calc.fmt(f),
    }
  }
}

/// Convenience function for creating the parser
pub fn length_percentage_parser() -> TokenParser<LengthPercentage> {
  LengthPercentage::parser()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_length_percentage_creation() {
    let length = LengthPercentage::Length(Length::new(16.0, "px".to_string()));
    let percentage = LengthPercentage::Percentage(Percentage::new(50.0));

    assert!(length.is_length());
    assert!(!length.is_percentage());

    assert!(!percentage.is_length());
    assert!(percentage.is_percentage());
  }

  #[test]
  fn test_length_percentage_display() {
    let length = LengthPercentage::Length(Length::new(16.0, "px".to_string()));
    assert_eq!(length.to_string(), "16px");

    let percentage = LengthPercentage::Percentage(Percentage::new(50.0));
    assert_eq!(percentage.to_string(), "50%");

    let zero_length = LengthPercentage::Length(Length::new(0.0, String::new()));
    assert_eq!(zero_length.to_string(), "0");
  }

  #[test]
  fn test_length_percentage_accessors() {
    let length_val = Length::new(20.0, "em".to_string());
    let percentage_val = Percentage::new(75.0);

    let length = LengthPercentage::Length(length_val.clone());
    let percentage = LengthPercentage::Percentage(percentage_val.clone());

    // Test as_length
    assert_eq!(length.as_length(), Some(&length_val));
    assert_eq!(percentage.as_length(), None);

    // Test as_percentage
    assert_eq!(length.as_percentage(), None);
    assert_eq!(percentage.as_percentage(), Some(&percentage_val));
  }

  #[test]
  fn test_length_percentage_parser_creation() {
    // Test both creation methods
    let _parser1 = LengthPercentage::parser();
    let _parser2 = length_percentage_parser();
  }

  #[test]
  fn test_length_percentage_equality() {
    let length1 = LengthPercentage::Length(Length::new(16.0, "px".to_string()));
    let length2 = LengthPercentage::Length(Length::new(16.0, "px".to_string()));
    let length3 = LengthPercentage::Length(Length::new(20.0, "px".to_string()));

    let percentage1 = LengthPercentage::Percentage(Percentage::new(50.0));
    let percentage2 = LengthPercentage::Percentage(Percentage::new(50.0));

    assert_eq!(length1, length2);
    assert_ne!(length1, length3);
    assert_ne!(length1, percentage1);
    assert_eq!(percentage1, percentage2);
  }

  #[test]
  fn test_length_percentage_edge_cases() {
    // Zero length
    let zero_length = LengthPercentage::Length(Length::new(0.0, String::new()));
    assert!(zero_length.is_length());
    assert_eq!(zero_length.to_string(), "0");

    // Zero percentage
    let zero_percentage = LengthPercentage::Percentage(Percentage::new(0.0));
    assert!(zero_percentage.is_percentage());
    assert_eq!(zero_percentage.to_string(), "0%");

    // Different length units
    let px_length = LengthPercentage::Length(Length::new(16.0, "px".to_string()));
    let em_length = LengthPercentage::Length(Length::new(1.0, "em".to_string()));
    assert_ne!(px_length, em_length);
  }
}
