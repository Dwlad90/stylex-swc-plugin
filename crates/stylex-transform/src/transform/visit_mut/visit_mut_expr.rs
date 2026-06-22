use swc_core::{
  common::{Spanned, comments::Comments},
  ecma::{ast::Expr, visit::VisitMutWith},
};

use crate::{StyleXTransform, shared::utils::common::normalize_expr};
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_expr_impl(&mut self, expr: &mut Expr) {
    let previous_site_span = self.state.current_site_span;
    if self.state.cycle == TransformationCycle::Discover {
      let span = expr.span();
      if !span.is_dummy() {
        self.state.current_site_span = span;
      }
    }

    let normalized_expr = normalize_expr(expr);

    // During Discover, transform compiled JSX/VDOM calls with sx prop and
    // register call expressions for downstream lookup:
    // React/Vue: _jsx("div", { sx: expr }) → _jsx("div", { ...stylex.props(expr) })
    // Solid.js:  _$setAttribute(el, "sx", expr) → _$spread(el, _$mergeProps(() =>
    // stylex.props(expr)), false, true)
    if self.state.cycle == TransformationCycle::Discover {
      if let Some(transformed) = self.transform_sx_in_compiled_jsx(normalized_expr) {
        *expr = transformed;
        expr.visit_mut_children_with(self);
        self.state.current_site_span = previous_site_span;
        return;
      }
      if let Some(transformed) = self.transform_sx_in_solid_set_attribute(normalized_expr) {
        *expr = transformed;
        expr.visit_mut_children_with(self);
        self.state.current_site_span = previous_site_span;
        return;
      }

      if let Some(call_expr) = normalized_expr.as_call() {
        self.state.add_call_expression(call_expr);
      }
    }

    if (self.state.cycle == TransformationCycle::TransformProducers
      || self.state.cycle == TransformationCycle::TransformConsumers)
      && let Some(value) = self.transform_call_expression(normalized_expr)
    {
      *expr = value;
      if self.state.cycle == TransformationCycle::Discover {
        self.state.current_site_span = previous_site_span;
      }
      return;
    }

    expr.visit_mut_children_with(self);

    if self.state.cycle == TransformationCycle::Discover {
      self.state.current_site_span = previous_site_span;
    }
  }
}
