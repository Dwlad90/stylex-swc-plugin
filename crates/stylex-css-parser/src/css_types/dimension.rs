/*!
CSS Dimension type parsing.

Handles dimensional values that can be lengths, times, frequencies, or resolutions.
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

use super::{frequency::Frequency, length::Length, resolution::Resolution, time::Time};

/// Union type for all dimensional CSS values
#[derive(Debug, Clone, PartialEq)]
pub enum Dimension {
  Length(Length),
  Time(Time),
  Frequency(Frequency),
  Resolution(Resolution),
}

impl Dimension {
  /// Check if a unit belongs to any dimension type
  #[allow(dead_code)]
  fn is_valid_dimension_unit(unit: &str) -> bool {
    Length::is_valid_unit(unit)
      || Time::is_valid_unit(unit)
      || Frequency::is_valid_unit(unit)
      || Resolution::is_valid_unit(unit)
  }

  /// Create a Dimension from value and unit
  fn from_value_and_unit(value: f32, unit: String) -> Option<Dimension> {
    if Length::is_valid_unit(&unit) {
      Some(Dimension::Length(Length::new(value, unit)))
    } else if Time::is_valid_unit(&unit) {
      Some(Dimension::Time(Time::new(value, unit)))
    } else if Frequency::is_valid_unit(&unit) {
      Some(Dimension::Frequency(Frequency::new(value, unit)))
    } else if Resolution::is_valid_unit(&unit) {
      Some(Dimension::Resolution(Resolution::new(value, unit)))
    } else {
      None
    }
  }

  /// Parser for dimensional values
  pub fn parse() -> TokenParser<Dimension> {
    use crate::token_parser::tokens;

    tokens::dimension()
      .map(
        |token| {
          if let SimpleToken::Dimension { value, unit } = token {
            Self::from_value_and_unit(value as f32, unit)
          } else {
            None
          }
        },
        Some("extract_dimension"),
      )
      .where_fn(|opt| opt.is_some(), Some("valid_dimension"))
      .map(|opt| opt.unwrap(), Some("unwrap_dimension"))
  }
}

pub fn dimension() -> TokenParser<Dimension> {
  Dimension::parse()
}

impl Display for Dimension {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Dimension::Length(length) => length.fmt(f),
      Dimension::Time(time) => time.fmt(f),
      Dimension::Frequency(frequency) => frequency.fmt(f),
      Dimension::Resolution(resolution) => resolution.fmt(f),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dimension_from_value_and_unit() {
    // Length
    let length_dim = Dimension::from_value_and_unit(16.0, "px".to_string());
    assert!(matches!(length_dim, Some(Dimension::Length(_))));

    // Time
    let time_dim = Dimension::from_value_and_unit(1.5, "s".to_string());
    assert!(matches!(time_dim, Some(Dimension::Time(_))));

    // Frequency
    let freq_dim = Dimension::from_value_and_unit(440.0, "Hz".to_string());
    assert!(matches!(freq_dim, Some(Dimension::Frequency(_))));

    // Resolution
    let res_dim = Dimension::from_value_and_unit(96.0, "dpi".to_string());
    assert!(matches!(res_dim, Some(Dimension::Resolution(_))));

    // Invalid unit
    let invalid_dim = Dimension::from_value_and_unit(10.0, "invalid".to_string());
    assert!(invalid_dim.is_none());
  }

  #[test]
  fn test_dimension_display() {
    let length = Dimension::Length(Length::new(16.0, "px".to_string()));
    assert_eq!(length.to_string(), "16px");

    let time = Dimension::Time(Time::new(1.5, "s".to_string()));
    assert_eq!(time.to_string(), "1.5s");

    let freq = Dimension::Frequency(Frequency::new(440.0, "Hz".to_string()));
    assert_eq!(freq.to_string(), "0.44KHz");

    let res = Dimension::Resolution(Resolution::new(96.0, "dpi".to_string()));
    assert_eq!(res.to_string(), "96dpi");
  }

  #[test]
  fn test_is_valid_dimension_unit() {
    // Length units
    assert!(Dimension::is_valid_dimension_unit("px"));
    assert!(Dimension::is_valid_dimension_unit("em"));
    assert!(Dimension::is_valid_dimension_unit("vh"));

    // Time units
    assert!(Dimension::is_valid_dimension_unit("s"));
    assert!(Dimension::is_valid_dimension_unit("ms"));

    // Frequency units
    assert!(Dimension::is_valid_dimension_unit("Hz"));
    assert!(Dimension::is_valid_dimension_unit("KHz"));

    // Resolution units
    assert!(Dimension::is_valid_dimension_unit("dpi"));
    assert!(Dimension::is_valid_dimension_unit("dpcm"));
    assert!(Dimension::is_valid_dimension_unit("dppx"));

    // Invalid units
    assert!(!Dimension::is_valid_dimension_unit("invalid"));
    assert!(!Dimension::is_valid_dimension_unit("deg")); // This is an angle unit, handled by Angle type separately
  }

  #[test]
  fn test_dimension_parser_creation() {
    // Basic test that parser can be created
    let _parser = Dimension::parse();
  }

  #[test]
  fn test_dimension_equality() {
    let dim1 = Dimension::Length(Length::new(16.0, "px".to_string()));
    let dim2 = Dimension::Length(Length::new(16.0, "px".to_string()));
    let dim3 = Dimension::Length(Length::new(20.0, "px".to_string()));

    assert_eq!(dim1, dim2);
    assert_ne!(dim1, dim3);
  }
}
