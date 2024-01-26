use swc_core::ecma::ast::Expr;

#[derive(Debug, Clone)]
pub(crate) struct IncludedStyle {
expr: Box<Expr>,
}
