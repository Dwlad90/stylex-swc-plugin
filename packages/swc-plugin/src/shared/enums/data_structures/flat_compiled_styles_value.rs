use swc_ecma_ast::Expr;

use crate::shared::structures::{
  base_css_type::BaseCSSType, included_style::IncludedStyle, injectable_style::InjectableStyle,
  pair::Pair,
};

use super::css_syntax::CSSSyntax;

#[derive(Debug, PartialEq, Clone, Hash)]
pub(crate) enum FlatCompiledStylesValue {
  String(String),
  KeyValue(Pair),
  Null,
  IncludedStyle(IncludedStyle),
  InjectableStyle(InjectableStyle),
  Bool(bool),
  Tuple(String, Box<Expr>, Option<BaseCSSType>),
  CSSType(String, CSSSyntax, String),
}

impl FlatCompiledStylesValue {
  pub(crate) fn as_tuple(&self) -> Option<(&String, &Expr, &Option<BaseCSSType>)> {
    match self {
      FlatCompiledStylesValue::Tuple(key, value, css_type) => Some((key, value, css_type)),
      _ => None,
    }
  }

  #[cfg(test)]
  pub(crate) fn as_string(&self) -> Option<&String> {
    match self {
      FlatCompiledStylesValue::String(value) => Some(value),
      _ => None,
    }
  }

  pub(crate) fn as_injectable_style(&self) -> Option<&InjectableStyle> {
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

  pub(crate) fn _as_included_style(&self) -> Option<&IncludedStyle> {
    match self {
      FlatCompiledStylesValue::IncludedStyle(value) => Some(value),
      _ => None,
    }
  }

  pub(crate) fn as_key_value(&self) -> Option<&Pair> {
    match self {
      FlatCompiledStylesValue::KeyValue(value) => Some(value),
      _ => None,
    }
  }
}
