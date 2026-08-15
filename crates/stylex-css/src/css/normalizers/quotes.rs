//! Ported normalizer 7 of 9. See `normalize_value.rs` for the ordered list.

use postcss_value_parser::{NodeKind, ValueParser};

/// Gives an empty string a double quote, so `''` and `""` hash alike.
///
/// Every other string keeps the quote character the author typed. That is the
/// whole normalizer: it does not re-quote, re-escape, or unify anything else.
pub fn normalize_quotes(ast: &mut ValueParser, _key: &str) {
  ast.walk(
    |node, _| {
      if node.kind != NodeKind::String {
        return true;
      }

      if node.value.is_empty() {
        node.quote = Some('"');
      }

      true
    },
    false,
  );
}
