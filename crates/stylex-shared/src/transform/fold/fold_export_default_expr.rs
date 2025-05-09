use swc_core::{
  common::comments::Comments,
  ecma::{ast::ExportDefaultExpr, visit::FoldWith},
};

use crate::{
  StyleXTransform,
  shared::{enums::core::TransformationCycle, utils::common::normalize_expr},
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_export_default_expr_impl(
    &mut self,
    mut export_default_expr: ExportDefaultExpr,
  ) -> ExportDefaultExpr {
    if self.state.cycle == TransformationCycle::Skip {
      return export_default_expr;
    }

    if self.state.cycle == TransformationCycle::StateFilling {
      return export_default_expr.fold_children_with(self);
    }

    if self.state.cycle == TransformationCycle::TransformEnter
      || self.state.cycle == TransformationCycle::TransformExit
    {
      let normalized_expr = normalize_expr(&mut export_default_expr.expr);

      if let Some(value) = self.transform_call_expression(normalized_expr) {
        *export_default_expr.expr = value;
      }
    }

    export_default_expr
  }
}
