use swc_core::ecma::ast::Expr;

#[derive(Debug, Clone)]
pub(crate) struct IncludedStyle {
    expr: Expr,
}

impl IncludedStyle {
    pub(crate) fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub(crate) fn get_expr(&self) -> &Expr {
        &self.expr
    }
}
