/*!
CSS Alpha value parsing.

Handles alpha values for colors - numbers (0.0-1.0) and percentages (0%-100%).
*/

use stylex_macros::stylex_unreachable;
use stylex_utils::number::to_js_string;

use crate::{
  CssParseError,
  token_parser::TokenParser,
  token_types::{SimpleToken, TokenList},
};
use std::fmt::{self, Display};

/// Alpha value for CSS colors
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue {
  pub value: f64, // 0.0 to 1.0
}

impl AlphaValue {
  /// Create a new AlphaValue
  pub fn new(value: f64) -> Self {
    Self { value }
  }

  /// Extract an `AlphaValue` from a `SimpleToken::Percentage`.
  ///
  /// Panics via `stylex_unreachable!` for any other token variant, which cannot
  /// occur through the public parser because the token combinator guarantees a
  /// `Percentage` token. The named function makes that defensive branch reachable
  /// from coverage tests.
  pub(crate) fn extract_percentage_token(token: SimpleToken) -> AlphaValue {
    if let SimpleToken::Percentage(value) = token {
      // An alpha is a fraction, and the token carries the authored percent, so
      // this is where `50%` becomes `0.5`.
      AlphaValue::new(value / 100.0)
    } else {
      stylex_unreachable!()
    }
  }

  /// Extract an `AlphaValue` from a `SimpleToken::Number`.
  ///
  /// Panics via `stylex_unreachable!` for any other token variant, which cannot
  /// occur through the public parser because the token combinator guarantees a
  /// `Number` token. The named function makes that defensive branch reachable
  /// from coverage tests.
  pub(crate) fn extract_number_token(token: SimpleToken) -> AlphaValue {
    if let SimpleToken::Number(value) = token {
      // Handle sign and use directly as alpha value
      AlphaValue::new(value)
    } else {
      stylex_unreachable!()
    }
  }

  /// Parser for alpha values
  pub fn parser() -> TokenParser<AlphaValue> {
    TokenParser::one_of(vec![
      // Percentage: v[4].signCharacter === '-' ? -1 : 1) * v[4].value) / 100
      TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
        .map(Self::extract_percentage_token, Some("percentage_to_alpha")),
      // Number: (v[4].signCharacter === '-' ? -1 : 1) * v[4].value
      TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
        .map(Self::extract_number_token, Some("number_to_alpha")),
    ])
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for AlphaValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", to_js_string(self.value))
  }
}

/// Helper function to get alpha as number for color parsing
pub fn alpha_as_number() -> TokenParser<f64> {
  AlphaValue::parser().map(|alpha| alpha.value, Some("alpha_to_number"))
}

/// Reads an alpha and refuses one outside `0..=1`, which is what `rgba()` and
/// `hsla()` require: they reject an out-of-range alpha rather than carrying it
/// through, where `alpha_as_number` above accepts whatever was written. A
/// difference in the legacy grammar, not in the width of the number.
///
/// The range is checked against the alpha, not against the token: `50%` is a
/// percentage token holding `50`, and it is the `0.5` it divides down to that
/// has to be in range.
///
/// Kept as a token reader rather than a `TokenParser` because the two range
/// arms carry different messages -- one names the fraction, the other the
/// percentage -- which a single combinator predicate could not say.
pub(crate) fn parse_alpha_in_unit_range(tokens: &mut TokenList) -> Result<f64, CssParseError> {
  let token = tokens
    .consume_next_token_infallible()
    .ok_or(CssParseError::ParseError {
      message: "Expected alpha value token".to_string(),
    })?;

  match token {
    SimpleToken::Number(value) => {
      if (0.0..=1.0).contains(&value) {
        Ok(value)
      } else {
        Err(CssParseError::ParseError {
          message: format!("Alpha number must be 0.0-1.0, got {}", value),
        })
      }
    },
    SimpleToken::Percentage(value) => {
      // An alpha is a fraction, and the token carries the authored percent.
      let value = value / 100.0;
      if (0.0..=1.0).contains(&value) {
        Ok(value)
      } else {
        Err(CssParseError::ParseError {
          message: format!(
            "Alpha percentage must be 0%-100% (stored as 0.0-1.0), got {}",
            value
          ),
        })
      }
    },
    _ => Err(CssParseError::ParseError {
      message: format!(
        "Expected Number or Percentage token for alpha, got {:?}",
        token
      ),
    }),
  }
}

#[cfg(test)]
#[path = "../tests/css_types/alpha_value_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/css_types/alpha_value_test.rs"]
mod alpha_value_test;

#[cfg(test)]
#[path = "../tests/css_types/alpha_value_coverage_test.rs"]
mod alpha_value_coverage_test;
