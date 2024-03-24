use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{Decl, ExportDecl, Stmt},
        visit::FoldWith,
    },
};

use crate::{
    shared::{
        enums::ModuleCycle,
        utils::common::{increase_ident_count, increase_ident_count_by_count},
    },
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_export_decl_impl(&mut self, export_decl: ExportDecl) -> ExportDecl {
        if self.cycle == ModuleCycle::Skip {
            return export_decl;
        }

        if self.cycle == ModuleCycle::Initializing {
            match &export_decl.decl {
                Decl::Var(var_decl) => {
                    for decl in &var_decl.decls {
                        if let Some(ident) = decl.name.as_ident() {
                            // HACK: For preventing removing named export declarations need to increase the count by 2.
                            increase_ident_count_by_count(&mut self.state, ident, 2);
                        }
                    }
                }
                _ => {}
            }
        }

        export_decl.fold_children_with(self)
    }
}
