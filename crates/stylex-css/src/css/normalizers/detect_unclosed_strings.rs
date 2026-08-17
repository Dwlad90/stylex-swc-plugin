//! Ported normalizer 2 of 9 in upstream's sequence, which is what that count
//! names. See `normalize_value.rs` for the order the passes run in here — a
//! tenth pass that upstream has no equivalent for runs among them.

use postcss_value_parser::{NodeKind, ValueParser, stringify};
use stylex_constants::constants::messages::LINT_UNCLOSED_STRING;
use stylex_macros::stylex_panic;

use crate::css::common::build_error_css_rule;

/// Rejects a value carrying a string that was never closed.
///
/// The scanner invents the missing closing quote rather than failing, so this
/// is the only thing standing between an unterminated string and a declaration
/// that swallows whatever followed it.
///
/// Rejects with the same payload the other two rejecting passes attach, so all
/// three speak one diagnostic shape. The value is spelled back out for it,
/// which is why the key is read here where the sibling passes read it too.
pub fn detect_unclosed_strings(ast: &mut ValueParser, key: &str) {
  let mut unclosed = false;

  ast.walk(
    |node, _| {
      if node.kind == NodeKind::String && node.unclosed {
        unclosed = true;
      }

      true
    },
    false,
  );

  if unclosed {
    let value = stringify(&ast.nodes);

    stylex_panic!(
      "{}, css rule: {}",
      LINT_UNCLOSED_STRING,
      build_error_css_rule(key, &value)
    );
  }
}
