use swc_core::{
  common::comments::Comments,
  ecma::{ast::JSXAttrOrSpread, utils::drop_span, visit::VisitMutWith},
};

use crate::StyleXTransform;
use stylex_ast::ast::factories::{create_jsx_attr_or_spread, create_jsx_spread_attr};
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_jsx_attr_or_spreads_impl(
    &mut self,
    jsx_attrs: &mut Vec<JSXAttrOrSpread>,
  ) {
    match self.state.cycle {
      TransformationCycle::Initializing => {
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
        jsx_attrs.visit_mut_children_with(self);
      },
      TransformationCycle::PreCleaning => {
        let mut result: Vec<JSXAttrOrSpread> = jsx_attrs
          .iter()
          .flat_map(|jsx_attr| match jsx_attr {
            JSXAttrOrSpread::SpreadElement(spread) => {
              let expr = drop_span(spread.expr.as_ref().clone());
              if let Some(updated_exprs) = self.state.jsx_spread_attr_exprs_map.get(&expr).cloned()
              {
                if updated_exprs.is_empty() {
                  // If the spread was resolved to nothing, remove it
                  vec![jsx_attr.clone()]
                } else {
                  // Replace the spread with the updated expressions
                  updated_exprs
                }
              } else {
                // If no replacement found, keep the original spread element
                vec![create_jsx_spread_attr(*spread.expr.clone())]
              }
            },
            JSXAttrOrSpread::JSXAttr(attr) => {
              // Keep regular attributes as-is (wrapped in vec for flat_map)
              vec![create_jsx_attr_or_spread(attr.clone())]
            },
          })
          .collect();

        result.visit_mut_children_with(self);
        *jsx_attrs = result;
      },
      _ => jsx_attrs.visit_mut_children_with(self),
    }
  }
}
