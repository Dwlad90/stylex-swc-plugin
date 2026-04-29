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

    if let Some(expr) = exprs.get(i)
      && let Some(evaluated_expr) = evaluate_cached(expr, state, traversal_state, fns)
      && let Some(lit_str) = evaluated_expr
        .as_expr()
        .and_then(|expr| expr.as_lit())
        .and_then(convert_lit_to_string)
    {
      strng.push_str(&lit_str);
    }
  }

  if !state.confident {
    return None;
  }

  Some(EvaluateResultValue::Expr(create_string_expr(&strng)))
}
