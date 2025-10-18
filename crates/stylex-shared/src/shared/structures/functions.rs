use std::{fmt::Debug, rc::Rc, sync::Arc};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{atoms::Atom, ecma::ast::Expr};

use crate::shared::{
  enums::{
    data_structures::value_with_default::ValueWithDefault,
    js::{ArrayJS, MathJS, ObjectJS, StringJS},
  },
  structures::types::FlatCompiledStyles,
};

use super::{
  state_manager::StateManager,
  types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
};

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum CallbackType {
  Array(ArrayJS),
  Object(ObjectJS),
  Math(MathJS),
  String(StringJS),
  Custom(Expr),
}

pub type StylexTypeFn = Rc<dyn Fn(ValueWithDefault) -> Expr + 'static>;
pub type StylexExprFn = fn(Expr, &mut StateManager) -> Expr;

pub enum FunctionType {
  ArrayArgs(fn(Vec<Expr>) -> Expr),
  StylexExprFn(StylexExprFn),
  StylexTypeFn(StylexTypeFn),
  StylexFnsFactory(fn(input: String) -> StylexTypeFn),

  Mapper(Rc<dyn Fn() -> Expr + 'static>),
  Callback(Box<CallbackType>),
  DefaultMarker(Arc<IndexMap<String, StylexExprFn>>),
}

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
    }
  }
}

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
    }
  }
}

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
      _ => false,
    }
  }
}

impl std::hash::Hash for FunctionType {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      FunctionType::ArrayArgs(_) => {
        std::mem::discriminant(self).hash(state);
      }
      FunctionType::StylexExprFn(_) => {
        std::mem::discriminant(self).hash(state);
      }
      FunctionType::StylexTypeFn(_) => {
        std::mem::discriminant(self).hash(state);
      }
      FunctionType::StylexFnsFactory(_) => {
        std::mem::discriminant(self).hash(state);
      }
      FunctionType::Mapper(_) => {
        std::mem::discriminant(self).hash(state);
      }
      FunctionType::Callback(_) => {
        std::mem::discriminant(self).hash(state);
      }
      FunctionType::DefaultMarker(_) => {
        std::mem::discriminant(self).hash(state);
      }
    }
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
}

impl std::fmt::Debug for FunctionConfigType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Regular(config) => f.debug_tuple("Regular").field(config).finish(),
      Self::Map(map) => f.debug_tuple("Map").field(map).finish(),
      Self::IndexMap(map) => f.debug_tuple("IndexMap").field(map).finish(),
    }
  }
}

impl Clone for FunctionConfigType {
  fn clone(&self) -> Self {
    match self {
      Self::Regular(config) => Self::Regular(config.clone()),
      Self::Map(map) => Self::Map(map.clone()),
      Self::IndexMap(map) => Self::IndexMap(map.clone()),
    }
  }
}

impl PartialEq for FunctionConfigType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Regular(a), Self::Regular(b)) => a == b,
      (Self::Map(a), Self::Map(b)) => a == b,
      (Self::IndexMap(a), Self::IndexMap(b)) => a == b,
      _ => false,
    }
  }
}

impl FunctionConfigType {
  pub(crate) fn _as_function_config(&self) -> Option<&FunctionConfig> {
    match self {
      Self::Regular(config) => Some(config),
      Self::Map(_) => None,
      Self::IndexMap(_) => None,
    }
  }

  pub(crate) fn _as_map(&self) -> Option<&FxHashMap<Atom, FunctionConfig>> {
    match self {
      Self::Regular(_) => None,
      Self::Map(map) => Some(map),
      Self::IndexMap(_) => None,
    }
  }

  pub(crate) fn as_map_mut(&mut self) -> Option<&mut FxHashMap<Atom, FunctionConfig>> {
    match self {
      Self::Regular(_) => None,
      Self::Map(map) => Some(map),
      Self::IndexMap(_) => None,
    }
  }

  pub(crate) fn _as_function_config_mut(&mut self) -> Option<&mut FunctionConfig> {
    match self {
      Self::Regular(config) => Some(config),
      Self::Map(_) => None,
      Self::IndexMap(_) => None,
    }
  }

  pub(crate) fn _as_index_map(&self) -> Option<&FlatCompiledStyles> {
    match self {
      Self::Regular(_) => None,
      Self::Map(_) => None,
      Self::IndexMap(map) => Some(map),
    }
  }

  pub(crate) fn _as_index_map_mut(&mut self) -> Option<&mut FlatCompiledStyles> {
    match self {
      Self::Regular(_) => None,
      Self::Map(_) => None,
      Self::IndexMap(map) => Some(map),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct FunctionMap {
  pub identifiers: FunctionMapIdentifiers,
  pub member_expressions: FunctionMapMemberExpression,
}
