use rustc_hash::FxHashMap;
use stylex_ast::ast::convertors::{convert_lit_to_string, key_value_name, normalize_expr};

use swc_core::ecma::{
  ast::{
    BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, JSXAttrOrSpread, JSXAttrValue, Lit,
    ObjectLit, Prop, PropName, PropOrSpread,
  },
  visit::{VisitMut, VisitMutWith, VisitWith},
};

use crate::shared::{
  enums::data_structures::fn_result::FnResult,
  transformers::stylex_default_marker,
  utils::core::{
    make_string_expression::make_string_expression,
    member_expression::MemberTransform,
    parse_nullable_style::{ResolvedArg, StyleObject, parse_nullable_style},
  },
};
use stylex_ast::ast::factories::{create_jsx_attr, create_jsx_attr_or_spread};
use stylex_constants::constants::{api_names::STYLEX_DEFAULT_MARKER, common::COMPILED_KEY};
use stylex_enums::style_vars_to_keep::NonNullProps;
use stylex_state::{
  functions::{FunctionConfigType, FunctionMap},
  state_manager::{ImportKind, StateManager},
  types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
};

/// Merges the arguments of a `stylex.props`-family call into one value.
///
/// The caller supplies `hoist_expression`, which lifts an expression into a
/// module-scoped `const` and returns a reference to it.
pub(crate) fn stylex_merge(
  call: &mut CallExpr,
  transform: fn(&[ResolvedArg]) -> FnResult,
  hoist_expression: fn(Expr, &mut StateManager) -> Expr,
  state: &mut StateManager,
) -> Option<Expr> {
  let mut bail_out = false;
  let mut conditional = 0;
  let mut current_index = -1;
  let mut bail_out_index = None;

  let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

  // The marker is the same for every name it is registered under, so it is
  // built once for both loops below. It made two strings, an index map and two
  // counted pointers per import before, and the two loops built it twice over.
  let marker_values = stylex_default_marker::stylex_default_marker_values(&state.options);

  if let Some(set) = state.get_stylex_api_import(ImportKind::DefaultMarker)
    && !set.is_empty()
  {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::IndexMap(marker_values.clone())),
      );
    }
  }

  for name in state.stylex_imports() {
    // `or_default` gives back the entry it made, so the second look-up that
    // stood here, and the refusal that could never run, are both unnecessary.
    member_expressions.entry(name.clone()).or_default().insert(
      STYLEX_DEFAULT_MARKER.into(),
      Box::new(FunctionConfigType::IndexMap(marker_values.clone())),
    );
  }

  state.apply_stylex_env(&mut identifiers, &mut member_expressions);

  let evaluate_path_fn_config = FunctionMap {
    identifiers,
    member_expressions,
    disable_imports: true,
  };

  // A parenthesis is not a different argument, at either level. Read bare,
  // `stylex.props(([a, b]))` was not flattened into its elements and
  // `stylex.props((styles.root))` reached no arm of the match below, so both
  // handed the whole merge back to the runtime.
  let args_path = call
    .args
    .iter()
    .flat_map(|arg| match normalize_expr(&arg.expr) {
      Expr::Array(arr) => arr.elems.clone(),
      _ => vec![Some(arg.clone())],
    })
    .flatten()
    .collect::<Vec<ExprOrSpread>>();
  let mut resolved_args = Vec::with_capacity(args_path.len());

  for arg_path in args_path.iter() {
    current_index += 1;

    let arg = normalize_expr(&arg_path.expr);

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
          // Never `Unreachable`: it is written twenty lines above, for an
          // argument that is none of object, name, member or call, and this arm
          // is the member one.
          resolved => resolved_args.push(ResolvedArg::style_object(resolved)),
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
        non_null_props,
        state: &mut *state,
        functions: &evaluate_path_fn_config,
      };

      arg_path.expr.visit_with(&mut member_transform);

      index = member_transform.index;
      bail_out_index = member_transform.bail_out_index;
      non_null_props = member_transform.non_null_props;

      // Hoist any inline compiled-style objects (produced by atoms) to module
      // scope so the runtime `stylex.props` receives a stable reference instead
      // of a re-created object literal.
      //
      // A second walk, and it stays one. The reader above stops at a member
      // expression, because counting a nested one would move the bail-out
      // point, while this walk must reach an object wherever it sits. One walk
      // could serve only one of those two rules.
      let mut object_hoister = CompiledStyleObjectHoister {
        state: &mut *state,
        hoist_expression,
      };
      arg_path.expr.visit_mut_with(&mut object_hoister);
    }
  } else {
    let string_expression = make_string_expression(&resolved_args, transform);

    if let Expr::Object(string_expression) = &string_expression
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

    return Some(string_expression);
  }

  None
}

/// The JSX attribute a compiled property spells, where it spells one.
///
/// The properties read here are the ones `make_string_expression` built: a
/// key-value pair under a plain name, never a spread and never computed. What
/// is left to decide is the value, and only a literal can be written into an
/// attribute.
fn static_jsx_attr_from_prop(prop: &PropOrSpread) -> Option<JSXAttrOrSpread> {
  prop
    .as_prop()
    .and_then(|prop| prop.as_key_value())
    .and_then(|key_value| {
      key_value
        .value
        .as_lit()
        .and_then(convert_lit_to_string)
        .map(|value| {
          create_jsx_attr_or_spread(create_jsx_attr(
            key_value_name(key_value).as_ref(),
            JSXAttrValue::Str(value.into()),
          ))
        })
    })
}

/// Hoists inline compiled-style objects (those carrying the `$$css: true`
/// marker, produced by the atoms transform) out of a `stylex.props(...)`
/// argument and into a module-scoped `const`, replacing the object with a
/// reference to it.
struct CompiledStyleObjectHoister<'a> {
  state: &'a mut StateManager,
  hoist_expression: fn(Expr, &mut StateManager) -> Expr,
}

impl VisitMut for CompiledStyleObjectHoister<'_> {
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    expr.visit_mut_children_with(self);

    if let Expr::Object(object) = expr
      && object_has_css_marker(object)
    {
      let hoisted = (self.hoist_expression)(expr.clone(), self.state);
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

#[cfg(test)]
#[path = "tests/stylex_merge_tests.rs"]
mod tests;
