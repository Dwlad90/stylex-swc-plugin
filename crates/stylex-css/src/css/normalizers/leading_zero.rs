//! Ported normalizer 6 of 9. See `normalize_value.rs` for the ordered list.

use postcss_value_parser::{NodeKind, ValueParser, unit};
use stylex_utils::number::{parse_js_float, to_js_string};

/// Strips the zero in front of a decimal point: `0.5px` becomes `.5px`.
///
/// The number is re-spelled through JavaScript's own number-to-string rules
/// rather than copied, so `0.50px` becomes `.5px` and `1e-7px` stays in
/// exponent form. That re-spelling is why this cannot be a text substitution:
/// the bytes it produces are the bytes that get hashed.
///
/// Only a value in `0 ..= 1`, exclusive at the top, is touched, and only the
/// first `0.` in the spelling — which is the sole one a number in that range
/// can have.
pub fn normalize_leading_zero(ast: &mut ValueParser, _key: &str) {
  ast.walk(
    |node, _| {
      if node.kind != NodeKind::Word {
        return true;
      }

      let value = match parse_js_float(&node.value) {
        Some(value) => value,
        None => return true,
      };

      let dimension = unit(&node.value);

      // Left as two comparisons rather than a range check so the condition
      // still reads character for character like the one it was ported from.
      // The asymmetry it encodes -- a negative decimal keeps its zero -- is
      // easy to reintroduce as a bug and hard to spot as a difference.
      #[allow(clippy::manual_range_contains)]
      if value < 1.0 && value >= 0.0 {
        // A word whose leading number just parsed is a word `unit` splits, so
        // the absent case is the reference implementation's ternary rather than
        // a case this can reach. Written as one so it stays unreachable instead
        // of becoming a branch nothing can cover.
        let unit = dimension
          .map(|dimension| dimension.unit)
          .unwrap_or_default();

        node.value = format!("{}{}", to_js_string(value).replacen("0.", ".", 1), unit);
      }

      true
    },
    false,
  );
}
