//! The values this compiler refuses.
//!
//! `class_name_edge_cases` pins byte parity with `@stylexjs/babel-plugin`;
//! these are the opposite contract, and they live apart so neither is read as
//! the other. Upstream accepts every value below. This compiler rejects each
//! one deliberately, and the line between "passed through untouched" and
//! "refused" is worth asserting rather than assuming — a value that escapes its
//! own declaration writes CSS nobody asked for, and nesting past the guard used
//! to take the process down rather than reporting a file.

use crate::utils::prelude::*;

// A semicolon followed by anything that reads as a second declaration would
// smuggle a rule past the property allowlist.
stylex_test_panic!(
  a_value_starting_a_second_declaration_is_rejected,
  "outside of a string or comment",
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: "red; margin: 10px" } });
  "#
);

// An opening brace would open a block inside the generated rule.
stylex_test_panic!(
  a_value_carrying_an_opening_brace_is_rejected,
  "outside of a string or comment",
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: "red {" } });
  "#
);

// A closing brace would end the generated rule early, leaving whatever follows
// it at the top level of the stylesheet.
stylex_test_panic!(
  a_value_carrying_a_closing_brace_is_rejected,
  "outside of a string or comment",
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { height: "1px solid } color: red" } });
  "#
);

// Nesting past the depth guard is reported as a diagnostic naming the depths.
// Before the guard existed this exhausted the stack, which is not a panic and
// therefore not catchable — the process died without reporting the file.
stylex_test_panic!(
  nesting_past_the_depth_guard_is_reported_not_fatal,
  "limit 64, found 65",
  r#"
    import stylex from 'stylex';
    const nested =
      "calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(" +
      "calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(" +
      "calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(" +
      "calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(" +
      "calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(calc(" +
      "1px" +
      ")))))))))))))" +
      ")))))))))))))" +
      ")))))))))))))" +
      ")))))))))))))" +
      ")))))))))))))";
    const styles = stylex.create({ x: { width: nested } });
  "#
);

// A custom-property reference whose name is missing its leading double hyphen
// resolves to nothing at runtime, so it is a compile-time error rather than a
// silently dead declaration.
stylex_test_panic!(
  a_custom_property_reference_without_double_hyphen_is_rejected,
  "Unprefixed custom properties",
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: "var(x)" } });
  "#
);
