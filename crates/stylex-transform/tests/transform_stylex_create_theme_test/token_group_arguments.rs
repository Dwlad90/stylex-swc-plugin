//! A `defineVars` group handed to a function the author wrote.
//!
//! A token group has no expression form, so it used to refuse every call it was
//! an argument to — even one whose body never looks at it. It binds through the
//! same factory a module's own token import binds through, so a parameter
//! holding one answers a member read exactly as the imported name does.
//!
//! An argument with no form binds nothing and leaves the parameter unbound, so
//! the fold is decided by whether the body needed it. Which of the two refusals
//! a body that answered nothing gets is decided by `binds_a_parameter`, whose own
//! cases are unit tests. What is measured here is the answer a build gets.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0` with
//! the same options and the same file layout.

use crate::utils::prelude::*;
use swc_core::common::FileName;

fn virtual_app_path(rel: &str) -> String {
  format!(
    "{}/tests/__virtual__/app/{}",
    env!("CARGO_MANIFEST_DIR"),
    rel
  )
}

fn stylex_transform(comments: TestComments) -> impl Pass {
  let filename = virtual_app_path("src/components/Card.js");
  let root_dir = virtual_app_path("");

  build_test_transform(comments, move |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real(filename.clone().into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(root_dir)))
  })
}

// A body that never reads the argument. Nothing about the group is asked, and
// the only thing that used to stop the fold was the argument's own form.
stylex_test!(
  a_function_ignoring_a_token_group_folds,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    const paint = () => 'red';
    export const styles = stylex.create({
      root: { color: paint(colors) },
    });
  "#
);

// A name the module bound over a global. The call is the author's own function
// and is called as one, so `String` here is not the conversion — which is the
// distinction `unshadowed_global` draws, measured on an argument that used to
// refuse before either side of it could be reached.
stylex_test!(
  a_shadowed_global_called_with_a_token_group_folds,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    const String = () => 'red';
    export const styles = stylex.create({
      root: { color: String(colors) },
    });
  "#
);

// The parameter read as the group it holds: a member off it, and the group
// coerced whole. `var(--x17y9eti)` and the variable-group hash `x13pcrg7` — the
// same two answers the imported name gives outside a call.
stylex_test!(
  a_bound_token_group_answers_like_the_name_it_came_from,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    const member = (c) => c.primary;
    const coerced = (c) => String(c);
    export const styles = stylex.create({
      read: { color: member(colors) },
      converted: { color: coerced(colors) },
    });
  "#
);

// The argument in a position other than the first, and a resolved member passed
// instead of the group, so which parameter holds what is read from the call
// rather than assumed.
stylex_test!(
  a_token_group_binds_at_any_position,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    const second = (a, b) => b.primary;
    const through = (c) => c;
    export const styles = stylex.create({
      positioned: { color: second('x', colors) },
      resolved: { color: through(colors.primary) },
    });
  "#
);

// A body that hands the group straight back. The argument binds; what refuses is
// the body, because a token group is not a value a stylesheet can hold — and the
// sentence names the body rather than the argument for exactly that reason.
// Upstream refuses the same source at the same point, in its own words — `A
// style value can only contain an array, string or number.`
stylex_test_panic!(
  a_function_returning_a_token_group_is_rejected,
  "The function's body has no compile-time value.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    const paint = (c) => c;
    export const styles = stylex.create({
      root: { color: paint(colors) },
    });
  "#
);
