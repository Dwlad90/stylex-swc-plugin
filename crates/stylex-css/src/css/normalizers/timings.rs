//! Ported normalizer 4 of 9 in upstream's sequence, which is what that count
//! names. See `normalize_value.rs` for the order the passes run in here — a
//! tenth pass that upstream has no equivalent for runs among them.

use postcss_value_parser::ValueParser;
use stylex_utils::number::{parse_js_float, to_js_string};

use super::dimensions::walk_dimensions;

/// Rewrites a duration of ten milliseconds or more as seconds.
///
/// Runs before the leading-zero normalizer, and the order is load-bearing:
/// `100ms` becomes `0.1s` here and `.1s` there. Swapping them leaves the
/// leading zero in place and changes the class name.
///
/// Anything under ten milliseconds is left alone, so `9ms` stays `9ms` — the
/// conversion would spell it `.009s`, which is longer than what it replaced.
pub fn normalize_timings(ast: &mut ValueParser, _key: &str) {
  walk_dimensions(ast, |word, dimension| {
    let value = parse_js_float(word)?;

    if dimension.unit != "ms" || value < 10.0 {
      return None;
    }

    Some(format!("{}s", to_js_string(value / 1000.0)))
  });
}
