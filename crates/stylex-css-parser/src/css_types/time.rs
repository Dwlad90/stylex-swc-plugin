/*!
CSS Time type parsing.

Handles time values with 's' (seconds) and 'ms' (milliseconds) units.
Mirrors: packages/style-value-parser/src/css-types/time.js
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// Valid time units
pub const TIME_UNITS: &[&str] = &["s", "ms"];

/// CSS Time value with unit
/// Mirrors: Time class in time.js
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
  pub value: f32,
  pub unit: String, // "s" or "ms"
}

impl Time {
  /// Create a new Time value
  pub fn new(value: f32, unit: String) -> Self {
    Self { value, unit }
  }

  /// All valid time units
  /// Mirrors: Time.UNITS
  pub fn units() -> &'static [&'static str] {
    TIME_UNITS
  }

  /// Check if a unit is a valid time unit
  pub fn is_valid_unit(unit: &str) -> bool {
    TIME_UNITS.contains(&unit)
  }

  /// Parser for CSS time values
  /// Mirrors: Time.parser
  pub fn parser() -> TokenParser<Time> {
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
      Some("extract_time_dimension"),
    )
    .where_fn(|opt| opt.is_some(), Some("valid_time"))
    .map(
      |opt| {
        let (value, unit) = opt.unwrap();
        Time::new(value, unit)
      },
      Some("to_time"),
    )
  }
}

impl Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Always use the shortest representation (as per JS implementation)
    if self.unit == "ms" && self.value >= 1000.0 && self.value % 1000.0 == 0.0 {
      write!(f, "{}s", self.value / 1000.0)
    } else {
      write!(f, "{}{}", self.value, self.unit)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_time_creation() {
    let time = Time::new(1.5, "s".to_string());
    assert_eq!(time.value, 1.5);
    assert_eq!(time.unit, "s");
  }

  #[test]
  fn test_time_display() {
    let seconds = Time::new(1.5, "s".to_string());
    assert_eq!(seconds.to_string(), "1.5s");

    let milliseconds = Time::new(500.0, "ms".to_string());
    assert_eq!(milliseconds.to_string(), "500ms");

    // Test shortest representation conversion
    let full_second = Time::new(1000.0, "ms".to_string());
    assert_eq!(full_second.to_string(), "1s");

    let two_seconds = Time::new(2000.0, "ms".to_string());
    assert_eq!(two_seconds.to_string(), "2s");

    // Should not convert if not a whole second
    let partial = Time::new(1500.0, "ms".to_string());
    assert_eq!(partial.to_string(), "1500ms");
  }

  #[test]
  fn test_valid_time_units() {
    assert!(Time::is_valid_unit("s"));
    assert!(Time::is_valid_unit("ms"));

    // Invalid units
    assert!(!Time::is_valid_unit("px"));
    assert!(!Time::is_valid_unit("deg"));
    assert!(!Time::is_valid_unit("Hz"));
  }

  #[test]
  fn test_time_units_constant() {
    let units = Time::units();
    assert_eq!(units.len(), 2);
    assert!(units.contains(&"s"));
    assert!(units.contains(&"ms"));
  }

  #[test]
  fn test_time_parser_creation() {
    // Basic test that parser can be created
    let _parser = Time::parser();
  }
}
