use swc_core::{
  common::comments::Comments,
  ecma::{ast::PropName, visit::FoldWith},
};

use crate::{StyleXTransform, shared::enums::core::TransformationCycle};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_prop_name_impl(&mut self, prop_name: PropName) -> PropName {
    match self.state.cycle {
      TransformationCycle::Skip => prop_name,
      TransformationCycle::StateFilling | TransformationCycle::Recounting
        if prop_name.is_ident() =>
      {
        prop_name
      }
      _ => prop_name.fold_children_with(self),
    }
  }
}
