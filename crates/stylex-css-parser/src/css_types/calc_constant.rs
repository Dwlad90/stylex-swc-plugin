/*!
CSS Calc Constants type parsing.

Handles calc constants like 'pi', 'e', 'infinity', '-infinity', 'NaN'.
*/

use crate::token_parser::TokenParser;
use std::fmt::{self, Display};

/// Valid calc constants
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcConstant {
  Pi,
  E,
  Infinity,
  NegativeInfinity,
  NaN,
}

impl CalcConstant {
  /// All valid calc constants as strings
  pub fn all_constants() -> &'static [&'static str] {
    &["pi", "e", "infinity", "-infinity", "NaN"]
  }

  /// Convert from string representation
  pub fn from_str(s: &str) -> Option<CalcConstant> {
    match s {
      "pi" => Some(CalcConstant::Pi),
      "e" => Some(CalcConstant::E),
      "infinity" => Some(CalcConstant::Infinity),
      "-infinity" => Some(CalcConstant::NegativeInfinity),
      "NaN" => Some(CalcConstant::NaN),
      _ => None,
    }
  }

  /// Convert to string representation
  pub fn as_str(&self) -> &'static str {
    match self {
      CalcConstant::Pi => "pi",
      CalcConstant::E => "e",
      CalcConstant::Infinity => "infinity",
      CalcConstant::NegativeInfinity => "-infinity",
      CalcConstant::NaN => "NaN",
    }
  }

  /// Check if a string is a valid calc constant
  pub fn is_valid_constant(s: &str) -> bool {
    Self::all_constants().contains(&s)
  }

  /// Parser for calc constants
  pub fn parser() -> TokenParser<CalcConstant> {
    TokenParser::<CalcConstant>::one_of(vec![
      // Order matters - check longer strings first to avoid partial matches
      TokenParser::<String>::string("-infinity").map(
        |_| CalcConstant::NegativeInfinity,
        Some("to_negative_infinity"),
      ),
      TokenParser::<String>::string("infinity")
        .map(|_| CalcConstant::Infinity, Some("to_infinity")),
      TokenParser::<String>::string("pi").map(|_| CalcConstant::Pi, Some("to_pi")),
      TokenParser::<String>::string("e").map(|_| CalcConstant::E, Some("to_e")),
      TokenParser::<String>::string("NaN").map(|_| CalcConstant::NaN, Some("to_nan")),
    ])
  }
}

impl Display for CalcConstant {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_constant_from_str() {
    assert_eq!(CalcConstant::from_str("pi"), Some(CalcConstant::Pi));
    assert_eq!(CalcConstant::from_str("e"), Some(CalcConstant::E));
    assert_eq!(
      CalcConstant::from_str("infinity"),
      Some(CalcConstant::Infinity)
    );
    assert_eq!(
      CalcConstant::from_str("-infinity"),
      Some(CalcConstant::NegativeInfinity)
    );
    assert_eq!(CalcConstant::from_str("NaN"), Some(CalcConstant::NaN));

    // Invalid constants
    assert_eq!(CalcConstant::from_str("invalid"), None);
    assert_eq!(CalcConstant::from_str("PI"), None); // Case sensitive
    assert_eq!(CalcConstant::from_str(""), None);
  }

  #[test]
  fn test_calc_constant_as_str() {
    assert_eq!(CalcConstant::Pi.as_str(), "pi");
    assert_eq!(CalcConstant::E.as_str(), "e");
    assert_eq!(CalcConstant::Infinity.as_str(), "infinity");
    assert_eq!(CalcConstant::NegativeInfinity.as_str(), "-infinity");
    assert_eq!(CalcConstant::NaN.as_str(), "NaN");
  }

  #[test]
  fn test_calc_constant_display() {
    assert_eq!(CalcConstant::Pi.to_string(), "pi");
    assert_eq!(CalcConstant::E.to_string(), "e");
    assert_eq!(CalcConstant::Infinity.to_string(), "infinity");
    assert_eq!(CalcConstant::NegativeInfinity.to_string(), "-infinity");
    assert_eq!(CalcConstant::NaN.to_string(), "NaN");
  }

  #[test]
  fn test_calc_constant_is_valid() {
    assert!(CalcConstant::is_valid_constant("pi"));
    assert!(CalcConstant::is_valid_constant("e"));
    assert!(CalcConstant::is_valid_constant("infinity"));
    assert!(CalcConstant::is_valid_constant("-infinity"));
    assert!(CalcConstant::is_valid_constant("NaN"));

    // Invalid
    assert!(!CalcConstant::is_valid_constant("invalid"));
    assert!(!CalcConstant::is_valid_constant("PI"));
    assert!(!CalcConstant::is_valid_constant(""));
  }

  #[test]
  fn test_calc_constant_all_constants() {
    let constants = CalcConstant::all_constants();
    assert_eq!(constants.len(), 5);
    assert!(constants.contains(&"pi"));
    assert!(constants.contains(&"e"));
    assert!(constants.contains(&"infinity"));
    assert!(constants.contains(&"-infinity"));
    assert!(constants.contains(&"NaN"));
  }

  #[test]
  fn test_calc_constant_parser_creation() {
    // Basic test that parser can be created
    let _parser = CalcConstant::parser();
  }

  #[test]
  fn test_calc_constant_equality() {
    let pi1 = CalcConstant::Pi;
    let pi2 = CalcConstant::Pi;
    let e = CalcConstant::E;

    assert_eq!(pi1, pi2);
    assert_ne!(pi1, e);
  }

  #[test]
  fn test_calc_constant_round_trip() {
    // Test that from_str and as_str are consistent
    for constant_str in CalcConstant::all_constants() {
      let constant = CalcConstant::from_str(constant_str).unwrap();
      assert_eq!(constant.as_str(), *constant_str);
    }
  }

  #[test]
  fn test_calc_constant_math_constants() {
    // Test mathematical constants specifically
    assert_eq!(CalcConstant::Pi.as_str(), "pi");
    assert_eq!(CalcConstant::E.as_str(), "e");

    // Test special values
    assert_eq!(CalcConstant::Infinity.as_str(), "infinity");
    assert_eq!(CalcConstant::NegativeInfinity.as_str(), "-infinity");
    assert_eq!(CalcConstant::NaN.as_str(), "NaN");
  }
}
