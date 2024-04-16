use swc_core::{
  common::{comments::Comments, util::take::Take},
  ecma::{
    ast::{Decl, Stmt},
    visit::FoldWith,
  },
};

use crate::{shared::enums::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_stmt_impl(&mut self, stmt: Stmt) -> Stmt {
    if self.cycle == ModuleCycle::Skip {
      return stmt;
    }

    if self.cycle == ModuleCycle::Cleaning {
      let mut stmt = stmt.fold_children_with(self);

      match &stmt {
        Stmt::Decl(Decl::Var(var)) => {
          dbg!(&var);
          if var.decls.is_empty() {
            // Variable declaration without declarator is invalid.
            //
            // After this, `stmt` becomes `Stmt::Empty`.
            stmt.take();
          }
        }
        _ => {}
      }
      stmt
    } else {
      stmt.fold_children_with(self)
    }
  }
}
