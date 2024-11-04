use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::ecma::ast::ObjectLit;

use super::flat_compiled_styles_value::FlatCompiledStylesValue;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ObjMapType {
  Object(ObjectLit),
  Map(IndexMap<String, Rc<FlatCompiledStylesValue>>),
}
