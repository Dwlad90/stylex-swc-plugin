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
    match self.cycle {
      ModuleCycle::Skip => {
        return var_declarators;
      }
      // ModuleCycle::Initializing => {}
      ModuleCycle::Cleaning => {
        var_declarators.retain(|decl| {
          if let Pat::Ident(bind_ident) = &decl.name {
            let decl_id = &bind_ident.sym;

            if self.state.var_decl_count_map.contains_key(decl_id) {
              let count = self.state.var_decl_count_map.get(decl_id).unwrap();

              // Remove the variable declaration if it is used only once after transformation.
              let is_used = count > &1;

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
