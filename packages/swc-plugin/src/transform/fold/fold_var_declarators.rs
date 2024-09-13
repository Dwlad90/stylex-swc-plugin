use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{Pat, VarDeclarator},
    visit::FoldWith,
  },
};

use crate::{shared::enums::core::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_var_declarators_impl(
    &mut self,
    mut var_declarators: Vec<VarDeclarator>,
  ) -> Vec<VarDeclarator> {
    match self.state.cycle {
      ModuleCycle::Skip => {
        return var_declarators;
      }
      ModuleCycle::Cleaning => {
        var_declarators.retain(|decl| {
          if let Pat::Ident(bind_ident) = &decl.name {
            let decl_id = &bind_ident.sym;

            if let Some(&count) = self.state.var_decl_count_map.get(decl_id) {
              // Remove the variable declaration if it is used only once after transformation.
              let is_used = count > 1;

              if !is_used {
                self.state.cycle = ModuleCycle::Recounting;

                decl.clone().fold_children_with(self);

                self.state.cycle = ModuleCycle::Cleaning;
              }

              return is_used;
            }
          }

          true
        });
      }
      _ => {}
    };

    var_declarators.fold_children_with(self)
  }
}
