use rustc_hash::FxHashMap;
use stylex_macros::{stylex_panic, stylex_unreachable};
use swc_core::ecma::{
  ast::{
    BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, JSXAttrOrSpread, JSXAttrValue, Prop,
    PropName, PropOrSpread,
  },
  utils::drop_span,
  visit::VisitMutWith,
};

use crate::shared::{
  enums::data_structures::fn_result::FnResult,
  structures::{
    functions::{FunctionConfigType, FunctionMap},
    member_transform::MemberTransform,
    state_manager::{ImportKind, StateManager},
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  transformers::stylex_default_marker,
  utils::{
    ast::convertors::{convert_key_value_to_str, convert_lit_to_string},
    core::{
      make_string_expression::make_string_expression,
      parse_nullable_style::{ResolvedArg, StyleObject, parse_nullable_style},
    },
  },
};
use stylex_ast::ast::factories::{create_jsx_attr, create_jsx_attr_or_spread};
use stylex_constants::constants::{
  api_names::STYLEX_DEFAULT_MARKER, messages::EXPECTED_COMPILED_STYLES,
};
use stylex_enums::style_vars_to_keep::NonNullProps;

pub(crate) fn stylex_merge(
  call: &mut CallExpr,
  transform: fn(&[ResolvedArg]) -> Option<FnResult>,
  state: &mut StateManager,
) -> Option<Expr> {
  let mut bail_out = false;
  let mut conditional = 0;
  let mut current_index = -1;
  let mut bail_out_index = None;

  let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

  if let Some(set) = state.get_stylex_api_import(ImportKind::DefaultMarker) {
    for name in set {
      let values = match stylex_default_marker::stylex_default_marker(&state.options).as_values() {
        Some(v) => v.clone(),
        None => stylex_panic!("{}", EXPECTED_COMPILED_STYLES),
      };
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::IndexMap(values.clone())),
      );
    }
  }

  for name in state.stylex_imports() {
    member_expressions.entry(name.clone()).or_default();

    let member_expression = match member_expressions.get_mut(name) {
      Some(m) => m,
      None => stylex_panic!("Could not resolve the member expression for the import."),
    };

    let values = match stylex_default_marker::stylex_default_marker(&state.options).as_values() {
      Some(v) => v.clone(),
      None => stylex_panic!("{}", EXPECTED_COMPILED_STYLES),
    };
    member_expression.insert(
      STYLEX_DEFAULT_MARKER.into(),
      Box::new(FunctionConfigType::IndexMap(values)),
    );
  }

  state.apply_stylex_env(&mut identifiers, &mut member_expressions);

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
  let mut resolved_args = Vec::with_capacity(args_path.len());

  for arg_path in args_path.iter() {
    current_index += 1;

    let arg = arg_path.expr.as_ref();

    let resolved = if arg.is_object() || arg.is_ident() || arg.is_member() {
      let resolved = parse_nullable_style(arg, state, &evaluate_path_fn_config);

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
        resolved_args.push(ResolvedArg::style_object(resolved));
      },
      Expr::Ident(_) => {
        resolved_args.push(ResolvedArg::style_object(resolved));
      },
      Expr::Member(_) => {
        match resolved {
          StyleObject::Other => {
            //  Already processed in the conditional block above; bail_out flag
            // set if needed.
          },
          StyleObject::Style(_) | StyleObject::Nullable => {
            resolved_args.push(ResolvedArg::style_object(resolved));
          },
          StyleObject::Unreachable => {
            stylex_unreachable!("StyleObject::Unreachable");
          },
        }
      },
      Expr::Cond(CondExpr {
        test,
        cons: consequent,
        alt: alternate,
        ..
      }) => {
        let primary = parse_nullable_style(consequent, state, &evaluate_path_fn_config);
        let fallback = parse_nullable_style(alternate, state, &evaluate_path_fn_config);

        if primary.eq(&StyleObject::Other) || fallback.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          resolved_args.push(ResolvedArg::conditional(
            *test.clone(),
            Some(primary),
            Some(fallback),
          ));

          conditional += 1;
        }
      },
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

        let left_resolved = parse_nullable_style(left_path, state, &evaluate_path_fn_config);
        let right_resolved = parse_nullable_style(right_path, state, &evaluate_path_fn_config);

        if !left_resolved.eq(&StyleObject::Other) || right_resolved.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          resolved_args.push(ResolvedArg::conditional(
            *left_path.clone(),
            Some(right_resolved),
            None,
          ));

          conditional += 1;
        }
      },
      _ => {
        bail_out_index = Some(current_index);
        bail_out = true;
      },
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
        state: &mut *state,
        functions: &evaluate_path_fn_config,
      };

      arg_path.expr.visit_mut_with(&mut member_transform);

      index = member_transform.index;
      bail_out_index = member_transform.bail_out_index;
      non_null_props = member_transform.non_null_props.clone();
    }
  } else {
    let string_expression = make_string_expression(&resolved_args, transform);

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
                let attr_name = convert_key_value_to_str(key_value);
                let attr_value = key_value.value.as_lit().map(|lit| {
                  let s = match convert_lit_to_string(&lit.clone()) {
                    Some(s) => s,
                    None => stylex_panic!("Expected a string class name in compiled styles."),
                  };
                  JSXAttrValue::Str(s.into())
                });

                attr_value.map(|attr_value| {
                  create_jsx_attr_or_spread(create_jsx_attr(attr_name.as_str(), attr_value))
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
