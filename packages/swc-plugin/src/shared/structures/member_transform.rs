use std::collections::HashMap;

use swc_core::ecma::visit::{noop_fold_type, Fold, FoldWith};
use swc_ecma_ast::{Expr, MemberExpr};

use crate::shared::{
  enums::data_structures::style_vars_to_keep::NonNullProps,
  utils::core::member_expression::member_expression,
};

use super::{functions::FunctionMap, state_manager::StateManager};

#[derive(Clone, Debug)]
pub(crate) struct MemberTransform {
  pub(crate) index: i32,
  pub(crate) bail_out_index: Option<i32>,
  pub(crate) non_null_props: NonNullProps,
  pub(crate) state: StateManager,
  pub(crate) parents: Vec<Expr>,
}

impl Fold for MemberTransform {
  noop_fold_type!();

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    self.parents.push(expr.clone());
    expr.fold_children_with(self)
  }

  fn fold_member_expr(&mut self, member: MemberExpr) -> MemberExpr {
    member_expression(
      &member,
      &mut self.index,
      &mut self.bail_out_index,
      &mut self.non_null_props,
      &mut self.state,
      &FunctionMap {
        identifiers: HashMap::new(),
        member_expressions: HashMap::new(),
      },
    );

    member
  }
}
