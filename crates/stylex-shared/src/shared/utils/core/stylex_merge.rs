use swc_core::ecma::{
  ast::{BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, Lit},
  utils::ExprExt,
  visit::FoldWith,
};

use crate::shared::{
  enums::data_structures::{fn_result::FnResult, style_vars_to_keep::NonNullProps},
  structures::{member_transform::MemberTransform, state_manager::StateManager},
  utils::{
    common::{reduce_ident_count, reduce_member_expression_count},
    core::{
      make_string_expression::make_string_expression,
      parse_nullable_style::{parse_nullable_style, ResolvedArg, StyleObject},
    },
  },
};

pub(crate) fn stylex_merge(
  call: &mut CallExpr,
  transform: fn(&[ResolvedArg]) -> Option<FnResult>,
  state: &mut StateManager,
) -> Option<Expr> {
  let mut bail_out = false;
  let mut conditional = 0;
  let mut current_index = -1;
  let mut bail_out_index = None;
  let mut resolved_args = vec![];

  let args = call
    .args
    .iter()
    .flat_map(|arg| match arg.expr.as_ref() {
      Expr::Array(arr) => arr.elems.clone(),
      _ => vec![Some(arg.clone())],
    })
    .flatten()
    .collect::<Vec<ExprOrSpread>>();

  for arg in args.iter() {
    current_index += 1;

    let arg = arg.expr.as_ref();

    match &arg {
      Expr::Member(member) => {
        let resolved = parse_nullable_style(arg, state, false);

        match resolved {
          StyleObject::Other => {
            bail_out_index = Some(current_index);
            bail_out = true;
          }
          StyleObject::Style(_) | StyleObject::Nullable => {
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
        let primary = parse_nullable_style(cons, state, true);
        let fallback = parse_nullable_style(alt, state, true);

        if primary.eq(&StyleObject::Other) || fallback.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          let ident = match alt.as_ref() {
            Expr::Ident(ident) => ident,
            Expr::Member(member) => member.obj.as_ident().expect("Member obj is not an ident"),
            Expr::Lit(Lit::Null(_) | Lit::Bool(_)) => return None,
            _ => panic!("Illegal argument: {:?}", alt.get_type()),
          };

          let member = match alt.as_ref() {
            Expr::Member(member) => member,
            _ => panic!("Illegal argument: {:?}", alt.get_type()),
          };

          resolved_args.push(ResolvedArg::ConditionalStyle(
            *test.clone(),
            Some(primary),
            Some(fallback),
            ident.clone(),
            member.clone(),
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

        let left_resolved = parse_nullable_style(left, state, true);
        let right_resolved = parse_nullable_style(right, state, true);

        if !left_resolved.eq(&StyleObject::Other) || right_resolved.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          let ident = match right.as_ref() {
            Expr::Ident(ident) => ident,
            Expr::Member(member) => member.obj.as_ident().expect("Member obj is not an ident"),
            _ => panic!("Illegal argument: {:?}", right.get_type()),
          };

          let member = match right.as_ref() {
            Expr::Member(member) => member,
            _ => panic!("Illegal argument: {:?}", right.get_type()),
          };

          resolved_args.push(ResolvedArg::ConditionalStyle(
            *left.clone(),
            Some(right_resolved),
            None,
            ident.clone(),
            member.clone(),
          ));

          conditional += 1;
        }
      }
      _ => {
        bail_out_index = Some(current_index);
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
    let mut non_null_props: NonNullProps = NonNullProps::Vec(vec![]);
    let mut index = -1;

    for arg_path in call.args.iter_mut() {
      index += 1;

      let mut member_transform = MemberTransform {
        index,
        bail_out_index,
        non_null_props: non_null_props.clone(),
        state: state.clone(),
        parents: vec![],
      };

      let transformed_expr = arg_path.expr.clone().fold_with(&mut member_transform);

      arg_path.expr = transformed_expr;

      index = member_transform.index;
      bail_out_index = member_transform.bail_out_index;
      non_null_props = member_transform.non_null_props;

      *state = member_transform.state.clone();
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
          reduce_ident_count(state, ident);
          reduce_member_expression_count(state, member_expr)
        }
        ResolvedArg::ConditionalStyle(_, _, _, ident, member_expr) => {
          reduce_ident_count(state, ident);
          reduce_member_expression_count(state, member_expr)
        }
      }
    }

    return string_expression;
  }

  None
}
