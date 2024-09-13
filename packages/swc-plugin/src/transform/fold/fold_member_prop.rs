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
    if self.state.cycle == TransformationCycle::Skip {
      return member_prop;
    }

    if (self.state.cycle == TransformationCycle::StateFilling || self.state.cycle == TransformationCycle::Recounting)
      && member_prop.is_ident()
    {
      return member_prop;
    }

    member_prop.fold_children_with(self)
  }
}
