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
