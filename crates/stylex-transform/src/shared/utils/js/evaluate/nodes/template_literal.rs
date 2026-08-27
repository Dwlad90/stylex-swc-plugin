use super::super::*;

pub(in super::super) fn evaluate_quasis(
  path: &Expr,
  exprs: &[Box<Expr>],
  quasis: &[TplElement],
  raw: bool,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let quasi_len = quasis
    .iter()
    .map(|elem| {
      if raw {
        elem.raw.len()
      } else {
        extract_tpl_cooked_value(elem).len()
      }
    })
    .sum::<usize>();
  // Grown through a buffer that measures every append against the character
  // ceiling, rather than measured once at the end: a template that interpolates a
  // long value has to be refused at the piece that passes the ceiling, not after
  // the last one is copied in.
  let mut strng = GrownString::with_capacity(quasi_len, TEMPLATE_LITERAL);

  for (i, elem) in quasis.iter().enumerate() {
    if !state.confident {
      return None;
    }

    let quasi = if raw {
      &*elem.raw
    } else {
      extract_tpl_cooked_value(elem)
    };

    strng
      .push(quasi, || path.clone(), state, traversal_state)
      .ok()?;

    let Some(expr) = exprs.get(i) else {
      continue;
    };

    let Some(evaluated_expr) = evaluate_cached(expr, state, traversal_state, fns) else {
      // The evaluator gave no value for the interpolation, so the template has
      // no text. Returning here rather than falling through to the loop's
      // `state.confident` guard on the next turn is not an optimization: the
      // guard would let this turn finish and append the *next* quasi first, and
      // the whole point of the change around this line is that a missing
      // interpolation must never leave a shorter string standing. Whether the
      // evaluator also cleared `confident` is its business and is not assumed
      // here -- either way this template has no compile-time text.
      return None;
    };

    // Interpolation is `ToString`, not "is this a literal I recognize". The
    // chain here used to require an `Expr`, then a `Lit`, then a spelling --
    // and contributed the empty string whenever any link failed, so `${null}`,
    // `${true}`, `${undefined}`, `${{}}` and `${[1, 2]}` each declared a value
    // the author did not write and hashed a class name to match. Every one of
    // them has a `ToString`, and the bridge below is the one the rest of the
    // evaluator already reads.
    let Some(text) = evaluate_result_to_js_string(&evaluated_expr) else {
      // No string form at all, which is a function: `String(fn)` is its source
      // text and this evaluator keeps no source. Refused rather than skipped,
      // because a missing interpolation is what this whole change is about --
      // and refused rather than answered, because the reference implementation's
      // answer here is the source text of its *own* evaluator closure, an
      // internal artifact that has no business in a stylesheet.
      deopt(expr, state, EXPRESSION_IS_NOT_A_STRING);

      return None;
    };

    strng
      .push(&text, || path.clone(), state, traversal_state)
      .ok()?;
  }

  if !state.confident {
    return None;
  }

  Some(EvaluateResultValue::Expr(create_string_expr(
    &strng.into_text(),
  )))
}
