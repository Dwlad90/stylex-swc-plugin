//! Ported normalizer 9 of 9. See `normalize_value.rs` for the ordered list.

use postcss_value_parser::{NodeKind, ValueParser, unit};
use stylex_constants::constants::common::ROOT_FONT_SIZE;
use stylex_utils::number::{parse_js_float, to_js_string};

/// Restates a font size given in pixels as a multiple of the root font size.
///
/// Appended to the list only when the option asking for it is on, and applied
/// only to `fontSize`. It runs last, after the leading zero has already been
/// stripped from everything else, which is why `8px` comes out as `0.5rem`
/// rather than `.5rem`: nothing runs after it to take that zero off.
pub fn convert_font_size_to_rem(ast: &mut ValueParser, key: &str) {
  if key != "fontSize" {
    return;
  }

  ast.walk(
    |node, _| {
      if node.kind != NodeKind::Word {
        return true;
      }

      let dimension = match unit(&node.value) {
        Some(dimension) => dimension,
        None => return true,
      };

      if dimension.unit != "px" {
        return true;
      }

      match parse_js_float(&dimension.number) {
        Some(number) => {
          node.value = format!("{}rem", to_js_string(number / f64::from(ROOT_FONT_SIZE)));
        },
        // `unit` splits on a leading number, so the number half always reads
        // back. Reported rather than unwrapped, per the crate rules.
        None => return true,
      }

      true
    },
    false,
  );
}
