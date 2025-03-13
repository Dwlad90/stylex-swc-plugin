use swc_core::{
  common::comments::Comments,
  ecma::{ast::Expr, visit::FoldWith},
};

use crate::{
  StyleXTransform,
  shared::{enums::core::TransformationCycle, utils::common::normalize_expr},
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_expr_impl(&mut self, mut expr: Expr) -> Expr {
    if self.state.cycle == TransformationCycle::Skip {
      return expr;
    }

    let normalized_expr = normalize_expr(&mut expr);

    if self.state.cycle == TransformationCycle::StateFilling {
      if let Some(call_expr) = normalized_expr.as_call() {
        self.state.all_call_expressions.push(call_expr.clone());
      }
    }

    if self.state.cycle == TransformationCycle::TransformEnter
      || self.state.cycle == TransformationCycle::TransformExit
    {
      if let Some(value) = self.transform_call_expression(normalized_expr) {
        return value;
      }
    }

    expr.fold_children_with(self)
  }
}
