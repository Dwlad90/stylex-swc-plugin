use super::*;
use stylex_ast::ast::convertors::create_ident_expr;

/// `undefined`, as a value the evaluator is confident about.
///
/// Spelled as the identifier rather than as no value, because a confident
/// `None` is how the evaluator says it *failed* to resolve something and the
/// caller turns one into a deopt — so an expression whose value genuinely is
/// `undefined` has to hand back a value, or it fails a build it should have
/// folded.
///
/// One helper rather than one construction per site, because the four places
/// that answer `undefined` have to agree on what they hand back: `void x`, a
/// key an object does not carry, an index past the end of an array, and the
/// winning operand of a logical that evaluated confidently to nothing. `??`
/// reads all four through the same nullish bridge, and a site that answered
/// differently would fold differently for no reason an author could see.
pub(super) fn js_undefined() -> EvaluateResultValue {
  EvaluateResultValue::Expr(create_ident_expr("undefined"))
}

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
  evaluate_result_to_string_of(value, coercions::FunctionForm::Refuse)
}

/// `ToNumber` over an evaluated value, bridging the evaluator's own value
/// representation to the ECMAScript coercion.
///
/// Everything that is not already a number reaches its number through its
/// primitive string form, so this refuses where the string coercion does —
/// except on the functions, which have a number even though they have no
/// string, whether they stand alone or sit inside an array.
pub(super) fn evaluate_result_to_js_number(value: &EvaluateResultValue) -> Option<f64> {
  match value {
    EvaluateResultValue::Expr(expr) => coercions::to_js_number(expr),
    _ => evaluate_result_to_string_of(value, coercions::FunctionForm::NotANumber)
      .map(|strng| coercions::string_to_js_number(&strng)),
  }
}

/// `ToObject` over an evaluated value, bridging the evaluator's own value
/// representation to the ECMAScript coercion.
///
/// `None` means the value's kind cannot be read, so the caller deopts. Every
/// variant the evaluator has of its own stands for either an object or a
/// function upstream, so only the expression variant can reach a wrapper.
pub(super) fn evaluate_result_to_js_object(
  value: &EvaluateResultValue,
) -> Option<coercions::ObjectCoercion> {
  match value {
    EvaluateResultValue::Expr(expr) => coercions::to_object(expr),

    // Unreachable, and refused rather than answered for that reason.
    //
    // `Null` stands for a confidently evaluated value that is absent, which is
    // `undefined` -- whose `ToObject` is a fresh empty object. But no caller
    // can hand one over: every `Null` the evaluator builds is placed inside a
    // `Vec`, and an argument list is collected from `evaluate_cached`, which
    // answers `None` rather than `Some(Null)` for a value that is absent. A
    // bare `Null` therefore only becomes reachable if that changes, and on the
    // day it does the meaning may be "absent" or may be "unknown" -- so this
    // refuses, which deopts under either, where answering an empty object
    // would fold `Object(x)` to `{}` under the second. The nested case, which
    // *is* reachable, is decided in `evaluate_result_to_string_of` below.
    EvaluateResultValue::Null => None,

    EvaluateResultValue::Vec(_)
    | EvaluateResultValue::Map(_)
    | EvaluateResultValue::Entries(_)
    | EvaluateResultValue::EnvObject(_)
    | EvaluateResultValue::ThemeRef(_) => Some(coercions::ObjectCoercion::Identity),

    EvaluateResultValue::Callback(_)
    | EvaluateResultValue::FunctionConfig(_)
    | EvaluateResultValue::FunctionConfigMap(_) => Some(coercions::ObjectCoercion::Function),
  }
}

