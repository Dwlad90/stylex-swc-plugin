use super::super::*;
use stylex_ast::ast::convertors::create_ident_expr;
use swc_core::ecma::ast::{Lit, UnaryExpr, UnaryOp};

pub(in super::super) fn evaluate(
  unary: &UnaryExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  // `void x` is `undefined` whatever `x` is, so the operand is never evaluated
  // and an operand that would have deopted cannot deopt this.
  //
  // Answered as the `undefined` identifier rather than as no value: the
  // evaluator's caller turns a confident `None` into a deopt, so returning
  // nothing here would fail the build on an expression that has a value.
  if unary.op == UnaryOp::Void {
    return Some(EvaluateResultValue::Expr(create_ident_expr("undefined")));
  }

  let argument = &unary.arg;

  if unary.op == UnaryOp::TypeOf && (argument.is_fn_expr() || argument.is_class()) {
    return Some(EvaluateResultValue::Expr(create_string_expr("function")));
  }

  let arg = evaluate_cached(argument, state, traversal_state, fns);

  if !state.confident {
    return None;
  }

  let arg = match match arg {
    Some(v) => v,
    None => stylex_panic!("The operand of a unary expression must be a static expression."),
  } {
    EvaluateResultValue::Expr(expr) => expr,
    _ => {
      let path = Expr::Unary(unary.clone());
      stylex_panic_with_context!(
        &path,
        traversal_state,
        "The operand of a unary expression must be a static expression."
      )
    },
  };

  match unary.op {
    UnaryOp::Bang => {
      let value = convert_expr_to_bool(&arg, traversal_state, fns);

      Some(EvaluateResultValue::Expr(create_bool_expr(!value)))
    },
    UnaryOp::Plus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| v),
    UnaryOp::Minus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| -v),
    UnaryOp::Tilde => {
      evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| (!(v as i64)) as f64)
    },
    UnaryOp::TypeOf => {
      let arg_type = match &arg {
        Expr::Lit(Lit::Str(_)) => "string",
        Expr::Lit(Lit::Bool(_)) => "boolean",
        Expr::Lit(Lit::Num(_)) => "number",
        Expr::Lit(Lit::Null(_)) => "object",
        Expr::Fn(_) => "function",
        Expr::Class(_) => "function",
        Expr::Ident(ident) if ident.sym == *"undefined" => "undefined",
        Expr::Object(_) => "object",
        Expr::Array(_) => "object",
        _ => {
          let path = Expr::Unary(unary.clone());
          stylex_panic_with_context!(
            &path,
            traversal_state,
            "This unary operator is not supported in static evaluation."
          )
        },
      };

      Some(EvaluateResultValue::Expr(create_string_expr(arg_type)))
    },
    _ => deopt(
      &Expr::from(unary.clone()),
      state,
      &unsupported_operator(unary.op.as_str()),
    ),
  }
}
