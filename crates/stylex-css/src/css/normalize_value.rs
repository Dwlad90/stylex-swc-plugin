//! Rewriting a declaration value into the canonical text that gets hashed.
//!
//! The value is scanned into a loose token list, a fixed list of small
//! transformations is folded over it in a fixed order, and the list is spelled
//! back out. Nothing else happens to it — which is the point. A token no
//! normalizer names survives byte for byte, so the author's hex spelling,
//! letter case, quote character and whitespace positions all reach the hash
//! exactly as written.
//!
//! ## The order is part of the behaviour
//!
//! [`normalize_timings`] runs before [`normalize_leading_zero`] so that a
//! duration converted from milliseconds to seconds is then stripped of its
//! leading zero: `100ms` becomes `0.1s` and then `.1s`. Reordering the two
//! produces `0.1s`, a different class name, and no error anywhere.
//!
//! The two detectors run first so that a value carrying an unfinished
//! construct is rejected before anything rewrites it, and
//! [`convert_font_size_to_rem`] runs last — appended only when its option is
//! on — so that the number it produces keeps its leading zero.
//!
//! ## What is not here
//!
//! No normalizer understands hex colours, letter case, quote characters or
//! exponent notation, so none of them can alter those. Read the absence as
//! deliberate: it is what makes two compilers agree on a value neither of them
//! has an opinion about.

use postcss_value_parser::ValueParser;
use stylex_structures::stylex_state_options::StyleXStateOptions;

use crate::css::normalizers::{
  convert_camel_case_values::convert_camel_cased_values, detect_unclosed_fns::detect_unclosed_fns,
  detect_unclosed_strings::detect_unclosed_strings, font_size_px_to_rem::convert_font_size_to_rem,
  leading_zero::normalize_leading_zero, quotes::normalize_quotes, timings::normalize_timings,
  whitespace::normalize_whitespace, zero_dimensions::normalize_zero_dimensions,
};

/// One transformation over the token list, named by the property it is being
/// applied for.
type Normalizer = fn(&mut ValueParser, &str);

/// The eight normalizers that always run, in the order they run in.
const NORMALIZERS: [Normalizer; 8] = [
  detect_unclosed_fns,
  detect_unclosed_strings,
  normalize_whitespace,
  normalize_timings,
  normalize_zero_dimensions,
  normalize_leading_zero,
  normalize_quotes,
  convert_camel_cased_values,
];

/// Normalizes `value`, declared for property `key`.
///
/// Fails — as a panic the compiler catches and reports — on an unclosed
/// function, an unclosed string, and on a value that scans to no tokens at all.
pub fn normalize_value(value: &str, key: &str, options: &StyleXStateOptions) -> String {
  let mut ast = ValueParser::new(value);

  for normalizer in NORMALIZERS {
    normalizer(&mut ast, key);
  }

  if options.enable_font_size_px_to_rem {
    convert_font_size_to_rem(&mut ast, key);
  }

  ast.to_string()
}
