/*!
Universal CSS Value System for Dynamic Parsing

This module implements a flexible value system that can represent any CSS value type,
enabling dynamic parsing and type mixing in parser sequences.

Key features:
- Universal CssValue enum for any parsed result
- Convenient property access (value.as_number(), etc.)
- Dynamic type checking and conversion
- Support for heterogeneous parser sequences
*/

use crate::{
  css_types::{Angle, Color, Length, Percentage},
  token_types::SimpleToken,
};
use std::fmt;

/// Universal CSS value that can represent any parsed result

#[derive(Debug, Clone, PartialEq)]
pub enum CssValue {
  /// Raw number value: 42, 3.14, -5
  Number(f64),

  /// Percentage value: 50%, 100%
  Percentage(f64),

  /// Dimension with unit: 10px, 2em, 45deg
  Dimension {
    value: f64,
    unit: String,
  },

  /// String value: "Arial", 'Times New Roman'
  String(String),

  /// Identifier: auto, none, inherit
  Ident(String),

  /// Function with arguments: rgb(255, 0, 0), calc(1px + 2em)
  Function {
    name: String,
    args: Vec<CssValue>,
  },

  /// Sequence of values (for parser combinations)
  Sequence(Vec<CssValue>),

  /// Typed CSS values
  Angle(Angle),
  Color(Color),
  Length(Length),

  /// Unit tokens (for internal parsing)
  Token(SimpleToken),

  /// None/null value
  None,
}

impl CssValue {
  pub fn as_number(&self) -> Option<f64> {
    match self {
      CssValue::Number(n) => Some(*n),
      CssValue::Percentage(p) => Some(*p),
      CssValue::Dimension { value, .. } => Some(*value),
      _ => None,
    }
  }

  /// Extract percentage value: 50% -> 50.0
  pub fn as_percentage(&self) -> Option<f64> {
    match self {
      CssValue::Percentage(p) => Some(*p),
      _ => None,
    }
  }

  /// Extract string/ident value
  pub fn as_string(&self) -> Option<&String> {
    match self {
      CssValue::String(s) | CssValue::Ident(s) => Some(s),
      _ => None,
    }
  }

  /// Extract angle value
  pub fn as_angle(&self) -> Option<&Angle> {
    match self {
      CssValue::Angle(a) => Some(a),
      _ => None,
    }
  }

  /// Extract color value
  pub fn as_color(&self) -> Option<&Color> {
    match self {
      CssValue::Color(c) => Some(c),
      _ => None,
    }
  }

  /// Extract dimension parts
  pub fn as_dimension(&self) -> Option<(f64, &String)> {
    match self {
      CssValue::Dimension { value, unit } => Some((*value, unit)),
      _ => None,
    }
  }

  /// Extract sequence items
  pub fn as_sequence(&self) -> Option<&Vec<CssValue>> {
    match self {
      CssValue::Sequence(seq) => Some(seq),
      _ => None,
    }
  }

  /// Extract function name and args
  pub fn as_function(&self) -> Option<(&String, &Vec<CssValue>)> {
    match self {
      CssValue::Function { name, args } => Some((name, args)),
      _ => None,
    }
  }

  pub fn is_number(&self) -> bool {
    matches!(self, CssValue::Number(_))
  }

  pub fn is_percentage(&self) -> bool {
    matches!(self, CssValue::Percentage(_))
  }

  pub fn is_dimension(&self) -> bool {
    matches!(self, CssValue::Dimension { .. })
  }

  pub fn is_string(&self) -> bool {
    matches!(self, CssValue::String(_))
  }

  pub fn is_ident(&self) -> bool {
    matches!(self, CssValue::Ident(_))
  }

  pub fn is_function(&self) -> bool {
    matches!(self, CssValue::Function { .. })
  }

  pub fn is_sequence(&self) -> bool {
    matches!(self, CssValue::Sequence(_))
  }

  pub fn is_angle(&self) -> bool {
    matches!(self, CssValue::Angle(_))
  }

  pub fn is_color(&self) -> bool {
    matches!(self, CssValue::Color(_))
  }

  pub fn is_none(&self) -> bool {
    matches!(self, CssValue::None)
  }

  /// Check if has specific unit
  pub fn has_unit(&self, unit: &str) -> bool {
    match self {
      CssValue::Dimension { unit: u, .. } => u == unit,
      _ => false,
    }
  }

  /// Get unit if it's a dimension
  pub fn get_unit(&self) -> Option<&String> {
    match self {
      CssValue::Dimension { unit, .. } => Some(unit),
      _ => None,
    }
  }

  /// Convenience constructors
  pub fn number(value: f64) -> Self {
    CssValue::Number(value)
  }

  pub fn percentage(value: f64) -> Self {
    CssValue::Percentage(value)
  }

  pub fn dimension(value: f64, unit: impl Into<String>) -> Self {
    CssValue::Dimension {
      value,
      unit: unit.into(),
    }
  }

  pub fn string(value: impl Into<String>) -> Self {
    CssValue::String(value.into())
  }

  pub fn ident(value: impl Into<String>) -> Self {
    CssValue::Ident(value.into())
  }

  pub fn function(name: impl Into<String>, args: Vec<CssValue>) -> Self {
    CssValue::Function {
      name: name.into(),
      args,
    }
  }

