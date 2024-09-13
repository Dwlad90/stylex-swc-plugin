use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{Decl, ExportDecl},
    visit::FoldWith,
  },
};

use crate::{
  shared::{enums::core::TransformationCycle, utils::common::increase_ident_count_by_count},
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_export_decl_impl(&mut self, export_decl: ExportDecl) -> ExportDecl {
    if self.state.cycle == TransformationCycle::Skip {
      return export_decl;
    }

    if self.state.cycle == TransformationCycle::StateFilling {
      if let Decl::Var(var_decl) = &export_decl.decl {
        for decl in &var_decl.decls {
          if let Some(ident) = decl.name.as_ident() {
            // HACK: For preventing removing named export declarations need to increase the count by 2.
            increase_ident_count_by_count(&mut self.state, ident, 2);
          }
        }
      }
    }

    export_decl.fold_children_with(self)
  }
}
