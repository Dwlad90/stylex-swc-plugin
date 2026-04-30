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
    BlockStmtOrExpr::Expr(body_expr) => {
      if ident_params.len() == params.len() {
        let arrow_closure_fabric =
          |identifiers: FunctionMapIdentifiers, ident_params: Vec<Atom>, body_expr: Box<Expr>| {
            move |cb_args: Vec<EvaluateResultValue>, traversal_state: &mut StateManager| {
              let mut identifiers = identifiers.clone();

              let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

              ident_params.iter().enumerate().for_each(|(index, ident)| {
                if let Some(arg) = cb_args.get(index) {
                  let expr = arg.as_expr().unwrap_or_else(|| {
                    #[cfg_attr(coverage_nightly, coverage(off))]
                    {
                      stylex_panic!("{}", ARGUMENT_NOT_EXPRESSION)
                    }
                  });

                  let cl = |arg: Expr| move || arg.clone();

                  let result = (cl)(expr.clone());
                  let function = FunctionConfig {
                    fn_ptr: FunctionType::Mapper(Rc::new(result)),
                    takes_path: false,
                  };
                  identifiers.insert(
                    ident.clone(),
                    Box::new(FunctionConfigType::Regular(function)),
                  );

                  member_expressions.insert(
                    ImportSources::Regular("entry".to_string()),
                    Box::new(identifiers.clone()),
                  );
                }
              });

              let mut local_state = traversal_state.clone();

              let result = evaluate_with_functions(
                &body_expr,
                &mut local_state,
                Rc::new(FunctionMap {
                  identifiers,
                  member_expressions,
                  disable_imports: false,
                }),
              );

              let value = result.value;

              match value {
                Some(res) => match res {
                  EvaluateResultValue::Expr(expr) => expr,
                  EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(&items),
                  #[cfg_attr(coverage_nightly, coverage(off))]
                  _ => stylex_unimplemented!(
                    "The evaluation result must resolve to a static expression."
                  ),
                },
                None => *body_expr.clone(),
              }
            }
          };

        let identifiers = state.functions.identifiers.clone();

        let arrow_closure = Rc::new(arrow_closure_fabric(
          identifiers,
          ident_params,
          Box::new(*body_expr.clone()),
        ));

        return Some(EvaluateResultValue::Callback(arrow_closure));
      }

      None
    },
    BlockStmtOrExpr::BlockStmt(_) => None,
  }
}
