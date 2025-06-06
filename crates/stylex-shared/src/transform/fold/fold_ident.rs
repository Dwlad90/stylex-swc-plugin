use swc_core::{common::comments::Comments, ecma::ast::Ident};

use crate::{
  StyleXTransform,
  shared::{
    enums::core::TransformationCycle,
    utils::common::{increase_ident_count, reduce_ident_count},
  },
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_ident_impl(&mut self, ident: Ident) -> Ident {
    match self.state.cycle {
      TransformationCycle::StateFilling => {
        increase_ident_count(&mut self.state, &ident);
      }
      TransformationCycle::Recounting => {
        reduce_ident_count(&mut self.state, &ident);
      }
      _ => {}
    };

    ident
  }
}
