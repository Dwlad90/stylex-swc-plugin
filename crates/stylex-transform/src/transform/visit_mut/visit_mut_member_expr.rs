use swc_core::{
  common::comments::Comments,
  ecma::{ast::MemberExpr, visit::VisitMutWith},
};

use stylex_enums::core::TransformationCycle;

use crate::{StyleXTransform, shared::utils::common::increase_member_ident_count};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_member_expr_impl(&mut self, member_expression: &mut MemberExpr) {
    if self.state.cycle == TransformationCycle::Discover
      && let Some(obj_ident) = member_expression.obj.as_ident()
    {
      increase_member_ident_count(&mut self.state, &obj_ident.sym);
    }

    member_expression.visit_mut_children_with(self);
  }
}
