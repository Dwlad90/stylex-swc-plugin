//! Cutting a shorthand value into the parts its expansion hands out.
//!
//! A shorthand assigns one authored value to several longhands, so something
//! has to decide where one part ends and the next begins. That decision is a
//! compatibility contract rather than a detail: each part becomes the value of
//! a generated declaration, and a declaration's text is what its class name is
//! hashed from. Cut in a different place and every longhand gets a different
//! class name — and in the worst case a value that is not a declaration at all.
//!
//! The cut is made on the [value scanner]'s node kinds, which is the same
//! answer the reference compiler reaches, and it is made there for a reason a
//! character test cannot reproduce: `/` and `:` end a part at the top level and
//! are ordinary characters inside a function, so `10px/1.5` is two parts while
//! `calc(100% / 3)` is one. Only a scanner that has already decided what is a
//! separator and what is inside a function can tell those apart.
//!
//! [value scanner]: postcss_value_parser

use crate::css::common::nests_too_deeply;
use postcss_value_parser::{Node, NodeKind, ValueParser};

/// The parts of `css_string`, in the order the expansion consumes them.
///
/// Whitespace and separators are structure here, not content: they decide where
/// a part ends and then contribute nothing to it. Everything else is echoed
/// with the author's own characters — an escape stays an escape, a quote keeps
/// the character it was written with, a hex colour keeps its spelling, and a
/// number keeps the digits it was typed with. Nothing here re-spells anything,
/// which is why there is no number formatting and no identifier serialization
/// in this file.
pub fn split_value_parts(css_string: &str) -> Vec<String> {
  let trimmed = js_trim(css_string);

  // Not a split at all for a value nested past the compiler's budget, and not a
  // refusal either. Scanning one builds a tree whose *destructor* recurses once
  // per level, which overruns the stack and aborts the process -- unwindable
  // panics never enter it, so no diagnostic would ever be produced. Expansion
  // runs before the guard in front of normalization, so this is the earliest
  // place the question can be asked at all.
  //
  // Handing the value on whole rather than refusing here keeps the diagnostic
  // where it is documented: normalization asks the same question of the same
  // scan and rejects, so the author gets the nesting-depth message either way.
  if nests_too_deeply(trimmed) {
    return vec![trimmed.to_string()];
  }

  let parts: Vec<String> = ValueParser::new(trimmed)
    .nodes
    .iter()
    .filter(|node| !matches!(node.kind, NodeKind::Space | NodeKind::Div))
    .map(print_node)
    .collect();

  apply_importance(parts)
}

/// Moves a trailing `!important` onto every part it qualifies.
///
/// An author writes it once and means it of the whole shorthand, so it cannot
/// stay a part of its own: as one it would become the value of whichever
/// longhand happened to be next in line, and `padding: '1px !important'` would
/// emit `padding-inline-end: important`. Re-attached to each part instead, it
/// survives on all four sides.
///
/// A lone `!important` with nothing to qualify is left alone — there is no part
/// to move it onto, and dropping it would silently discard what the author
/// wrote.
fn apply_importance(parts: Vec<String>) -> Vec<String> {
  let Some(last) = parts.last() else {
    return parts;
  };

  // The `!` test is what keeps the case fold off the common path: no character
  // lowercases *into* `!`, so a part that does not start with one cannot be an
  // importance annotation however it is spelled.
  if parts.len() < 2 || !last.starts_with('!') || last.to_lowercase() != "!important" {
    return parts;
  }

  parts[..parts.len() - 1]
    .iter()
    .map(|part| format!("{part} !important"))
    .collect()
}

/// One node as the text it contributes to its part.
///
/// Written against an explicit stack rather than recursively: the scanner's own
/// pass over the input is iterative and accepts nesting far deeper than a call
/// stack survives, and a value reaches here *before* the depth guard in front
/// of normalization has had a chance to reject anything.
fn print_node(node: &Node) -> String {
  let mut out = String::with_capacity(node.source_end_index.saturating_sub(node.source_index));
  let mut pending: Vec<Emit<'_>> = vec![Emit::Node(node)];

  while let Some(step) = pending.pop() {
    match step {
      Emit::CloseParen => out.push(')'),
      Emit::Node(node) => match (node.kind, node.quote, node.nodes.as_deref()) {
        (NodeKind::String, Some(quote), _) => {
          out.push(quote);
          out.push_str(&node.value);
          out.push(quote);
        },
        (NodeKind::Function, _, Some(arguments)) => {
          out.push_str(&node.value);
          out.push('(');
          pending.push(Emit::CloseParen);
          pending.extend(arguments.iter().rev().map(Emit::Node));
        },
        // Every other kind contributes its own text, and so does a string or a
        // function missing the field the arm above reads. The fallback is not a
        // guess: the reference implementation matches on the fields being
        // present too, and an absent one takes it down the same path.
        _ => out.push_str(&node.value),
      },
    }
  }

  out
}

/// A step of [`print_node`]'s traversal: a node to spell, or the parenthesis
/// that closes the function whose arguments were just queued.
enum Emit<'a> {
  Node(&'a Node),
  CloseParen,
}

/// `css_string` with the whitespace JavaScript's `String.prototype.trim`
/// removes taken off both ends.
///
/// Not `str::trim`, and the difference is observable twice. A byte-order mark
/// is trimmed by JavaScript and is not Unicode whitespace, so `str::trim`
/// leaves it attached to the first part; `U+0085` is Unicode whitespace and is
/// not in JavaScript's set, so `str::trim` removes one the reference compiler
/// keeps. Neither character is whitespace to the scanner, so whichever survives
/// the trim ends up inside a word and inside a class name.
fn js_trim(css_string: &str) -> &str {
  css_string.trim_matches(|character: char| {
    character == '\u{feff}' || (character.is_whitespace() && character != '\u{85}')
  })
}

#[cfg(test)]
#[path = "../tests/values_parser_tests.rs"]
mod tests;
