use swc_core::{
  common::comments::Comments,
  ecma::{ast::MemberProp, visit::FoldWith},
};

use crate::{shared::enums::core::TransformationCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_member_prop_impl(&mut self, member_prop: MemberProp) -> MemberProp {
    match self.state.cycle {
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
