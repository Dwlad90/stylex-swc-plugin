use postcss_value_parser::{ValueParser, stringify};
use stylex_macros::stylex_panic;

use crate::css::common::build_error_css_rule;

pub mod convert_camel_case_values;
pub mod detect_unclosed_fns;
pub mod detect_unclosed_strings;
mod dimensions;
pub mod font_size_px_to_rem;
pub mod leading_zero;
pub mod quotes;
pub mod timings;
pub mod unprefixed_custom_properties;
pub mod whitespace;
pub mod zero_dimensions;

/// Rejects the value under normalization, quoting the rule it would have built.
///
/// Shared by the two unclosed detectors, which reject for different reasons but
/// have nothing else to say about them: the author needs the rule text back, and
/// the value has to be spelled out of the token list to produce it. Written once
/// so the two cannot drift into reporting the same kind of problem two ways.
///
/// The unprefixed-property pass deliberately does not use this. It quotes the
/// offending `var()` name instead of the whole rule, because the value is
/// otherwise fine and the name is the entire complaint.
///
/// `#[track_caller]` so the location in the diagnostic is the pass that
/// rejected, not this line.
#[track_caller]
pub(super) fn reject_value(ast: &ValueParser, key: &str, message: &str) -> ! {
  let value = stringify(&ast.nodes);

  stylex_panic!(
    "{}, css rule: {}",
    message,
    build_error_css_rule(key, &value)
  )
}
