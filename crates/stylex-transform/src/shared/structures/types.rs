use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_utils::collections::FxIndexMap;
use swc_core::{
  atoms::Atom,
  ecma::ast::{BindingIdent, Expr, Ident, Program},
};

use crate::shared::enums::data_structures::{
  evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
};
use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;
pub(crate) use stylex_types::structures::style_key::{ClassName, RuleKey};

use super::{
  functions::{FunctionConfig, FunctionConfigType},
  state_manager::StateManager,
};
use stylex_structures::{inline_style::InlineStyle, named_import_source::ImportSources};

pub(crate) type FlatCompiledStyles = IndexMap<String, Rc<FlatCompiledStylesValue>>;
pub(crate) type DynamicFns = IndexMap<String, (Vec<BindingIdent>, TInlineStyles)>;

pub(crate) type EvaluationCallback =
  Rc<dyn Fn(Vec<EvaluateResultValue>, &mut StateManager) -> Expr + 'static>;
pub(crate) type FunctionMapMemberExpression =
  FxHashMap<ImportSources, Box<FxHashMap<Atom, Box<FunctionConfigType>>>>;
pub(crate) type FunctionMapIdentifiers = FxHashMap<Atom, Box<FunctionConfigType>>;

/// The entries a single function-map name carries, standing for a plain JS
/// object on the reference implementation's side.
///
/// Ordered rather than hashed, and the one place that decision lives.
///
/// JS-parity: `visitors/stylex-create.js:206` --
/// `identifiers[name] = { ...(identifiers[name] ?? {}), when: stylexWhen }`. It
/// is a plain object, so its keys are read in insertion order, and the object a
/// style-value position materializes from it decides which key
/// `Invalid pseudo or at-rule.` names. `FxIndexMap` keeps the workspace hasher
/// while preserving that order.
///
/// The index vector `IndexMap` carries on top of the table is not worth trading
/// that order for. One of these is built while a `create`, `defineVars` or
/// `createTheme` call sets its evaluation up, holds a handful of entries keyed by
/// API name, and is dropped with it -- nothing builds one per declaration or
/// walks one in a loop.
pub(crate) type FunctionConfigMap = FxIndexMap<Atom, FunctionConfig>;

pub(crate) type StylesObjectMap = IndexMap<String, Rc<FlatCompiledStyles>>;
pub(crate) type InjectableStylesMap = IndexMap<RuleKey, Rc<InjectableStyleKind>>;
pub(crate) type ClassPathsMap = IndexMap<String, Rc<ClassPathsInNamespace>>;
pub(crate) type ClassesToOriginalPaths = IndexMap<String, Vec<String>>;
pub(crate) type ClassNameToOriginalPaths = IndexMap<ClassName, Vec<String>>;
pub(crate) type ClassPathsInNamespace = ClassesToOriginalPaths;
pub(crate) type TInlineStyles = IndexMap<String, Box<InlineStyle>>;

#[derive(Clone, Debug)]
pub(crate) struct InjectImportIdents {
  pub(crate) module: Ident,
  pub(crate) var: Ident,
}

#[derive(Clone, Debug)]
pub(crate) struct SeenModuleSource {
  pub(crate) program: Program,
  pub(crate) source_code: Option<String>,
}
