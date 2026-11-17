//! Removes or canonicalizes the unit on a zero value. See `normalize_value.rs`
//! for where this pass runs among the others.

use postcss_value_parser::{NodeKind, ValueParser, unit};

const ANGLES: [&str; 4] = ["deg", "grad", "turn", "rad"];
const TIMINGS: [&str; 2] = ["ms", "s"];
const FRACTION: &str = "fr";
const PERCENTAGE: &str = "%";

/// Canonicalizes a zero written with a unit: every zero angle becomes `0deg`,
/// every zero duration `0s`, and a zero with any other unit loses its unit
/// entirely — but only outside a function, where dropping the unit would change
/// what the function computes.
///
/// Whether a token is inside a function is decided by comparing source offsets
/// against the furthest function end seen so far. The walk visits a function
/// before its children, so each function widens the window before its own
/// zeros are read. The window closes once the walk moves past the last
/// function end, so a bare zero after every function still loses its unit.
///
/// A custom property is exempt outright. Its value has no grammar the compiler
/// can reason about, so `0px` there may well be read back as text by something
/// that needs the unit.
pub fn normalize_zero_dimensions(ast: &mut ValueParser, key: &str) {
  if key.starts_with("--") {
    return;
  }

  let mut end_function = 0;

  ast.walk(
    |node, _| {
      if node.kind == NodeKind::Function {
        end_function = end_function.max(node.source_end_index);
      }

      if end_function > 0 && node.source_index > end_function {
        end_function = 0;
      }

      if node.kind != NodeKind::Word {
        return true;
      }

      let dimension = match unit(&node.value) {
        Some(dimension) => dimension,
        None => return true,
      };

      if dimension.number != "0" {
        return true;
      }

      if ANGLES.contains(&dimension.unit) {
        node.value = "0deg".to_owned();
      } else if TIMINGS.contains(&dimension.unit) {
        node.value = "0s".to_owned();
      } else if dimension.unit == FRACTION {
        node.value = "0fr".to_owned();
      } else if dimension.unit == PERCENTAGE {
        node.value = "0%".to_owned();
      } else if end_function == 0 {
        node.value = "0".to_owned();
      }

      true
    },
    false,
  );
}
