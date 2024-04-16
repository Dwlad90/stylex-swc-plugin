use swc_core::ecma::ast::{Expr, ExprOrSpread};

use crate::shared::utils::css::factories::array_expression_factory;

pub(crate) fn stylex_first_that_works(args: Vec<Expr>) -> Expr {
  let elems = args
    .into_iter()
    .rev()
    .map(|arg| {
      Option::Some(ExprOrSpread {
        spread: Option::None,
        expr: Box::new(arg),
      })
    })
    .collect::<Vec<Option<ExprOrSpread>>>();

  array_expression_factory(elems).unwrap()
}
