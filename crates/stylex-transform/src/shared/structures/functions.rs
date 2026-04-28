use std::{fmt::Debug, rc::Rc, sync::Arc};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{atoms::Atom, ecma::ast::Expr};

use crate::shared::structures::types::FlatCompiledStyles;
use stylex_enums::{
  js::{ArrayJS, MathJS, ObjectJS, StringJS},
  value_with_default::ValueWithDefault,
};

use super::types::{FunctionMapIdentifiers, FunctionMapMemberExpression};
use stylex_structures::stylex_env::JSFunction;

use stylex_types::traits::StyleOptions;

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum CallbackType {
  Array(ArrayJS),
  Object(ObjectJS),
  Math(MathJS),
  String(StringJS),
  Custom(Expr),
}

pub type StylexTypeFn = Rc<dyn Fn(ValueWithDefault) -> Expr + 'static>;
pub type StylexExprFn = fn(Expr, &mut dyn StyleOptions) -> Expr;

pub enum FunctionType {
  ArrayArgs(fn(Vec<Expr>, &mut dyn StyleOptions, &FunctionMap) -> Expr),
  StylexExprFn(StylexExprFn),
  StylexTypeFn(StylexTypeFn),
  StylexFnsFactory(fn(input: String) -> StylexTypeFn),

  Mapper(Rc<dyn Fn() -> Expr + 'static>),
  Callback(Box<CallbackType>),
  DefaultMarker(Arc<IndexMap<String, StylexExprFn>>),
  /// An env function from the `env` config option.
  /// Takes evaluated arguments as `Expr`s and returns an `Expr`.
  EnvFunction(JSFunction),
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Clone for FunctionType {
  fn clone(&self) -> Self {
    match self {
      Self::ArrayArgs(e) => Self::ArrayArgs(*e),
      Self::StylexExprFn(e) => Self::StylexExprFn(*e),
      Self::StylexTypeFn(e) => Self::StylexTypeFn(e.clone()),
      Self::StylexFnsFactory(e) => Self::StylexFnsFactory(*e),
      Self::Callback(v) => Self::Callback(v.clone()),
      Self::Mapper(c) => Self::Mapper(Rc::clone(c)),
      Self::DefaultMarker(e) => Self::DefaultMarker(Arc::clone(e)),
      Self::EnvFunction(e) => Self::EnvFunction(e.clone()),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl std::fmt::Debug for FunctionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FunctionType::ArrayArgs(_) => write!(f, "ArrayArgs"),
      FunctionType::StylexExprFn(_) => write!(f, "StylexExprWithStateFn"),
      FunctionType::StylexTypeFn(_) => write!(f, "StylexExprFn"),
      FunctionType::StylexFnsFactory(_) => write!(f, "StylexFnsFactory"),
      FunctionType::Mapper(_) => write!(f, "Mapper"),
      FunctionType::Callback(_) => write!(f, "Callback"),
      FunctionType::DefaultMarker(_) => write!(f, "DefaultMarker"),
      FunctionType::EnvFunction(_) => write!(f, "EnvFunction"),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for FunctionType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (FunctionType::ArrayArgs(_), FunctionType::ArrayArgs(_)) => false,
      (FunctionType::StylexExprFn(_), FunctionType::StylexExprFn(_)) => false,
      (FunctionType::StylexTypeFn(_), FunctionType::StylexTypeFn(_)) => false,
      (FunctionType::StylexFnsFactory(_), FunctionType::StylexFnsFactory(_)) => false,
      (FunctionType::Mapper(_), FunctionType::StylexExprFn(_)) => false,
      (FunctionType::Callback(_), FunctionType::Callback(_)) => false,
      (FunctionType::DefaultMarker(_), FunctionType::DefaultMarker(_)) => false,
      (FunctionType::EnvFunction(_), FunctionType::EnvFunction(_)) => false,
      _ => false,
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl std::hash::Hash for FunctionType {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::mem::discriminant(self).hash(state);
  }
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct FunctionConfig {
  pub fn_ptr: FunctionType,
  pub takes_path: bool,
}

pub enum FunctionConfigType {
  Regular(FunctionConfig),
  Map(FxHashMap<Atom, FunctionConfig>),
  IndexMap(FlatCompiledStyles),
  /// An env object from the `env` config option. Contains both values and
  /// functions.
  EnvObject(IndexMap<String, stylex_structures::stylex_env::EnvEntry>),
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl std::fmt::Debug for FunctionConfigType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Regular(config) => f.debug_tuple("Regular").field(config).finish(),
      Self::Map(map) => f.debug_tuple("Map").field(map).finish(),
      Self::IndexMap(map) => f.debug_tuple("IndexMap").field(map).finish(),
      Self::EnvObject(map) => f.debug_tuple("EnvObject").field(map).finish(),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Clone for FunctionConfigType {
  fn clone(&self) -> Self {
    match self {
      Self::Regular(config) => Self::Regular(config.clone()),
      Self::Map(map) => Self::Map(map.clone()),
      Self::IndexMap(map) => Self::IndexMap(map.clone()),
      Self::EnvObject(map) => Self::EnvObject(map.clone()),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for FunctionConfigType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Regular(a), Self::Regular(b)) => a == b,
      (Self::Map(a), Self::Map(b)) => a == b,
      (Self::IndexMap(a), Self::IndexMap(b)) => a == b,
      (Self::EnvObject(_), Self::EnvObject(_)) => false,
      _ => false,
    }
  }
}

impl FunctionConfigType {
  pub(crate) fn as_map_mut(&mut self) -> Option<&mut FxHashMap<Atom, FunctionConfig>> {
    match self {
      Self::Map(map) => Some(map),
      Self::Regular(_) | Self::IndexMap(_) | Self::EnvObject(_) => None,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct FunctionMap {
  pub identifiers: FunctionMapIdentifiers,
  pub member_expressions: FunctionMapMemberExpression,
  /// If `true`, disables the generation or processing of imports for this
  /// function map. Set to `true` when imports should not be generated (e.g.,
  /// for built-in or inlined functions). Set to `false` to allow normal
  /// import handling.
  pub disable_imports: bool,
}
