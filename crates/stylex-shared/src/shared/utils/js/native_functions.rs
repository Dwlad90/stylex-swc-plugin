use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::ast::{
    convertors::{expr_to_str, lit_to_num, string_to_expression},
    factories::array_expression_factory,
  },
};
use std::rc::Rc;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{ArrayLit, Expr, ExprOrSpread},
};

pub(crate) fn evaluate_map(
  funcs: &[EvaluateResultValue],
  args: &[Option<EvaluateResultValue>],
) -> Option<EvaluateResultValue> {
  let cb = funcs.first()?;

  let cb = cb.as_callback()?;

  let func_result = args
    .iter()
    .filter_map(|arg| {
      let result = arg.as_ref()?;

      match result {
        EvaluateResultValue::Expr(_) => Some(evaluate_map_cb(cb, arg)),
        EvaluateResultValue::Vec(vec) => {
          let func_result = vec
            .iter()
            .map(|expr| {
              let expr = evaluate_map_cb(cb, expr);

              EvaluateResultValue::Expr(expr)
            })
            .collect::<Vec<EvaluateResultValue>>();

          let elems = func_result
            .into_iter()
            .map(|item| {
              Some(ExprOrSpread {
                spread: None,
                expr: Box::new(item.as_expr()?.clone()),
              })
            })
            .collect::<Vec<Option<ExprOrSpread>>>();

          Some(array_expression_factory(elems))
        }
        _ => unimplemented!(),
      }
    })
    .collect::<Vec<Expr>>();

  match func_result.first() {
    Some(Expr::Array(array)) => Some(EvaluateResultValue::Expr(Expr::from(array.clone()))),
    _ => Some(EvaluateResultValue::Expr(array_expression_factory(
      func_result
        .into_iter()
        .map(|expr| {
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect(),
    ))),
  }
}

pub(crate) fn evaluate_join(
  funcs: &[EvaluateResultValue],
  args: &[Option<EvaluateResultValue>],
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let join_arg = funcs.first()?;

  let join_arg =
    expr_to_str(join_arg.as_expr()?, state, functions).expect("Join argument is not a string");

  let result = args
    .iter()
    .map(|arg_ref| {
      arg_ref
        .as_ref()
        .and_then(|arg| arg.as_expr())
        .map(|arg| expr_to_str(arg, state, functions).expect("Argument is not a string"))
        .expect("Failed parsing \"join\" argument to string")
    })
    .collect::<Vec<String>>()
    .join(&join_arg);

  Some(EvaluateResultValue::Expr(string_to_expression(&result)))
}

pub(crate) fn evaluate_filter(
  funcs: &[EvaluateResultValue],
  args: &[Option<EvaluateResultValue>],
) -> Option<EvaluateResultValue> {
  let cb = funcs.first()?;

  let cb = cb.as_callback()?;

  let func_result = args
    .iter()
    .filter_map(|arg| {
      let result = arg.as_ref()?;

      match result {
        EvaluateResultValue::Expr(expr) => evaluate_filter_cb(cb, arg, expr),
        EvaluateResultValue::Vec(vec) => {
          let func_result = vec
            .iter()
            .filter_map(|expr| {
              let result =
                evaluate_filter_cb(cb, &expr.clone(), &expr.as_ref()?.as_expr()?.clone());

              result.map(EvaluateResultValue::Expr)
            })
            .collect::<Vec<EvaluateResultValue>>();

          let elems = func_result
            .into_iter()
            .map(|item| {
              Some(ExprOrSpread {
                spread: None,
                expr: Box::new(item.as_expr()?.clone()),
              })
            })
            .collect::<Vec<Option<ExprOrSpread>>>();

          Some(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems,
          }))
        }
        _ => unimplemented!(),
      }
    })
    .collect::<Vec<Expr>>();

  match func_result.first() {
    Some(Expr::Array(array)) => Some(EvaluateResultValue::Expr(Expr::from(array.clone()))),
    _ => Some(EvaluateResultValue::Expr(Expr::from(ArrayLit {
      span: DUMMY_SP,
      elems: func_result
        .into_iter()
        .map(|expr| {
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect(),
    }))),
  }
}

pub(crate) fn evaluate_map_cb(
  cb: &Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr>,
  cb_arg: &Option<EvaluateResultValue>,
) -> Expr {
  (cb)(vec![cb_arg.clone()])
}

pub(crate) fn evaluate_filter_cb(
  cb: &Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr>,
  cb_arg: &Option<EvaluateResultValue>,
  item: &Expr,
) -> Option<Expr> {
  let result = evaluate_map_cb(cb, cb_arg);

  let Some(lit) = result.as_lit() else {
    panic!("Expr is not a literal");
  };

  if lit_to_num(lit).unwrap_or_else(|error| panic!("{}", error)) == 0.0 {
    None
  } else {
    Some(item.clone())
  }
}
