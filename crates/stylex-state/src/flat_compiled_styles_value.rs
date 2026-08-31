use swc_core::ecma::ast::Expr;

use stylex_styleq::StyleqValue;

use stylex_structures::{base_css_type::BaseCSSType, pair::Pair};
use stylex_types::structures::injectable_style::InjectableStyle;

use stylex_enums::css_syntax::CSSSyntax;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum FlatCompiledStylesValue {
  String(String),
  KeyValue(Pair),
  KeyValues(Vec<Pair>),
  Null,
  InjectableStyle(InjectableStyle),
  Bool(bool),
  Tuple(String, Box<Expr>, Option<BaseCSSType>),
  CSSType(String, CSSSyntax, String),
}

impl FlatCompiledStylesValue {
  pub fn as_tuple(&self) -> Option<(&String, &Expr, &Option<BaseCSSType>)> {
    match self {
      FlatCompiledStylesValue::Tuple(key, value, css_type) => Some((key, value, css_type)),
      _ => None,
    }
  }

  pub fn as_string(&self) -> Option<&String> {
    match self {
      FlatCompiledStylesValue::String(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_injectable_style(&self) -> Option<&InjectableStyle> {
    match self {
      FlatCompiledStylesValue::InjectableStyle(value) => Some(value),
      _ => None,
    }
  }

  pub(crate) fn _as_bool(&self) -> Option<&bool> {
    match self {
      FlatCompiledStylesValue::Bool(value) => Some(value),
      _ => None,
    }
  }

  pub(crate) fn _as_null(&self) -> Option<()> {
    match self {
      FlatCompiledStylesValue::Null => Some(()),
      _ => None,
    }
  }

  pub fn as_key_value(&self) -> Option<&Pair> {
    match self {
      FlatCompiledStylesValue::KeyValue(value) => Some(value),
      _ => None,
    }
  }
  pub fn as_key_values(&self) -> Option<&Vec<Pair>> {
    match self {
      FlatCompiledStylesValue::KeyValues(value) => Some(value),
      _ => None,
    }
  }
}

impl StyleqValue for FlatCompiledStylesValue {
  fn as_class_name(&self) -> Option<&str> {
    match self {
      FlatCompiledStylesValue::String(value) => Some(value.as_str()),
      _ => None,
    }
  }

  fn is_null(&self) -> bool {
    matches!(self, FlatCompiledStylesValue::Null)
  }

  fn is_true_bool(&self) -> bool {
    matches!(self, FlatCompiledStylesValue::Bool(true))
  }
}
