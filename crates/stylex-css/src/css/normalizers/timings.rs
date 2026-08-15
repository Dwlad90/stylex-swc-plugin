//! Ported normalizer 4 of 9. See `normalize_value.rs` for the ordered list.

use postcss_value_parser::{NodeKind, ValueParser, unit};
use stylex_utils::number::{parse_js_float, to_js_string};

/// Rewrites a duration of ten milliseconds or more as seconds.
///
/// Runs before the leading-zero normalizer, and the order is load-bearing:
/// `100ms` becomes `0.1s` here and `.1s` there. Swapping them leaves the
/// leading zero in place and changes the class name.
///
/// Anything under ten milliseconds is left alone, so `9ms` stays `9ms` — the
/// conversion would spell it `.009s`, which is longer than what it replaced.
pub fn normalize_timings(ast: &mut ValueParser, _key: &str) {
  ast.walk(
    |node, _| {
      if node.kind != NodeKind::Word {
        return true;
      }

      let value = match parse_js_float(&node.value) {
        Some(value) => value,
        None => return true,
      };

      let dimension = match unit(&node.value) {
        Some(dimension) => dimension,
        None => return true,
      };

      if dimension.unit != "ms" || value < 10.0 {
        return true;
      }

      node.value = format!("{}s", to_js_string(value / 1000.0));

      true
    },
    false,
  );
}
