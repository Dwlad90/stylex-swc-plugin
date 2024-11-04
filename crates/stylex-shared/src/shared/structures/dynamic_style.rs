use swc_core::ecma::ast::Expr;

#[derive(Debug, Clone)]
pub(crate) struct DynamicStyle {
  pub(crate) expression: Expr,
  pub(crate) key: String,
  pub(crate) path: String,
}
