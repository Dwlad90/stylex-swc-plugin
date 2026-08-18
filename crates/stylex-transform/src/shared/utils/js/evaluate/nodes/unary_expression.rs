use super::super::*;
use crate::deopt_unsupported;
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
    return Some(js_undefined());
  }

  let argument = &unary.arg;

  if unary.op == UnaryOp::TypeOf && (argument.is_fn_expr() || argument.is_class()) {
    return Some(EvaluateResultValue::Expr(create_string_expr("function")));
  }

  let arg = evaluate_cached(argument, state, traversal_state, fns);

  if !state.confident {
    return None;
  }

  // An operand with no expression form has no compile-time value to apply the
  // operator to. `typeof someObject.method` is ordinary JavaScript, so this
  // refuses the fold rather than aborting the build.
  let path = Expr::Unary(unary.clone());

  let Some(EvaluateResultValue::Expr(arg)) = arg else {
    deopt_unsupported!(&path, state, ILLEGAL_PROP_VALUE);
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
        // Every other expression kind is one `typeof` would answer for at
        // runtime and this evaluator has no reading of, so it refuses rather
        // than guessing a type name.
        _ => deopt_unsupported!(
          &path,
          state,
          &unsupported_expression(&format!("{:?}", arg.get_type(get_default_expr_ctx())))
        ),
      };

      Some(EvaluateResultValue::Expr(create_string_expr(arg_type)))
    },
    _ => deopt(&path, state, &unsupported_operator(unary.op.as_str())),
  }
}
