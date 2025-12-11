use rustc_hash::FxHashMap;
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, Ident, IdentName, JSXAttr,
      JSXAttrName, JSXAttrOrSpread, JSXAttrValue, Lit, MemberExpr, Prop, PropName, PropOrSpread,
    },
    utils::{ExprExt, drop_span},
    visit::FoldWith,
  },
};

use crate::shared::{
  enums::data_structures::{fn_result::FnResult, style_vars_to_keep::NonNullProps},
  structures::{
    functions::{FunctionConfigType, FunctionMap},
    member_transform::MemberTransform,
    state_manager::StateManager,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  swc::get_default_expr_ctx,
  transformers::stylex_default_maker,
  utils::{
    ast::convertors::{key_value_to_str, lit_to_string},
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

  let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

  for name in &state.stylex_default_marker_import {
    identifiers.insert(
      name.clone(),
      Box::new(FunctionConfigType::IndexMap(
        stylex_default_maker::stylex_default_marker(&state.options)
          .as_values()
          .expect("Expected FlatCompiledStylesValues")
          .clone(),
      )),
    );
  }

  for name in &state.stylex_import {
    member_expressions.entry(name.clone()).or_default();

    let member_expression = member_expressions.get_mut(name).unwrap();

    member_expression.insert(
      "defaultMarker".into(),
      Box::new(FunctionConfigType::IndexMap(
        stylex_default_maker::stylex_default_marker(&state.options)
          .as_values()
          .expect("Expected FlatCompiledStylesValues")
          .clone(),
      )),
    );
  }

  let evaluate_path_fn_config = FunctionMap {
    identifiers,
    member_expressions,
    disable_imports: true,
  };

  let args_path = call
    .args
    .iter()
    .flat_map(|arg| match arg.expr.as_ref() {
      Expr::Array(arr) => arr.elems.clone(),
      _ => vec![Some(arg.clone())],
    })
    .flatten()
    .collect::<Vec<ExprOrSpread>>();

  for arg_path in args_path.iter() {
    current_index += 1;

    let arg = arg_path.expr.as_ref();

    let resolved = if arg.is_object() || arg.is_ident() || arg.is_member() {
      let resolved = parse_nullable_style(arg, state, &evaluate_path_fn_config, false);

      if let StyleObject::Other = resolved {
        bail_out_index = Some(current_index);
        bail_out = true;
      }

      resolved
    } else {
      StyleObject::Unreachable
    };

    match &arg {
      Expr::Object(_) => {
        resolved_args.push(ResolvedArg::StyleObject(
          resolved,
          Vec::default(),
          Vec::default(),
        ));
      }
      Expr::Ident(ident) => {
        resolved_args.push(ResolvedArg::StyleObject(
          resolved,
          vec![ident.clone()],
          Vec::default(),
        ));
      }
      Expr::Member(member) => {
        match resolved {
          StyleObject::Other => {
            //  Already processed in the conditional block above; bail_out flag set if needed.
          }
          StyleObject::Style(_) | StyleObject::Nullable => {
            let ident = member
              .obj
              .as_ident()
              .expect("Member obj is not an ident")
              .clone();

            resolved_args.push(ResolvedArg::StyleObject(
              resolved,
              vec![ident],
              vec![member.clone()],
            ));
          }
          StyleObject::Unreachable => {
            unreachable!("StyleObject::Unreachable");
          }
        }
      }
      Expr::Cond(CondExpr {
        test,
        cons: consequent,
        alt: alternate,
        ..
      }) => {
        let primary = parse_nullable_style(consequent, state, &evaluate_path_fn_config, true);
        let fallback = parse_nullable_style(alternate, state, &evaluate_path_fn_config, true);

        if primary.eq(&StyleObject::Other) || fallback.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          let idents = get_conditional_expr_idents(alternate.as_ref())?;
          let members = get_conditional_expr_members(alternate.as_ref())?;

          resolved_args.push(ResolvedArg::ConditionalStyle(
            *test.clone(),
            Some(primary),
            Some(fallback),
            idents,
            members,
          ));

          conditional += 1;
        }
      }
      Expr::Bin(BinExpr {
        left: left_path,
        op,
        right: right_path,
        ..
      }) => {
        if !op.eq(&BinaryOp::LogicalAnd) {
          bail_out_index = Some(current_index);
          bail_out = true;
          break;
        }

        let left_resolved = parse_nullable_style(left_path, state, &evaluate_path_fn_config, true);
        let right_resolved =
          parse_nullable_style(right_path, state, &evaluate_path_fn_config, true);

        if !left_resolved.eq(&StyleObject::Other) || right_resolved.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          let ident = match right_path.as_ref() {
            Expr::Ident(ident) => ident,
            Expr::Member(member) => member.obj.as_ident().expect("Member obj is not an ident"),
            _ => panic!(
              "Illegal argument: {:?}",
              right_path.get_type(get_default_expr_ctx())
            ),
          };

          let member = match right_path.as_ref() {
            Expr::Member(member) => member,
            _ => panic!(
              "Illegal argument: {:?}",
              right_path.get_type(get_default_expr_ctx())
            ),
          };

          resolved_args.push(ResolvedArg::ConditionalStyle(
            *left_path.clone(),
            Some(right_resolved),
            None,
            vec![ident.clone()],
            vec![member.clone()],
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
        functions: evaluate_path_fn_config.clone(),
      };

      let transformed_expr = arg_path.expr.clone().fold_with(&mut member_transform);

      arg_path.expr = transformed_expr;

      index = member_transform.index;
      bail_out_index = member_transform.bail_out_index;
      non_null_props = member_transform.non_null_props;

      *state = member_transform.state;
    }

    for arg in args_path.iter() {
      if let Expr::Member(member_expression) = arg.expr.as_ref() {
        reduce_member_expression_count(state, member_expression)
      }
    }
  } else {
    let string_expression = make_string_expression(&resolved_args, transform);

    for arg in &resolved_args {
      match arg {
        ResolvedArg::StyleObject(_, idents, member_expr) => {
          for ident in idents {
            reduce_ident_count(state, ident);
          }

          for member_expr in member_expr {
            reduce_member_expression_count(state, member_expr);
          }
        }
        ResolvedArg::ConditionalStyle(_, _, _, idents, member_expr) => {
          for ident in idents {
            reduce_ident_count(state, ident);
          }
          for member_expr in member_expr {
            reduce_member_expression_count(state, member_expr);
          }
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
                let attr_value = key_value.value.as_lit().map(|lit| {
                  JSXAttrValue::Str(
                    lit_to_string(&lit.clone())
                      .expect("Failed to get string value")
                      .into(),
                  )
                });

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

fn get_conditional_expr_idents(alternate: &Expr) -> Option<Vec<Ident>> {
  match alternate {
    Expr::Ident(ident) => {
      if ident.sym == "undefined" {
        return None;
      }

      Some(vec![ident.clone()])
    }
    Expr::Member(member) => Some(vec![
      member
        .obj
        .as_ident()
        .expect("Member obj is not an ident")
        .clone(),
    ]),
    Expr::Lit(Lit::Null(_) | Lit::Bool(_)) => None,
    Expr::Array(array) => {
      let mut idents = Vec::new();

      for elem in array.elems.iter().flatten() {
        match get_conditional_expr_idents(&elem.expr) {
          Some(mut elem_idents) => {
            idents.append(&mut elem_idents);
          }
          None => {
            return None;
          }
        }
      }

      Some(idents)
    }
    Expr::Cond(cond_expr) => {
      let mut idents = Vec::new();

      match get_conditional_expr_idents(&cond_expr.alt) {
        Some(mut alt_idents) => {
          idents.append(&mut alt_idents);
        }
        None => {
          return None;
        }
      }

      Some(idents)
    }
    _ => {
      panic!(
        "Illegal argument: {:?}",
        alternate.get_type(get_default_expr_ctx())
      )
    }
  }
}

fn get_conditional_expr_members(alternate: &Expr) -> Option<Vec<MemberExpr>> {
  match alternate {
    Expr::Member(member) => Some(vec![member.clone()]),
    Expr::Array(array) => {
      let mut members = Vec::new();

      for elem in array.elems.iter().flatten() {
        if let Some(mut elem_members) = get_conditional_expr_members(&elem.expr) {
          members.append(&mut elem_members);
        }
      }

      Some(members)
    }
    Expr::Cond(cond_expr) => get_conditional_expr_members(&cond_expr.alt),
    _ => {
      panic!(
        "Illegal argument in get_conditional_expr_member: {:?}",
        alternate.get_type(get_default_expr_ctx())
      )
    }
  }
}
