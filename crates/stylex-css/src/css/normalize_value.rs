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
//! [`detect_unprefixed_custom_properties`] sits third: after the detectors, so
//! that `var(foo` is reported as the unfinished function it is rather than as a
//! missing prefix, and before every rewrite, so that the name it reads is the
//! name the author typed.
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
  unprefixed_custom_properties::detect_unprefixed_custom_properties,
  whitespace::normalize_whitespace, zero_dimensions::normalize_zero_dimensions,
};

/// One pass over the token list, rewriting it in place — or rejecting it — for
/// the property its second argument names.
///
/// Four of the ten read it — three to decide whether they apply at all, one
/// to name the declaration in a rejection. It is passed to all ten anyway,
/// which is what lets the fold below be a list rather than ten call sites.
type Normalizer = fn(&mut ValueParser, &str);

/// The nine passes that always run, in the order they run in.
const NORMALIZERS: [Normalizer; 9] = [
  detect_unclosed_fns,
  detect_unclosed_strings,
  detect_unprefixed_custom_properties,
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
/// function, an unclosed string, an unprefixed custom-property reference, and
/// on a value that scans to no tokens at all.
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
