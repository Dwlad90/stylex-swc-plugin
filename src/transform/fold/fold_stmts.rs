use swc_core::{
    common::comments::Comments,
    ecma::{ast::Stmt, visit::FoldWith},
};

use crate::{shared::enums::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_stmts_impl(&mut self, mut stmts: Vec<Stmt>) -> Vec<Stmt> {
        if self.cycle == ModuleCycle::Skip {
            return stmts;
        }

        if self.cycle == ModuleCycle::Cleaning {
            stmts.retain(|s| {
                // We use `matches` macro as this match is trivial.
                !matches!(s, Stmt::Empty(..))
            });

            stmts
        } else {
            stmts.fold_children_with(self)
        }
    }
}
