use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::MemberProp,
    visit::FoldWith,
  },
};

use crate::{shared::enums::core::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_member_prop_impl(&mut self, member_prop: MemberProp) -> MemberProp {
    if self.cycle == ModuleCycle::Skip {
      return member_prop;
    }

    if self.cycle == ModuleCycle::StateFilling && member_prop.is_ident() {
      return member_prop;
    }

    member_prop.fold_children_with(self)
  }
}
