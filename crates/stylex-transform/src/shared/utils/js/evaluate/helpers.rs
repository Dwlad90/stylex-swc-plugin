use super::*;

/// Normalizes different argument types into an ObjectLit for JavaScript object
/// methods.
pub(super) fn normalize_js_object_method_args(
  cached_arg: Option<EvaluateResultValue>,
) -> Option<ObjectLit> {
  cached_arg.and_then(|arg| match arg {
    EvaluateResultValue::Expr(expr) => expr.as_object().cloned().or_else(|| {
      if let Expr::Lit(Lit::Str(ref strng)) = expr {
        let keys = convert_atom_to_string(&strng.value)
          .chars()
          .enumerate()
          .map(|(i, c)| {
            create_ident_key_value_prop(&i.to_string(), create_string_expr(&c.to_string()))
          })
          .collect::<Vec<PropOrSpread>>();

        Some(create_object_lit(keys))
      } else {
        None
      }
    }),

    EvaluateResultValue::Vec(arr) => {
      let props = arr
        .iter()
        .enumerate()
        .filter_map(|(index, elem)| {
          if matches!(elem, EvaluateResultValue::Null) {
            return None;
          }

          let expr = match elem {
            EvaluateResultValue::Expr(expr) => expr.clone(),
            EvaluateResultValue::Vec(vec)
              if vec
                .iter()
                .any(|item| matches!(item, EvaluateResultValue::Null)) =>
            {
              return None;
            },
            EvaluateResultValue::Vec(vec) => normalize_js_object_method_nested_vector_arg(vec),
            _ => stylex_panic!("{}", ILLEGAL_PROP_ARRAY_VALUE),
          };

          Some(create_ident_key_value_prop(&index.to_string(), expr))
        })
        .collect();

      Some(create_object_lit(props))
    },

    _ => None,
  })
}

pub(super) fn normalize_js_object_method_array_arg(
  arr: &ArrayLit,
  traversal_state: &mut StateManager,
  functions: Rc<FunctionMap>,
) -> ObjectLit {
  let mut props = Vec::with_capacity(arr.elems.len());

  for (index, elem) in arr.elems.iter().enumerate() {
    let Some(elem) = elem else {
      continue;
    };

    let result = evaluate_with_functions(&elem.expr, traversal_state, Rc::clone(&functions));

    if !result.confident {
      continue;
    }

    let Some(value) = result.value else {
      continue;
    };

    let expr = match value {
      EvaluateResultValue::Expr(expr) => expr,
      EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(&items),
      EvaluateResultValue::Null => continue,
      _ => continue,
    };

    props.push(create_ident_key_value_prop(&index.to_string(), expr));
  }

  create_object_lit(props)
}

/// Helper function to convert a nested vector of EvaluateResultValues to an
/// array expression
fn normalize_js_object_method_nested_vector_arg(vec: &[EvaluateResultValue]) -> Expr {
  let elems = vec
    .iter()
    .filter(|entry| !matches!(entry, EvaluateResultValue::Null))
    .map(|entry| {
      let expr = entry
        .as_vec()
        .map(|nested_vec| {
          let nested_elems = nested_vec
            .iter()
            .filter(|item| !matches!(item, EvaluateResultValue::Null))
            .map(|item| {
              let expr = match item.as_expr() {
                Some(e) => e,
                None => stylex_panic!("{}", ARGUMENT_NOT_EXPRESSION),
              };
              Some(create_expr_or_spread(expr.clone()))
            })
            .collect();

          create_array_expression(nested_elems)
        })
        .or_else(|| entry.as_expr().cloned())
        .unwrap_or_else(|| stylex_panic!("{}", ILLEGAL_PROP_ARRAY_VALUE));

      Some(create_expr_or_spread(expr))
    })
    .collect();

  create_array_expression(elems)
}

