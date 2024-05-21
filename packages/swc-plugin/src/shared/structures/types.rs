use std::{collections::HashMap, rc::Rc};

use indexmap::IndexMap;
use swc_ecma_ast::{BindingIdent, Expr, Id};

use crate::shared::enums::FlatCompiledStylesValue;

use super::{
  evaluate_result::EvaluateResultValue, functions::FunctionConfigType,
  named_import_source::ImportSources,
};

pub(crate) type FlatCompiledStyles = IndexMap<String, Box<FlatCompiledStylesValue>>;
pub(crate) type EvaluateResultFns =
  IndexMap<String, (Vec<BindingIdent>, IndexMap<String, Box<Expr>>)>;
pub(super) type EvaluationCallback = Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr + 'static>;
pub(crate) type FunctionMapMemberExpression =
  HashMap<Box<ImportSources>, Box<HashMap<Box<Id>, Box<FunctionConfigType>>>>;
pub(crate) type FunctionMapIdentifiers = HashMap<Box<Id>, Box<FunctionConfigType>>;
pub(crate) type StylesObjectMap =
  IndexMap<String, Box<IndexMap<String, Box<FlatCompiledStylesValue>>>>;
