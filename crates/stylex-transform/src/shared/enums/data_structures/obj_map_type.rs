use swc_core::ecma::ast::ObjectLit;

use stylex_state::types::FlatCompiledStyles;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ObjMapType {
  Object(ObjectLit),
  Map(FlatCompiledStyles),
}
