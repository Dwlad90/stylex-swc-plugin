use stylex_macros::stylex_panic;
use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{ComputedPropName, Ident},
    visit::FoldWith,
  },
};

use crate::{
  StyleXTransform,
  shared::utils::{ast::convertors::convert_expr_to_str, common::increase_ident_count},
};
use stylex_constants::constants::messages::EXPRESSION_IS_NOT_A_STRING;
use stylex_enums::core::TransformationCycle;

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
      let expt_str = convert_expr_to_str(
        &computed_prop_name.expr,
        &mut self.state,
        &Default::default(),
      );

      increase_ident_count(
        &mut self.state,
        &Ident::from(
          match expt_str {
            Some(s) => s,
            None => stylex_panic!("{}", EXPRESSION_IS_NOT_A_STRING),
          }
          .as_str(),
        ),
      );
    }

    computed_prop_name.fold_children_with(self)
  }
}
