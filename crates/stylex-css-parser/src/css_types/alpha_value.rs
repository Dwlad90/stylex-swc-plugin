use super::common_types::Percentage;
use crate::parser::Parser;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A parser that validates a float value is between 0 and 1
pub fn alpha_number<'a>() -> Parser<'a, f32> {
  Parser::<f32>::float().where_fn(|v| *v >= 0.0 && *v <= 1.0)
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue {
  pub value: f32,
}

impl AlphaValue {
  pub fn new(value: f32) -> Self {
    AlphaValue { value }
  }

  /// Parse an alpha value (either percentage or number between 0 and 1)
  pub fn parse<'a>() -> Parser<'a, AlphaValue> {
    // Create a parser that tries either a percentage or direct alpha number
    Parser::one_of(vec![
      Percentage::parse()
        .map(|p| p.expect("Percentage value must be between 0 and 100").value / 100.0),
      alpha_number(),
    ])
    .map(|v| AlphaValue::new(v.expect("Alpha value must be between 0 and 1")))
  }
}

// Implement Display for string conversion
impl Display for AlphaValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.value)
  }
}
