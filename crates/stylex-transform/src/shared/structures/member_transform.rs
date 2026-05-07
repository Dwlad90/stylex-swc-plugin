use swc_core::ecma::{
  ast::{Expr, MemberExpr},
  visit::{VisitMut, VisitMutWith, noop_visit_mut_type},
};

use crate::shared::utils::core::member_expression::member_expression;
use stylex_enums::style_vars_to_keep::NonNullProps;

use super::{functions::FunctionMap, state_manager::StateManager};

pub(crate) struct MemberTransform<'a> {
  pub(crate) index: i32,
  pub(crate) bail_out_index: Option<i32>,
  pub(crate) non_null_props: NonNullProps,
  pub(crate) state: &'a mut StateManager,
  pub(crate) functions: &'a FunctionMap,
}

impl VisitMut for MemberTransform<'_> {
  noop_visit_mut_type!();

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    expr.visit_mut_children_with(self);
  }

  fn visit_mut_member_expr(&mut self, member: &mut MemberExpr) {
    member_expression(
      member,
      &mut self.index,
      &mut self.bail_out_index,
      &mut self.non_null_props,
      &mut *self.state,
      self.functions,
    );
  }
}
