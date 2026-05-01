use swc_core::{
  common::comments::Comments,
  ecma::{ast::ExportDefaultExpr, visit::VisitMutWith},
};

use crate::{StyleXTransform, shared::utils::common::normalize_expr};
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_export_default_expr_impl(
    &mut self,
    export_default_expr: &mut ExportDefaultExpr,
  ) {
    if self.state.cycle == TransformationCycle::Discover {
      export_default_expr.visit_mut_children_with(self);
      return;
    }

    if self.state.cycle == TransformationCycle::TransformEnter
      || self.state.cycle == TransformationCycle::TransformExit
    {
      let normalized_expr = normalize_expr(&mut export_default_expr.expr);

      if let Some(value) = self.transform_call_expression(normalized_expr) {
        *export_default_expr.expr = value;
      }
    }
  }
}
