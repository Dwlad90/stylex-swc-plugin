use swc_core::{common::SyntaxContext, ecma::utils::ExprCtx};

pub fn get_default_expr_ctx() -> ExprCtx {
  // NOTE: ExprCtx does not have a default constructor, so we have to manually set the fields
  ExprCtx {
    unresolved_ctxt: SyntaxContext::empty(),
    is_unresolved_ref_safe: false,
    in_strict: false,
    remaining_depth: 4,
  }
}

#[cfg(test)]
#[path = "tests/swc_test.rs"]
mod tests;
