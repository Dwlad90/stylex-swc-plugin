use swc_core::{
  common::{comments::Comments, util::take::Take},
  ecma::{
    ast::{Decl, Stmt},
    utils::IsDirective,
    visit::FoldWith,
  },
};

use crate::{shared::enums::core::TransformationCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_stmt_impl(&mut self, stmt: Stmt) -> Stmt {
    if self.state.cycle == TransformationCycle::Skip {
      return stmt;
    }

    if self.state.cycle == TransformationCycle::Cleaning {
      let mut stmt = stmt.fold_children_with(self);

      if let Some(Stmt::Decl(Decl::Var(var))) = stmt.as_ref() {
        if var.decls.is_empty() {
          // Variable declaration without declarator is invalid.
          //
          // After this, `stmt` becomes `Stmt::Empty`.
          stmt.take();
        }
      }

      stmt
    } else {
      stmt.fold_children_with(self)
    }
  }
}
