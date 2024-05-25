use swc_core::{common::comments::Comments, ecma::ast::Ident};

use crate::{
  shared::{enums::core::ModuleCycle, utils::common::increase_ident_count},
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_ident_impl(&mut self, ident: Ident) -> Ident {
    if self.cycle == ModuleCycle::Skip {
      return ident;
    }

    if self.cycle == ModuleCycle::Initializing {
      increase_ident_count(&mut self.state, &ident);
    }

    ident
  }
}
