use swc_core::{
  common::comments::Comments,
  ecma::{ast::Stmt, visit::FoldWith},
};

use crate::{StyleXTransform, shared::enums::core::TransformationCycle};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_stmts_impl(&mut self, mut stmts: Vec<Stmt>) -> Vec<Stmt> {
    if self.state.cycle == TransformationCycle::Skip {
      return stmts;
    }

    if self.state.cycle == TransformationCycle::Cleaning {
      stmts.retain(|stmt| {
        // We use `matches` macro as this match is trivial.
        !matches!(stmt, Stmt::Empty(..))
      });

      stmts
    } else {
      stmts.fold_children_with(self)
    }
  }
}
