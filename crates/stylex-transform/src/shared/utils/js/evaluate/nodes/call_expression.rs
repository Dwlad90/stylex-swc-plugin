use super::super::*;
use crate::deopt_unsupported;
use stylex_ast::ast::convertors::get_key_values_from_object;
use swc_core::ecma::ast::CallExpr;

/// Applies a call to one of the JavaScript globals the compiler folds.
///
/// Surplus arguments are ignored and a missing one is `undefined`, as in
/// JavaScript: `String(1, 2)` is `"1"` and `String()` is `""`.
fn evaluate_callable_global(
  global: CallableGlobalJS,
  call: &CallExpr,
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

  if !state.confident {
    return None;
  }

  // An argument that evaluated to nothing while staying confident was dropped
  // rather than deopted, so the remaining arguments no longer line up with
  // what was written. Refuse rather than fold a shifted argument list.
  if args.len() != call.args.len() {
    return deopt(path, state, ARGUMENT_WITHOUT_VALUE);
  }

  // Each coercion below reads the one argument that matters through this, so
  // the no-argument form and the refusal read once rather than three times.
  // `None` is the caller's cue to return: the deopt is already recorded.
  fn coerce_first<T>(
    args: &[EvaluateResultValue],
    path: &Expr,
    state: &mut EvaluationState,
    callee: CallableGlobalJS,
    no_arguments: T,
    coerce: impl Fn(&EvaluateResultValue) -> Option<T>,
  ) -> Option<T> {
    match args.first() {
      Some(arg) => match coerce(arg) {
        Some(coerced) => Some(coerced),
        None => {
          deopt(path, state, &uncoercible_value(callee.name()));
          None
        },
      },
      None => Some(no_arguments),
    }
  }

  match global {
    CallableGlobalJS::String => {
      // `String()` is the empty string, not `String(undefined)`.
      let coerced = coerce_first(
        &args,
        path,
        state,
        global,
        String::new(),
        evaluate_result_to_js_string,
      )?;

      Some(EvaluateResultValue::Expr(create_string_expr(&coerced)))
    },
    CallableGlobalJS::Number => {
      // `Number()` is zero, not `Number(undefined)`. `NaN` is not a refusal: it
      // arrives as a value and flows into the declaration, the same as
      // upstream, where `Number('10px')` writes `NaN` into the rule.
      let coerced = coerce_first(
        &args,
        path,
        state,
        global,
        0.0,
        evaluate_result_to_js_number,
      )?;

      Some(EvaluateResultValue::Expr(create_number_expr(coerced)))
    },
    CallableGlobalJS::Array => {
      // One numeric argument is a length rather than an element: `Array(3)`
      // is three holes where `Array('3')` is the one-element list. Every
      // other argument list folds to itself, including no arguments at all.
      if let [EvaluateResultValue::Expr(count)] = args.as_slice()
        && let Some(count) = coercions::js_number_value(count)
      {
        let Some(length) = coercions::to_array_length(count) else {
          return deopt(path, state, INVALID_ARRAY_LENGTH);
        };

        if length > coercions::MAX_FOLDED_ARRAY_LENGTH {
          return deopt(
            path,
            state,
            &array_length_too_large(coercions::MAX_FOLDED_ARRAY_LENGTH),
          );
        }

        // A hole holds the same absent value a confidently evaluated element
        // with no value already does. The fold succeeds and the
        // holes reach the existing style-array check, which is what refuses
        // them — a counted array is rejected as a value, not as a call.
        return Some(EvaluateResultValue::Vec(vec![
          EvaluateResultValue::Null;
          length
        ]));
      }

      Some(EvaluateResultValue::Vec(args))
    },
    CallableGlobalJS::Object => {
      // `Object()` is `Object(undefined)`, unlike `String()` and `Number()`,
      // whose no-argument forms are not their `undefined` ones.
      let coercion = coerce_first(
        &args,
        path,
        state,
        global,
        coercions::ObjectCoercion::EmptyObject,
        evaluate_result_to_js_object,
      )?;

      match coercion {
        coercions::ObjectCoercion::EmptyObject => Some(EvaluateResultValue::Expr(Expr::Object(
          create_object_lit(vec![]),
        ))),
        coercions::ObjectCoercion::Identity => args.into_iter().next(),
        // A boxed wrapper and a function are told apart by the coercion and
        // refused alike here: neither is an array, a string or a number, so
        // both end at this rejection, and neither is represented. See
        // `docs/adr/0001-a-refused-fold-borrows-a-later-diagnostic.md`.
        coercions::ObjectCoercion::Function | coercions::ObjectCoercion::Wrapper => {
          deopt(path, state, ILLEGAL_PROP_VALUE)
        },
      }
    },
  }
}

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

  let mut func: Option<Box<FunctionConfig>> = None;

  if let Callee::Expr(callee_expr) = &call.callee {
    if get_binding(callee_expr, traversal_state).is_none() && is_valid_callee(callee_expr) {
      // A valid callee with no binding in scope is the global itself, not a
      // function the module declared, so calling it folds.
      let callee_name = get_callee_name(callee_expr);

      if let Ok(global) = CallableGlobalJS::try_from(callee_name) {
        // A spread argument is refused by `evaluate_func_call_args`, which every
        // callee's arguments now go through, so there is no case for one here.
        func = Some(Box::new(FunctionConfig {
          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Global(global))),
          takes_path: false,
        }));
      } else {
        // A valid callee that is not a callable global contributes methods and
        // nothing else — `Math` today, and any later addition of that shape.
        // There is nothing to fold, so it names the callee rather than deopting
        // into the catch-all's `Unsupported expression`.
        return deopt(path, state, &not_a_function(callee_name));
      }
    } else if let Expr::Ident(ident) = callee_expr.as_ref() {
      let ident_id = ident.to_id();

      if state.functions.identifiers.contains_key(&ident_id.0) {
        // The key was just found by `contains_key`, so the lookup cannot
        // answer `None`.
        let Some(identifier) = state.functions.identifiers.get(&ident_id.0) else {
          stylex_unreachable!("A function identifier vanished between two lookups.")
        };

        match identifier.as_ref() {
          FunctionConfigType::Map(_) => deopt_unsupported!(path, state, NON_CONSTANT),
          FunctionConfigType::Regular(fc) => func = Some(Box::new(fc.clone())),
          FunctionConfigType::IndexMap(_) => deopt_unsupported!(path, state, NON_CONSTANT),
          FunctionConfigType::EnvObject(_) => {
            // EnvObject is not directly callable; access is done via member expressions
            return deopt(path, state, NON_CONSTANT);
          },
        }
      } else {
        let _maybe_function = evaluate_cached(callee_expr, state, traversal_state, fns);

        if state.confident {
          match _maybe_function {
            Some(EvaluateResultValue::FunctionConfig(fc)) => func = Some(Box::new(fc)),
            Some(EvaluateResultValue::Callback(cb)) => {
              return Some(EvaluateResultValue::Callback(cb));
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
      let object = &member.obj;
      let property = &member.prop;

      if object.is_ident() {
        // `object.is_ident()` was just asked, so this cannot answer `None`.
        let Some(obj_ident) = object.as_ident() else {
          stylex_unreachable!("{}", MEMBER_OBJ_NOT_IDENT)
        };

        if property.is_ident() {
          if is_mutating_object_method(property) {
            return deopt(path, state, NON_CONSTANT);
          }

          if is_valid_callee(object) && !is_invalid_method(property) {
            let callee_name = get_callee_name(object);
            let method_name = get_method_name(property);

            // The statics of these globals fold in the engine, on every receiver
            // and in every position, so a static reaching here is one the fold
            // declined — and the only thing it can have declined is the receiver.
            // Two of those exist: `Object.keys(stylex)` asks for the own keys of
            // this compiler's own function fold, which is not a JavaScript value
            // at all, and `Object.keys([, 'p'])` asks them of an array with a
            // hole in it, which the fold will not print.
            //
            // So the one question left here is the own-keys one, in its three
            // spellings. It is not a surface of names this compiler picked —
            // `Object.getOwnPropertyNames`, `Object.fromEntries` and the whole of
            // `Math` fold above — and nothing but a receiver the language never
            // sees comes this far.
            if callee_name == "Object"
              && let Ok(question) = OwnKeysQuestion::try_from(method_name)
            {
              let Some(arg) = call.args.first() else {
                deopt_unsupported!(path, state, &unfoldable_call(method_name));
              };

              if arg.spread.is_some() {
                deopt_unsupported!(path, state, SPREAD_ELEMENT);
              }

              // An array literal is read from the syntax rather than from its
              // evaluated form, because a hole has no value to evaluate and the
              // receiver reader is what knows a hole carries no key.
              let cached_arg = if arg.expr.is_array() {
                None
              } else {
                evaluate_cached(&arg.expr, state, traversal_state, fns)
              };

              let receiver = normalize_object_method_receiver(
                cached_arg,
                &arg.expr,
                traversal_state,
                Rc::clone(&state.functions),
              );

              return match receiver.own_keys(question) {
                Ok(list) => Some(EvaluateResultValue::Expr(list)),
                Err(reason) => deopt(path, state, reason),
              };
            }

            deopt_unsupported!(path, state, &unfoldable_call(method_name));
          } else {
            let Some(prop_ident) = property.as_ident() else {
              deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
            };

            let obj_name = obj_ident.sym.to_string();
            let prop_id = prop_ident.sym.to_id();

            if let Some(member_expr) = state
              .functions
              .member_expressions
              .get(&ImportSources::Regular(obj_name))
              && let Some(member_expr_fn) = member_expr.get(&prop_id.0)
            {
              match member_expr_fn.as_ref() {
                FunctionConfigType::Regular(fc) => {
                  func = Some(Box::new(fc.clone()));
                },
                FunctionConfigType::Map(_) => deopt_unsupported!(path, state, NON_CONSTANT),
                FunctionConfigType::IndexMap(_) => deopt_unsupported!(path, state, NON_CONSTANT),
                FunctionConfigType::EnvObject(_) => {
                  // This shouldn't happen - env object isn't directly callable.
                  // But if it does, try to evaluate it as a member expression call.
                  return deopt(path, state, NON_CONSTANT);
                },
              }
            }
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
            deopt_unsupported!(path, state, NON_CONSTANT);

            // context = Some(member_expr.clone());

            // TODO: uncomment this for implementation of member expressions
            // match member_expr.get(&prop_id).unwrap() {
            //   FunctionConfigType::Regular(fc) => {
            //     func = Some(Box::new(fc.clone()));
            //   }
            //   FunctionConfigType::Map(_) =>
            // unimplemented!("FunctionConfigType::Map"), }
          }
        }
      }

      if object.is_lit() {
        // `object.is_lit()` was just asked, so this cannot answer `None`.
        let Some(obj_lit) = object.as_lit() else {
          stylex_unreachable!("A literal receiver stopped being a literal between two lookups.")
        };

        if property.is_ident()
          && let Lit::Bool(_) = obj_lit
        {
          deopt_unsupported!(path, state, &unsupported_expression("BooleanLiteral"));
        }
      }

      if func.is_none() {
        let parsed_obj =
          evaluate_with_functions(object, traversal_state, Rc::clone(&state.functions));

        if parsed_obj.confident {
          if property.is_ident() {
            let Some(prop_ident) = property.as_ident() else {
              deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
            };

            let prop_name = prop_ident.sym.to_string();

            let Some(value) = parsed_obj.value else {
              deopt_unsupported!(
                path,
                state,
                format!(
                  "The receiver of '.{}()' has no compile-time value.",
                  prop_name
                )
                .as_str()
              );
            };

            // A string or an array reaching this dispatch means the fold
            // declined the call, and the whole of both prototypes folds there —
            // so what is left is a call whose receiver or arguments hold
            // something with no compile-time value, or a shape the fold's guard
            // does not read. The evaluator answers an array in two shapes, its
            // own list and the literal it was written as, and the two arms that
            // answered for them separately are what let `join` be known for one
            // and unknown for the other; one arm cannot disagree with itself.
            //
            // The arguments are evaluated first because a spread reads the same
            // sentence whatever the callee, and the shared argument evaluation
            // is what owns that sentence.
            //
            // An object receiver is deliberately not among them, though the
            // fold carries one inward: this is a claim about which prototypes
            // the fold owns whole, and the object arm below is where a folded
            // function map's own methods are looked up.
            if matches!(
              &value,
              EvaluateResultValue::Vec(_)
                | EvaluateResultValue::Expr(Expr::Array(_) | Expr::Lit(Lit::Str(_)))
            ) {
              evaluate_func_call_args(call, state, traversal_state, fns)?;

              deopt_unsupported!(path, state, &unfoldable_call(&prop_name))
            }

            match value {
              EvaluateResultValue::Map(map) => {
                let result_fn = map.get(&Expr::from(prop_ident.clone()));

                func = match result_fn {
                  Some(_) => deopt_unsupported!(path, state, NON_CONSTANT),
                  None => None,
                };
              },
              EvaluateResultValue::Expr(expr) => match expr {
                Expr::Object(object) => {
                  let key_values = get_key_values_from_object(&object);

                  let key_value =
                    key_values
                      .into_iter()
                      .find(|key_value| match key_value.key.as_ident() {
                        Some(key_ident) => key_ident.sym == prop_name,
                        _ => false,
                      });

                  let Some(key_value) = key_value else {
                    deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
                  };

                  func = Some(Box::new(FunctionConfig {
                    fn_ptr: FunctionType::Callback(Box::new(CallbackType::Custom(
                      *key_value.value,
                    ))),
                    takes_path: false,
                  }));
                },
                Expr::Lit(Lit::Regex(_)) => {
                  // Regex methods like .test(), .exec(), etc. require runtime evaluation
                  // We can't statically evaluate them, so we deopt
                  return deopt(path, state, "Regex methods cannot be statically evaluated");
                },
                // A method call on a receiver whose kind carries no methods
                // this evaluator folds: a number, a boolean, a nested call.
                _ => deopt_unsupported!(
                  path,
                  state,
                  &unsupported_expression(get_expr_node_kind(&expr))
                ),
              },
              EvaluateResultValue::FunctionConfig(fc) => match fc.fn_ptr {
                FunctionType::StylexFnsFactory(sxfns) => {
                  let fc = sxfns(prop_name);

                  func = Some(Box::new(FunctionConfig {
                    fn_ptr: FunctionType::StylexTypeFn(fc),
                    takes_path: false,
                  }));
                },
                FunctionType::DefaultMarker(default_marker) => {
                  if let Some(expr_fn) = default_marker.get(&prop_name) {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::StylexWhenFn(*expr_fn),
                      takes_path: false,
                    }));
                  };
                },
                _ => deopt_unsupported!(path, state, NON_CONSTANT),
              },
              EvaluateResultValue::EnvObject(env_map) => {
                // Handle env function calls like `env.colorMix(...)` or
                // `stylex.env.colorMix(...)`
                if let Some(env_val) = env_map.get(&prop_name) {
                  if let Some(env_fn) = env_val.as_function() {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::EnvFunction(env_fn.clone()),
                      takes_path: false,
                    }));
                  } else if let Some(result) = resolve_env_entry_to_result(env_val, &env_map) {
                    // It's a value, not a function - return it directly
                    return Some(result);
                  }
                } else {
                  deopt_unsupported!(
                    path,
                    state,
                    format!(
                      "The property '{}' was not found in the stylex.env configuration.",
                      prop_name
                    )
                    .as_str()
                  );
                }
              },
              // A receiver the evaluator carries in a representation with no
              // methods of its own — an entries map, a callback, a theme ref.
              _ => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
            }
          } else if let Some(prop_id) = is_id_prop(property) {
            let prop_id_owned = prop_id.to_string();

            let Some(value) = parsed_obj.value else {
              deopt_unsupported!(
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
              deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
            };

            let result_fn = map.get(&create_string_expr(&prop_id_owned));

            func = match result_fn {
              Some(_) => deopt_unsupported!(path, state, NON_CONSTANT),
              None => None,
            };
          }
        } else {
          // The receiver has no compile-time value, and the evaluation that
          // found that out said why. Its sentence is raised here rather than
          // discarded: falling through to the terminal refusal below named the
          // node the author wrote — `CallExpression` — where the reason is that
          // the receiver is not defined, or was reassigned, or was mutated. The
          // receiver is evaluated under a state of its own, so the reason has to
          // be carried over deliberately or it is lost with that state.
          return match parsed_obj.reason {
            Some(reason) => deopt(path, state, &reason),
            None => deopt(path, state, UNDEFINED_CONST),
          };
        }
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
        | FunctionType::EnvFunction(_) => deopt_unsupported!(path, state, NON_CONSTANT),
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
              None => deopt_unsupported!(path, state, ARGUMENT_NOT_EXPRESSION),
            }
          }

          let func_result = (func)(fn_args, traversal_state, fns);

          return Some(EvaluateResultValue::Expr(func_result));
        },
        FunctionType::StylexExprFn(func) => {
          let args = evaluate_func_call_args(call, state, traversal_state, fns)?;
          let Some(first_arg) = args.first().and_then(|arg| arg.as_expr().cloned()) else {
            deopt_unsupported!(
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
            deopt_unsupported!(path, state, ARGUMENT_NOT_EXPRESSION);
          };

          match expr {
            Expr::Object(obj) => {
              for prop in &obj.props {
                let Some(prop) = prop.as_prop() else {
                  deopt_unsupported!(path, state, SPREAD_NOT_SUPPORTED);
                };

                let Some(key_value) = prop.as_key_value() else {
                  deopt_unsupported!(path, state, KEY_VALUE_EXPECTED);
                };

                let Some(key) = key_value.key.as_ident().map(|ident| ident.sym.to_string()) else {
                  deopt_unsupported!(path, state, OBJECT_KEY_MUST_BE_IDENT);
                };

                let Some(value) = key_value.value.as_lit() else {
                  deopt_unsupported!(path, state, VALUE_MUST_BE_LITERAL);
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
        FunctionType::Callback(func) => match func.as_ref() {
          // A callable global takes its arguments and nothing else, so it is
          // applied without reading a receiver at all.
          CallbackType::Global(global) => {
            return evaluate_callable_global(*global, call, path, state, traversal_state, fns);
          },
          CallbackType::Custom(arrow_fn) => {
            let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

            let evaluation_result = evaluate_cached(arrow_fn, state, traversal_state, fns);

            let Some(EvaluateResultValue::Callback(cb)) = evaluation_result.as_ref() else {
              deopt_unsupported!(path, state, NON_CONSTANT);
            };

            let expr_result = cb(args, traversal_state);

            return Some(EvaluateResultValue::Expr(expr_result));
          },
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
              None => deopt_unsupported!(path, state, ARGUMENT_NOT_EXPRESSION),
            }
          }
          let result = env_fn.call(env_args);
          return Some(EvaluateResultValue::Expr(result));
        },
        _ => deopt_unsupported!(path, state, NON_CONSTANT),
      }
    }
  }

  deopt(
    path,
    state,
    &unsupported_expression(get_expr_node_kind(path)),
  )
}
