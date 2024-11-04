use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{
  atoms::Atom,
  ecma::ast::{BindingIdent, Expr},
};

use crate::shared::enums::data_structures::{
  evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
};

use super::{
  functions::FunctionConfigType, inline_style::InlineStyle, named_import_source::ImportSources,
};

pub(crate) type FlatCompiledStyles = IndexMap<String, Rc<FlatCompiledStylesValue>>;
pub(crate) type DynamicFns = IndexMap<String, (Vec<BindingIdent>, TInlineStyles)>;

pub(crate) type EvaluationCallback = Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr + 'static>;
pub(crate) type FunctionMapMemberExpression =
  FxHashMap<ImportSources, Box<FxHashMap<Atom, Box<FunctionConfigType>>>>;
pub(crate) type FunctionMapIdentifiers = FxHashMap<Atom, Box<FunctionConfigType>>;
pub(crate) type StylesObjectMap =
  IndexMap<String, Rc<IndexMap<String, Rc<FlatCompiledStylesValue>>>>;
pub(crate) type ClassesToOriginalPaths = IndexMap<String, Vec<String>>;
pub(crate) type ClassPathsInNamespace = ClassesToOriginalPaths;
pub(crate) type TInlineStyles = IndexMap<String, Box<InlineStyle>>;
