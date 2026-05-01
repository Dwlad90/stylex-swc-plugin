use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{Expr, MemberExpr, MemberProp},
    visit::VisitMutWith,
  },
};

use stylex_enums::{
  core::TransformationCycle,
  style_vars_to_keep::{NonNullProp, NonNullProps},
};
use stylex_structures::style_vars_to_keep::StyleVarsToKeep;

use crate::{
  StyleXTransform,
  shared::utils::common::{increase_ident_count, increase_member_ident_count},
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_member_expr_impl(&mut self, member_expression: &mut MemberExpr) {
    match self.state.cycle {
      TransformationCycle::Discover => {
        if let Some(obj_ident) = member_expression.obj.as_ident() {
          increase_member_ident_count(&mut self.state, &obj_ident.sym);
        }
        member_expression.visit_mut_children_with(self);
      },
      TransformationCycle::PreCleaning => {
        if let Expr::Ident(ident) = member_expression.obj.as_ref()
          && self.state.style_map.contains_key(ident.sym.as_ref())
        {
          if let MemberProp::Ident(prop_ident) = &member_expression.prop
            && self
              .state
              .member_object_ident_count_map
              .get(&ident.sym)
              .is_some_and(|&c| c > 0)
          {
            increase_ident_count(&mut self.state, ident);
            self.state.style_vars_to_keep.insert(StyleVarsToKeep(
              ident.sym.clone(),
              NonNullProp::Atom(prop_ident.sym.clone()),
              NonNullProps::True,
            ));
          };

          if let MemberProp::Computed(_) = &member_expression.prop {
            increase_ident_count(&mut self.state, ident);
            self.state.style_vars_to_keep.insert(StyleVarsToKeep(
              ident.sym.clone(),
              NonNullProp::True,
              NonNullProps::True,
            ));
          }
        }

        member_expression.visit_mut_children_with(self);
      },
      _ => member_expression.visit_mut_children_with(self),
    }
  }
}
