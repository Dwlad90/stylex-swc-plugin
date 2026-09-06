//! `String(x)` around a theme override folds at compile time.
//!
//! The token group is imported from a module that genuinely resolves — a
//! package under `tests/__virtual__/app/node_modules` — so the override is
//! matched against real token names rather than against a failed import.
//!
//! The expected class names and rule text are measured output of
//! `@stylexjs/babel-plugin@0.19.0` for the same file layout.

use crate::utils::prelude::*;
use swc_core::common::FileName;

fn virtual_app_path(rel: &str) -> String {
  format!(
    "{}/tests/__virtual__/app/{}",
    env!("CARGO_MANIFEST_DIR"),
    rel
  )
}

fn stylex_transform(comments: TestComments, rel_filename: &'static str) -> impl Pass {
  let filename = virtual_app_path(rel_filename);
  let root_dir = virtual_app_path("");

  build_test_transform(comments, move |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real(filename.clone().into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(root_dir)))
  })
}

stylex_test!(
  theme_override_wrapped_in_string,
  |tr| stylex_transform(tr.comments.clone(), "src/themes/light.stylex.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const lightTheme = stylex.createTheme(colors, {
      primary: String('#fff'),
    });
  "#
);

// A coerced token *reference* keeps the `var(…)` it resolves to, because that
// reference is already a string by the time the coercion sees it.
stylex_test!(
  theme_override_wrapped_in_string_around_a_token_reference,
  |tr| stylex_transform(tr.comments.clone(), "src/themes/dark.stylex.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const darkTheme = stylex.createTheme(colors, {
      primary: String(colors.surface),
    });
  "#
);

// The token group itself is an object carrying its own `toString`, which
// answers the variable group hash rather than the object default.
stylex_test!(
  create_with_a_coerced_token_group,
  |tr| stylex_transform(tr.comments.clone(), "src/components/Card.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      root: { color: String(colors) },
      reference: { color: String(colors.primary) },
    });
  "#
);

// An object argument is returned unchanged, so a coerced override emits what
// the bare one does: `.x1dd033s, .x1dd033s:root{--x17y9eti:#fff;}`.
stylex_test!(
  theme_override_wrapped_in_object,
  |tr| stylex_transform(tr.comments.clone(), "src/themes/light.stylex.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const lightTheme = stylex.createTheme(colors, {
      primary: Object({ default: '#fff' }),
    });
  "#
);

// The token group is an object, so it is returned unchanged: its own
// `toString` still answers the variable group hash, and a member of the
// coerced group still resolves to the `var(…)` it names.
stylex_test!(
  create_with_a_token_group_wrapped_in_object,
  |tr| stylex_transform(tr.comments.clone(), "src/components/Card.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      root: { color: String(Object(colors)) },
      reference: { color: Object(colors).primary },
    });
  "#
);

// --- The shapes a coerced token group has to survive ------------------------
//
// The conversions above are the plain positions. These pin the ones that
// compose, because a token group crossing back unchanged is what every one of
// them rests on, and a narrowing of that would show here first.

// Wrapping the group over and over changes nothing: each `Object()` hands back
// what it was given, so the member read at the end still resolves to its
// `var(…)`. Measured against upstream, which folds it the same way.
stylex_test!(
  a_repeatedly_wrapped_token_group_still_reads_its_member,
  |tr| stylex_transform(tr.comments.clone(), "src/components/Card.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      root: { color: Object(Object(Object(colors))).primary },
    });
  "#
);

// A group and one of its members inside one array, so the join renders each
// through its own `toString` rather than through the array's: the group answers
// the variable-group hash and the member the `var(…)` it names.
stylex_test!(
  an_array_of_a_token_group_and_its_member_joins_both,
  |tr| stylex_transform(tr.comments.clone(), "src/components/Card.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      root: { color: String([colors, colors.primary]) },
    });
  "#
);

// Surplus arguments are ignored and a number conversion answers `NaN` rather
// than refusing — both the language's answers, and both upstream's.
stylex_test!(
  a_coerced_token_group_reads_only_its_first_argument,
  |tr| stylex_transform(tr.comments.clone(), "src/components/Card.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      surplus: { color: String(colors, 1) },
      notANumber: { color: String(Number(colors)) },
    });
  "#
);

// A group inside an array the conversion is applied to, which is the one shape
// where the value the bridge cannot carry is nested rather than the argument
// itself. The array's own `ToString` joins it, so the group still answers its
// hash.
stylex_test!(
  a_token_group_nested_in_a_converted_array,
  |tr| stylex_transform(tr.comments.clone(), "src/components/Card.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      root: { color: String(Array(colors)) },
    });
  "#
);
