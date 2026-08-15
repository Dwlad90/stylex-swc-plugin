//! Ported normalizer 1 of 9. See `normalize_value.rs` for the ordered list.

use postcss_value_parser::{NodeKind, ValueParser};
use stylex_constants::constants::messages::LINT_UNCLOSED_FUNCTION;
use stylex_macros::stylex_panic;

use crate::css::common::build_error_css_rule;

/// Rejects a value carrying a function that was never closed.
///
/// Runs first so that no later normalizer gets to rewrite tokens the author
/// never finished writing.
///
/// The reference implementation raises a bare message here; the rule text is
/// appended so the report names the declaration it came from, which is what
/// this compiler's message has always carried.
pub fn detect_unclosed_fns(ast: &mut ValueParser, key: &str) {
  let mut unclosed = false;

  ast.walk(
    |node, _| {
      if node.kind == NodeKind::Function && node.unclosed {
        unclosed = true;
      }

      true
    },
    false,
  );

  if unclosed {
    let value = ast.to_string();

    stylex_panic!(
      "{}, css rule: {}",
      LINT_UNCLOSED_FUNCTION,
      build_error_css_rule(key, &value)
    );
  }
}
