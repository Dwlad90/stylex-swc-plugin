use swc_ecma_ast::{Expr, Id};

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
  pub(crate) Option<Id>,
);