/// `ToBoolean` over an evaluated value, bridging the evaluator's own value
/// representation to the ECMAScript coercion.
///
/// `None` means the value's truthiness cannot be read, so the caller deopts.
/// Only the expression variant can reach a primitive and so reach the falsy
/// list at all: every variant the evaluator has of its own stands for an object
/// or a function upstream, and those are truthy whatever they hold.
pub(super) fn evaluate_result_to_js_boolean(value: &EvaluateResultValue) -> Option<bool> {
  match value {
    EvaluateResultValue::Expr(expr) => coercions::to_js_boolean(expr),

    // Unreachable for the reason given on `evaluate_result_to_js_object`, and
    // refused on the same terms: read as "absent" a bare `Null` is falsy, read
    // as "unknown" it has no truthiness at all, and a refusal deopts under
    // either where `false` would let `x && y` fold to the wrong operand under
    // the second.
    EvaluateResultValue::Null => None,

    EvaluateResultValue::Vec(_)
    | EvaluateResultValue::Map(_)
    | EvaluateResultValue::Entries(_)
    | EvaluateResultValue::EnvObject(_)
    | EvaluateResultValue::ThemeRef(_)
    | EvaluateResultValue::Callback(_)
    | EvaluateResultValue::FunctionConfig(_)
    | EvaluateResultValue::FunctionConfigMap(_) => Some(true),
  }
}

/// Whether an evaluated value is nullish, bridging the evaluator's own value
/// representation to the coercion crate's question.
///
/// Answers rather than refuses, because nullishness is a question about the
/// value's identity that every variant can settle: only the expression variant
/// can hold `null` or one of the spellings of `undefined`, and every variant
/// the evaluator has of its own stands for an object or a function.
///
/// The absent-value variant is nullish here, where the `ToBoolean` bridge
/// refuses on it. The parting is possible rather than merely chosen: this
/// question is a `bool`, so there is no refusal to give, and a total match has
/// to pick one of the two readings of the variant. It picks "absent", which is
/// the reading the marker slot of a `when` call needs — an absent marker and a
/// marker that evaluated to nothing hand the slot to the options alike.
///
/// The other reading, "unknown", would want a refusal, and its absence costs
/// nothing only because the variant cannot arrive at either caller: every
/// `Null` the evaluator builds is placed inside a `Vec`, and both callers take
/// their value from `evaluate_cached`, which answers `None` rather than
/// `Some(Null)` for a value that is absent. Should that change, `??` is the
/// caller to revisit — it would fold to its right side under a reading that
/// meant "no idea", where the `ToBoolean` bridge's refusal deopts.
pub(crate) fn evaluate_result_is_nullish(value: &EvaluateResultValue) -> bool {
  match value {
    EvaluateResultValue::Expr(expr) => coercions::is_nullish(expr),

    EvaluateResultValue::Null => true,

    EvaluateResultValue::Vec(_)
    | EvaluateResultValue::Map(_)
    | EvaluateResultValue::Entries(_)
    | EvaluateResultValue::EnvObject(_)
    | EvaluateResultValue::ThemeRef(_)
    | EvaluateResultValue::Callback(_)
    | EvaluateResultValue::FunctionConfig(_)
    | EvaluateResultValue::FunctionConfigMap(_) => false,
  }
}

fn evaluate_result_to_string_of(
  value: &EvaluateResultValue,
  function_form: coercions::FunctionForm,
) -> Option<String> {
  match value {
    EvaluateResultValue::Expr(expr) => coercions::to_js_string_with(expr, function_form),

    // The evaluator's own array representation, joined by the same rule as an
    // array literal's.
    EvaluateResultValue::Vec(items) => coercions::join_js_elements(items, |item| match item {
      // A confidently evaluated element with no value is `undefined`, which
      // joins as nothing.
      EvaluateResultValue::Null => Some(String::new()),
      EvaluateResultValue::Expr(expr) if coercions::joins_as_empty(expr) => Some(String::new()),
      item => evaluate_result_to_string_of(item, function_form),
    }),

    // A `defineVars` group carries its own `toString`, which answers the var
    // group hash rather than the object default.
    EvaluateResultValue::ThemeRef(theme_ref) => Some(theme_ref.to_string_value()),

    EvaluateResultValue::Map(_)
    | EvaluateResultValue::Entries(_)
    | EvaluateResultValue::EnvObject(_) => Some(coercions::OBJECT_TO_STRING.to_string()),

    EvaluateResultValue::Callback(_)
    | EvaluateResultValue::FunctionConfig(_)
    | EvaluateResultValue::FunctionConfigMap(_) => function_form.render(),

    // Unreachable for the reason given on `evaluate_result_to_js_object`, and
    // refused on the same terms. The `Vec` arm above is where a `Null` that
    // reaches this bridge is actually decided.
    EvaluateResultValue::Null => None,
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

#[cfg(test)]
#[path = "tests/helpers_tests.rs"]
mod tests;
