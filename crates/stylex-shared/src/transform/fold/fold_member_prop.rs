use swc_core::{
  common::comments::Comments,
  ecma::{ast::MemberProp, visit::FoldWith},
};

use crate::{shared::enums::core::TransformationCycle, StyleXTransform};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_member_prop_impl(&mut self, member_prop: MemberProp) -> MemberProp {
    let cycle = self.state.cycle;

    match cycle {
      TransformationCycle::Skip => member_prop,
      TransformationCycle::StateFilling | TransformationCycle::Recounting
        if member_prop.is_ident() =>
      {
        member_prop
      }
      _ => member_prop.fold_children_with(self),
    }
  }
}
