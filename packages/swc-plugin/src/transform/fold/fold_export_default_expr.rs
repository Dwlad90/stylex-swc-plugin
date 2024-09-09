use swc_core::{
  common::comments::Comments,
  ecma::{ast::ExportDefaultExpr, visit::FoldWith},
};

use crate::{
  shared::{enums::core::ModuleCycle, utils::common::normalize_expr},
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_export_default_expr_impl(
    &mut self,
    mut export_default_expr: ExportDefaultExpr,
  ) -> ExportDefaultExpr {
    if self.cycle == ModuleCycle::Skip {
      return export_default_expr;
    }

    if self.cycle == ModuleCycle::StateFilling {
      return export_default_expr.fold_children_with(self);
    }

    if self.cycle == ModuleCycle::TransformEnter || self.cycle == ModuleCycle::TransformExit {
      let normalized_expr = normalize_expr(&mut export_default_expr.expr);

      if let Some(value) = self.transform_call_expression(normalized_expr) {
        *export_default_expr.expr = value;
      }
    }

    export_default_expr
  }
}
