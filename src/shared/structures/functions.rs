use std::{collections::HashMap, rc::Rc};

use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, Id};

use crate::shared::utils::js::enums::{ArrayJS, ObjectJS};

use super::{
  injectable_style::InjectableStyle, named_import_source::ImportSources,
  state_manager::StateManager,
};

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum CallbackType {
  Array(ArrayJS),
  Object(ObjectJS),
}

pub enum FunctionType {
  ArrayArgs(fn(Vec<Expr>) -> Expr),
  StylexFns(fn(Expr, StateManager) -> (Expr, StateManager)),
  // OneArg(
  //     Rc<
  //         dyn Fn(
  //                 Expr,
  //                 StateManager,
  //             ) -> (Expr, StateManager)
  //             + 'static,
  //     >,
  // ), // Expr,
  Mapper(Rc<dyn Fn() -> Expr + 'static>),
  // Callback(CallbackType, Expr),
  Callback(CallbackType),
}

impl Clone for FunctionType {
  fn clone(&self) -> Self {
    match self {
      Self::ArrayArgs(e) => Self::ArrayArgs(e.clone()),
      Self::StylexFns(e) => Self::StylexFns(e.clone()),
      Self::Callback(v) => Self::Callback(v.clone()),
      Self::Mapper(c) => Self::Mapper(Rc::clone(c)),
    }
  }
}

impl std::fmt::Debug for FunctionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FunctionType::ArrayArgs(_) => write!(f, "ArrayArgs"),
      FunctionType::StylexFns(_) => write!(f, "OneArg"),
      FunctionType::Mapper(_) => write!(f, "Mapper"),
      FunctionType::Callback(_) => write!(f, "Callback"),
    }
  }
}

impl PartialEq for FunctionType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (FunctionType::ArrayArgs(_), FunctionType::ArrayArgs(_)) => true,
      (FunctionType::StylexFns(_), FunctionType::StylexFns(_)) => true,
      (FunctionType::Mapper(_), FunctionType::StylexFns(_)) => true,
      (FunctionType::Callback(_), FunctionType::Callback(_)) => true,
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
      FunctionType::StylexFns(_) => {
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

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct Functions {
  pub(crate) include: FunctionConfig,
  pub(crate) first_that_works: FunctionConfig,
  pub(crate) keyframes: FunctionConfig,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionMap {
  pub identifiers: HashMap<Id, FunctionConfig>,
  pub member_expressions: HashMap<ImportSources, HashMap<Id, FunctionConfig>>,
}

impl Default for FunctionMap {
  fn default() -> Self {
    Self {
      identifiers: HashMap::new(),
      member_expressions: HashMap::new(),
    }
  }
}
