use std::{collections::HashMap, rc::Rc};

use indexmap::IndexMap;
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

pub(crate) type FlatCompiledStyles = IndexMap<String, Box<FlatCompiledStylesValue>>;
pub(crate) type DynamicFns = IndexMap<String, (Vec<BindingIdent>, TInlineStyles)>;
pub(crate) type EvaluationCallback = Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr + 'static>;
pub(crate) type FunctionMapMemberExpression =
  HashMap<ImportSources, Box<HashMap<Atom, Box<FunctionConfigType>>>>;
pub(crate) type FunctionMapIdentifiers = HashMap<Atom, Box<FunctionConfigType>>;
pub(crate) type StylesObjectMap =
  IndexMap<String, Box<IndexMap<String, Box<FlatCompiledStylesValue>>>>;
pub(crate) type ClassesToOriginalPaths = IndexMap<String, Vec<String>>;
pub(crate) type ClassPathsInNamespace = ClassesToOriginalPaths;

pub(crate) type TInlineStyles = IndexMap<String, Box<InlineStyle>>;
