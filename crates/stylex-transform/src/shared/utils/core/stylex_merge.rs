use rustc_hash::FxHashMap;
use stylex_macros::{stylex_panic, stylex_unreachable};
use swc_core::ecma::{
  ast::{
    BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, JSXAttrOrSpread, JSXAttrValue, Lit,
    ObjectLit, Prop, PropName, PropOrSpread,
  },
  visit::{VisitMut, VisitMutWith},
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
use crate::transform::stylex::transform_stylex_create_call::hoist_expression;
use stylex_ast::ast::factories::{create_jsx_attr, create_jsx_attr_or_spread};
use stylex_constants::constants::{
  api_names::STYLEX_DEFAULT_MARKER, common::COMPILED_KEY, messages::EXPECTED_COMPILED_STYLES,
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

    let resolved = if arg.is_object() || arg.is_ident() || arg.is_member() || arg.is_call() {
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
      Expr::Call(_) => {
        // A call argument (dynamic atom `_temp.color(c)`, dynamic create style
        // `styles.opacity(1)`, etc.) cannot be statically merged. The bail-out
        // recorded above keeps it in the runtime `stylex.props` call.
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
        } else {
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
      // Stop at the first bail. On the bail path `resolved_args`/`conditional`
      // are discarded entirely and the output is produced by the
      // `MemberTransform` re-walk over *all* `call.args` below — which already
      // registers and keeps every member arg's `stylex.create` styles. Scanning
      // further here would only repeat `parse_nullable_style`/`evaluate` work on
      // args past the bail and risk choking on a shape only that path rejects.
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

      // Hoist any inline compiled-style objects (produced by atoms) to module
      // scope so the runtime `stylex.props` receives a stable reference instead
      // of a re-created object literal.
      let mut object_hoister = CompiledStyleObjectHoister { state: &mut *state };
      arg_path.expr.visit_mut_with(&mut object_hoister);
    }
  } else {
    let string_expression = make_string_expression(&resolved_args, transform);

    if let Some(Expr::Object(string_expression)) = string_expression.as_ref()
      && state.has_jsx_spread_call(call)
      && !string_expression.props.is_empty()
    {
      let jsx_attr_expressions = string_expression
        .props
        .iter()
        .map(static_jsx_attr_from_prop)
        .collect::<Option<Vec<_>>>();

      // Store the JSX attributes to replace the spread element
      if let Some(jsx_attr_expressions) = jsx_attr_expressions {
        state.set_jsx_spread_replacement(call, jsx_attr_expressions);

        return None; // Early return to skip normal object creation
      }
    }

    return string_expression;
  }

  None
}

fn static_jsx_attr_from_prop(prop: &PropOrSpread) -> Option<JSXAttrOrSpread> {
  let PropOrSpread::Prop(prop) = prop else {
    return None;
  };
  let Prop::KeyValue(key_value) = prop.as_ref() else {
    return None;
  };
  if matches!(key_value.key, PropName::Computed(_)) {
    return None;
  }

  let value = key_value
    .value
    .as_lit()
    .and_then(convert_lit_to_string)
    .map(|value| JSXAttrValue::Str(value.into()))?;
  let attr_name = convert_key_value_to_str(key_value);

  Some(create_jsx_attr_or_spread(create_jsx_attr(
    attr_name.as_str(),
    value,
  )))
}

/// Hoists inline compiled-style objects (those carrying the `$$css: true`
/// marker, produced by the atoms transform) out of a `stylex.props(...)`
/// argument and into a module-scoped `const`, replacing the object with a
/// reference to it.
struct CompiledStyleObjectHoister<'a> {
  state: &'a mut StateManager,
}

impl VisitMut for CompiledStyleObjectHoister<'_> {
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    expr.visit_mut_children_with(self);

    if let Expr::Object(object) = expr
      && object_has_css_marker(object)
    {
      let hoisted = hoist_expression(expr.clone(), self.state);
      *expr = hoisted;
    }
  }
}

/// Whether an object literal carries a `$$css: true` property, marking it as a
/// compiled StyleX style object.
fn object_has_css_marker(object: &ObjectLit) -> bool {
  object.props.iter().any(|prop| {
    let PropOrSpread::Prop(prop) = prop else {
      return false;
    };
    let Prop::KeyValue(key_value) = prop.as_ref() else {
      return false;
    };

    let is_css_key = match &key_value.key {
      PropName::Ident(ident) => ident.sym.as_ref() == COMPILED_KEY,
      PropName::Str(strng) => strng.value.as_str() == Some(COMPILED_KEY),
      _ => false,
    };

    is_css_key
      && matches!(key_value.value.as_ref(), Expr::Lit(Lit::Bool(bool_lit)) if bool_lit.value)
  })
}
