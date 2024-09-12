use swc_core::{
  common::comments::Comments,
  ecma::{ast::PropName, visit::FoldWith},
};

use crate::{shared::enums::core::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_prop_name_impl(&mut self, prop_name: PropName) -> PropName {
    if self.cycle == ModuleCycle::Skip {
      return prop_name;
    }

    if self.cycle == ModuleCycle::StateFilling && prop_name.is_ident() {
      return prop_name;
    }

    prop_name.fold_children_with(self)
  }
}
