/*!
CSS Frequency type parsing.

Handles frequency values with 'Hz' (hertz) and 'KHz' (kilohertz) units.
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// Valid frequency units
pub const FREQUENCY_UNITS: &[&str] = &["Hz", "KHz"];

/// CSS Frequency value with unit
#[derive(Debug, Clone, PartialEq)]
pub struct Frequency {
  pub value: f32,
  pub unit: String, // "Hz" or "KHz"
}

impl Frequency {
  /// Create a new Frequency value
  pub fn new(value: f32, unit: String) -> Self {
    Self { value, unit }
  }

  /// All valid frequency units
  pub fn units() -> &'static [&'static str] {
    FREQUENCY_UNITS
  }

  /// Check if a unit is a valid frequency unit
  pub fn is_valid_unit(unit: &str) -> bool {
    FREQUENCY_UNITS.contains(&unit)
  }

  /// Parser for CSS frequency values
  pub fn parser() -> TokenParser<Frequency> {
    TokenParser::<SimpleToken>::token(
      SimpleToken::Dimension {
        value: 0.0,
        unit: String::new(),
      },
      Some("Dimension"),
    )
    .map(
      |token| {
        if let SimpleToken::Dimension { value, unit } = token {
          if Self::is_valid_unit(&unit) {
            Some((value as f32, unit))
          } else {
            None
          }
        } else {
          None
        }
      },
      Some("extract_frequency_dimension"),
    )
    .where_fn(|opt| opt.is_some(), Some("valid_frequency"))
    .map(
      |opt| {
        let (value, unit) = opt.unwrap();
        Frequency::new(value, unit)
      },
      Some("to_frequency"),
    )
  }
}

impl Display for Frequency {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.unit == "Hz" {
      write!(f, "{}KHz", self.value / 1000.0)
    } else {
      write!(f, "{}{}", self.value, self.unit)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_frequency_creation() {
    let freq = Frequency::new(440.0, "Hz".to_string());
    assert_eq!(freq.value, 440.0);
    assert_eq!(freq.unit, "Hz");
  }

  #[test]
  fn test_frequency_display() {
    let hertz = Frequency::new(440.0, "Hz".to_string());
    assert_eq!(hertz.to_string(), "0.44KHz");

    let kilohertz = Frequency::new(2.4, "KHz".to_string());
    assert_eq!(kilohertz.to_string(), "2.4KHz");

    let full_kilohertz = Frequency::new(1000.0, "Hz".to_string());
    assert_eq!(full_kilohertz.to_string(), "1KHz");

    let two_kilohertz = Frequency::new(2000.0, "Hz".to_string());
    assert_eq!(two_kilohertz.to_string(), "2KHz");

    let partial = Frequency::new(1500.0, "Hz".to_string());
    assert_eq!(partial.to_string(), "1.5KHz");
  }

  #[test]
  fn test_valid_frequency_units() {
    assert!(Frequency::is_valid_unit("Hz"));
    assert!(Frequency::is_valid_unit("KHz"));

    // Invalid units
    assert!(!Frequency::is_valid_unit("px"));
    assert!(!Frequency::is_valid_unit("s"));
    assert!(!Frequency::is_valid_unit("deg"));
  }

  #[test]
  fn test_frequency_units_constant() {
    let units = Frequency::units();
    assert_eq!(units.len(), 2);
    assert!(units.contains(&"Hz"));
    assert!(units.contains(&"KHz"));
  }

  #[test]
  fn test_frequency_parser_creation() {
    // Basic test that parser can be created
    let _parser = Frequency::parser();
  }
}
