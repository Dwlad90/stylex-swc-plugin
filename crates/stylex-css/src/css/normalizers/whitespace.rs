//! Ported normalizer 3 of 9. See `normalize_value.rs` for the ordered list.

use postcss_value_parser::{Node, NodeKind, ValueParser};
use stylex_constants::constants::messages::{LINT_IMPORTANT_NOT_LAST, LINT_VALUE_HAS_NO_TOKENS};
use stylex_macros::stylex_panic;

/// The importance annotation, which the scanner hands over as one word token.
const IMPORTANT: &str = "!important";

/// Collapses every run of whitespace to a single space and fixes the spacing
/// around separators and function parentheses.
///
/// Spaces are rewritten in place rather than moved: a space between two words
/// stays between those two words, and the only node this ever removes is the
/// one before an importance annotation. That is what keeps a value the author
/// spelled unusually from being re-spelled.
pub fn normalize_whitespace(ast: &mut ValueParser, _key: &str) {
  trim_edges(&mut ast.nodes);

  // Planned against the untouched list before anything is rewritten, because
  // whether the reference implementation survives this normalizer at all is
  // decided by what its `!important` handler does to the list it is walking.
  let removals = match plan_important_removals(&ast.nodes) {
    ImportantPlan::Remove(removals) => removals,
    ImportantPlan::Overrun => stylex_panic!("{}", LINT_IMPORTANT_NOT_LAST),
  };

  ast.walk(
    |node, _| {
      match node.kind {
        NodeKind::Space => node.value = " ".to_owned(),
        NodeKind::Div => {
          let padding = match node.value.as_str() {
            "," => "",
            _ => " ",
          };

          node.before = Some(padding.to_owned());
          node.after = Some(padding.to_owned());
        },
        NodeKind::Function => {
          node.before = Some(String::new());
          node.after = Some(String::new());
        },
        _ => {},
      }

      true
    },
    false,
  );

  for index in removals {
    ast.nodes.remove(index);
  }
}

/// Drops a leading and a trailing space node.
///
/// The reference implementation reads the first and last elements without
/// guarding, so a value that scans to no tokens at all — an empty string, or
/// one that is nothing but whitespace — fails here. That failure is reproduced;
/// only the wording is local, since nothing depends on the text of a JavaScript
/// runtime error.
fn trim_edges(nodes: &mut Vec<Node>) {
  match nodes.first() {
    Some(first) if first.kind == NodeKind::Space => {
      nodes.remove(0);
    },
    Some(_) => {},
    None => stylex_panic!("{}", LINT_VALUE_HAS_NO_TOKENS),
  }

  match nodes.last() {
    Some(last) if last.kind == NodeKind::Space => {
      nodes.pop();
    },
    Some(_) => {},
    None => stylex_panic!("{}", LINT_VALUE_HAS_NO_TOKENS),
  }
}

/// What the reference implementation's walk does to the top-level list when it
/// meets an importance annotation.
enum ImportantPlan {
  /// Top-level indices to remove, in the order they are removed. Empty for
  /// every value that carries no importance annotation, which is nearly all of
  /// them.
  Remove(Vec<usize>),
  /// The walk read past the end of the list it had already shortened.
  Overrun,
}

/// Works out which top-level nodes the importance handler removes, and whether
/// the walk survives having done so.
///
/// Three details of the original are load-bearing here, and none of them is an
/// accident this port gets to correct — each one moves bytes into the class-name
/// hash or decides whether a value compiles at all.
///
/// The index the handler tests is the node's index *among its own siblings*,
/// but the list it tests and removes from is always the top-level one. An
/// annotation written inside a function therefore removes whichever top-level
/// node happens to sit at that index, which is not the space before it and
/// need not be a space at all.
///
/// The walk reads the list's length once, before it starts. Removing an element
/// shortens the list without shortening the walk, so every iteration still to
/// come now ends one index past the list — and reading that index is where the
/// reference implementation crashes. Only a removal during the final iteration
/// escapes it, which is why `red !important` normalizes and
/// `red !important blue` does not.
///
/// A removal is skipped, rather than crashing, when the preceding top-level
/// node is not a space — including when the annotation is the very first node
/// and there is no preceding one.
fn plan_important_removals(nodes: &[Node]) -> ImportantPlan {
  let max = nodes.len();
  let mut top_level_kinds: Vec<NodeKind> = nodes.iter().map(|node| node.kind).collect();
  let mut removals = Vec::new();

  for (index, node) in nodes.iter().enumerate() {
    visit(node, index, &mut top_level_kinds, &mut removals);

    if !removals.is_empty() && index + 1 < max {
      return ImportantPlan::Overrun;
    }
  }

  ImportantPlan::Remove(removals)
}

/// One node of the plan walk, and its children when it has any.
fn visit(
  node: &Node,
  index: usize,
  top_level_kinds: &mut Vec<NodeKind>,
  removals: &mut Vec<usize>,
) {
  if node.kind == NodeKind::Word
    && node.value == IMPORTANT
    && index > 0
    && top_level_kinds.get(index - 1) == Some(&NodeKind::Space)
  {
    removals.push(index - 1);
    top_level_kinds.remove(index - 1);
  }

  if node.kind == NodeKind::Function
    && let Some(children) = node.nodes.as_deref()
  {
    for (child_index, child) in children.iter().enumerate() {
      visit(child, child_index, top_level_kinds, removals);
    }
  }
}
