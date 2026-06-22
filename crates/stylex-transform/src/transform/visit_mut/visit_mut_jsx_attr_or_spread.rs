use swc_core::{
  common::{EqIgnoreSpan, comments::Comments},
  ecma::{ast::JSXAttrOrSpread, visit::VisitMutWith},
};

use crate::StyleXTransform;
use stylex_enums::core::TransformationCycle;
use stylex_utils::hash::stable_hash_unspanned;

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
          let expr_key = stable_hash_unspanned(spread.expr.as_ref());
          let bucket = self
            .state
            .jsx_spread_attr_exprs_map
            .entry(expr_key)
            .or_default();
          if !bucket
            .iter()
            .any(|(expr, _)| expr.eq_ignore_span(spread.expr.as_ref()))
          {
            bucket.push((spread.expr.as_ref().clone(), Vec::new()));
          }
        }
      }
    }

    // JSX-spread replacement no longer happens in this hook; it now runs in
    // `mark_style_vars_to_keep` at the start of the finalize phase. The
    // cycle-specific work here is only collecting spread expressions during
    // `Discover`; traversal still descends into children on every cycle.
    jsx_attrs.visit_mut_children_with(self);
  }
}
