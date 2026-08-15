//! Spelling a node list back out. See the crate documentation in `lib.rs`
//! for what it is and who holds its copyright.

use crate::{Node, NodeKind};

/// A per-node override, consulted before anything else and free to decline.
///
/// Returning `Some` replaces that node's text outright, children and
/// delimiters included; returning `None` leaves it to be spelled normally. The
/// override reaches nested nodes too, so a function inside a function can be
/// replaced without the outer one knowing.
pub type Custom<'call> = &'call mut dyn FnMut(&Node) -> Option<String>;

/// The override, or its absence, threaded down the recursion. Held behind a
/// second `&mut` so that each level can reborrow it for the next.
type MaybeCustom<'outer, 'call> = &'outer mut Option<Custom<'call>>;

fn node_to_string(node: &Node, custom: MaybeCustom<'_, '_>) -> String {
  if let Some(over) = custom.as_mut()
    && let Some(replacement) = over(node)
  {
    return replacement;
  }

  match node.kind {
    NodeKind::Word | NodeKind::Space | NodeKind::UnicodeRange => node.value.clone(),
    NodeKind::String => {
      let quote = node.quote.map(String::from).unwrap_or_default();
      let closing = match node.unclosed {
        true => "",
        false => quote.as_str(),
      };
      format!("{quote}{}{closing}", node.value)
    },
    NodeKind::Comment => {
      let closing = match node.unclosed {
        true => "",
        false => "*/",
      };
      format!("/*{}{closing}", node.value)
    },
    NodeKind::Div => {
      let before = node.before.as_deref().unwrap_or_default();
      let after = node.after.as_deref().unwrap_or_default();
      format!("{before}{}{after}", node.value)
    },
    NodeKind::Function => match node.nodes.as_ref() {
      // The JavaScript reaches its function branch through an `Array.isArray`
      // test on `nodes`, so a function that somehow lost its child list spells
      // out as its bare name. That is the same fallback.
      None => node.value.clone(),
      Some(nodes) => {
        let buf = list_to_string(nodes, custom);
        let before = node.before.as_deref().unwrap_or_default();
        let after = node.after.as_deref().unwrap_or_default();
        let closing = match node.unclosed {
          true => "",
          false => ")",
        };
        format!("{}({before}{buf}{after}{closing}", node.value)
      },
    },
  }
}

fn list_to_string(nodes: &[Node], custom: MaybeCustom<'_, '_>) -> String {
  let mut result = String::new();

  for node in nodes {
    result.push_str(&node_to_string(node, custom));
  }

  result
}

/// Spells a node list back out, in order.
pub fn stringify(nodes: &[Node]) -> String {
  list_to_string(nodes, &mut None)
}

/// Spells one node back out.
///
/// A node whose delimiter was never closed is emitted without a closing
/// delimiter, so an unclosed construct survives the round trip as it was
/// written rather than acquiring a terminator the author did not type.
///
/// The kind decides everything: a node whose kind says word spells out as its
/// value even if it is still carrying the children and parentheses of the
/// function it used to be.
pub fn stringify_node(node: &Node) -> String {
  node_to_string(node, &mut None)
}

/// [`stringify`], with an override consulted for every node it reaches.
pub fn stringify_with(nodes: &[Node], custom: Custom<'_>) -> String {
  list_to_string(nodes, &mut Some(custom))
}

/// [`stringify_node`], with an override consulted for that node and every node
/// beneath it.
pub fn stringify_node_with(node: &Node, custom: Custom<'_>) -> String {
  node_to_string(node, &mut Some(custom))
}
