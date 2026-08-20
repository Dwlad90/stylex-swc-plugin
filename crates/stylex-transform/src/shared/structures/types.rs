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
/// `stylex-create.js:206` builds it as
/// `identifiers[name] = { ...(identifiers[name] ?? {}), when: stylexWhen }`, so
/// its keys are read in insertion order -- and the object a style-value position
/// materializes from it decides which key `Invalid pseudo or at-rule.` names.
/// `FxIndexMap` keeps the workspace hasher while preserving that order.
///
/// The index vector `IndexMap` carries on top of the table costs nothing worth
/// measuring here: one of these holds one or two entries and is built once per
/// `create()` call, which is orders below the ~16-34% cross-run noise the
/// performance policy is written around.
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
