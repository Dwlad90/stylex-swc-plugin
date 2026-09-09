use super::super::*;
use stylex_ast::ast::factories::create_unary_expr;
use stylex_macros::deopt_unsupported;
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

  // One question rather than two, for a shape no source reaches twice.
  //
  // The memo is the one thing that answers nothing while the walk stays
  // confident, and a warmed memo was probed here: every operator over the
  // subtree it holds refuses before this line, because the operand itself
  // records the refusal. So the second arm is entered from no source, and
  // there is one answer rather than two. `typeof someObject.method` is
  // ordinary JavaScript, and what it reads is the member's own refusal rather
  // than one this node invents.
  let arg = evaluate_cached(argument, state, traversal_state, fns)?;

  // `!` is answered off the evaluated value rather than off an expression form
  // of it, and through the one `ToBoolean` bridge the logical operators read.
  // An operand with no expression form still has a truthiness -- the
  // evaluator's own array and function-map spellings all stand for objects,
  // which are truthy -- so `![]` folds where reading it as an expression
  // refused. The bridge's refusal is the operand whose *kind* cannot be read,
  // and that deopts.
  if unary.op == UnaryOp::Bang {
    let Some(value) = evaluate_result_to_js_boolean(&arg) else {
      deopt_unsupported!(deopt, &create_unary_expr(unary), state, ILLEGAL_PROP_VALUE);
    };

    return Some(EvaluateResultValue::Expr(create_bool_expr(!value)));
  }

  // `typeof` reads the operand's *kind*, which every evaluated value has,
  // rather than a primitive out of it -- so it is answered off the value like
  // `!` is, and before the expression-form guard below.
  if unary.op == UnaryOp::TypeOf {
    let Some(arg_type) = type_of(&arg) else {
      // A value with no kind to read. What reaches this is a value that is not
      // there: a fold answered nothing for it while staying confident, and the
      // array it sat in kept the slot. The sentence is the one `!` gives for
      // the same value, because it is the same complaint -- the operand has no
      // reading at all, rather than a shape that could be named.
      deopt_unsupported!(deopt, &create_unary_expr(unary), state, ILLEGAL_PROP_VALUE);
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
    // Every value the evaluator has of its own stands for an object or a
    // function upstream, and the one `ToObject` bridge decides which. It
    // refuses the absent element of an array, whose meaning that bridge
    // deliberately leaves undecided, and the refusal travels out from here.
    return match evaluate_result_to_js_object(value)? {
      coercions::ObjectCoercion::Function => Some("function"),
      _ => Some("object"),
    };
  };

  match expr {
    Expr::Lit(Lit::Str(_)) => Some("string"),
    Expr::Lit(Lit::Bool(_)) => Some("boolean"),
    Expr::Lit(Lit::Num(_)) => Some("number"),
    Expr::Ident(ident) if is_js_undefined(ident) => Some("undefined"),
    // Every other kind an evaluated value holds is an object or a function
    // upstream, and the same bridge decides which: `null` is the object
    // `typeof` names, an array and an object literal are objects, and the three
    // function spellings are functions.
    //
    // `None` is a kind this evaluator has no reading of. It cannot arrive from
    // a fold -- the dispatch refuses every expression kind the table above and
    // the bridge do not cover -- so what it guards is a caller asking about a
    // value the evaluator never made.
    _ => match coercions::to_object(expr)? {
      coercions::ObjectCoercion::Function => Some("function"),
      _ => Some("object"),
    },
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
  // The first reading's own wording, carried alongside its answer so that an
  // operand with no numeric reading still names its own shape where the bridge
  // has nothing to add. Only ever read on the refusal below, so nothing is
  // spelled out for an operand that folds.
  let (numeric_reading, first_refusal) = match arg {
    EvaluateResultValue::Expr(expr) => match expr_to_num(expr, state, traversal_state, fns) {
      Ok(value) => (Ok(value), None),
      Err(error) => (
        evaluate_result_to_js_number(arg, traversal_state),
        Some(error.to_string()),
      ),
    },
    _ => (evaluate_result_to_js_number(arg, traversal_state), None),
  };

  let value = match numeric_reading {
    Ok(value) => value,
    Err(NumberRefusal::NoNumberForm) => deopt_unsupported!(
      deopt,
      &create_unary_expr(unary),
      state,
      first_refusal.as_deref().unwrap_or(ILLEGAL_PROP_VALUE)
    ),
    // The operand's number is the number of a string, and that string is past
    // the ceiling. Named as the operator the author wrote rather than as the
    // join inside it, the way a growing string's refusal names the `+` or the
    // interpolation it grew in.
    Err(NumberRefusal::TooLarge) => deopt_unsupported!(
      deopt,
      &create_unary_expr(unary),
      state,
      &grown_string_too_large(
        NUMERIC_CONVERSION,
        traversal_state.character_ceiling() as u64
      )
    ),
  };

  Some(EvaluateResultValue::Expr(create_number_expr(transform(
    value,
  ))))
}

#[cfg(test)]
#[path = "tests/type_of_tests.rs"]
mod type_of_tests;

#[cfg(test)]
#[path = "tests/unary_operator_tests.rs"]
mod unary_operator_tests;
