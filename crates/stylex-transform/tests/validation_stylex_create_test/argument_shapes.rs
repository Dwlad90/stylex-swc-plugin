use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_unstable_module_resolution(ModuleResolution::common_js(None))
        .with_runtime_injection(),
    )
  })
}

// A single-object call takes one argument, and the compiler says how many it
// was given rather than reading the first and ignoring the rest.
stylex_test_panic!(
  keyframes_takes_one_argument,
  "keyframes() should have 1 argument",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fade = stylex.keyframes({ from: { opacity: 0 } }, {});
  "#
);

stylex_test_panic!(
  position_try_takes_one_argument,
  "positionTry() should have 1 argument",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fallback = stylex.positionTry({ top: 0 }, {});
  "#
);

stylex_test_panic!(
  view_transition_class_takes_one_argument,
  "viewTransitionClass() should have 1 argument",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const cls = stylex.viewTransitionClass({ group: {} }, {});
  "#
);

// The result has to be bound to a variable the compiler can name, because the
// name is what the styles are keyed by.
stylex_test_panic!(
  a_keyframes_call_below_the_top_level_is_not_bound,
  "keyframes() calls must be bound to a bare variable",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function make() {
      const fade = stylex.keyframes({ from: { opacity: 0 } });
      return fade;
    }
  "#
);

// `defaultMarker()` names the marker class and takes nothing.
stylex_test_panic!(
  default_marker_takes_no_argument,
  "defaultMarker() should have 1 argument",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const marker = stylex.props(stylex.defaultMarker(1));
  "#
);

// The second argument of a theme is the override object, or a name that stands
// for one. Anything else spells no overrides.
stylex_test_panic!(
  a_theme_override_that_is_not_an_object_is_refused,
  "createTheme() can only accept an object as the second argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const theme = stylex.createTheme({ __varGroupHash__: 'x568ih9' }, makeOverrides());
  "#
);

// A theme read off its own result is not bound to a bare variable: the name
// holds the read, not the theme.
stylex_test_panic!(
  a_theme_read_off_its_own_result_is_not_bound,
  "createTheme() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const theme = stylex.createTheme({ __varGroupHash__: 'x568ih9' }, {}).name;
  "#
);

// A declaration written as absent declares nothing, and the property survives
// carrying that absence so a later declaration of it is unset.
stylex_test!(
  a_declaration_written_as_absent_is_kept_unset,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: null, margin: { default: '1px', ':hover': '2px' } },
    });
  "#
);

// A value of a kind no declaration can carry is refused rather than written out
// as something the browser cannot read.
stylex_test_panic!(
  a_value_of_a_kind_no_declaration_carries_is_refused,
  "root > color > Unsupported expression: NewExpression",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ root: { color: new Colour() } });
  "#
);

// A theme overrides a variable group. A first argument that is not one names no
// group to override.
stylex_test_panic!(
  a_theme_target_that_is_not_a_group_is_refused,
  "Can only override variables theme created with defineVars().",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const theme = stylex.createTheme(1, {});
  "#
);

// A `create` call that is bound to nothing is refused, and the search for a
// binding reads past every top-level expression that is not one.
stylex_test_panic!(
  a_create_call_bound_to_nothing_is_refused,
  "create() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const other = {};
    other.name;
    stylex.create({ root: { color: 'red' } });
  "#
);
