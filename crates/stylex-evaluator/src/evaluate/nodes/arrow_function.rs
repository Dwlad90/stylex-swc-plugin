use super::super::*;
use swc_core::ecma::ast::ArrowExpr;

pub(in super::super) fn evaluate(
  arrow: &ArrowExpr,
  state: &mut EvaluationState,
) -> Option<EvaluateResultValue> {
  let body = &arrow.body;
  let params = &arrow.params;

  let ident_params = params
    .iter()
    .filter_map(|param| {
      if let Pat::Ident(ident) = param {
        Some(ident.sym.clone())
      } else {
        None
      }
    })
    .collect::<Vec<Atom>>();

  match body.as_ref() {
    ArrowFunctionBody::Expr(body_expr) => {
      if ident_params.len() == params.len() {
        // The member entries are kept beside the names, so a body that calls
        // `stylex.keyframes(…)` reaches the function the module reaches.
        //
        // The map of the module is shared, and it is copied only for a call
        // that binds a parameter.
        let arrow_closure_fabric =
          |functions: Rc<FunctionMap>, ident_params: Vec<Atom>, body_expr: Box<Expr>| {
            move |cb_args: Vec<EvaluateResultValue>, traversal_state: &mut StateManager| {
              let mut bound_params = Vec::new();

              ident_params.iter().enumerate().for_each(|(index, ident)| {
                // An argument with no form to bind binds nothing, leaving the
                // parameter unresolved so the body answers on its own terms —
                // which is what the language does with a missing argument too.
                // The callback has no deopt to record and must not abort; see the
                // fallback at the end of this closure. A body that never reads
                // the parameter therefore folds, and a body that does reads an
                // unresolved name and refuses for that.
                //
                // A theme reference has no expression to write down and binds
                // through the factory the module's own token imports bind
                // through, so a parameter holding one resolves a member exactly
                // as the imported name does.
                let bound = match cb_args.get(index) {
                  Some(EvaluateResultValue::ThemeRef(theme)) => {
                    let theme = theme.clone();

                    Some(FunctionType::ThemeRefMapper(Rc::new(move || theme.clone())))
                  },
                  Some(arg) => evaluate_result_as_expr(arg).map(|expr| {
                    let cl = |arg: Expr| move || arg.clone();

                    FunctionType::Mapper(Rc::new((cl)(expr)))
                  }),
                  None => None,
                };

                if let Some(fn_ptr) = bound {
                  bound_params.push((ident.clone(), fn_ptr));
                }
              });

              // The body always reads imports, so a map that holds them back is
              // copied too.
              let functions = match bound_params.is_empty() && !functions.disable_imports {
                true => Rc::clone(&functions),
                false => Rc::new(with_parameters(&functions, bound_params)),
              };

              // Once per invocation of the callback, not once per callback, so
              // what this copies is worth keeping small. The parsed module and
              // its key-span index are behind an `Rc` for that reason -- both are
              // read-only once memoized, and copying them here is what made a
              // dynamic style's cost scale with the size of the file it sits in.
              let mut local_state = traversal_state.clone();

              let result = evaluate_with_functions(&body_expr, &mut local_state, functions);

              // A compiler function in the body filed its rule on the copy.
              traversal_state.keep_rules_filed_on(&local_state);

              let value = result.value;

              // A body that did not fold answers nothing, which is how a
              // callback declines: it has no deopt to record, and the caller
              // that applied it is what can name the call in a sentence.
              // Aborting here would fail a build over a callback that was only
              // ever going to run at runtime.
              match value {
                Some(EvaluateResultValue::Expr(expr)) => Some(expr),
                Some(EvaluateResultValue::Vec(items)) => evaluate_result_vec_to_array_expr(&items),
                _ => None,
              }
            }
          };

        let arrow_closure = Rc::new(arrow_closure_fabric(
          Rc::clone(&state.functions),
          ident_params,
          Box::new(*body_expr.clone()),
        ));

        return Some(EvaluateResultValue::Callback(arrow_closure));
      }

      None
    },
    ArrowFunctionBody::FunctionBody(_) => None,
  }
}

/// A copy of `functions` with each bound parameter added as a name, for the
/// body of one call of a callback.
///
/// The parameters are also the members of `entry`, where a call binds any.
/// The copy always reads imports.
fn with_parameters(functions: &FunctionMap, bound: Vec<(Atom, FunctionType)>) -> FunctionMap {
  let mut identifiers = functions.identifiers.clone();
  let mut member_expressions = functions.member_expressions.clone();
  let binds_any = !bound.is_empty();

  identifiers.extend(bound.into_iter().map(|(ident, fn_ptr)| {
    let function = FunctionConfig {
      fn_ptr,
      takes_path: false,
    };

    (ident, Box::new(FunctionConfigType::Regular(function)))
  }));

  member_expressions.extend(binds_any.then(|| {
    (
      ImportSources::Regular("entry".to_string()),
      Box::new(identifiers.clone()),
    )
  }));

  FunctionMap {
    identifiers,
    member_expressions,
    disable_imports: false,
  }
}
