use swc_core::{atoms::Atom, ecma::ast::Expr};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub(crate) enum TopLevelExpressionKind {
  NamedExport,
  DefaultExport,
  Stmt,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct TopLevelExpression(
  pub(crate) TopLevelExpressionKind,
  pub(crate) Expr,
  pub(crate) Option<Atom>,
);
