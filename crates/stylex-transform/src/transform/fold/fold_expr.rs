use swc_core::{
  common::comments::Comments,
  ecma::{ast::Expr, visit::FoldWith},
};

use crate::{StyleXTransform, shared::utils::common::normalize_expr};
use stylex_enums::core::TransformationCycle;
use stylex_utils::hash::stable_hash;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_expr_impl(&mut self, mut expr: Expr) -> Expr {
    if self.state.cycle == TransformationCycle::Skip {
      return expr;
    }

    let normalized_expr = normalize_expr(&mut expr);

    // During Initializing, transform compiled JSX/VDOM calls with sx prop:
    // React/Vue: _jsx("div", { sx: expr }) → _jsx("div", { ...stylex.props(expr) })
    // Solid.js:  _$setAttribute(el, "sx", expr) → _$spread(el, _$mergeProps(() => stylex.props(expr)), false, true)
    if self.state.cycle == TransformationCycle::Initializing {
      if let Some(transformed) = self.transform_sx_in_compiled_jsx(normalized_expr) {
        return transformed.fold_children_with(self);
      }
      if let Some(transformed) = self.transform_sx_in_solid_set_attribute(normalized_expr) {
        return transformed.fold_children_with(self);
      }
    }

    if self.state.cycle == TransformationCycle::StateFilling
      && let Some(call_expr) = normalized_expr.as_call()
    {
      self
        .state
        .all_call_expressions
        .insert(stable_hash(&call_expr), call_expr.clone());
    }

    if (self.state.cycle == TransformationCycle::TransformEnter
      || self.state.cycle == TransformationCycle::TransformExit)
      && let Some(value) = self.transform_call_expression(normalized_expr)
    {
      return value;
    }

    expr.fold_children_with(self)
  }
}
