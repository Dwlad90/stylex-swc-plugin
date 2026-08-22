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
      let mut props = Vec::with_capacity(arr.len());

      for (index, elem) in arr.iter().enumerate() {
        let expr = match elem {
          EvaluateResultValue::Expr(expr) => expr.clone(),
          // A hole, and a nested array holding one, are skipped rather than
          // refused: an absent element has no key of its own in the object
          // form, exactly as `Object.keys([, 1])` omits index zero.
          EvaluateResultValue::Null => continue,
          EvaluateResultValue::Vec(vec)
            if vec
              .iter()
              .any(|item| matches!(item, EvaluateResultValue::Null)) =>
          {
            continue;
          },
          EvaluateResultValue::Vec(vec) => normalize_js_object_method_nested_vector_arg(vec)?,
          // An element with no expression form leaves the whole receiver
          // unreadable, which is the same answer the arms below give for a
          // value that is not an object at all.
          _ => return None,
        };

        props.push(create_ident_key_value_prop(&index.to_string(), expr));
      }

      Some(create_object_lit(props))
    },

    _ => None,
  })
}

/// What an `Object.keys`/`values`/`entries` receiver reads as.
///
/// Three answers rather than two, because "no own keys" and "cannot be read"
/// are both spelled by an absent object and mean opposite things:
/// `Object.keys(5)` is `[]` in JavaScript and folds, while a receiver holding
/// an element with no expression form has to refuse — answering `[]` there
/// would write a shorter list into the stylesheet than the source describes.
pub(super) enum ObjectMethodReceiver {
  /// Read as an object carrying these properties.
  Object(ObjectLit),
  /// Not an object, so it contributes no own keys. `Object.keys(5)` is `[]`.
  NoOwnKeys,
  /// An element has no expression form, so the receiver cannot be read at all
  /// and the caller refuses rather than answering a short list.
  Unreadable,
}

/// Reads the receiver of `Object.keys`, `Object.values` or `Object.entries`,
/// from the evaluated argument where there is one and from the array literal
/// otherwise.
///
/// One function rather than the same `or_else` chain at all three call sites:
/// they have to agree on what an unreadable element means, and three copies
/// edited separately is the shape of the bug this split exists to remove.
pub(super) fn normalize_object_method_receiver(
  cached_arg: Option<EvaluateResultValue>,
  arg: &Expr,
  traversal_state: &mut StateManager,
  functions: Rc<FunctionMap>,
) -> ObjectMethodReceiver {
  if let Some(object) = normalize_js_object_method_args(cached_arg) {
    return ObjectMethodReceiver::Object(object);
  }

  match arg.as_array() {
    Some(array) => normalize_js_object_method_array_arg(array, traversal_state, functions),
    None => ObjectMethodReceiver::NoOwnKeys,
  }
}

fn normalize_js_object_method_array_arg(
  arr: &ArrayLit,
  traversal_state: &mut StateManager,
  functions: Rc<FunctionMap>,
) -> ObjectMethodReceiver {
  let mut props = Vec::with_capacity(arr.elems.len());

  for (index, elem) in arr.elems.iter().enumerate() {
    // A hole, an element that refused to fold, and one that folded to nothing
    // are all absent rather than unreadable: an absent element has no key of
    // its own, exactly as `Object.keys([, 1])` omits index zero. Only the last
    // arm below is a value the evaluator holds and cannot write down.
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
      EvaluateResultValue::Vec(items) => match evaluate_result_vec_to_array_expr(&items) {
        Some(expr) => expr,
        None => return ObjectMethodReceiver::Unreadable,
      },
      EvaluateResultValue::Null => continue,
      _ => return ObjectMethodReceiver::Unreadable,
    };

    props.push(create_ident_key_value_prop(&index.to_string(), expr));
  }

  ObjectMethodReceiver::Object(create_object_lit(props))
}

/// Converts a nested vector of `EvaluateResultValue`s to an array expression.
///
/// `None` means some element has no expression form, which is a receiver the
/// caller cannot read rather than a broken invariant — see
/// [`normalize_js_object_method_args`].
fn normalize_js_object_method_nested_vector_arg(vec: &[EvaluateResultValue]) -> Option<Expr> {
  let mut elems = Vec::with_capacity(vec.len());

  for entry in vec {
    if matches!(entry, EvaluateResultValue::Null) {
      continue;
    }

    let expr = match entry.as_vec() {
      Some(nested_vec) => {
        let mut nested_elems = Vec::with_capacity(nested_vec.len());

        for item in nested_vec {
          if matches!(item, EvaluateResultValue::Null) {
            continue;
          }

          nested_elems.push(Some(create_expr_or_spread(item.as_expr()?.clone())));
        }

        create_array_expression(nested_elems)
      },
      None => entry.as_expr()?.clone(),
    };

    elems.push(Some(create_expr_or_spread(expr)));
  }

  Some(create_array_expression(elems))
}

