use swc_core::{
  common::comments::Comments,
  ecma::{ast::Decl, visit::FoldWith},
};

use crate::StyleXTransform;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_decl_impl(&mut self, decl: Decl) -> Decl {
    match &decl {
      Decl::Class(class_decl) => {
        if !self
          .state
          .class_name_declarations
          .contains(&class_decl.ident)
        {
          self
            .state
            .class_name_declarations
            .push(class_decl.ident.clone());
        }
      }
      Decl::Fn(fn_decl) => {
        if !self
          .state
          .function_name_declarations
          .contains(&fn_decl.ident)
        {
          self
            .state
            .function_name_declarations
            .push(fn_decl.ident.clone());
        }
      }
      _ => {}
    }

    decl.fold_children_with(self)
  }
}
