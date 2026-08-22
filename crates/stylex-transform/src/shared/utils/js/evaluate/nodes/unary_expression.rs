use super::super::*;
use crate::deopt_unsupported;
use stylex_ast::ast::factories::create_unary_expr;
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

  // Answered without folding the operand: `typeof` never evaluates a function,
  // and folding one would answer for its body rather than for its kind. All
  // three spellings, because an arrow is neither of the other two -- it used to
  // fall through, fold to a callback, and then be refused for having no
  // expression form, where the language says `"function"`. Read through the
  // parentheses a function expression in a value position has to be written
  // with, which the fold unwraps but this check runs ahead of.
  if unary.op == UnaryOp::TypeOf {
    let bare = normalize_expr(argument);

    if bare.is_fn_expr() || bare.is_class() || bare.is_arrow() {
      return Some(EvaluateResultValue::Expr(create_string_expr("function")));
    }
  }

  let arg = evaluate_cached(argument, state, traversal_state, fns);

  if !state.confident {
    return None;
  }

  // An operand that folded to nothing has no compile-time value to apply the
  // operator to. `typeof someObject.method` is ordinary JavaScript, so this
  // refuses the fold rather than aborting the build.
  let Some(arg) = arg else {
    deopt_unsupported!(&create_unary_expr(unary), state, ILLEGAL_PROP_VALUE);
  };

  // `!` is answered off the evaluated value rather than off an expression form
  // of it, and through the one `ToBoolean` bridge the logical operators read.
  // An operand with no expression form still has a truthiness -- the
  // evaluator's own array and function-map spellings all stand for objects,
  // which are truthy -- so `![]` folds where reading it as an expression
  // refused. The bridge's refusal is the operand whose *kind* cannot be read,
  // and that deopts.
  if unary.op == UnaryOp::Bang {
    let Some(value) = evaluate_result_to_js_boolean(&arg) else {
      deopt_unsupported!(&create_unary_expr(unary), state, ILLEGAL_PROP_VALUE);
    };

    return Some(EvaluateResultValue::Expr(create_bool_expr(!value)));
  }

  // `typeof` reads the operand's *kind*, which every evaluated value has,
  // rather than a primitive out of it -- so it is answered off the value like
  // `!` is, and before the expression-form guard below.
  if unary.op == UnaryOp::TypeOf {
    let Some(arg_type) = type_of(&arg) else {
      deopt_unsupported!(
        &create_unary_expr(unary),
        state,
        // A kind this evaluator has no reading of. Named as the expression it
        // could not read where there is one to name, so the message says which
        // shape stopped it rather than only that something did.
        &match &arg {
          EvaluateResultValue::Expr(expr) => unsupported_expression(get_expr_node_kind(expr)),
          _ => ILLEGAL_PROP_VALUE.to_string(),
        }
      );
    };

    return Some(EvaluateResultValue::Expr(create_string_expr(arg_type)));
  }

  match unary.op {
    UnaryOp::Plus => evaluate_unary_numeric_of(unary, &arg, state, traversal_state, fns, |v| v),
    UnaryOp::Minus => evaluate_unary_numeric_of(unary, &arg, state, traversal_state, fns, |v| -v),
    // `~` applies `ToInt32` first, so the negation happens in 32 bits: JS says
    // `~[4294967296]` is `-1`, where a 64-bit negation says `-4294967297`. The
    // operands that reach the wrap are the ones the number bridge newly made
    // reachable, an array or an object whose string form is a large number.
    UnaryOp::Tilde => evaluate_unary_numeric_of(unary, &arg, state, traversal_state, fns, |v| {
      f64::from(!coercions::to_int32(v))
    }),
    _ => deopt(
      &create_unary_expr(unary),
      state,
      &unsupported_operator(unary.op.as_str()),
    ),
  }
}

/// The type name `typeof` answers for an evaluated value, or `None` where the
/// value's kind cannot be read.
///
/// The expression arm is the primitive table; every variant the evaluator has of
/// its own stands for an object or a function upstream, which is the same
/// classification the `ToObject` bridge makes, so it is asked rather than
/// restated.
fn type_of(value: &EvaluateResultValue) -> Option<&'static str> {
  let EvaluateResultValue::Expr(expr) = value else {
    return match evaluate_result_to_js_object(value)? {
      coercions::ObjectCoercion::Function => Some("function"),
      _ => Some("object"),
    };
  };

  match expr {
    Expr::Lit(Lit::Str(_)) => Some("string"),
    Expr::Lit(Lit::Bool(_)) => Some("boolean"),
    Expr::Lit(Lit::Num(_)) => Some("number"),
    Expr::Lit(Lit::Null(_)) => Some("object"),
    Expr::Fn(_) => Some("function"),
    Expr::Class(_) => Some("function"),
    Expr::Arrow(_) => Some("function"),
    Expr::Ident(ident) if is_js_undefined(ident) => Some("undefined"),
    Expr::Object(_) => Some("object"),
    Expr::Array(_) => Some("object"),
    // Every other expression kind is one `typeof` would answer for at runtime
    // and this evaluator has no reading of, so it refuses rather than guessing
    // a type name.
    _ => None,
  }
}

/// `ToNumber` over an evaluated operand, then the operator's arithmetic.
///
/// Two readings, in this order because each can do what the other cannot.
/// `expr_to_num` resolves an identifier through its binding and folds a binary
/// expression, neither of which is a coercion; the number bridge reaches an
/// object or an array through its primitive string form, which `expr_to_num`
/// bails on. Requiring only the first refused `-({})`, `+({})`, `~({})` and
/// `-[1, 2, 3]`, all four of which upstream folds.
fn evaluate_unary_numeric_of(
  unary: &UnaryExpr,
  arg: &EvaluateResultValue,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  transform: impl FnOnce(f64) -> f64,
) -> Option<EvaluateResultValue> {
  let value = match arg {
    EvaluateResultValue::Expr(expr) => match expr_to_num(expr, state, traversal_state, fns) {
      Ok(value) => Some(value),
      // Kept as the refusal's wording where the bridge has nothing to add, so
      // an operand with no numeric reading still names its own shape.
      Err(error) => match evaluate_result_to_js_number(arg) {
        Some(value) => Some(value),
        None => deopt_unsupported!(&create_unary_expr(unary), state, error.to_string().as_str()),
      },
    },
    _ => evaluate_result_to_js_number(arg),
  };

  let Some(value) = value else {
    deopt_unsupported!(&create_unary_expr(unary), state, ILLEGAL_PROP_VALUE);
  };

  Some(EvaluateResultValue::Expr(create_number_expr(transform(
    value,
  ))))
}
