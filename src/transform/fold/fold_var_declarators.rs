use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{Pat, VarDeclarator},
        visit::FoldWith,
    },
};

use crate::{shared::enums::ModuleCycle, ModuleTransformVisitor};

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
            ModuleCycle::Initializing => {
                var_declarators.iter().for_each(|decl| {
                    if let Pat::Ident(_) = &decl.name {
                        let var = decl.clone();

                        if !self.declarations.contains(&var) {
                            self.declarations.push(var);
                        }
                    }
                });
            }
            ModuleCycle::Cleaning => {
                var_declarators.retain(|decl| {
                    match &decl.name {
                        Pat::Ident(bind_ident) => {
                            let decl_id = &bind_ident.to_id();

                            if self.var_decl_count_map.contains_key(&decl_id) {
                                let count = self.var_decl_count_map.get(decl_id).unwrap();

                                let is_used = count > &0;

                                return is_used;
                            }
                        }
                        _ => {}
                    }

                    true
                });
            }
            _ => {}
        };

        var_declarators.fold_children_with(self)
    }
}
