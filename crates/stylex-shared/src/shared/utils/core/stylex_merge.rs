use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, IdentName, JSXAttr, JSXAttrName,
      JSXAttrOrSpread, JSXAttrValue, Lit, Prop, PropName, PropOrSpread,
    },
    utils::{ExprExt, drop_span},
    visit::FoldWith,
  },
};

use crate::shared::{
  enums::data_structures::{fn_result::FnResult, style_vars_to_keep::NonNullProps},
  structures::{member_transform::MemberTransform, state_manager::StateManager},
  swc::get_default_expr_ctx,
  utils::{
    ast::convertors::key_value_to_str,
    common::{reduce_ident_count, reduce_member_expression_count},
    core::{
      make_string_expression::make_string_expression,
      parse_nullable_style::{ResolvedArg, StyleObject, parse_nullable_style},
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
            Expr::Ident(ident) => {
              if ident.sym == "undefined" {
                return None;
              }

              ident
            }
            Expr::Member(member) => member.obj.as_ident().expect("Member obj is not an ident"),
            Expr::Lit(Lit::Null(_) | Lit::Bool(_)) => return None,
            _ => panic!(
              "Illegal argument: {:?}",
              alt.get_type(get_default_expr_ctx())
            ),
          };

          let member = match alt.as_ref() {
            Expr::Member(member) => member,
            _ => panic!(
              "Illegal argument: {:?}",
              alt.get_type(get_default_expr_ctx())
            ),
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
            _ => panic!(
              "Illegal argument: {:?}",
              right.get_type(get_default_expr_ctx())
            ),
          };

          let member = match right.as_ref() {
            Expr::Member(member) => member,
            _ => panic!(
              "Illegal argument: {:?}",
              right.get_type(get_default_expr_ctx())
            ),
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

  if !state.enable_inlined_conditional_merge() && conditional > 0 {
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

      *state = member_transform.state;
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

    if let Some(Expr::Object(string_expression)) = string_expression.as_ref() {
      let attr_expr = drop_span(Expr::Call(call.clone()));

      if state.jsx_spread_attr_exprs_map.contains_key(&attr_expr)
        && !string_expression.props.is_empty()
        && string_expression.props.iter().all(|prop| {
          matches!(prop, PropOrSpread::Prop(prop)
            if matches!(prop.as_ref(), Prop::KeyValue(kv)
              if !matches!(kv.key, PropName::Computed(_))))
        })
      {
        // Check if this is used as a JSX spread attribute and optimize
        // Convert each property to JSX attributes for direct use
        let jsx_attr_expressions: Vec<JSXAttrOrSpread> = string_expression
          .props
          .iter()
          .filter_map(|prop| {
            if let PropOrSpread::Prop(prop) = prop {
              if let Prop::KeyValue(key_value) = prop.as_ref() {
                // Create JSX attribute directly
                let attr_name = key_value_to_str(key_value);
                let attr_value = key_value
                  .value
                  .as_lit()
                  .map(|lit| JSXAttrValue::Lit(lit.clone()));

                attr_value.map(|attr_value| {
                  JSXAttrOrSpread::JSXAttr(JSXAttr {
                    span: DUMMY_SP,
                    name: JSXAttrName::Ident(IdentName::from(attr_name.as_str())),
                    value: Some(attr_value),
                  })
                })
              } else {
                None
              }
            } else {
              None
            }
          })
          .collect();

        // Store the JSX attributes to replace the spread element
        if !jsx_attr_expressions.is_empty() {
          state
            .jsx_spread_attr_exprs_map
            .insert(attr_expr, jsx_attr_expressions);

          return None; // Early return to skip normal object creation
        }
      }
    }

    return string_expression;
  }

  None
}
