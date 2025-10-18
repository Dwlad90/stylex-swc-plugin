use swc_core::ecma::ast::ObjectLit;

use crate::shared::structures::types::FlatCompiledStyles;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ObjMapType {
  Object(ObjectLit),
  Map(FlatCompiledStyles),
}
