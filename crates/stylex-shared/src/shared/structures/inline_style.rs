use swc_core::ecma::ast::Expr;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct InlineStyle {
  pub(crate) path: Vec<String>,
  pub(crate) original_expression: Expr,
  pub(crate) expression: Expr,
}
