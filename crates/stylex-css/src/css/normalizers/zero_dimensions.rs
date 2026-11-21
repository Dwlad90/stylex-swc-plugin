//! Removes or canonicalizes the unit on a zero value. See `normalize_value.rs`
//! for where this pass runs among the others.

use postcss_value_parser::{NodeKind, ValueParser, unit};

use super::is_url_function;

const ANGLES: [&str; 4] = ["deg", "grad", "turn", "rad"];
const TIMINGS: [&str; 2] = ["ms", "s"];
const FRACTION: &str = "fr";
const PERCENTAGE: &str = "%";

/// Canonicalizes a zero written with a unit: every zero angle becomes `0deg`
/// and every zero duration `0s`. A zero flex value or percentage keeps its
/// unit. A zero with any other unit loses its unit — but only outside a
/// function, where dropping the unit would change what the function computes.
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
///
/// The contents of a `url()` are never touched.
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

      // After the window update, so a `url()` widens the window like any other
      // function. Its contents are skipped, so this changes no output.
      if is_url_function(node) {
        return false;
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

      // CSS units are ASCII case-insensitive: `0MS` is a time and `0FR` a flex
      // value, and dropping their unit would change what the value means.
      if is_any_unit(dimension.unit, &ANGLES) {
        node.value = "0deg".to_owned();
      } else if is_any_unit(dimension.unit, &TIMINGS) {
        node.value = "0s".to_owned();
      } else if dimension.unit.eq_ignore_ascii_case(FRACTION) {
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

/// Whether `unit` is one of `names`, ignoring ASCII case.
fn is_any_unit(unit: &str, names: &[&str]) -> bool {
  names.iter().any(|name| unit.eq_ignore_ascii_case(name))
}
