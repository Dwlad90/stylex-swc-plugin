/*!
Common CSS types and shared utilities.

This module implements the foundational types used across all CSS value parsing,
providing essential shared utilities for CSS processing.
*/

use crate::{
  token_parser::{TokenParser, tokens},
  token_types::SimpleToken,
};
use std::fmt::{self, Display};

/// CSS-wide keywords that can be used with any CSS property
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CssWideKeyword {
  Inherit,
  Initial,
  Unset,
  Revert,
}

impl Display for CssWideKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CssWideKeyword::Inherit => write!(f, "inherit"),
      CssWideKeyword::Initial => write!(f, "initial"),
      CssWideKeyword::Unset => write!(f, "unset"),
      CssWideKeyword::Revert => write!(f, "revert"),
    }
  }
}

impl CssWideKeyword {
  /// Parser for CSS-wide keywords
  pub fn parser() -> TokenParser<CssWideKeyword> {
    tokens::ident()
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            value
          } else {
            unreachable!()
          }
        },
        Some(".value"),
      )
      .where_fn(
        |value| matches!(value.as_str(), "inherit" | "initial" | "unset" | "revert"),
        Some("css_wide_keyword"),
      )
      .map(
        |value| match value.as_str() {
          "inherit" => CssWideKeyword::Inherit,
          "initial" => CssWideKeyword::Initial,
          "unset" => CssWideKeyword::Unset,
          "revert" => CssWideKeyword::Revert,
          _ => unreachable!(),
        },
        Some("to_keyword"),
      )
  }

  /// Parser specifically for 'inherit'
  pub fn inherit_parser() -> TokenParser<CssWideKeyword> {
    Self::parser().where_fn(
      |keyword| matches!(keyword, CssWideKeyword::Inherit),
      Some("inherit"),
    )
  }

  /// Parser specifically for 'initial'
  pub fn initial_parser() -> TokenParser<CssWideKeyword> {
    Self::parser().where_fn(
      |keyword| matches!(keyword, CssWideKeyword::Initial),
      Some("initial"),
    )
  }

  /// Parser specifically for 'unset'
  pub fn unset_parser() -> TokenParser<CssWideKeyword> {
    Self::parser().where_fn(
      |keyword| matches!(keyword, CssWideKeyword::Unset),
      Some("unset"),
    )
  }

  /// Parser specifically for 'revert'
  pub fn revert_parser() -> TokenParser<CssWideKeyword> {
    Self::parser().where_fn(
      |keyword| matches!(keyword, CssWideKeyword::Revert),
      Some("revert"),
    )
  }
}

/// CSS 'auto' keyword
pub fn auto_parser() -> TokenParser<String> {
  TokenParser::<String>::string("auto").map(|_| "auto".to_string(), Some("auto_keyword"))
}

/// CSS variable reference: var(--name)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssVariable {
  pub name: String,
}

impl CssVariable {
  pub fn new(name: String) -> Self {
    Self { name }
  }

  /// Parser for CSS variables: var(--name)
  pub fn parser() -> TokenParser<CssVariable> {
    // var(
    let fn_var = crate::token_parser::TokenParser::<String>::fn_name("var");

    // --ident
    let dashed_ident = crate::token_parser::TokenParser::<crate::token_types::SimpleToken>::token(
      crate::token_types::SimpleToken::Ident(String::new()),
      Some("Ident"),
    )
    .map(
      |tok| {
        if let crate::token_types::SimpleToken::Ident(s) = tok {
          s
        } else {
          String::new()
        }
      },
      Some(".value"),
    )
    .where_fn(|s| s.starts_with("--"), Some("starts_with_--"));

    // )
    let close_paren = crate::token_parser::TokenParser::<crate::token_types::SimpleToken>::token(
      crate::token_types::SimpleToken::RightParen,
      Some("RightParen"),
    );

    fn_var
      .flat_map(move |_| dashed_ident.clone(), Some("name"))
      .flat_map(
        move |name| close_paren.map(move |_| name.clone(), Some(")")),
        Some("close"),
      )
      .map(CssVariable::new, Some("to_css_variable"))
  }
}

