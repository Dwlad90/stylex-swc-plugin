//! The walk two of the ported normalizers share.

use postcss_value_parser::{Dimension, Node, NodeKind, ValueParser, unit};

/// Visits every word node that splits into a number and a unit.
///
/// [`super::timings`] and [`super::font_size_px_to_rem`] open with the same
/// three steps: walk the token list, skip anything that is not a word, and
/// split the word with [`unit`]. Only what they do afterwards differs, so the
/// opening is stated here once and each of them states only its own rewrite.
///
/// Two more normalizers look like they belong and do not. Both exclusions are
/// behavioural, so read them before adding a caller:
///
/// - [`super::zero_dimensions`] tracks function boundaries across *every* node,
///   words included, so its walk has to see what this one filters out.
/// - [`super::leading_zero`] acts on words this one skips. It selects on
///   `parse_js_float`, which — unlike [`unit`] — skips leading JavaScript
///   whitespace, and the value parser does not treat every such character as a
///   space: a word can begin with U+00A0, read back as a number, and split into
///   no dimension at all. Its unit-less branch is that word, and dropping it
///   would leave the character in place and change the class name.
pub(super) fn walk_dimensions<F>(ast: &mut ValueParser, mut visit: F)
where
  F: FnMut(&mut Node, Dimension),
{
  ast.walk(
    |node, _| {
      if node.kind != NodeKind::Word {
        return true;
      }

      if let Some(dimension) = unit(&node.value) {
        visit(node, dimension);
      }

      true
    },
    false,
  );
}
