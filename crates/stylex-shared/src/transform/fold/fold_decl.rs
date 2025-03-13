use swc_core::{
  common::comments::Comments,
  ecma::{ast::Decl, utils::drop_span, visit::FoldWith},
};

use crate::StyleXTransform;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_decl_impl(&mut self, decl: Decl) -> Decl {
    match &decl {
      Decl::Class(class_decl) => {
        let class_decl_ident = drop_span(class_decl.ident.clone());

        if !self
          .state
          .class_name_declarations
          .contains(&class_decl_ident)
        {
          self.state.class_name_declarations.push(class_decl_ident);
        }
      }
      Decl::Fn(fn_decl) => {
        let fn_decl_ident = drop_span(fn_decl.ident.clone());

        if !self
          .state
          .function_name_declarations
          .contains(&fn_decl_ident)
        {
          self.state.function_name_declarations.push(fn_decl_ident);
        }
      }
      _ => {}
    }

    decl.fold_children_with(self)
  }
}