impl Display for CssVariable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "var({})", self.name)
  }
}

/// CSS percentage value
#[derive(Debug, Clone, PartialEq)]
pub struct Percentage {
  pub value: f32,
}

impl Percentage {
  pub fn new(value: f32) -> Self {
    Self { value }
  }

  /// Parser for percentage values
  pub fn parser() -> TokenParser<Percentage> {
    tokens::percentage().map(
      |token| {
        if let SimpleToken::Percentage(value) = token {
          // cssparser stores percentage as unit_value (already converted: 50% = 0.50)

          Percentage::new((value * 100.0) as f32)
        } else {
          unreachable!()
        }
      },
      Some("to_percentage"),
    )
  }
}

impl Display for Percentage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}%", self.value)
  }
}

/// CSS number value
#[derive(Debug, Clone, PartialEq)]
pub struct Number {
  pub value: f32,
}

impl Number {
  pub fn new(value: f32) -> Self {
    Self { value }
  }

  /// Parser for number values
  pub fn parser() -> TokenParser<Number> {
    tokens::number().map(
      |token| {
        if let SimpleToken::Number(value) = token {
          Number::new(value as f32)
        } else {
          unreachable!()
        }
      },
      Some("to_number"),
    )
  }
}

impl Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

/// Union type for number or percentage values
#[derive(Debug, Clone, PartialEq)]
pub enum NumberOrPercentage {
  Number(Number),
  Percentage(Percentage),
}

impl Display for NumberOrPercentage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      NumberOrPercentage::Number(n) => n.fmt(f),
      NumberOrPercentage::Percentage(p) => p.fmt(f),
    }
  }
}

/// Parser for number or percentage values
pub fn number_or_percentage_parser() -> TokenParser<NumberOrPercentage> {
  TokenParser::one_of(vec![
    Number::parser().map(NumberOrPercentage::Number, Some("number")),
    Percentage::parser().map(NumberOrPercentage::Percentage, Some("percentage")),
  ])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_css_wide_keyword_display() {
    assert_eq!(CssWideKeyword::Inherit.to_string(), "inherit");
    assert_eq!(CssWideKeyword::Initial.to_string(), "initial");
    assert_eq!(CssWideKeyword::Unset.to_string(), "unset");
    assert_eq!(CssWideKeyword::Revert.to_string(), "revert");
  }

  #[test]
  fn test_percentage_display() {
    let p = Percentage::new(50.0);
    assert_eq!(p.to_string(), "50%");
  }

  #[test]
  fn test_number_display() {
    let n = Number::new(42.5);
    assert_eq!(n.to_string(), "42.5");
  }

  #[test]
  fn test_css_variable_display() {
    let var = CssVariable::new("--main-color".to_string());
    assert_eq!(var.to_string(), "var(--main-color)");
  }

  #[test]
  fn test_css_wide_keyword_parser() {
    // Basic test that parser can be created
    let _parser = CssWideKeyword::parser();
    let _inherit = CssWideKeyword::inherit_parser();
    let _initial = CssWideKeyword::initial_parser();
    let _unset = CssWideKeyword::unset_parser();
    let _revert = CssWideKeyword::revert_parser();
  }

  #[test]
  fn test_number_percentage_parsers() {
    // Basic test that parsers can be created
    let _number = Number::parser();
    let _percentage = Percentage::parser();
    let _number_or_percentage = number_or_percentage_parser();
  }

  #[test]
  fn test_auto_parser() {
    // Basic test that parser can be created
    let _parser = auto_parser();
  }

  #[test]
  fn test_number_or_percentage_display() {
    let num = NumberOrPercentage::Number(Number::new(42.0));
    let pct = NumberOrPercentage::Percentage(Percentage::new(50.0));

    assert_eq!(num.to_string(), "42");
    assert_eq!(pct.to_string(), "50%");
  }
}
