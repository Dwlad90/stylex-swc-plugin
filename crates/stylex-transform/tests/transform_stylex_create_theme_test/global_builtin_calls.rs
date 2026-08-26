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

// A token reference is this compiler's own value rather than a JavaScript one,
// so it has no form the bridge carries into the engine and the coercion refuses.
// Upstream folds it to the `var(…)` the reference resolves to; a written
// divergence, in the safe direction — a refused build never names a class the
// other build does not define. A token reference used *without* a coercion is
// untouched, which is what `create_theme.rs` pins.
stylex_test_panic!(
  theme_override_wrapped_in_string_around_a_token_reference_is_rejected,
  "Only static values are allowed inside of a createTheme() call.",
  |tr| stylex_transform(tr.comments.clone(), "src/themes/dark.stylex.js"),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const darkTheme = stylex.createTheme(colors, {
      primary: String(colors.surface),
    });
  "#
);

// The token group and its members are this compiler's own values, so neither
// crosses into the engine and both coercions refuse. Upstream folds the group to
// its variable-group hash and a member to its `var(…)`; a written divergence,
// for the reason above.
stylex_test_panic!(
  create_with_a_coerced_token_group_is_rejected,
  "Only static values can be passed to String().",
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

// The identity is no different: an object crossing back from the engine is a
// plain object literal, so a token group could not survive one even if it
// crossed inward. Refused on the way in, for the reason above.
stylex_test_panic!(
  create_with_a_token_group_wrapped_in_object_is_rejected,
  "Only static values can be passed to Object().",
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
