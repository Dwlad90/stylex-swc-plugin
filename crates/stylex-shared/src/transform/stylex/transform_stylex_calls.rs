use swc_core::{
  atoms::Atom,
  common::comments::Comments,
  ecma::ast::{CallExpr, Callee, Expr, MemberProp},
};

use crate::shared::enums::core::TransformationCycle;
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn transform_call_expression_to_stylex_expr(
    &mut self,
    ex: &mut CallExpr,
  ) -> Option<Expr> {
    if let Callee::Expr(callee) = &ex.callee {
      match callee.as_ref() {
        Expr::Member(member) => {
          if let MemberProp::Ident(ident_name) = &member.prop {
            return self.transform_stylex_fns(&ident_name.sym.clone(), ex);
          }
        }
        Expr::Ident(ident) => return self.transform_stylex_fns(&ident.sym.clone(), ex),
        _ => {}
      }
    }

    None
  }

  fn transform_stylex_fns(&mut self, ident_name: &Atom, call_expr: &mut CallExpr) -> Option<Expr> {
    if self.state.cycle == TransformationCycle::TransformEnter {
      let (_, parent_var_decl) = &self.get_call_var_name(call_expr);

      if let Some(parent_var_decl) = parent_var_decl {
        if let Some(value) = self.transform_stylex_keyframes_call(parent_var_decl) {
          return Some(value);
        }
      }

      if let Some(value) = self.transform_stylex_define_vars(call_expr) {
        return Some(value);
      }

      if let Some(value) = self.transform_stylex_create_theme_call(call_expr) {
        return Some(value);
      }

      if let Some(value) = self.transform_stylex_create(call_expr) {
        return Some(value);
      }
    }

    if self.state.cycle == TransformationCycle::TransformExit {
      if self.state.stylex_props_import.contains(ident_name) {
        if let Some(value) = self.transform_stylex_props_call(call_expr) {
          return Some(value);
        }
      }

      if self.state.stylex_attrs_import.contains(ident_name) {
        if let Some(value) = self.transform_stylex_attrs_call(call_expr) {
          return Some(value);
        }
      }

      if let Some(value) = self.transform_stylex_call(call_expr) {
        return Some(value);
      }

      if let Some(value) = self.transform_stylex_attrs_call(call_expr) {
        return Some(value);
      }

      if let Some(value) = self.transform_stylex_props_call(call_expr) {
        return Some(value);
      }
    }

    None
  }
}