/// Evaluates a call's arguments, refusing a spread among them.
///
/// The reference implementation maps `evaluateCached` over the argument
/// *paths*, so a spread argument arrives as a `SpreadElement` node and falls to
/// its terminal `UNSUPPORTED_EXPRESSION(path.node.type)` arm — one answer for
/// every callee, and given before the operand is looked at. This reads
/// `arg.expr`, which unwraps the spread, so the refusal is made here for the
/// two to agree.
///
/// Refused in the shared helper rather than at each callee, because upstream's
/// answer does not vary by callee and ours used to: `Math.max(...ns)` and
/// `Object.keys(...o)` said the spread was unsupported in this context,
/// `'a'.concat(...xs)` said all arguments must be a string, `xs.join(...s)`
/// named the call, and `stylex.firstThatWorks(...xs)` said the argument must be
/// static — five sentences for one mistake, none of them upstream's.
///
/// `None` is the refusal, and it is an `Option` rather than a short list so that
/// no caller can miss it. Handing back the arguments read so far would be
/// indistinguishable from a call written with fewer, and a callee applied to
/// that list runs at the wrong arity — folding a value, and reaching
/// `StateManager` to queue an import or inject a rule on the way. Upstream stops
/// at the same point, on the same reasoning: `if (!state.confident) return;`.
pub(super) fn evaluate_func_call_args(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<Vec<EvaluateResultValue>> {
  let mut args = Vec::with_capacity(call.args.len());

  for arg in &call.args {
    if arg.spread.is_some() {
      deopt(&Expr::Call(call.clone()), state, SPREAD_ELEMENT);

      return None;
    }

    if let Some(value) = evaluate_cached(&arg.expr, state, traversal_state, fns) {
      args.push(value);
    }
  }

  Some(args)
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
    | EvaluateResultValue::ThemeRef(_)
    // The namespace object, and so `ToObject`'s identity rather than a wrapper.
    // Classified on `evaluate_result_to_string_of`'s arm for the same variant,
    // which is where the reason is written down.
    | EvaluateResultValue::FunctionConfigMap(_) => Some(coercions::ObjectCoercion::Identity),

    EvaluateResultValue::Callback(_) | EvaluateResultValue::FunctionConfig(_) => {
      Some(coercions::ObjectCoercion::Function)
    },
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

    // A folded *map* of function configs is the namespace object, and an object
    // upstream rather than a function: `import * as stylex` binds an object
    // whose properties happen to be functions, so `String(stylex)` is the
    // object default. This is the canonical statement of that classification;
    // `evaluate_result_to_js_object` reads the same fact and points here, and
    // `the_two_bridges_agree_a_function_map_is_an_object` fails if they part
    // again -- which they had, and a template interpolating the fold refused
    // where the reference implementation wrote `[object Object]`.
    EvaluateResultValue::FunctionConfigMap(_) => Some(coercions::OBJECT_TO_STRING.to_string()),

    // A single config and a callback *are* functions, and a function has no
    // compile-time string: `String(fn)` is its source text and this evaluator
    // keeps none, so the form decides -- refuse, or answer `NaN` where a number
    // was wanted.
    EvaluateResultValue::Callback(_) | EvaluateResultValue::FunctionConfig(_) => {
      function_form.render()
    },

    // Unreachable for the reason given on `evaluate_result_to_js_object`, and
    // refused on the same terms. The `Vec` arm above is where a `Null` that
    // reaches this bridge is actually decided.
    EvaluateResultValue::Null => None,
  }
}

/// Reads every argument as a number, flattening nested argument vectors.
///
/// `None` means one of them has no numeric reading — `Math.max({}, 1)` — which
/// is an ordinary call this evaluator does not fold, so the caller deopts on
/// it. The reason of the refusal is already recorded on `state`.
pub(super) fn args_to_numbers(
  args: &[EvaluateResultValue],
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<Vec<f64>> {
  let mut numbers = Vec::with_capacity(args.len());

  push_args_to_numbers(args, path, state, traversal_state, fns, &mut numbers)?;

  Some(numbers)
}

fn push_args_to_numbers(
  args: &[EvaluateResultValue],
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  numbers: &mut Vec<f64>,
) -> Option<()> {
  for arg in args {
    match arg {
      // Deopted on the operand rather than on `path`, because the operand is
      // the thing an author has to change and the code frame points at it. The
      // catch-all below has no expression of its own to name and falls back to
      // the call.
      EvaluateResultValue::Expr(expr) => match expr_to_num(expr, state, traversal_state, fns) {
        Ok(number) => numbers.push(number),
        Err(error) => {
          deopt(expr, state, error.to_string().as_str());

          return None;
        },
      },
      EvaluateResultValue::Vec(vec) => {
        push_args_to_numbers(vec, path, state, traversal_state, fns, numbers)?;
      },
      // A confidently evaluated argument with no value is `undefined`, whose
      // `ToNumber` is `NaN` — and `Math.max(undefined, 1)` is `NaN`, not `1`.
      // Skipping it is what this has always done and is left alone here; the
      // divergence belongs with the prototype-surface work, not with the
      // panic/deopt split.
      EvaluateResultValue::Null => {},
      // Every remaining variant stands for an object or a function upstream,
      // neither of which has a numeric reading.
      _ => {
        deopt(path, state, ILLEGAL_PROP_VALUE);

        return None;
      },
    }
  }

  Some(())
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
