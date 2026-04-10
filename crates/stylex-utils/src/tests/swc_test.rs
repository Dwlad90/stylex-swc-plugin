use crate::swc::get_default_expr_ctx;

// ── get_default_expr_ctx ───────────────────────────────────────────

#[test]
fn returns_expr_ctx() {
  let ctx = get_default_expr_ctx();
  // Smoke test: should construct without panicking and have expected defaults
  assert!(!ctx.is_unresolved_ref_safe);
}

#[test]
fn unresolved_ctxt_is_empty() {
  let ctx = get_default_expr_ctx();
  // The context should use SyntaxContext::empty() (no marks applied)
  assert_eq!(ctx.unresolved_ctxt.as_u32(), 0);
}

#[test]
fn is_unresolved_ref_safe_is_false() {
  let ctx = get_default_expr_ctx();
  assert!(!ctx.is_unresolved_ref_safe);
}

#[test]
fn in_strict_is_false() {
  let ctx = get_default_expr_ctx();
  assert!(!ctx.in_strict);
}

#[test]
fn remaining_depth_is_four() {
  let ctx = get_default_expr_ctx();
  assert_eq!(ctx.remaining_depth, 4);
}

#[test]
fn two_calls_produce_identical_contexts() {
  let ctx1 = get_default_expr_ctx();
  let ctx2 = get_default_expr_ctx();

  assert_eq!(ctx1.unresolved_ctxt, ctx2.unresolved_ctxt);
  assert_eq!(ctx1.is_unresolved_ref_safe, ctx2.is_unresolved_ref_safe);
  assert_eq!(ctx1.in_strict, ctx2.in_strict);
  assert_eq!(ctx1.remaining_depth, ctx2.remaining_depth);
}
