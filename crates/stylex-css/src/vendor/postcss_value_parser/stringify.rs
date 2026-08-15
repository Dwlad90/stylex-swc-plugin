//! Spelling a node list back out. See the module documentation in `mod.rs`
//! for what it is and who holds its copyright.

use super::{Node, NodeKind};

/// Spells one node back out.
///
/// A node whose delimiter was never closed is emitted without a closing
/// delimiter, so an unclosed construct survives the round trip as it was
/// written rather than acquiring a terminator the author did not type.
fn stringify_node(node: &Node) -> String {
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
      // Upstream reaches its function branch through an `Array.isArray` test on
      // `nodes`, so a function that somehow lost its child list spells out as
      // its bare name. That is the same fallback.
      None => node.value.clone(),
      Some(nodes) => {
        let before = node.before.as_deref().unwrap_or_default();
        let after = node.after.as_deref().unwrap_or_default();
        let closing = match node.unclosed {
          true => "",
          false => ")",
        };
        format!(
          "{}({before}{}{after}{closing}",
          node.value,
          stringify(nodes)
        )
      },
    },
  }
}

/// Spells a node list back out, in order.
///
/// Upstream also accepts a per-node override callback. Nothing in this project
/// passes one, and an unreachable branch is worse than a named omission, so it
/// is left out.
pub fn stringify(nodes: &[Node]) -> String {
  let mut result = String::new();

  for node in nodes {
    result.push_str(&stringify_node(node));
  }

  result
}
