//! `String(x)` around a token value folds at compile time.
//!
//! The input reported in
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1253, which failed the
//! build with `Only static values are allowed inside of a defineVars() call.`
//! The expected variable names and rule text are measured output of
//! `@stylexjs/babel-plugin@0.19.0` for the same file name and root directory.

use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        ))),
    )
  })
}

// `:root, .xop34xu{--xcb2f4a:#fff;}` — the same rule a plain `'#fff'`
// produces, because the coercion is folded before the token is named.
stylex_test!(
  token_value_wrapped_in_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      background: String('#fff'),
    });
  "#
);

// `String()` with no argument is the empty string — not `"undefined"`, which
// is what `String(undefined)` gives. Whether an empty value should emit a
// declaration at all is a separate question, tracked apart from the fold.
stylex_test!(
  token_value_from_string_with_no_arguments,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      background: String(),
    });
  "#
);

// A coercion inside a conditional token value folds at each branch.
stylex_test!(
  conditional_token_value_wrapped_in_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      background: {
        default: String('#fff'),
        '@media (prefers-color-scheme: dark)': String('#000'),
      },
    });
  "#
);

// `Number(x)` folds in the same position, through the numeric-literal grammar:
// `:root, .xop34xu{--xu6xznv:31;--x138e37c:NaN;}`. `NaN` is a value here too —
// upstream writes it into the rule rather than failing.
stylex_test!(
  token_values_wrapped_in_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      size: Number('0x1f'),
      ratio: Number('10px'),
    });
  "#
);

// An object argument is returned unchanged, so a coerced conditional token
// value emits what the bare one does: `:root, .xop34xu{--xcb2f4a:#fff;}` and
// the dark-scheme variant beside it.
stylex_test!(
  conditional_token_value_wrapped_in_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      background: Object({
        default: '#fff',
        '@media (prefers-color-scheme: dark)': '#000',
      }),
    });
  "#
);

// A primitive is a wrapper object carrying no token value at all, so it fails
// the build here as it does in `create`. Upstream reports the missing default
// the wrapper leaves behind; this compiler refuses the wrapper one step
// earlier, and `defineVars` reports its own refusal rather than the reason the
// fold recorded. What both compilers agree on is that no declaration is
// emitted.
stylex_test_panic!(
  token_value_wrapped_in_object_of_a_primitive_is_rejected,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      background: Object('#fff'),
    });
  "#
);

// A bare call to `Math` fails here for the same reason it does in `create`: the
// callee is recognised and the call is not. The refusal `create` reports
// verbatim reaches an author of a token file as this API's own wording.
stylex_test_panic!(
  token_value_from_a_bare_math_call_is_rejected,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      size: Math(1),
    });
  "#
);
