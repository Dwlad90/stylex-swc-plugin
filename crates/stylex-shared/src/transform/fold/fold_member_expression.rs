use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{Expr, MemberExpr, MemberProp},
    visit::FoldWith,
  },
};

use crate::{
  StyleXTransform,
  shared::{
    enums::{
      core::TransformationCycle,
      data_structures::style_vars_to_keep::{NonNullProp, NonNullProps, StyleVarsToKeep},
    },
    utils::common::{increase_ident_count, increase_member_ident_count, reduce_member_ident_count},
  },
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_member_expr_impl(&mut self, member_expression: MemberExpr) -> MemberExpr {
    match self.state.cycle {
      TransformationCycle::Skip => member_expression,
      TransformationCycle::StateFilling | TransformationCycle::Recounting => {
        if let Some(obj_ident) = member_expression.obj.as_ident() {
          match self.state.cycle {
            TransformationCycle::StateFilling => {
              increase_member_ident_count(&mut self.state, &obj_ident.sym);
            }
            TransformationCycle::Recounting => {
              reduce_member_ident_count(&mut self.state, &obj_ident.sym);
            }
            _ => {}
          };
        }
        member_expression.fold_children_with(self)
      }
      TransformationCycle::PreCleaning => {
        if let Expr::Ident(ident) = member_expression.obj.as_ref() {
          let obj_ident_name = ident.sym.to_string();
          if self.state.style_map.contains_key(&obj_ident_name) {
            if let MemberProp::Ident(prop_ident) = &member_expression.prop {
              if let Some(count) = self.state.member_object_ident_count_map.get(&ident.sym) {
                if count > &0 {
                  increase_ident_count(&mut self.state, ident);
                  let style_var_to_keep = StyleVarsToKeep(
                    ident.sym.clone(),
                    NonNullProp::Atom(prop_ident.sym.clone()),
                    NonNullProps::True,
                  );
                  self.state.style_vars_to_keep.insert(style_var_to_keep);
                }
              }
            }
          }
        }
        member_expression.fold_children_with(self)
      }
      _ => member_expression.fold_children_with(self),
    }
  }
}
