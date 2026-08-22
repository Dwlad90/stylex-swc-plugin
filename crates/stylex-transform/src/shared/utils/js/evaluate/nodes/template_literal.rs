use super::super::*;

pub(in super::super) fn evaluate_quasis(
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
  let mut strng = String::with_capacity(quasi_len);

  for (i, elem) in quasis.iter().enumerate() {
    if !state.confident {
      return None;
    }

    if raw {
      strng.push_str(&elem.raw);
    } else {
      strng.push_str(extract_tpl_cooked_value(elem));
    }

    let Some(expr) = exprs.get(i) else {
      continue;
    };

    let Some(evaluated_expr) = evaluate_cached(expr, state, traversal_state, fns) else {
      // The evaluator refused the interpolation and has already recorded why.
      // The loop's own guard reads `state.confident` on the next turn; this
      // returns rather than waiting for it, because the interpolation whose
      // text is missing is this one.
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

    strng.push_str(&text);
  }

  if !state.confident {
    return None;
  }

  Some(EvaluateResultValue::Expr(create_string_expr(&strng)))
}