  pub fn sequence(values: Vec<CssValue>) -> Self {
    CssValue::Sequence(values)
  }
}

impl fmt::Display for CssValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CssValue::Number(n) => write!(f, "{}", n),
      CssValue::Percentage(p) => write!(f, "{}%", p),
      CssValue::Dimension { value, unit } => write!(f, "{}{}", value, unit),
      CssValue::String(s) => write!(f, "\"{}\"", s),
      CssValue::Ident(s) => write!(f, "{}", s),
      CssValue::Function { name, args } => {
        write!(f, "{}(", name)?;
        for (i, arg) in args.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", arg)?;
        }
        write!(f, ")")
      }
      CssValue::Sequence(seq) => {
        for (i, item) in seq.iter().enumerate() {
          if i > 0 {
            write!(f, " ")?;
          }
          write!(f, "{}", item)?;
        }
        Ok(())
      }
      CssValue::Angle(a) => write!(f, "{}", a),
      CssValue::Color(c) => write!(f, "{}", c),
      CssValue::Length(l) => write!(f, "{}", l),
      CssValue::Token(t) => write!(f, "{:?}", t), // Debug format for tokens
      CssValue::None => write!(f, "none"),
    }
  }
}

/// Convert from specific CSS types to CssValue
impl From<f64> for CssValue {
  fn from(value: f64) -> Self {
    CssValue::Number(value)
  }
}

impl From<String> for CssValue {
  fn from(value: String) -> Self {
    CssValue::String(value)
  }
}

impl From<&str> for CssValue {
  fn from(value: &str) -> Self {
    CssValue::String(value.to_string())
  }
}

impl From<Angle> for CssValue {
  fn from(value: Angle) -> Self {
    CssValue::Angle(value)
  }
}

impl From<Color> for CssValue {
  fn from(value: Color) -> Self {
    CssValue::Color(value)
  }
}

impl From<Length> for CssValue {
  fn from(value: Length) -> Self {
    CssValue::Length(value)
  }
}

impl From<Percentage> for CssValue {
  fn from(value: Percentage) -> Self {
    CssValue::Percentage(value.value as f64)
  }
}

impl From<SimpleToken> for CssValue {
  fn from(token: SimpleToken) -> Self {
    match token {
      SimpleToken::Number(n) => CssValue::Number(n),
      SimpleToken::Percentage(p) => CssValue::Percentage(p),
      SimpleToken::Dimension { value, unit } => CssValue::Dimension { value, unit },
      SimpleToken::String(s) => CssValue::String(s),
      SimpleToken::Ident(s) => CssValue::Ident(s),
      _ => CssValue::Token(token),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_css_value_creation() {
    let num = CssValue::number(42.0);
    let percent = CssValue::percentage(50.0);
    let dim = CssValue::dimension(10.0, "px");
    let str_val = CssValue::string("Arial");
    let ident = CssValue::ident("auto");

    assert_eq!(num.as_number(), Some(42.0));
    assert_eq!(percent.as_percentage(), Some(50.0));
    assert_eq!(dim.as_dimension(), Some((10.0, &"px".to_string())));
    assert_eq!(str_val.as_string(), Some(&"Arial".to_string()));
    assert_eq!(ident.as_string(), Some(&"auto".to_string()));
  }

  #[test]
  fn test_type_checking() {
    let num = CssValue::number(42.0);
    let percent = CssValue::percentage(50.0);
    let dim = CssValue::dimension(10.0, "px");

    assert!(num.is_number());
    assert!(!num.is_percentage());
    assert!(!num.is_dimension());

    assert!(percent.is_percentage());
    assert!(!percent.is_number());

    assert!(dim.is_dimension());
    assert!(dim.has_unit("px"));
    assert!(!dim.has_unit("em"));
  }

  #[test]
  fn test_function_value() {
    let func = CssValue::function(
      "rgb",
      vec![
        CssValue::number(255.0),
        CssValue::number(0.0),
        CssValue::number(0.0),
      ],
    );

    assert!(func.is_function());

    if let Some((name, args)) = func.as_function() {
      assert_eq!(name, "rgb");
      assert_eq!(args.len(), 3);
      assert_eq!(args[0].as_number(), Some(255.0));
    }
  }

  #[test]
  fn test_sequence_value() {
    let seq = CssValue::sequence(vec![
      CssValue::number(1.0),
      CssValue::ident("solid"),
      CssValue::ident("red"),
    ]);

    assert!(seq.is_sequence());

    if let Some(items) = seq.as_sequence() {
      assert_eq!(items.len(), 3);
      assert!(items[0].is_number());
      assert!(items[1].is_ident());
      assert!(items[2].is_ident());
    }
  }

  #[test]
  fn test_display_formatting() {
    let num = CssValue::number(42.0);
    let percent = CssValue::percentage(50.0);
    let dim = CssValue::dimension(10.0, "px");
    let func = CssValue::function(
      "calc",
      vec![
        CssValue::dimension(1.0, "px"),
        CssValue::ident("+"),
        CssValue::dimension(2.0, "em"),
      ],
    );

    assert_eq!(num.to_string(), "42");
    assert_eq!(percent.to_string(), "50%");
    assert_eq!(dim.to_string(), "10px");
    assert_eq!(func.to_string(), "calc(1px, +, 2em)");
  }
}
