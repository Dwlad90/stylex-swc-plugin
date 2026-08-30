use super::super::*;
use super::global_conversion::Conversion;
use crate::shared::structures::types::EvaluationCallback;
use stylex_ast::ast::convertors::get_key_values_from_object;
use stylex_macros::deopt_unsupported;
use swc_core::ecma::ast::{CallExpr, MemberExpr, MemberProp};

pub(in super::super) fn evaluate(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  // A method call whose every leaf resolves to a value the bridge can carry is
  // evaluated rather than matched against the names below, so the whole
  // prototype surface folds, a chain folds at every link, and naming a value
  // does not change whether the call on it folds. A call it never recognised
  // falls through to the globals here; one it recognised and declined is raised
  // as an ordinary deopt, so the refusal names its rule instead of reaching the
  // catch-all's `Unsupported expression: CallExpression`.
  //
  // Asked before the expression is cloned for a code frame, so a fold that
  // succeeds — the common case once a file folds anything — pays for no clone
  // it does not use.
  match super::super::engine_fold::try_fold(call, state, traversal_state, fns) {
    Some(Ok(value)) => return Some(value),
    Some(Err(reason)) => return deopt(&Expr::Call(call.clone()), state, &reason),
    None => {},
  }

  let path = Expr::Call(call.clone());
  let path = &path;

  // A conversion the engine handed back, because what it was applied to is one
  // of this compiler's own values rather than a JavaScript one.
  if let Callee::Expr(callee) = &call.callee
    && let Some(global) = engine_fold::unshadowed_applied_global(callee, traversal_state)
    && let Some(conversion) = Conversion::named(global)
  {
    return conversion.evaluate(call, path, state, traversal_state, fns);
  }

  let mut func: Option<Box<FunctionConfig>> = None;

  if let Callee::Expr(callee_expr) = &call.callee {
    // A bare name reaching here is one the module bound — an unbound global is a
    // native function and was folded above, whether it could be applied or not.
    // So this is the author's own function and is called as one.
    if let Expr::Ident(ident) = callee_expr.as_ref() {
      let entry = state
        .functions
        .identifiers
        .get(&ident.to_id().0)
        .map(|entry| applied_entry(entry.as_ref()));

      if let Some(entry) = entry {
        match entry {
          MapEntry::Function(config) => func = Some(config),
          MapEntry::NotAFunction => deopt_unsupported!(deopt, path, state, NON_CONSTANT),
        }
      } else {
        let named = evaluate_cached(callee_expr, state, traversal_state, fns);

        if state.confident {
          match named {
            Some(EvaluateResultValue::FunctionConfig(fc)) => func = Some(Box::new(fc)),
            // The name resolved to one of the author's own arrows, so the call
            // on it is *applied* here rather than handed back as the function
            // it names. Handing it back made the answer depend on who asked:
            // the style value position ran the callback itself, so
            // `inner('a')` folded there and the same call nested inside another
            // one arrived at the arrow as a function no parameter could bind --
            // which left the body unresolved and reported an internal
            // expectation about a binary operand. Applied where the call is,
            // one rule answers every position.
            Some(EvaluateResultValue::Callback(cb)) => {
              return apply_own_arrow(&cb, call, path, state, traversal_state, fns);
            },
            _ => {
              return deopt(path, state, NON_CONSTANT);
            },
          }
        } else {
          return deopt(path, state, NON_CONSTANT);
        }
      }
    }

    if let Expr::Member(member) = callee_expr.as_ref() {
      match member_callee(member, call, path, state, traversal_state, fns)? {
        MemberCallee::Function(config) => func = Some(config),
        MemberCallee::Value(value) => return Some(value),
        MemberCallee::Unnamed => {},
      }
    }
  }

  if let Some(func) = func {
    if func.takes_path {
      let args = call.args.iter().map(|arg| &*arg.expr).collect::<Vec<_>>();

      match func.fn_ptr {
        FunctionType::ArrayArgs(func) => {
          let func_result = (func)(
            args.iter().map(|arg| (*arg).clone()).collect(),
            traversal_state,
            fns,
          );
          return Some(EvaluateResultValue::Expr(func_result));
        },
        FunctionType::StylexExprFn(func) => {
          let Some(first_arg) = args.first() else {
            deopt_unsupported!(
              deopt,
              path,
              state,
              "StyleX expression function requires at least one argument."
            );
          };

          let func_result = (func)((**first_arg).clone(), traversal_state);

          return Some(EvaluateResultValue::Expr(func_result));
        },
        // Every remaining function kind takes evaluated arguments rather
        // than raw paths, so reaching one here means the call cannot be
        // folded in this position rather than that the evaluator broke.
        FunctionType::StylexWhenFn(_)
        | FunctionType::StylexTypeFn(_)
        | FunctionType::StylexFnsFactory(_)
        | FunctionType::Callback(_)
        | FunctionType::Mapper(_)
        | FunctionType::ThemeRefMapper(_)
        | FunctionType::DefaultMarker(_)
        | FunctionType::EnvFunction(_) => deopt_unsupported!(deopt, path, state, NON_CONSTANT),
      }
    } else {
      if !state.confident {
        return None;
      }

      match func.fn_ptr {
        FunctionType::ArrayArgs(func) => {
          let args = evaluate_func_call_args(call, state, traversal_state, fns)?;
          let mut fn_args = Vec::with_capacity(args.len());

          for arg in args {
            match arg.as_expr().cloned() {
              Some(expr) => fn_args.push(expr),
              None => deopt_unsupported!(deopt, path, state, ARGUMENT_NOT_EXPRESSION),
            }
          }

          let func_result = (func)(fn_args, traversal_state, fns);

          return Some(EvaluateResultValue::Expr(func_result));
        },
        FunctionType::StylexExprFn(func) => {
          let args = evaluate_func_call_args(call, state, traversal_state, fns)?;
          let Some(first_arg) = args.first().and_then(|arg| arg.as_expr().cloned()) else {
            deopt_unsupported!(
              deopt,
              path,
              state,
              "StyleX expression function requires an expression argument."
            );
          };

          let func_result = (func)(first_arg, traversal_state);

          return Some(EvaluateResultValue::Expr(func_result));
        },
        FunctionType::StylexWhenFn(func) => {
          let mut args = evaluate_func_call_args(call, state, traversal_state, fns)?.into_iter();
          let Some(pseudo) = args.next() else {
            deopt_unsupported!(
              deopt,
              path,
              state,
              "stylex.when functions require a selector argument."
            );
          };
          let func_result = (func)(pseudo, args.next(), traversal_state);
          return Some(EvaluateResultValue::Expr(func_result));
        },
        FunctionType::StylexTypeFn(func) => {
          let args = evaluate_func_call_args(call, state, traversal_state, fns)?;
          let mut fn_args = IndexMap::default();
          let Some(expr) = args.first().and_then(|expr| expr.as_expr()) else {
            deopt_unsupported!(deopt, path, state, ARGUMENT_NOT_EXPRESSION);
          };

          match expr {
            Expr::Object(obj) => {
              for prop in &obj.props {
                let Some(prop) = prop.as_prop() else {
                  deopt_unsupported!(deopt, path, state, SPREAD_NOT_SUPPORTED);
                };

                let Some(key_value) = prop.as_key_value() else {
                  deopt_unsupported!(deopt, path, state, KEY_VALUE_EXPECTED);
                };

                let Some(key) = key_value.key.as_ident().map(|ident| ident.sym.to_string()) else {
                  deopt_unsupported!(deopt, path, state, OBJECT_KEY_MUST_BE_IDENT);
                };

                let Some(value) = key_value.value.as_lit() else {
                  deopt_unsupported!(deopt, path, state, VALUE_MUST_BE_LITERAL);
                };

                fn_args.insert(
                  key,
                  ValueWithDefault::String(convert_lit_to_string(value).unwrap_or_default()),
                );
              }
            },
            Expr::Lit(lit) => {
              fn_args.insert(
                "default".to_string(),
                ValueWithDefault::String(convert_lit_to_string(lit).unwrap_or_default()),
              );
            },
            _ => {},
          }

          let func_result = (func)(ValueWithDefault::Map(fn_args));
          return Some(EvaluateResultValue::Expr(func_result));
        },
        FunctionType::Callback(arrow_fn) => {
          let evaluation_result = evaluate_cached(&arrow_fn, state, traversal_state, fns);

          let Some(EvaluateResultValue::Callback(cb)) = evaluation_result else {
            deopt_unsupported!(deopt, path, state, NON_CONSTANT);
          };

          return apply_own_arrow(&cb, call, path, state, traversal_state, fns);
        },
        FunctionType::DefaultMarker(default_marker) => {
          return Some(EvaluateResultValue::FunctionConfig(FunctionConfig {
            fn_ptr: FunctionType::DefaultMarker(Arc::clone(&default_marker)),
            takes_path: false,
          }));
        },
        FunctionType::EnvFunction(env_fn) => {
          let args = evaluate_func_call_args(call, state, traversal_state, fns)?;
          let mut env_args = Vec::with_capacity(args.len());

          for arg in &args {
            match arg.as_expr() {
              Some(expr) => env_args.push(expr.clone()),
              None => deopt_unsupported!(deopt, path, state, ARGUMENT_NOT_EXPRESSION),
            }
          }
          let result = env_fn.call(env_args);
          return Some(EvaluateResultValue::Expr(result));
        },
        _ => deopt_unsupported!(deopt, path, state, NON_CONSTANT),
      }
    }
  }

  deopt(
    path,
    state,
    &unsupported_expression(get_expr_node_kind(path)),
  )
}

