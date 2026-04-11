/*!
CSS Angle type parsing.

Handles angle values with 'deg' (degrees), 'grad' (gradians), 'rad' (radians), and 'turn' (turns) units.
*/

use stylex_macros::stylex_unreachable;

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// Valid angle units
pub const ANGLE_UNITS: &[&str] = &["deg", "grad", "rad", "turn"];

/// CSS Angle value with unit
#[derive(Debug, Clone, PartialEq)]
pub struct Angle {
  pub value: f32,
  pub unit: String, // "deg", "grad", "rad", or "turn"
}

impl Angle {
  /// Create a new Angle value
  pub fn new(value: f32, unit: impl Into<String>) -> Self {
    Self {
      value,
      unit: unit.into(),
    }
  }

  /// All valid angle units
  pub fn units() -> &'static [&'static str] {
    ANGLE_UNITS
  }

  /// Check if a unit is a valid angle unit
  pub fn is_valid_unit(unit: &str) -> bool {
    ANGLE_UNITS.contains(&unit)
  }

  /// Parser for CSS angle values
  pub fn parser() -> TokenParser<Angle> {
    // Parser for dimension tokens with valid angle units
    let dimension_parser = TokenParser::<SimpleToken>::token(
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
      Some("valid_angle_unit"),
    )
    .map(
      |token| {
        if let SimpleToken::Dimension { value, unit } = token {
          Angle::new(value as f32, unit)
        } else {
          stylex_unreachable!()
        }
      },
      Some("to_angle"),
    );

    // Parser for zero without unit (special case for angles - defaults to 'deg')
    let zero_parser = TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
      .where_fn(
        |token| {
          if let SimpleToken::Number(value) = token {
            *value == 0.0
          } else {
            false
          }
        },
        Some("zero_value"),
      )
      .map(|_| Angle::new(0.0, "deg".to_string()), Some("zero_angle"));

    // Combine both parsers
    TokenParser::one_of(vec![dimension_parser, zero_parser])
  }
}

#[cfg(not(tarpaulin_include))]
impl Display for Angle {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[cfg(test)]
#[path = "../tests/css_types/angle_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/css_types/angle_test.rs"]
mod angle_test;
