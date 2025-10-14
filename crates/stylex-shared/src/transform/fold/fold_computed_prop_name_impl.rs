use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{ComputedPropName, Ident},
    visit::FoldWith,
  },
};

use crate::{
  StyleXTransform,
  shared::{
    enums::core::TransformationCycle,
    utils::{ast::convertors::expr_to_str, common::increase_ident_count},
  },
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_computed_prop_name_impl(
    &mut self,
    computed_prop_name: ComputedPropName,
  ) -> ComputedPropName {
    if self.state.cycle == TransformationCycle::Skip {
      return computed_prop_name;
    }

    if self.state.cycle == TransformationCycle::StateFilling && computed_prop_name.expr.is_lit() {
      let expt_str = expr_to_str(
        &computed_prop_name.expr,
        &mut self.state,
        &Default::default(),
      );

      increase_ident_count(
        &mut self.state,
        &Ident::from(expt_str.expect("Expression is not a string").as_str()),
      );
    }

    computed_prop_name.fold_children_with(self)
  }
}
