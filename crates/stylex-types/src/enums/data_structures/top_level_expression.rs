use swc_core::{atoms::Atom, ecma::ast::Expr};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum TopLevelExpressionKind {
  NamedExport,
  DefaultExport,
  Stmt,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TopLevelExpression(
  pub TopLevelExpressionKind,
  pub Expr,
  pub Option<Atom>,
);
