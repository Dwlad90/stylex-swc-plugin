use swc_core::{
  common::comments::Comments,
  ecma::{ast::JSXAttrOrSpread, utils::drop_span, visit::VisitMutWith},
};

use crate::StyleXTransform;
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_jsx_attr_or_spreads_impl(
    &mut self,
    jsx_attrs: &mut Vec<JSXAttrOrSpread>,
  ) {
    if self.state.cycle == TransformationCycle::Discover {
      for jsx_attr in jsx_attrs.iter() {
        if let JSXAttrOrSpread::SpreadElement(spread) = jsx_attr {
          let expr = drop_span(spread.expr.as_ref().clone());
          self
            .state
            .jsx_spread_attr_exprs_map
            .entry(expr)
            .or_default();
        }
      }
    }

    // The JSX-spread replacement that used to live in the `PreCleaning` arm
    // now happens in the `mark_style_vars_to_keep` helper that runs at the
    // start of the finalize phase, so this hook only needs to descend in
    // every other cycle.
    jsx_attrs.visit_mut_children_with(self);
  }
}
