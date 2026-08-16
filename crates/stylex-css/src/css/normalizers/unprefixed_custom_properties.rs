//! Local pass, not a port. See `normalize_value.rs` for the ordered list.
//!
//! The reference compiler has no equivalent: it accepts `var(foo)` and emits it
//! verbatim. This pass is kept anyway because it changes only *which programs
//! are rejected* and never the bytes of an accepted one, so it cannot move a
//! class name, and because the mistake it catches — a custom property named
//! without its leading `--` — otherwise resolves to nothing at all in a browser
//! with no diagnostic from anywhere.

use postcss_value_parser::{Node, NodeKind, ValueParser};
use stylex_constants::constants::messages::UNPREFIXED_CUSTOM_PROPERTIES;
use stylex_macros::stylex_panic;

/// The function name a custom-property reference is spelled with.
///
/// A browser reads function names case-insensitively, so `VAR(foo)` is the
/// same broken reference as `var(foo)`. This match is exact anyway, which is
/// what the rule has always done: it narrows the rule to the spelling every
/// author actually writes, and matching more widely would start rejecting
/// programs that compile today.
const CUSTOM_PROPERTY_FN: &str = "var";

/// The prefix every custom property name carries.
const CUSTOM_PROPERTY_PREFIX: &str = "--";

/// Answers whether `node` is a custom-property reference naming an unprefixed
/// property.
///
/// Reads one level: the reference's own first argument. A word there is the
/// property name — `var(--foo, red)` names `--foo`, `var(foo)` names `foo`.
/// Anything that is not a word is not a name at all (a nested `var()`, a
/// string, a comment) and is left alone, so a reference whose first argument
/// the author wrote as something else is never reported as a missing prefix.
fn names_unprefixed_property(node: &Node) -> bool {
  if node.kind != NodeKind::Function || node.value != CUSTOM_PROPERTY_FN {
    return false;
  }

  let Some(first) = node.nodes.as_ref().and_then(|nodes| nodes.first()) else {
    return false;
  };

  first.kind == NodeKind::Word && !first.value.starts_with(CUSTOM_PROPERTY_PREFIX)
}

/// Rejects a custom-property reference whose property name lacks its `--`
/// prefix.
///
/// Runs after the two unclosed detectors and before anything rewrites the
/// token list. Both halves of that placement are load-bearing. Running after
/// them means `var(foo` — unprefixed *and* unfinished — is reported as the
/// unfinished function it is, which is the more useful of the two things to say
/// and what this compiler has always said about it. Running before the
/// rewrites means the name reported is the name the author typed.
///
/// Only top-level references are inspected, matching what this rule has always
/// checked: a `var()` nested inside `calc()` or a colour function is not
/// reached. Widening the walk would reject programs that compile today, which
/// is a decision about the rule rather than about where it reads from.
pub fn detect_unprefixed_custom_properties(ast: &mut ValueParser, _key: &str) {
  if ast.nodes.iter().any(names_unprefixed_property) {
    stylex_panic!("{}", UNPREFIXED_CUSTOM_PROPERTIES);
  }
}

#[cfg(test)]
#[path = "../../tests/unprefixed_custom_properties_predicate_tests.rs"]
mod tests;
