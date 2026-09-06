use crate::utils::prelude::*;
use rustc_hash::FxHashMap;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_pass(PluginPass::test_default())
        .with_runtime_injection(),
    )
  })
}

stylex_test_panic!(
  local_variable_keyframes_object,
  "keyframes() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const keyframes = {
      from: {
        color: 'red',
      },
      to: {
        color: 'blue',
      }
    };
    export const name = stylex.keyframes(keyframes);
  "#
);

stylex_test_panic!(
  only_argument_must_be_an_object_of_objects_null,
  "keyframes() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const name = stylex.keyframes(null);
  "#
);

stylex_test_panic!(
  only_argument_must_be_an_object_of_objects_false,
  "Every frame within a keyframes() call must be an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const name = stylex.keyframes({
      from: false
    });
  "#
);

stylex_test!(
  only_argument_must_be_an_object_of_objects_valid_percentage,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const name = stylex.keyframes({
      '0%': {
        opacity: 0
      },
      '50%': {
        opacity: 0.5
      },
    });
  "#
);

stylex_test!(
  only_argument_must_be_an_object_of_objects_valid_from_to,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const name = stylex.keyframes({
      from: {},
      to: {},
    });
  "#
);

stylex_test!(
  allow_defined_css_variables_in_keyframes,
  |tr| {
    let mut defined_stylex_css_variables = FxHashMap::default();
    defined_stylex_css_variables.insert("bar".to_string(), "1".to_string());
    stylex_transform(tr.comments.clone(), |b| {
      b.with_defined_stylex_css_variables(defined_stylex_css_variables)
        .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    })
  },
  r#"
    import stylex from 'stylex';
    const styles = stylex.keyframes({
      from: {
        backgroundColor: 'var(--bar)',
      },
    });
  "#
);

stylex_test!(
  allow_undefined_css_variables_in_keyframes,
  |tr| {
    let mut defined_stylex_css_variables = FxHashMap::default();
    defined_stylex_css_variables.insert("bar".to_string(), "1".to_string());
    stylex_transform(tr.comments.clone(), |b| {
      b.with_defined_stylex_css_variables(defined_stylex_css_variables)
        .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    })
  },
  r#"
    import stylex from 'stylex';
    const styles = stylex.keyframes({
      from: {
        backgroundColor: 'var(--foobar)',
      },
    });
  "#
);

// A step value the compiler cannot resolve is still an error, not a dropped
// declaration. A step value that is merely *not a string* -- an object, an
// array, `null` -- declares nothing and compiles, so this pins the line between
// the two: a name with no binding to read is not the same as a value with
// nothing to say, and reading one as the other would swallow a typo.
stylex_test_panic!(
  unresolvable_step_value_is_rejected,
  "Only static values are allowed inside of a keyframes() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const name = stylex.keyframes({
      from: { color: someUndeclaredBinding },
      to: { color: 'blue' },
    });
  "#
);

// A theme reference read as a step value. The reference implementation drops the
// declaration and emits `@keyframes …{from{}to{z-index:1px;}}` -- the step
// survives, empty, and nothing says why. Refused here instead: a `defineVars`
// group carries no value of its own, and every other shape the evaluator cannot
// write down is refused in this position already. A decided divergence, recorded
// as `modules-1266-theme-reference-in-a-keyframes-step`.
stylex_test_panic!(
  a_theme_reference_read_as_a_step_value_is_refused,
  "Only static values are allowed inside of a keyframes() call.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const name = stylex.keyframes({
      from: { zIndex: zIndex },
      to: { zIndex: 1 },
    });
  "#
);

// A folded function map read as a step value. The reference implementation folds
// the same reference to a plain object, drops the declaration and emits
// `@keyframes …{from{}}` -- the step survives, empty, and nothing says why.
// Refused here, for the reason a theme reference in this position is: a decided
// divergence, recorded as
// `modules-1266-a-folded-function-map-in-a-keyframes-step`.
stylex_test_panic!(
  a_folded_function_map_read_as_a_step_value_is_refused,
  "Only static values are allowed inside of a keyframes() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const name = stylex.keyframes({ from: { height: stylex } });
  "#
);

// The member read beside it, which is what a step value carrying a theme is
// meant to be, and agrees with the reference implementation on the rule text.
stylex_test!(
  a_member_read_off_a_theme_import_is_a_valid_step_value,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const name = stylex.keyframes({
      from: { zIndex: zIndex.ten },
      to: { zIndex: zIndex.twenty },
    });
  "#
);
