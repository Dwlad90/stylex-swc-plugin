use std::collections::HashMap;

use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{CallExpr, Expr, MemberExpr},
    visit::{noop_fold_type, Fold},
  },
};

use crate::{
  shared::{
    enums::NonNullProps,
    structures::{functions::FunctionMap, state_manager::StateManager},
    utils::{
      stylex::{attrs::attrs, member_expression::member_expression, stylex_merge::stylex_merge},
      validators::is_attrs_call,
    },
  },
  ModuleTransformVisitor,
};

// #[derive(Clone, Debug)]
// struct MemberTransform {
//     pub(crate) index: i32,
//     pub(crate) bail_out_index: Option<i32>,
//     pub(crate) non_null_props: NonNullProps,
//     pub(crate) state: StateManager,
// }

// impl Fold for MemberTransform {
//     noop_fold_type!();

//     fn fold_member_expr(&mut self, member: MemberExpr) -> MemberExpr {
//         member_expression(
//             &member,
//             &mut self.index,
//             &mut self.bail_out_index,
//             &mut self.non_null_props,
//             &mut self.state,
//             &FunctionMap {
//                 identifiers: HashMap::new(),
//                 member_expressions: HashMap::new(),
//             },
//             &vec![],
//         );

//         member
//     }
// }

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_attrs_call(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_attrs_call = is_attrs_call(call, &mut self.state);

    if is_attrs_call {
      return stylex_merge(call, attrs, &mut self.state);
    }

    Option::None
  }
}
