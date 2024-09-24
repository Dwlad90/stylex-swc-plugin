use std::{collections::HashMap, rc::Rc};

use swc_core::{atoms::Atom, ecma::ast::Expr};

use crate::shared::enums::{
  data_structures::value_with_default::ValueWithDefault,
  js::{ArrayJS, MathJS, ObjectJS, StringJS},
};

use super::{
  state_manager::StateManager,
  types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
};

#[derive(Debug, Hash, PartialEq, Clone, Copy)]
pub enum CallbackType {
  Array(ArrayJS),
  Object(ObjectJS),
  Math(MathJS),
  String(StringJS),
}

pub type StylexTypeFn = Rc<dyn Fn(ValueWithDefault) -> Expr + 'static>;

pub enum FunctionType {
  ArrayArgs(fn(Vec<Expr>) -> Expr),
  StylexExprFn(fn(Expr, &mut StateManager) -> Expr),
  StylexTypeFn(StylexTypeFn),
  StylexFnsFactory(fn(input: String) -> StylexTypeFn),

  Mapper(Rc<dyn Fn() -> Expr + 'static>),
  Callback(Box<CallbackType>),
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
    }
  }
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct FunctionConfig {
  pub fn_ptr: FunctionType,
  pub takes_path: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionConfigType {
  Regular(FunctionConfig),
  Map(HashMap<Atom, FunctionConfig>),
}

impl FunctionConfigType {
  pub(crate) fn _as_function_config(&self) -> Option<&FunctionConfig> {
    match self {
      Self::Regular(config) => Some(config),
      Self::Map(_) => None,
    }
  }

  pub(crate) fn _as_map(&self) -> Option<&HashMap<Atom, FunctionConfig>> {
    match self {
      Self::Regular(_) => None,
      Self::Map(map) => Some(map),
    }
  }

  pub(crate) fn as_map_mut(&mut self) -> Option<&mut HashMap<Atom, FunctionConfig>> {
    match self {
      Self::Regular(_) => None,
      Self::Map(map) => Some(map),
    }
  }

  pub(crate) fn _as_function_config_mut(&mut self) -> Option<&mut FunctionConfig> {
    match self {
      Self::Regular(config) => Some(config),
      Self::Map(_) => None,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct FunctionMap {
  pub identifiers: FunctionMapIdentifiers,
  pub member_expressions: FunctionMapMemberExpression,
}
