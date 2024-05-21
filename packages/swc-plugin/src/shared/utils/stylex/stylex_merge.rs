use std::collections::HashMap;

use swc_core::ecma::{
  ast::{BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, MemberExpr},
  visit::{noop_fold_type, Fold, FoldWith},
};

use crate::shared::{
  enums::{FnResult, NonNullProps},
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::{
    common::{reduce_ident_count, reduce_member_expression_count},
    stylex::{
      make_string_expression::make_string_expression,
      parse_nullable_style::{parse_nullable_style, ResolvedArg, StyleObject},
    },
  },
};

use super::member_expression::member_expression;

#[derive(Clone, Debug)]
struct MemberTransform {
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

pub(crate) fn stylex_merge(
  call: &CallExpr,
  transform: fn(&Vec<ResolvedArg>) -> Option<FnResult>,
  state: &mut StateManager,
) -> Option<Expr> {
  let mut bail_out = false;
  let mut conditional = 0;
  let mut current_index = -1;
  let mut bail_out_index = Option::None;
  let mut resolved_args = vec![];

  let args = call
    .args
    .iter()
    .flat_map(|arg| {
      assert!(arg.spread.is_none(), "Spread not implemented yet");

      match arg.expr.as_ref() {
        Expr::Array(arr) => arr.elems.clone(),
        _ => vec![Some(arg.clone())],
      }
    })
    .flatten()
    .collect::<Vec<ExprOrSpread>>();

  for arg in args.iter() {
    current_index += 1;

    assert!(arg.spread.is_none(), "Spread not implemented yet");

    let arg = arg.expr.as_ref();

    match arg.clone() {
      Expr::Member(member) => {
        let resolved = parse_nullable_style(arg, state, false);

        match resolved {
          StyleObject::Other => {
            bail_out_index = Option::Some(current_index);
            bail_out = true;
          }
          StyleObject::Style(_) => {
            resolved_args.push(ResolvedArg::StyleObject(
              resolved,
              member
                .obj
                .as_ident()
                .expect("Member obj is not an ident")
                .clone(),
              member.clone(),
            ));
          }
          StyleObject::Nullable => {
            resolved_args.push(ResolvedArg::StyleObject(
              resolved,
              member
                .obj
                .as_ident()
                .expect("Member obj is not an ident")
                .clone(),
              member.clone(),
            ));
          }
        }
      }
      Expr::Cond(CondExpr {
        test, cons, alt, ..
      }) => {
        let primary = parse_nullable_style(&cons, state, true);
        let fallback = parse_nullable_style(&alt, state, true);

        if primary.eq(&StyleObject::Other) || fallback.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          let ident = match alt.as_ref() {
            Expr::Ident(ident) => ident.clone(),
            Expr::Member(meber) => meber
              .obj
              .as_ident()
              .expect("Member obj is not an ident")
              .clone(),
            _ => panic!("Illegal argument"),
          };

          let member = match alt.as_ref() {
            Expr::Member(meber) => meber.clone(),
            _ => panic!("Illegal argument"),
          };

          resolved_args.push(ResolvedArg::ConditionalStyle(
            test,
            Some(primary),
            Some(fallback),
            ident,
            member,
          ));

          conditional += 1;
        }
      }
      Expr::Bin(BinExpr {
        left, op, right, ..
      }) => {
        if !op.eq(&BinaryOp::LogicalAnd) {
          bail_out_index = Some(current_index);
          bail_out = true;
          break;
        }

        let left_resolved = parse_nullable_style(&left, state, true);
        let right_resolved = parse_nullable_style(&right, state, true);

        if !left_resolved.eq(&StyleObject::Other) || right_resolved.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          let ident = match right.as_ref() {
            Expr::Ident(ident) => ident.clone(),
            Expr::Member(meber) => meber
              .obj
              .as_ident()
              .expect("Member obj is not an ident")
              .clone(),
            _ => panic!("Illegal argument"),
          };

          let member = match right.as_ref() {
            Expr::Member(meber) => meber.clone(),
            _ => panic!("Illegal argument"),
          };

          resolved_args.push(ResolvedArg::ConditionalStyle(
            left,
            Some(right_resolved),
            None,
            ident,
            member,
          ));

          conditional += 1;
        }
      }

      _ => {
        bail_out_index = Option::Some(current_index);
        bail_out = true;
      }
    }

    if conditional > 4 {
      bail_out = true;
    }

    if bail_out {
      break;
    }
  }
  if !state.gen_conditional_classes() && conditional > 0 {
    bail_out = true;
  }

  if bail_out {
    let arguments_path = call.args.clone();

    let mut non_null_props: NonNullProps = NonNullProps::Vec(vec![]);

    let mut index = -1;

    for mut arg_path in arguments_path.into_iter() {
      index += 1;

      assert!(arg_path.spread.is_none(), "Spread not implemented yet");

      let mut member_transfom = MemberTransform {
        index,
        bail_out_index,
        non_null_props: non_null_props.clone(),
        state: state.clone(),
        parents: vec![],
      };

      arg_path.expr = arg_path.expr.clone().fold_with(&mut member_transfom);

      index = member_transfom.index;
      bail_out_index = member_transfom.bail_out_index;
      non_null_props = member_transfom.non_null_props;
      *state = member_transfom.state;
    }

    for arg in args.iter() {
      if let Expr::Member(member_expression) = arg.expr.as_ref() {
        reduce_member_expression_count(state, member_expression)
      }
    }
  } else {
    let string_expression = make_string_expression(&resolved_args, transform);

    for arg in &resolved_args {
      match arg {
        ResolvedArg::StyleObject(_, ident, member_expr) => {
          reduce_ident_count(&mut *state, ident);
          reduce_member_expression_count(state, member_expr)
        }
        ResolvedArg::ConditionalStyle(_, _, _, ident, member_expr) => {
          reduce_ident_count(&mut *state, ident);
          reduce_member_expression_count(state, member_expr)
        }
      }
    }

    return string_expression;
  }

  None
}
