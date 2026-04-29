use swc_core::{common::comments::Comments, ecma::ast::Ident};

use crate::{
  StyleXTransform,
  shared::utils::common::{increase_ident_count, reduce_ident_count},
};
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_ident_impl(&mut self, ident: &mut Ident) {
    match self.state.cycle {
      TransformationCycle::StateFilling => {
        increase_ident_count(&mut self.state, ident);
      },
      TransformationCycle::Recounting => {
        reduce_ident_count(&mut self.state, ident);
      },
      _ => {},
    };
  }
}