/// The value an author's own arrow answers when it is applied to this call's
/// arguments.
///
/// An argument with no expression form binds nothing and its parameter is left
/// unbound, which is what the language does with an argument nobody passed. A
/// body that never reads that parameter therefore folds -- `((fn) => 'red')`
/// applied to an arrow is a declaration on both compilers -- so the argument the
/// bridge cannot carry costs the call nothing on its own. A function is the
/// argument that arrives without a form: the reference compiler folds one as its
/// source text, and this compiler keeps none.
///
/// A body that *does* read an unbound parameter answers nothing, and that is
/// what is refused here. The callback cannot refuse for itself -- it answers a
/// value rather than a result -- so the sentence is owed by whoever applied it,
/// and there are two of them: an argument that bound nothing is the thing an
/// author can change, and a body that answered nothing with every argument bound
/// leaves only the body to name.
///
/// **Which of the two is a reading of the arguments alone**, and that is as fine
/// as it gets from here: the callback is a closure, so the parameters it declares
/// are not something this position can count, and an argument nobody has a
/// parameter for reads exactly like one that does. So a body failing for a reason
/// of its own, in a call that also passes an argument with no form, is named for
/// the argument -- a true sentence about the call, and not the most useful one
/// available. Naming the parameter instead would mean carrying the arrow's own
/// parameter list beside the closure, which no other position needs.
fn apply_own_arrow(
  cb: &EvaluationCallback,
  call: &CallExpr,
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let args = evaluate_func_call_args(call, state, traversal_state, fns)?;
  let every_argument_bound = args.iter().all(binds_a_parameter);

  match cb(args, traversal_state) {
    Some(expr) => Some(EvaluateResultValue::Expr(expr)),
    None => match every_argument_bound {
      true => deopt(path, state, FUNCTION_BODY_WITHOUT_VALUE),
      false => deopt(path, state, ARGUMENT_NOT_EXPRESSION),
    },
  }
}

