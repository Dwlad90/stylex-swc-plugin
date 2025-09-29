use swc_core::{
  common::comments::Comments,
  ecma::{ast::JSXAttrOrSpread, utils::drop_span, visit::FoldWith},
};

use crate::{StyleXTransform, shared::enums::core::TransformationCycle};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_jsx_attr_or_spreads(
    &mut self,
    jsx_attrs: Vec<JSXAttrOrSpread>,
  ) -> Vec<JSXAttrOrSpread> {
    match self.state.cycle {
      TransformationCycle::Initializing => {
        for jsx_attr in jsx_attrs.iter() {
          if let JSXAttrOrSpread::SpreadElement(spread) = jsx_attr {
            let expr = drop_span(*spread.expr.clone());
            self
              .state
              .jsx_spread_attr_exprs_map
              .entry(expr)
              .or_default();
          }
        }
        jsx_attrs.fold_children_with(self)
      }
      TransformationCycle::PreCleaning => {
        let result: Vec<JSXAttrOrSpread> = jsx_attrs
          .iter()
          .flat_map(|jsx_attr| {
            match jsx_attr {
              JSXAttrOrSpread::SpreadElement(spread) => {
                let expr = drop_span(*spread.expr.clone());
                if let Some(updated_exprs) =
                  self.state.jsx_spread_attr_exprs_map.get(&expr).cloned()
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
                  vec![JSXAttrOrSpread::SpreadElement(spread.clone())]
                }
              }
              JSXAttrOrSpread::JSXAttr(attr) => {
                // Keep regular attributes as-is (wrapped in vec for flat_map)
                vec![JSXAttrOrSpread::JSXAttr(attr.clone())]
              }
            }
          })
          .collect();

        result.fold_children_with(self)
      }
      _ => jsx_attrs.fold_children_with(self),
    }
  }
}
