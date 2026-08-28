//! Builders for the input text of a test, where the input is too big or too
//! repetitive to write out.
//!
//! Not every test binary uses every helper here, so an item unused by
//! the one being compiled is expected rather than dead. Said once for
//! the module rather than at each item, which was the same fact
//! repeated.
#![allow(dead_code)]

/// Wrap `seed` in `depth` copies of `open` .. `close`.
///
/// `nest_expression("(", " + 1)", "x", 3)` is `"(((x + 1) + 1) + 1)"`. Used by
/// the tests that measure how deep an expression the evaluator will fold, where
/// the depth is the subject and the shape around it varies per case.
pub(crate) fn nest_expression(open: &str, close: &str, seed: &str, depth: usize) -> String {
  let mut expr = String::from(seed);

  for _ in 0..depth {
    expr = format!("{}{}{}", open, expr, close);
  }

  expr
}
