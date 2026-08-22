use super::super::*;
use crate::deopt_unsupported;
use stylex_ast::ast::convertors::get_key_values_from_object;
use stylex_utils::math::js_math_round;
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
  let path = Expr::Call(call.clone());
  let path = &path;
  let mut context: Option<Vec<EvaluateResultValue>> = None;
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

            match callee_name {
              "Math" => {
                // `Math.max()` is `-Infinity` and `Math.round()` is `NaN`;
                // neither is folded here, and neither is a broken invariant.
                let Some(first_arg) = call.args.first() else {
                  deopt_unsupported!(
                    path,
                    state,
                    format!("Math.{}() requires an argument", method_name).as_str()
                  );
                };

                if first_arg.spread.is_some() {
                  deopt_unsupported!(path, state, SPREAD_ELEMENT);
                }

                match method_name {
                  "pow" => {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(MathJS::Pow))),
                      takes_path: false,
                    }));

                    let Some(second_arg) = call.args.get(1) else {
                      deopt_unsupported!(
                        path,
                        state,
                        "Math.pow() requires a second numeric argument."
                      );
                    };

                    if second_arg.spread.is_some() {
                      deopt_unsupported!(path, state, SPREAD_ELEMENT);
                    }

                    let cached_first_arg =
                      evaluate_cached(&first_arg.expr, state, traversal_state, fns);
                    let cached_second_arg =
                      evaluate_cached(&second_arg.expr, state, traversal_state, fns);

                    if let Some(cached_first_arg) = cached_first_arg
                      && let Some(cached_second_arg) = cached_second_arg
                    {
                      context = Some(vec![EvaluateResultValue::Vec(vec![
                        cached_first_arg,
                        cached_second_arg,
                      ])]);
                    }
                  },
                  "round" | "ceil" | "floor" => {
                    let math_method = MathJS::try_from(method_name)
                      .unwrap_or_else(|()| stylex_unreachable!("Invalid method: {}", method_name));

                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(math_method))),
                      takes_path: false,
                    }));

                    let cached_first_arg =
                      evaluate_cached(&first_arg.expr, state, traversal_state, fns);

                    if let Some(cached_first_arg) = cached_first_arg {
                      let Some(expr) = cached_first_arg.as_expr() else {
                        deopt_unsupported!(path, state, ARGUMENT_NOT_EXPRESSION);
                      };

                      context = Some(vec![EvaluateResultValue::Expr(expr.clone())]);
                    }
                  },
                  "min" | "max" => {
                    let math_method = MathJS::try_from(method_name)
                      .unwrap_or_else(|()| stylex_unreachable!("Invalid method: {}", method_name));

                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(math_method))),
                      takes_path: false,
                    }));

                    let cached_first_arg =
                      evaluate_cached(&first_arg.expr, state, traversal_state, fns);

                    if let Some(cached_first_arg) = cached_first_arg {
                      let mut result = Vec::with_capacity(call.args.len());
                      result.push(cached_first_arg);

                      result.extend(
                        call
                          .args
                          .iter()
                          .skip(1)
                          .map(|arg| evaluate_cached(&arg.expr, state, traversal_state, fns))
                          .map(|arg| arg.unwrap_or(EvaluateResultValue::Null)),
                      );

                      context = Some(vec![EvaluateResultValue::Vec(result)]);
                    }
                  },
                  "abs" => {
                    let cached_first_arg =
                      evaluate_cached(&first_arg.expr, state, traversal_state, fns);

                    if let Some(cached_first_arg) = cached_first_arg {
                      func = Some(Box::new(FunctionConfig {
                        fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(MathJS::Abs))),
                        takes_path: false,
                      }));

                      let Some(expr) = cached_first_arg.as_expr() else {
                        deopt_unsupported!(path, state, ARGUMENT_NOT_EXPRESSION);
                      };

                      context = Some(vec![EvaluateResultValue::Expr(expr.clone())]);
                    }
                  },
                  // `Math.sin`, `Math.hypot`, … — real methods this
                  // evaluator does not fold.
                  _ => deopt_unsupported!(
                    path,
                    state,
                    format!("{} - {}:{}", BUILT_IN_FUNCTION, callee_name, method_name).as_str()
                  ),
                }
              },
              "Object" => {
                let args = &call.args;

                let Some(arg) = args.first() else {
                  deopt_unsupported!(
                    path,
                    state,
                    format!("Object.{}() requires an argument", method_name).as_str()
                  );
                };

                if arg.spread.is_some() {
                  deopt_unsupported!(path, state, SPREAD_ELEMENT);
                }

                let object_method = ObjectJS::try_from(method_name);
                let cached_arg = if matches!(
                  object_method,
                  Ok(ObjectJS::Keys | ObjectJS::Values | ObjectJS::Entries)
                ) && arg.expr.is_array()
                {
                  None
                } else {
                  evaluate_cached(&arg.expr, state, traversal_state, fns)
                };

                match object_method {
                  Ok(ObjectJS::FromEntries) => {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                        ObjectJS::FromEntries,
                      ))),
                      takes_path: false,
                    }));

                    let mut from_entries_result = IndexMap::new();

                    // Every refusal below is the same complaint — the
                    // argument is not a list of `[key, value]` pairs the
                    // evaluator can read — and every one of them is an
                    // argument an author can write.
                    const NOT_ENTRIES: &str =
                      "Object.fromEntries() requires an array of [key, value] entries.";

                    match cached_arg {
                      Some(EvaluateResultValue::Expr(expr)) => {
                        let Some(array) = expr.as_array().cloned() else {
                          deopt_unsupported!(path, state, NOT_ENTRIES);
                        };

                        for entry in array.elems.into_iter().flatten() {
                          if entry.spread.is_some() {
                            deopt_unsupported!(path, state, SPREAD_ELEMENT);
                          }

                          let Some(array) = entry.expr.as_array() else {
                            deopt_unsupported!(path, state, NOT_ENTRIES);
                          };

                          let mut elems = array.elems.iter().flatten();

                          let Some(key) = elems.next().and_then(|e| e.expr.as_lit()) else {
                            deopt_unsupported!(path, state, OBJECT_KEY_MUST_BE_IDENT);
                          };

                          let Some(value) = elems.next().map(|e| e.expr.clone()) else {
                            deopt_unsupported!(path, state, VALUE_MUST_BE_LITERAL);
                          };

                          from_entries_result.insert(key.clone(), value.clone());
                        }
                      },
                      Some(EvaluateResultValue::Vec(vec)) => {
                        for entry in vec {
                          let Some(entry) = entry.as_vec().cloned() else {
                            deopt_unsupported!(path, state, NOT_ENTRIES);
                          };

                          let key = entry
                            .first()
                            .and_then(|item| item.as_expr().cloned())
                            .and_then(|expr| expr.as_lit().cloned());

                          let Some(key) = key else {
                            deopt_unsupported!(path, state, OBJECT_KEY_MUST_BE_IDENT);
                          };

                          let Some(value) = entry.get(1).and_then(|item| item.as_expr().cloned())
                          else {
                            deopt_unsupported!(path, state, VALUE_MUST_BE_LITERAL);
                          };

                          from_entries_result.insert(key.clone(), Box::new(value.clone()));
                        }
                      },
                      _ => deopt_unsupported!(path, state, NOT_ENTRIES),
                    };

                    context = Some(vec![EvaluateResultValue::Entries(from_entries_result)]);
                  },
                  Ok(ObjectJS::Keys) => {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                        ObjectJS::Keys,
                      ))),
                      takes_path: false,
                    }));

                    let object = match normalize_object_method_receiver(
                      cached_arg,
                      &arg.expr,
                      traversal_state,
                      Rc::clone(&state.functions),
                    )
                    .into_own_keys()
                    {
                      Ok(object) => object,
                      Err(reason) => deopt_unsupported!(path, state, reason),
                    };

                    if let Some(object) = object {
                      let mut keys = Vec::with_capacity(object.props.len());

                      for prop in &object.props {
                        let Some(expr) = prop.as_prop().cloned() else {
                          deopt_unsupported!(path, state, SPREAD_NOT_SUPPORTED);
                        };

                        let Some(key_values) = expr.as_key_value() else {
                          deopt_unsupported!(path, state, OBJECT_METHOD);
                        };

                        let key = convert_key_value_to_str(key_values);

                        keys.push(Some(create_expr_or_spread(create_string_expr(
                          key.as_str(),
                        ))));
                      }

                      context = Some(vec![EvaluateResultValue::Expr(create_array_expression(
                        keys,
                      ))]);
                    } else {
                      context = Some(vec![EvaluateResultValue::Expr(create_array_expression(
                        Vec::new(),
                      ))]);
                    }
                  },
                  Ok(ObjectJS::Values) => {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                        ObjectJS::Values,
                      ))),
                      takes_path: false,
                    }));

                    let object = match normalize_object_method_receiver(
                      cached_arg,
                      &arg.expr,
                      traversal_state,
                      Rc::clone(&state.functions),
                    )
                    .into_own_keys()
                    {
                      Ok(object) => object,
                      Err(reason) => deopt_unsupported!(path, state, reason),
                    };

                    if let Some(object) = object {
                      let mut values = Vec::with_capacity(object.props.len());

                      for prop in &object.props {
                        let Some(prop) = prop.as_prop().cloned() else {
                          deopt_unsupported!(path, state, SPREAD_NOT_SUPPORTED);
                        };

                        let Some(key_values) = prop.as_key_value() else {
                          deopt_unsupported!(path, state, OBJECT_METHOD);
                        };

                        values.push(Some(create_expr_or_spread(*key_values.value.clone())));
                      }

                      context = Some(vec![EvaluateResultValue::Expr(create_array_expression(
                        values,
                      ))]);
                    } else {
                      context = Some(vec![EvaluateResultValue::Expr(create_array_expression(
                        Vec::new(),
                      ))]);
                    }
                  },
                  Ok(ObjectJS::Entries) => {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                        ObjectJS::Entries,
                      ))),
                      takes_path: false,
                    }));

                    let object = match normalize_object_method_receiver(
                      cached_arg,
                      &arg.expr,
                      traversal_state,
                      Rc::clone(&state.functions),
                    )
                    .into_own_keys()
                    {
                      Ok(object) => object,
                      Err(reason) => deopt_unsupported!(path, state, reason),
                    };

                    let mut entries: IndexMap<Lit, Box<Expr>> = IndexMap::new();

                    if let Some(object) = object {
                      for prop in &object.props {
                        let Some(expr) = prop.as_prop().map(|prop| *prop.clone()) else {
                          deopt_unsupported!(path, state, SPREAD_NOT_SUPPORTED);
                        };

                        let Some(key_values) = expr.as_key_value() else {
                          deopt_unsupported!(path, state, OBJECT_METHOD);
                        };

                        let value = key_values.value.clone();

                        let key = convert_key_value_to_str(key_values);

                        entries.insert(create_string_lit(key.as_str()), value);
                      }
                    }

                    context = Some(vec![EvaluateResultValue::Entries(entries)]);
                  },
                  // `Object.assign`, `Object.freeze`, … — methods this
                  // evaluator does not fold.
                  Err(()) => deopt_unsupported!(
                    path,
                    state,
                    format!("{} - {}:{}", BUILT_IN_FUNCTION, callee_name, method_name).as_str()
                  ),
                }
              },
              _ => deopt_unsupported!(
                path,
                state,
                format!("{} - {}", BUILT_IN_FUNCTION, callee_name).as_str()
              ),
            }
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

            if is_mutating_array_method(property) {
              return deopt(path, state, NON_CONSTANT);
            }

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

            match value.clone() {
              EvaluateResultValue::Map(map) => {
                let result_fn = map.get(&Expr::from(prop_ident.clone()));

                func = match result_fn {
                  Some(_) => deopt_unsupported!(path, state, NON_CONSTANT),
                  None => None,
                };
              },
              EvaluateResultValue::Vec(expr) => {
                let callback_type = match ArrayJS::try_from(prop_name.as_str()) {
                  Ok(array_method) => CallbackType::Array(array_method),
                  Err(()) => match ObjectJS::try_from(prop_name.as_str()) {
                    Ok(ObjectJS::Entries) => CallbackType::Object(ObjectJS::Entries),
                    _ => deopt_unsupported!(
                      path,
                      state,
                      format!(
                        "The array method '{}' is not yet supported in static evaluation.",
                        prop_name
                      )
                      .as_str()
                    ),
                  },
                };

                func = Some(Box::new(FunctionConfig {
                  fn_ptr: FunctionType::Callback(Box::new(callback_type)),
                  takes_path: false,
                }));

                context = Some(expr)
              },
              EvaluateResultValue::Expr(expr) => match expr {
                Expr::Array(ArrayLit { elems, .. }) => {
                  let callback_type = match ArrayJS::try_from(prop_name.as_str()) {
                    Ok(array_method @ (ArrayJS::Map | ArrayJS::Filter)) => {
                      CallbackType::Array(array_method)
                    },
                    Ok(ArrayJS::Join) | Err(()) => match ObjectJS::try_from(prop_name.as_str()) {
                      Ok(ObjectJS::Entries) => CallbackType::Object(ObjectJS::Entries),
                      _ => deopt_unsupported!(
                        path,
                        state,
                        format!(
                          "The method '{}' is not yet supported in static evaluation.",
                          prop_name
                        )
                        .as_str()
                      ),
                    },
                  };

                  func = Some(Box::new(FunctionConfig {
                    fn_ptr: FunctionType::Callback(Box::new(callback_type)),
                    takes_path: false,
                  }));

                  let mut receiver = Vec::with_capacity(elems.len());

                  for elem in elems {
                    // A hole is `undefined`, which the array methods below
                    // join as the empty string. They do not carry one, so
                    // folding `[, 1].join('-')` would write `"1"` where
                    // JavaScript gives `"-1"` — a wrong value, where a refusal
                    // costs only a declaration that falls to the runtime.
                    let Some(elem) = elem else {
                      deopt_unsupported!(path, state, ILLEGAL_PROP_ARRAY_VALUE);
                    };

                    receiver.push(EvaluateResultValue::Expr(*elem.expr.clone()));
                  }

                  context = Some(vec![EvaluateResultValue::Vec(receiver)]);
                },
                Expr::Lit(Lit::Str(_)) => {
                  let string_method = match StringJS::try_from(prop_name.as_str()) {
                    Ok(string_method) => string_method,
                    Err(()) => deopt_unsupported!(
                      path,
                      state,
                      format!(
                        "The method '{}' is not yet supported in static evaluation.",
                        prop_name
                      )
                      .as_str()
                    ),
                  };

                  func = Some(Box::new(FunctionConfig {
                    fn_ptr: FunctionType::Callback(Box::new(CallbackType::String(string_method))),
                    takes_path: false,
                  }));

                  context = Some(vec![EvaluateResultValue::Expr(expr.clone())]);
                },
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

                  let args: Vec<EvaluateResultValue> = call
                    .args
                    .iter()
                    .map(|arg| {
                      let arg = evaluate_cached(&arg.expr, state, traversal_state, fns);

                      if !state.confident {
                        return EvaluateResultValue::Null;
                      }

                      arg.unwrap_or(EvaluateResultValue::Null)
                    })
                    .collect();

                  context = Some(args);
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

                  context = Some(vec![value]);
                },
                FunctionType::DefaultMarker(default_marker) => {
                  if let Some(expr_fn) = default_marker.get(&prop_name) {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::StylexWhenFn(*expr_fn),
                      takes_path: false,
                    }));

                    context = Some(vec![value]);
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
        FunctionType::Callback(func) => {
          // A callable global takes its arguments and nothing else — there is
          // no receiver for a `context` to carry.
          if let CallbackType::Global(global) = func.as_ref() {
            return evaluate_callable_global(*global, call, path, state, traversal_state, fns);
          }

          // The receiver never produced a value to apply the method to.
          let Some(context) = context else {
            deopt_unsupported!(path, state, ARGUMENT_WITHOUT_VALUE);
          };

          match func.as_ref() {
            CallbackType::Array(ArrayJS::Map) => {
              let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

              return evaluate_map(&args, &context, traversal_state);
            },
            CallbackType::Array(ArrayJS::Filter) => {
              let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

              return evaluate_filter(&args, &context, traversal_state);
            },
            CallbackType::Array(ArrayJS::Join) => {
              let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

              return evaluate_join(&args, &context, traversal_state, &state.functions);
            },
            CallbackType::Object(ObjectJS::Entries) => {
              let Some(EvaluateResultValue::Entries(entries)) = context.first() else {
                deopt_unsupported!(path, state, "Object.entries() requires an object argument.");
              };

              let mut entry_elems: Vec<Option<ExprOrSpread>> = Vec::with_capacity(entries.len());

              for (key, value) in entries {
                let key_spread = create_expr_or_spread(Expr::from(key.clone()));
                let value_spread = create_expr_or_spread(*value.clone());

                entry_elems.push(Some(create_expr_or_spread(create_array_expression(vec![
                  Some(key_spread),
                  Some(value_spread),
                ]))));
              }

              return Some(EvaluateResultValue::Expr(create_array_expression(
                entry_elems,
              )));
            },
            CallbackType::Object(ObjectJS::Keys) => {
              let Some(EvaluateResultValue::Expr(keys)) = context.first() else {
                deopt_unsupported!(path, state, "Object.keys() requires an argument.");
              };

              return Some(EvaluateResultValue::Expr(keys.clone()));
            },
            CallbackType::Object(ObjectJS::Values) => {
              let Some(EvaluateResultValue::Expr(values)) = context.first() else {
                deopt_unsupported!(path, state, "Object.values() requires an argument.");
              };

              return Some(EvaluateResultValue::Expr(values.clone()));
            },
            CallbackType::Object(ObjectJS::FromEntries) => {
              let Some(EvaluateResultValue::Entries(entries)) = context.first() else {
                deopt_unsupported!(
                  path,
                  state,
                  "Object.fromEntries() requires an array of [key, value] entries."
                );
              };

              let mut entry_elems = Vec::with_capacity(entries.len());

              for (key, value) in entries {
                let Lit::Str(lit_str) = key else {
                  deopt_unsupported!(path, state, OBJECT_KEY_MUST_BE_IDENT);
                };

                let key_str = convert_atom_to_str_ref(&lit_str.value);

                entry_elems.push(create_ident_key_value_prop(key_str, *value.clone()));
              }

              return Some(EvaluateResultValue::Expr(create_object_expression(
                entry_elems,
              )));
            },
            CallbackType::Math(MathJS::Pow) => {
              let Some(EvaluateResultValue::Vec(args)) = context.first() else {
                deopt_unsupported!(path, state, "Math.pow() requires an argument.");
              };

              // Cloned off the receiver so the numeric read can borrow the
              // evaluation state it records a refusal on.
              let args = args.clone();
              let num_args = args_to_numbers(&args, path, state, traversal_state, fns)?;

              let (Some(base), Some(exp)) = (num_args.first(), num_args.get(1)) else {
                deopt_unsupported!(path, state, "Math.pow() requires two numeric arguments.");
              };

              let result = base.powf(*exp);

              return Some(EvaluateResultValue::Expr(create_number_expr(result)));
            },
            CallbackType::Math(MathJS::Round | MathJS::Floor | MathJS::Ceil) => {
              let Some(EvaluateResultValue::Expr(expr)) = context.first() else {
                deopt_unsupported!(
                  path,
                  state,
                  "Math.round()/Math.ceil()/Math.floor() requires one numeric argument."
                );
              };

              let expr = expr.clone();

              let num = match expr_to_num(&expr, state, traversal_state, fns) {
                Ok(num) => num,
                Err(error) => deopt_unsupported!(path, state, error.to_string().as_str()),
              };

              let result = match func.as_ref() {
                // Not `f64::round`: it breaks ties away from zero, so it answers
                // `-2` where `Math.round(-1.5)` is `-1`.
                CallbackType::Math(MathJS::Round) => js_math_round(num),
                CallbackType::Math(MathJS::Ceil) => num.ceil(),
                CallbackType::Math(MathJS::Floor) => num.floor(),
                _ => stylex_unreachable!("Invalid function type"),
              };

              return Some(EvaluateResultValue::Expr(create_number_expr(result)));
            },
            CallbackType::Math(MathJS::Min | MathJS::Max) => {
              let Some(EvaluateResultValue::Vec(args)) = context.first() else {
                deopt_unsupported!(
                  path,
                  state,
                  "Math.min()/Math.max() requires at least one numeric argument."
                );
              };

              // Cloned for the reason given on the `Math.pow()` arm above.
              let args = args.clone();
              let num_args = args_to_numbers(&args, path, state, traversal_state, fns)?;

              let result = match func.as_ref() {
                CallbackType::Math(MathJS::Min) => {
                  num_args.iter().copied().min_by(sort_numbers_factory())
                },
                CallbackType::Math(MathJS::Max) => {
                  num_args.iter().copied().max_by(sort_numbers_factory())
                },
                _ => stylex_unreachable!("Invalid function type"),
              };

              // `Math.min()` is `Infinity` and `Math.max()` is `-Infinity`;
              // neither is folded, and an empty argument list is a call the
              // author wrote rather than a state the evaluator broke.
              let Some(result) = result else {
                deopt_unsupported!(
                  path,
                  state,
                  "Math.min()/Math.max() requires at least one numeric argument."
                );
              };

              return Some(EvaluateResultValue::Expr(create_number_expr(result)));
            },
            CallbackType::Math(MathJS::Abs) => {
              let Some(EvaluateResultValue::Expr(expr)) = context.first() else {
                deopt_unsupported!(path, state, "Math.abs() requires one numeric argument.");
              };

              let expr = expr.clone();

              let num = match expr_to_num(&expr, state, traversal_state, fns) {
                Ok(num) => num,
                Err(error) => deopt_unsupported!(path, state, error.to_string().as_str()),
              };

              return Some(EvaluateResultValue::Expr(create_number_expr(num.abs())));
            },
            CallbackType::String(StringJS::Concat) => {
              let Some(EvaluateResultValue::Expr(base_str)) = context.first() else {
                deopt_unsupported!(
                  path,
                  state,
                  "String.concat() requires at least one argument."
                );
              };

              let base_str = base_str.clone();

              let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

              let mut str_args_vec = Vec::with_capacity(args.len());
              for arg in &args {
                match arg.as_expr() {
                  Some(expr) => {
                    str_args_vec.push(expr_to_str_or_deopt!(
                      expr,
                      state,
                      traversal_state,
                      fns,
                      EXPRESSION_IS_NOT_A_STRING
                    ));
                  },
                  None => {
                    deopt(path, state, "All arguments must be a string");
                    return None;
                  },
                }
              }
              let str_args = str_args_vec.join("");

              let base_str = expr_to_str_or_deopt!(
                &base_str,
                state,
                traversal_state,
                fns,
                EXPRESSION_IS_NOT_A_STRING
              );

              let mut result = String::with_capacity(base_str.len() + str_args.len());
              result.push_str(&base_str);
              result.push_str(&str_args);

              return Some(EvaluateResultValue::Expr(create_string_expr(&result)));
            },
            CallbackType::String(StringJS::CharCodeAt) => {
              let Some(EvaluateResultValue::Expr(base_str)) = context.first() else {
                deopt_unsupported!(path, state, "String.charCodeAt() requires a receiver.");
              };

              let base_str = base_str.clone();

              let base_str = expr_to_str_or_deopt!(
                &base_str,
                state,
                traversal_state,
                fns,
                EXPRESSION_IS_NOT_A_STRING
              );

              let args = evaluate_func_call_args(call, state, traversal_state, fns)?;

              let num_args = args_to_numbers(&args, path, state, traversal_state, fns)?;

              let Some(char_index) = num_args.first() else {
                deopt_unsupported!(
                  path,
                  state,
                  "The first argument of String.charCodeAt() must be a number."
                );
              };

              // Out of range is `NaN` in JavaScript, which this evaluator does
              // not represent as a folded value, so it refuses instead.
              let Some(char_code) = char_code_at_f64(&base_str, *char_index) else {
                deopt_unsupported!(
                  path,
                  state,
                  "String.charCodeAt() has no result for the given index."
                );
              };

              return Some(EvaluateResultValue::Expr(create_number_expr(
                char_code as f64,
              )));
            },
            CallbackType::Global(_) => {
              stylex_unreachable!("Callable globals are applied before the receiver is read.")
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
          }
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
