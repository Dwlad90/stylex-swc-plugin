use indexmap::IndexMap;
use swc_ecma_ast::ObjectLit;

use super::flat_compiled_styles_value::FlatCompiledStylesValue;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ObjMapType {
  Object(ObjectLit),
  Map(IndexMap<String, Box<FlatCompiledStylesValue>>),
}