pub(super) fn evaluate_func_call_args(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Vec<EvaluateResultValue> {
  call
    .args
    .iter()
    .filter_map(|arg| evaluate_cached(&arg.expr, state, traversal_state, fns))
    .collect()
}

/// `ToString` over an evaluated value, bridging the evaluator's own value
/// representation to the ECMAScript coercion.
///
/// `None` means the value has no compile-time string form, so the caller
/// deopts. The variants that stand for a JavaScript object take the
/// `Object.prototype` default; the ones that stand for a function have none,
/// because `String(fn)` is its source text and the evaluator keeps no source.
pub(super) fn evaluate_result_to_js_string(value: &EvaluateResultValue) -> Option<String> {
  match value {
    EvaluateResultValue::Expr(expr) => coercions::to_js_string(expr),

    // The evaluator's own array representation, joined by the same rule as an
    // array literal's.
    EvaluateResultValue::Vec(items) => {
      let mut parts = Vec::with_capacity(items.len());

      for item in items {
        parts.push(match item {
          // A confidently evaluated element with no value is `undefined`,
          // which joins as nothing.
          EvaluateResultValue::Null => String::new(),
          EvaluateResultValue::Expr(expr) if coercions::joins_as_empty(expr) => String::new(),
          item => evaluate_result_to_js_string(item)?,
        });
      }

      Some(parts.join(","))
    },

    // A `defineVars` group carries its own `toString`, which answers the var
    // group hash rather than the object default.
    EvaluateResultValue::ThemeRef(theme_ref) => Some(theme_ref.to_string_value()),

    EvaluateResultValue::Map(_)
    | EvaluateResultValue::Entries(_)
    | EvaluateResultValue::EnvObject(_) => Some(coercions::OBJECT_TO_STRING.to_string()),

    EvaluateResultValue::Null
    | EvaluateResultValue::Callback(_)
    | EvaluateResultValue::FunctionConfig(_)
    | EvaluateResultValue::FunctionConfigMap(_) => None,
  }
}

/// `ToNumber` over an evaluated value, bridging the evaluator's own value
/// representation to the ECMAScript coercion.
///
/// Everything that is not already a number reaches its number through its
/// primitive string form, so this refuses on the values
/// `evaluate_result_to_js_string` refuses on — except the functions, which
/// have a number even though they have no string.
pub(super) fn evaluate_result_to_js_number(value: &EvaluateResultValue) -> Option<f64> {
  match value {
    EvaluateResultValue::Expr(expr) => coercions::to_js_number(expr),

    // A function is `NaN` without its source text being known, because no
    // source text is a numeric literal.
    EvaluateResultValue::Callback(_)
    | EvaluateResultValue::FunctionConfig(_)
    | EvaluateResultValue::FunctionConfigMap(_) => Some(f64::NAN),

    _ => evaluate_result_to_js_string(value).map(|strng| coercions::string_to_js_number(&strng)),
  }
}

pub(super) fn args_to_numbers(
  args: &[EvaluateResultValue],
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Vec<f64> {
  let mut numbers = Vec::with_capacity(args.len());
  push_args_to_numbers(args, state, traversal_state, fns, &mut numbers);
  numbers
}

fn push_args_to_numbers(
  args: &[EvaluateResultValue],
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  numbers: &mut Vec<f64>,
) {
  for arg in args {
    match arg {
      EvaluateResultValue::Expr(expr) => numbers.push(
        expr_to_num(expr, state, traversal_state, fns).unwrap_or_else(|error| {
          stylex_panic_with_context!(expr, traversal_state, error.to_string().as_str())
        }),
      ),
      EvaluateResultValue::Vec(vec) => {
        push_args_to_numbers(vec, state, traversal_state, fns, numbers);
      },
      EvaluateResultValue::Null => {},
      _ => stylex_unreachable!("Math.min/max requires a number"),
    }
  }
}

pub(super) fn get_binding<'a>(
  callee: &'a Expr,
  state: &'a StateManager,
) -> Option<&'a VarDeclarator> {
  match callee {
    Expr::Ident(ident) => get_var_decl_from(state, ident),
    _ => None,
  }
}

pub(super) fn evaluate_theme_ref(
  file_name: &str,
  export_name: impl Into<String>,
  state: &StateManager,
) -> ThemeRef {
  ThemeRef::new(
    file_name,
    export_name,
    state.options.class_name_prefix.clone(),
  )
}
