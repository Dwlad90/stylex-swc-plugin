use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::{
  atoms::Atom,
  ecma::ast::{Expr, KeyValueProp, Lit},
};

use crate::shared::enums::data_structures::evaluate_result_value::EvaluateResultValue;

use super::{
  functions::FunctionConfig,
  theme_ref::ThemeRef,
  types::{DynamicFns, EvaluationCallback, TInlineStyles},
};

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluateResult {
  pub(crate) confident: bool,
  pub value: Option<Box<EvaluateResultValue>>,
  pub(crate) deopt: Option<Box<Expr>>,
  pub(crate) inline_styles: Option<TInlineStyles>,
  pub(crate) fns: Option<DynamicFns>,
}

impl EvaluateResultValue {
  pub fn as_expr(&self) -> Option<&Expr> {
    match self {
      EvaluateResultValue::Expr(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_vec(&self) -> Option<&Vec<Option<EvaluateResultValue>>> {
    match self {
      EvaluateResultValue::Vec(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_map(&self) -> Option<&IndexMap<Box<Expr>, Vec<KeyValueProp>>> {
    match self {
      EvaluateResultValue::Map(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_entries(&self) -> Option<&IndexMap<Box<Lit>, Box<Lit>>> {
    match self {
      EvaluateResultValue::Entries(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_function(&self) -> Option<&FunctionConfig> {
    match self {
      EvaluateResultValue::FunctionConfig(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_function_map(&self) -> Option<&HashMap<Atom, FunctionConfig>> {
    match self {
      EvaluateResultValue::FunctionConfigMap(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_callback(&self) -> Option<&EvaluationCallback> {
    match self {
      EvaluateResultValue::Callback(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_theme_ref(&self) -> Option<&ThemeRef> {
    match self {
      EvaluateResultValue::ThemeRef(value) => Some(value),
      _ => None,
    }
  }
}