/// What a `receiver.method()` callee names.
enum MemberCallee {
  /// A function, applied below to whichever arguments its kind takes.
  Function(Box<FunctionConfig>),
  /// The answer itself. One member read resolves to a value rather than to
  /// something callable: an `env` entry the options configured as a value.
  Value(EvaluateResultValue),
  /// Nothing here names a function. The terminal refusal names the call.
  Unnamed,
}

/// The callee a member expression names, or `None` where the lookup refused and
/// recorded the sentence for it.
///
/// Split out of [`evaluate`] because it was the whole of that function's length
/// and none of its subject: what is left there is the three shapes a callee can
/// be, and the application of whatever one of them named.
///
/// Two lookups in order. First the injected function map, by the receiver's own
/// name -- this compiler's own functions, reached as members of an import. Then
/// the receiver's *value*, for a method on something the evaluator resolved. The
/// second is skipped where the first named a function, which is what the early
/// return says.
fn member_callee(
  member: &MemberExpr,
  call: &CallExpr,
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<MemberCallee> {
  let object = &member.obj;
  let property = &member.prop;

  if object.is_ident() {
    // `object.is_ident()` was just asked, so this cannot answer `None`.
    let Some(obj_ident) = object.as_ident() else {
      stylex_unreachable!("{}", MEMBER_OBJ_NOT_IDENT)
    };

    if property.is_ident() {
      if is_mutating_object_method(property) {
        deopt_unsupported!(deopt, path, state, NON_CONSTANT);
      }

      if is_valid_callee(object) && !is_invalid_method(property) {
        return global_static_callee(object, property, call, path, state, traversal_state, fns);
      }

      // Unreachable: `property.is_ident()` was asked one branch up, so the
      // destructuring cannot fail. Refused rather than asserted, because a
      // broken invariant is worth a sentence an author can report and not an
      // aborted build.
      let Some(prop_ident) = property.as_ident() else {
        deopt_unsupported!(deopt, path, state, UNEXPECTED_MEMBER_LOOKUP);
      };

      let obj_name = obj_ident.sym.to_string();
      let prop_id = prop_ident.sym.to_id();

      let entry = state
        .functions
        .member_expressions
        .get(&ImportSources::Regular(obj_name))
        .and_then(|entries| entries.get(&prop_id.0))
        .map(|entry| applied_entry(entry.as_ref()));

      if let Some(entry) = entry {
        return match entry {
          MapEntry::Function(config) => Some(MemberCallee::Function(config)),
          MapEntry::NotAFunction => deopt_unsupported!(deopt, path, state, NON_CONSTANT),
        };
      }
    }

    if let Some(prop_id) = is_id_prop(property) {
      let obj_name = obj_ident.sym.to_string();

      if let Some(member_expr) = state
        .functions
        .member_expressions
        .get(&ImportSources::Regular(obj_name))
        && member_expr.contains_key(prop_id)
      {
        deopt_unsupported!(deopt, path, state, NON_CONSTANT);

        // TODO: uncomment this for implementation of member expressions
        // return applied_entry(...) as the ident spelling above does.
      }
    }
  }

  let parsed_obj = evaluate_with_functions(object, traversal_state, Rc::clone(&state.functions));

  if !parsed_obj.confident {
    // The receiver has no compile-time value, and the evaluation that found that
    // out said why. Its sentence is raised here rather than discarded: falling
    // through to the terminal refusal named the node the author wrote —
    // `CallExpression` — where the reason is that the receiver is not defined, or
    // was reassigned, or was mutated. The receiver is evaluated under a state of
    // its own, so the reason has to be carried over deliberately or it is lost
    // with that state.
    match parsed_obj.reason {
      Some(reason) => deopt_unsupported!(deopt, path, state, &reason),
      None => deopt_unsupported!(deopt, path, state, UNDEFINED_CONST),
    }
  }

  if property.is_ident() {
    // Unreachable for the reason the same destructuring above is: the question
    // was asked on the line before.
    let Some(prop_ident) = property.as_ident() else {
      deopt_unsupported!(deopt, path, state, UNEXPECTED_MEMBER_LOOKUP);
    };

    let prop_name = prop_ident.sym.to_string();

    let Some(value) = parsed_obj.value else {
      deopt_unsupported!(
        deopt,
        path,
        state,
        format!(
          "The receiver of '.{}()' has no compile-time value.",
          prop_name
        )
        .as_str()
      );
    };

    // A receiver whose prototype the fold owns whole reaching this dispatch means
    // the fold declined the call — so what is left is a call whose receiver or
    // arguments hold something with no compile-time value, or a shape the fold's
    // guard does not read. The evaluator answers an array in two shapes, its own
    // list and the literal it was written as, and the two arms that answered for
    // them separately are what let `join` be known for one and unknown for the
    // other; one arm cannot disagree with itself.
    //
    // A number and a boolean are among them for the same reason a name may hold
    // one: their prototypes fold through the engine like a string's, so the
    // sentence a declined call reads has to be the string's too. Without them
    // `n.toFixed(undefined)` named the receiver's node kind instead of the rule
    // that declined it, which tells an author only that they wrote a number.
    //
    // The arguments are evaluated first because a spread reads the same sentence
    // whatever the callee, and the shared argument evaluation is what owns that
    // sentence.
    //
    // An object receiver is deliberately not among them, though the fold carries
    // one inward: this is a claim about which prototypes the fold owns whole, and
    // the object arm below is where a folded function map's own methods are
    // looked up.
    if matches!(
      &value,
      EvaluateResultValue::Vec(_)
        | EvaluateResultValue::Expr(
          Expr::Array(_) | Expr::Lit(Lit::Str(_) | Lit::Num(_) | Lit::Bool(_))
        )
    ) {
      evaluate_func_call_args(call, state, traversal_state, fns)?;

      deopt_unsupported!(deopt, path, state, &unfoldable_call(&prop_name))
    }

    return match value {
      EvaluateResultValue::Map(map) => {
        map_method(map.get(&Expr::from(prop_ident.clone())), path, state)
      },
      EvaluateResultValue::Expr(expr) => match expr {
        Expr::Object(object) => {
          let key_values = get_key_values_from_object(&object);

          let key_value = key_values
            .into_iter()
            .find(|key_value| match key_value.key.as_ident() {
              Some(key_ident) => key_ident.sym == prop_name,
              _ => false,
            });

          let Some(key_value) = key_value else {
            deopt_unsupported!(deopt, path, state, PROPERTY_NOT_FOUND);
          };

          Some(MemberCallee::Function(Box::new(FunctionConfig {
            fn_ptr: FunctionType::Callback(key_value.value),
            takes_path: false,
          })))
        },
        // Regex methods like .test(), .exec(), etc. require runtime evaluation
        // We can't statically evaluate them, so we deopt
        Expr::Lit(Lit::Regex(_)) => {
          deopt_unsupported!(
            deopt,
            path,
            state,
            "Regex methods cannot be statically evaluated"
          );
        },
        // A method call on a receiver whose kind carries no methods this
        // evaluator folds — a `null`, a template literal, a name that resolved to
        // something with no prototype here. The primitives whose prototypes do
        // fold are answered above, by the rule that declined them rather than by
        // their node kind.
        _ => deopt_unsupported!(
          deopt,
          path,
          state,
          &unsupported_expression(get_expr_node_kind(&expr))
        ),
      },
      EvaluateResultValue::FunctionConfig(fc) => match fc.fn_ptr {
        FunctionType::StylexFnsFactory(sxfns) => {
          Some(MemberCallee::Function(Box::new(FunctionConfig {
            fn_ptr: FunctionType::StylexTypeFn(sxfns(prop_name)),
            takes_path: false,
          })))
        },
        FunctionType::DefaultMarker(default_marker) => match default_marker.get(&prop_name) {
          Some(expr_fn) => Some(MemberCallee::Function(Box::new(FunctionConfig {
            fn_ptr: FunctionType::StylexWhenFn(*expr_fn),
            takes_path: false,
          }))),
          None => Some(MemberCallee::Unnamed),
        },
        _ => deopt_unsupported!(deopt, path, state, NON_CONSTANT),
      },
      // An `env` entry, which is a function or a value depending on how the
      // option was configured.
      EvaluateResultValue::EnvObject(env_map) => {
        let Some(env_val) = env_map.get(&prop_name) else {
          deopt_unsupported!(
            deopt,
            path,
            state,
            format!(
              "The property '{}' was not found in the stylex.env configuration.",
              prop_name
            )
            .as_str()
          );
        };

        match env_val.as_function() {
          Some(env_fn) => Some(MemberCallee::Function(Box::new(FunctionConfig {
            fn_ptr: FunctionType::EnvFunction(env_fn.clone()),
            takes_path: false,
          }))),
          None => match resolve_env_entry_to_result(env_val, &env_map) {
            Some(result) => Some(MemberCallee::Value(result)),
            None => Some(MemberCallee::Unnamed),
          },
        }
      },
      // A receiver the evaluator carries in a representation with no methods of
      // its own — an entries map, a callback, a theme ref.
      _ => deopt_unsupported!(deopt, path, state, UNEXPECTED_MEMBER_LOOKUP),
    };
  }

  if let Some(prop_id) = is_id_prop(property) {
    let prop_id_owned = prop_id.to_string();

    let Some(value) = parsed_obj.value else {
      deopt_unsupported!(
        deopt,
        path,
        state,
        format!(
          "The receiver of the computed call '[{}]()' has no compile-time value.",
          prop_id_owned
        )
        .as_str()
      );
    };

    let Some(map) = value.as_map() else {
      deopt_unsupported!(deopt, path, state, UNEXPECTED_MEMBER_LOOKUP);
    };

    return map_method(map.get(&create_string_expr(&prop_id_owned)), path, state);
  }

  Some(MemberCallee::Unnamed)
}

/// The one static left for this dispatch to answer: `Object.keys` and its two
/// siblings, on a receiver the fold declined.
///
/// The statics of these globals fold in the engine, on every receiver and in
/// every position, so a static reaching here is one the fold declined — and the
/// only thing it can have declined is the receiver. Two of those exist:
/// `Object.keys(stylex)` asks for the own keys of this compiler's own function
/// fold, which is not a JavaScript value at all, and `Object.keys([, 'p'])` asks
/// them of an array with a hole in it, which the fold will not print.
///
/// So the one question left is the own-keys one, in its three spellings. It is
/// not a surface of names this compiler picked — `Object.getOwnPropertyNames`,
/// `Object.fromEntries` and the whole of `Math` fold in the engine — and nothing
/// but a receiver the language never sees comes this far.
fn global_static_callee(
  object: &Expr,
  property: &MemberProp,
  call: &CallExpr,
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<MemberCallee> {
  let callee_name = get_callee_name(object);
  let method_name = get_method_name(property);

  if callee_name != "Object" {
    deopt_unsupported!(deopt, path, state, &unfoldable_call(method_name));
  }

  let Ok(question) = OwnKeysQuestion::try_from(method_name) else {
    deopt_unsupported!(deopt, path, state, &unfoldable_call(method_name));
  };

  let Some(arg) = call.args.first() else {
    deopt_unsupported!(deopt, path, state, &unfoldable_call(method_name));
  };

  if arg.spread.is_some() {
    deopt_unsupported!(deopt, path, state, SPREAD_ELEMENT);
  }

  // An array literal is read from the syntax rather than from its evaluated
  // form, because a hole has no value to evaluate and the receiver reader is
  // what knows a hole carries no key.
  let cached_arg = match arg.expr.is_array() {
    true => None,
    false => evaluate_cached(&arg.expr, state, traversal_state, fns),
  };

  let receiver = normalize_object_method_receiver(
    cached_arg,
    &arg.expr,
    traversal_state,
    Rc::clone(&state.functions),
  );

  match receiver.own_keys(question) {
    Ok(list) => Some(MemberCallee::Value(EvaluateResultValue::Expr(list))),
    Err(reason) => deopt_unsupported!(deopt, path, state, reason),
  }
}

/// What one of the injected maps holds under a name.
enum MapEntry {
  /// A function, which is the one entry shape a call can apply.
  Function(Box<FunctionConfig>),
  /// One of the three object shapes — a map of entries, an ordered map of them,
  /// or the `env` option's own object. Each is reached through a member read
  /// rather than called, so a call on one is a call on something that is not a
  /// function.
  NotAFunction,
}

/// What a function-map entry is, read as the one question a call asks of it.
///
/// One reading for the two places an entry is looked up — a bare name in the
/// identifier map, and a member read in the member-expression map — so the answer
/// cannot come to differ between the two spellings of the same lookup.
///
/// Answered as a value rather than refused here, because the lookup borrows the
/// map out of the state a refusal needs to write to.
fn applied_entry(entry: &FunctionConfigType) -> MapEntry {
  match entry {
    FunctionConfigType::Regular(config) => MapEntry::Function(Box::new(config.clone())),
    FunctionConfigType::Map(_)
    | FunctionConfigType::IndexMap(_)
    | FunctionConfigType::EnvObject(_) => MapEntry::NotAFunction,
  }
}

/// The answer for a method looked up on a folded function map.
///
/// An entry under that name means the receiver holds one of this compiler's own
/// functions, reached as a method rather than applied — which this dispatch does
/// not do, so it refuses. No entry names no callee, and the terminal refusal
/// names the call.
///
/// One reading for the two spellings a member can have, because the map is the
/// same map and only the key differs.
fn map_method(
  entry: Option<&Vec<KeyValueProp>>,
  path: &Expr,
  state: &mut EvaluationState,
) -> Option<MemberCallee> {
  if entry.is_some() {
    deopt_unsupported!(deopt, path, state, NON_CONSTANT);
  }

  Some(MemberCallee::Unnamed)
}
