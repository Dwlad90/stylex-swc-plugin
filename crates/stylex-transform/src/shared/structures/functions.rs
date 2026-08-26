use std::{fmt::Debug, rc::Rc, sync::Arc};

use indexmap::IndexMap;
use swc_core::ecma::ast::Expr;

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{theme_ref::ThemeRef, types::FlatCompiledStyles},
};
use stylex_enums::{
  js::{CallableGlobalJS, MathJS, ObjectJS},
  value_with_default::ValueWithDefault,
};

use super::types::{FunctionConfigMap, FunctionMapIdentifiers, FunctionMapMemberExpression};
use stylex_structures::stylex_env::JSFunction;

use stylex_types::traits::StyleOptions;

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum CallbackType {
  Object(ObjectJS),
  Math(MathJS),
  /// A call to the global itself — `String(x)` — rather than to one of its
  /// methods.
  Global(CallableGlobalJS),
  Custom(Expr),
}

pub type StylexTypeFn = Rc<dyn Fn(ValueWithDefault) -> Expr + 'static>;
pub type StylexExprFn = fn(Expr, &mut dyn StyleOptions) -> Expr;

/// The `stylex.when.*` functions, which alone among the StyleX helpers take a
/// second argument: an optional custom marker to observe instead of the
/// default one. Both arguments stay `EvaluateResultValue` because a marker
/// imported from another file resolves to a `ThemeRef`, which no `Expr` can
/// represent.
pub type StylexWhenFn =
  fn(EvaluateResultValue, Option<EvaluateResultValue>, &mut dyn StyleOptions) -> Expr;

pub enum FunctionType {
  ArrayArgs(fn(Vec<Expr>, &mut dyn StyleOptions, &FunctionMap) -> Expr),
  StylexExprFn(StylexExprFn),
  StylexWhenFn(StylexWhenFn),
  StylexTypeFn(StylexTypeFn),
  StylexFnsFactory(fn(input: String) -> StylexTypeFn),

  Mapper(Rc<dyn Fn() -> Expr + 'static>),
  ThemeRefMapper(Rc<dyn Fn() -> ThemeRef + 'static>),
  Callback(Box<CallbackType>),
  DefaultMarker(Arc<IndexMap<String, StylexWhenFn>>),
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
      Self::StylexWhenFn(e) => Self::StylexWhenFn(*e),
      Self::StylexTypeFn(e) => Self::StylexTypeFn(e.clone()),
      Self::StylexFnsFactory(e) => Self::StylexFnsFactory(*e),
      Self::Callback(v) => Self::Callback(v.clone()),
      Self::Mapper(c) => Self::Mapper(Rc::clone(c)),
      Self::ThemeRefMapper(c) => Self::ThemeRefMapper(Rc::clone(c)),
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
      FunctionType::StylexWhenFn(_) => write!(f, "StylexWhenFn"),
      FunctionType::StylexTypeFn(_) => write!(f, "StylexExprFn"),
      FunctionType::StylexFnsFactory(_) => write!(f, "StylexFnsFactory"),
      FunctionType::Mapper(_) => write!(f, "Mapper"),
      FunctionType::ThemeRefMapper(_) => write!(f, "ThemeRefMapper"),
      FunctionType::Callback(_) => write!(f, "Callback"),
      FunctionType::DefaultMarker(_) => write!(f, "DefaultMarker"),
      FunctionType::EnvFunction(_) => write!(f, "EnvFunction"),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for FunctionType {
  /// Two `FunctionType` values compare equal when they have the same variant.
  /// We cannot compare the inner function pointers / `Rc<dyn Fn>` payloads, so
  /// discriminant equality is the strongest invariant we can uphold while
  /// satisfying `a == a` reflexivity. (This pairs with the discriminant-based
  /// `Hash` impl below so values can be used as map keys.)
  fn eq(&self, other: &Self) -> bool {
    std::mem::discriminant(self) == std::mem::discriminant(other)
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
  Map(FunctionConfigMap),
  IndexMap(FlatCompiledStyles),
  /// An env object from the `env` config option. Contains both values and
  /// functions.
  /// The `env` option's object, shared rather than copied.
  ///
  /// Registered once per `stylex` import name per `create` call, so a copy here
  /// is a copy per style object. It is read-only after options construction,
  /// which is what makes the sharing sound.
  EnvObject(Rc<IndexMap<String, stylex_structures::stylex_env::EnvEntry>>),
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
      // One pointer compare, and an exact answer rather than a conservative
      // one. The payload was a bare map when this arm was written, where
      // answering meant a deep compare per probe; it is shared from the options
      // now, so two `EnvObject`s are the same object or they are not.
      (Self::EnvObject(a), Self::EnvObject(b)) => Rc::ptr_eq(a, b),
      _ => false,
    }
  }
}

impl FunctionConfigType {
  pub(crate) fn as_map_mut(&mut self) -> Option<&mut FunctionConfigMap> {
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
