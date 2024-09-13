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
    match self.state.cycle {
      ModuleCycle::Skip => prop_name,
      ModuleCycle::StateFilling | ModuleCycle::Recounting if prop_name.is_ident() => prop_name,
      _ => prop_name.fold_children_with(self),
    }
  }
}
