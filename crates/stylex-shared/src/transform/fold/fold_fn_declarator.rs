use swc_core::{
  common::{DUMMY_SP, comments::Comments},
  ecma::{
    ast::{FnDecl, VarDeclarator},
    visit::FoldWith,
  },
};

use crate::{StyleXTransform, shared::utils::common::fill_state_declarations};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_fn_declarator_impl(&mut self, fn_declarator: FnDecl) -> FnDecl {
    fill_state_declarations(
      &mut self.state,
      &VarDeclarator {
        name: fn_declarator.ident.clone().into(),
        init: None,
        span: DUMMY_SP,
        definite: false,
      },
    );

    fn_declarator.fold_children_with(self)
  }
}
