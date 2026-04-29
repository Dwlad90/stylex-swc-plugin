use swc_core::{
  common::comments::Comments,
  ecma::{ast::Decl, utils::drop_span, visit::VisitMutWith},
};

use crate::StyleXTransform;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_decl_impl(&mut self, decl: &mut Decl) {
    match &decl {
      Decl::Class(class_decl) => {
        let class_decl_ident = drop_span(class_decl.ident.clone());

        self.state.add_class_name_declaration(class_decl_ident);
      },
      Decl::Fn(fn_decl) => {
        let fn_decl_ident = drop_span(fn_decl.ident.clone());

        self.state.add_function_name_declaration(fn_decl_ident);
      },
      _ => {},
    }

    decl.visit_mut_children_with(self);
  }
}
