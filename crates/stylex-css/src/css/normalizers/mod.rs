use postcss_value_parser::{Node, NodeKind, ValueParser, stringify};
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

/// Whether a node is a `url()` call, in any letter case.
///
/// The contents of a `url()` are a resource path, not a value: `url(0.5x.png)`
/// names a file that `.5x.png` does not. The scanner keeps the contents of a
/// lowercase `url(` as one word but splits those of `URL(` into words and
/// separators, so the passes that rewrite words or spacing must not enter it.
///
/// An ASCII comparison is enough, because no other character lowercases to
/// `u`, `r` or `l`.
fn is_url_function(node: &Node) -> bool {
  node.kind == NodeKind::Function && node.value.eq_ignore_ascii_case("url")
}

/// Rejects the value under normalization, quoting the rule it would have built.
///
/// Shared by every pass that rejects a value: the author needs the rule text
/// back whatever the complaint was, and the value has to be spelled out of the
/// token list to produce it. Written once so the passes cannot drift into
/// quoting the same rule three ways.
///
/// `message` is the whole complaint and may carry detail of its own — the
/// unprefixed-property pass names the offending reference in it, because a
/// value can hold several and the rule text alone would leave the author to
/// find which. What this adds is the rule, which is what says *where*.
///
/// `#[track_caller]` so the location in the diagnostic is the pass that
/// rejected, not this line. The attribute chains: `stylex_panic!` expands to a
/// `#[track_caller]` call, so the location it reads is this function's caller
/// rather than the macro's line here.
#[track_caller]
pub(super) fn reject_value(ast: &ValueParser, key: &str, message: &str) -> ! {
  let value = stringify(&ast.nodes);

  stylex_panic!(
    "{}, css rule: {}",
    message,
    build_error_css_rule(key, &value)
  )
}
