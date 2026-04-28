use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state_manager::StateManager, types::EvaluationCallback},
  utils::ast::convertors::{convert_expr_to_str, convert_lit_to_number, create_string_expr},
};
use stylex_ast::ast::factories::{create_array_expression, create_expr_or_spread};
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::ecma::ast::{Expr, ExprOrSpread};

pub(crate) fn evaluate_map(
  funcs: &[EvaluateResultValue],
  args: &[EvaluateResultValue],
  traversal_state: &mut StateManager,
) -> Option<EvaluateResultValue> {
  let cb = funcs.first()?;

  let cb = cb.as_callback()?;

  let mut func_result = Vec::with_capacity(args.len());

  for arg in args {
    if matches!(arg, EvaluateResultValue::Null) {
      continue;
    }

    match arg {
      EvaluateResultValue::Expr(_) => func_result.push(evaluate_map_cb(cb, arg, traversal_state)),
      EvaluateResultValue::Vec(vec) => {
        let func_result_value = vec
          .iter()
          .map(|expr| {
            let expr = evaluate_map_cb(cb, expr, traversal_state);

            EvaluateResultValue::Expr(expr)
          })
          .collect::<Vec<EvaluateResultValue>>();

        let elems = func_result_value
          .into_iter()
          .map(|item| Some(create_expr_or_spread(item.as_expr()?.clone())))
          .collect::<Vec<Option<ExprOrSpread>>>();

        func_result.push(create_array_expression(elems));
      },
      #[cfg_attr(coverage_nightly, coverage(off))]
      _ => stylex_unimplemented!("Unhandled EvaluateResultValue in map callback"),
    }
  }

  match func_result.first() {
    Some(Expr::Array(array)) => Some(EvaluateResultValue::Expr(Expr::from(array.clone()))),
    _ => Some(EvaluateResultValue::Expr(create_array_expression(
      func_result
        .into_iter()
        .map(|expr| Some(create_expr_or_spread(expr)))
        .collect(),
    ))),
  }
}

pub(crate) fn evaluate_join(
  funcs: &[EvaluateResultValue],
  args: &[EvaluateResultValue],
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let join_arg = funcs.first()?;

  let join_arg = match convert_expr_to_str(join_arg.as_expr()?, state, functions) {
    Some(s) => s,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("The join() separator argument must be a string value."),
  };

  let result = args
    .iter()
    .map(|arg| {
      let arg_expr = match arg.as_expr() {
        Some(expr) => expr,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("Array element must evaluate to a string for join()."),
      };
      match convert_expr_to_str(arg_expr, state, functions) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("Array element must evaluate to a string for join()."),
      }
    })
    .collect::<Vec<String>>()
    .join(&join_arg);

  Some(EvaluateResultValue::Expr(create_string_expr(&result)))
}

pub(crate) fn evaluate_filter(
  funcs: &[EvaluateResultValue],
  args: &[EvaluateResultValue],
  traversal_state: &mut StateManager,
) -> Option<EvaluateResultValue> {
  let cb = funcs.first()?;

  let cb = cb.as_callback()?;

  let mut func_result = Vec::with_capacity(args.len());

  for arg in args {
    if matches!(arg, EvaluateResultValue::Null) {
      continue;
    }

    match arg {
      EvaluateResultValue::Expr(expr) => {
        if let Some(expr) = evaluate_filter_cb(cb, arg, expr, traversal_state) {
          func_result.push(expr);
        }
      },
      EvaluateResultValue::Vec(vec) => {
        let func_result_value = vec
          .iter()
          .filter_map(|expr| {
            if matches!(expr, EvaluateResultValue::Null) {
              return None;
            }

            let result = evaluate_filter_cb(cb, expr, &expr.as_expr()?.clone(), traversal_state);

            result.map(EvaluateResultValue::Expr)
          })
          .collect::<Vec<EvaluateResultValue>>();

        let elems = func_result_value
          .into_iter()
          .map(|item| Some(create_expr_or_spread(item.as_expr()?.clone())))
          .collect::<Vec<Option<ExprOrSpread>>>();

        func_result.push(create_array_expression(elems));
      },
      #[cfg_attr(coverage_nightly, coverage(off))]
      _ => stylex_unimplemented!("Unhandled EvaluateResultValue in filter callback"),
    }
  }

  match func_result.first() {
    Some(Expr::Array(array)) => Some(EvaluateResultValue::Expr(Expr::from(array.clone()))),
    _ => Some(EvaluateResultValue::Expr(create_array_expression(
      func_result
        .into_iter()
        .map(|expr| Some(create_expr_or_spread(expr)))
        .collect(),
    ))),
  }
}

pub(crate) fn evaluate_map_cb(
  cb: &EvaluationCallback,
  cb_arg: &EvaluateResultValue,
  traversal_state: &mut StateManager,
) -> Expr {
  (cb)(vec![cb_arg.clone()], traversal_state)
}

pub(crate) fn evaluate_filter_cb(
  cb: &EvaluationCallback,
  cb_arg: &EvaluateResultValue,
  item: &Expr,
  traversal_state: &mut StateManager,
) -> Option<Expr> {
  let result = evaluate_map_cb(cb, cb_arg, traversal_state);

  let Some(lit) = result.as_lit() else {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("Expr is not a literal");
    }
  };

  if convert_lit_to_number(lit).unwrap_or_else(|error| {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("{}", error)
    }
  }) == 0.0
  {
    None
  } else {
    Some(item.clone())
  }
}
