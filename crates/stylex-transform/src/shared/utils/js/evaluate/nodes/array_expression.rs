use super::super::*;
use swc_core::ecma::ast::ArrayLit;

pub(in super::super) fn evaluate(
  arr_path: &ArrayLit,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
) -> Option<EvaluateResultValue> {
  let mut arr: Vec<EvaluateResultValue> = Vec::with_capacity(arr_path.elems.len());

  for elem in arr_path.elems.iter().flatten() {
    // A spread is refused, whatever it spreads, and before the operand is
    // evaluated at all.
    //
    // The reference implementation evaluates each *element path*, so a spread
    // arrives as a `SpreadElement` node and falls to its terminal
    // `UNSUPPORTED_EXPRESSION(path.node.type)` arm -- every shape, uniformly,
    // ahead of any value validation and without ever looking at the operand.
    // The loop below reads `elem.expr`, which unwraps the spread and evaluates
    // the operand instead, so the refusal has to be made here for the two to
    // agree. Made first for the same reason: `[...unknownThing]` is a
    // `SpreadElement` refusal upstream, not the operand's own.
    //
    // Agreeing matters twice over. A spread of a binding or an object was
    // already refused, but by the value rule and with a different sentence
    // than an author reads from upstream. A spread of a literal was not
    // refused at all: `[..."ab"]` folded to `["ab"]` and shipped `color: ab`
    // where the language spreads two characters, and `[...1]` shipped
    // `color: 1` where the language throws.
    //
    // Reported at the array rather than at the operand, because SWC carries no
    // expression node for the spread itself and the array is the nearest node
    // that contains the `...`.
    if elem.spread.is_some() {
      return deopt(&Expr::Array(arr_path.clone()), state, SPREAD_ELEMENT);
    }

    let elem_value =
      evaluate_with_functions(&elem.expr, traversal_state, Rc::clone(&state.functions));

    if !elem_value.confident {
      // The element's own refusal is carried up, rather than left for the
      // caller's catch-all to overwrite with `Unsupported expression:
      // ArrayExpression`. An element is evaluated under a fresh state, so its
      // reason reaches this one only by being copied across -- which is what
      // the reference implementation does here:
      // `elemValue.deopt && deopt(elemValue.deopt, state, elemValue.reason ??
      // 'unknown error')`.
      //
      // Without it a nested refusal is renamed after its container: `[[...xs]]`
      // reported `ArrayExpression` where upstream reports `SpreadElement`,
      // naming the node the author did not write instead of the one they did.
      if let Some(elem_deopt) = &elem_value.deopt {
        deopt(
          elem_deopt,
          state,
          elem_value.reason.as_deref().unwrap_or("unknown error"),
        );
      }

      return None;
    }

    let value = elem_value.value.unwrap_or(EvaluateResultValue::Null);

    arr.push(value);
  }

  Some(EvaluateResultValue::Vec(arr))
}
