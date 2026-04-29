use swc_core::ecma::{
  ast::{Expr, MemberExpr},
  visit::{VisitMut, VisitMutWith, noop_visit_mut_type},
};

use crate::shared::utils::core::member_expression::member_expression;
use stylex_enums::style_vars_to_keep::NonNullProps;

use super::{functions::FunctionMap, state_manager::StateManager};

#[derive(Clone, Debug)]
pub(crate) struct MemberTransform {
  pub(crate) index: i32,
  pub(crate) bail_out_index: Option<i32>,
  pub(crate) non_null_props: NonNullProps,
  pub(crate) state: StateManager,
  pub(crate) parents: Vec<Expr>,
  pub(crate) functions: FunctionMap,
}

impl VisitMut for MemberTransform {
  noop_visit_mut_type!();

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    self.parents.push(expr.clone());
    expr.visit_mut_children_with(self);
  }

  fn visit_mut_member_expr(&mut self, member: &mut MemberExpr) {
    member_expression(
      member,
      &mut self.index,
      &mut self.bail_out_index,
      &mut self.non_null_props,
      &mut self.state,
      &self.functions,
    );
  }
}
