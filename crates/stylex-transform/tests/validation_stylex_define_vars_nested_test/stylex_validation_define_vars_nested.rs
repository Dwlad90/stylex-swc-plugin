use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_pass(PluginPass::test_default())
      .with_runtime_injection()
  })
}

stylex_test_panic!(
  throws_must_be_a_named_export,
  "The return value of unstable_defineVarsNested() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = stylex.unstable_defineVarsNested({});
  "#
);

stylex_test_panic!(
  invalid_export_not_bound_unbound,
  "unstable_defineVarsNested() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    stylex.unstable_defineVarsNested({});
  "#
);

stylex_test_panic!(
  throws_must_have_an_argument,
  "unstable_defineVarsNested() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested();
  "#
);

stylex_test_panic!(
  throws_must_have_exactly_1_argument,
  "unstable_defineVarsNested() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested({}, {});
  "#
);

stylex_test_panic!(
  invalid_argument_number,
  "unstable_defineVarsNested() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested(1);
  "#
);

stylex_test_panic!(
  invalid_argument_non_static,
  "Only static values are allowed inside of an unstable_defineVarsNested() call.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested(genStyles());
  "#
);

stylex_test_panic!(
  invalid_key_with_separator,
  "Key \"button.primary\" must not contain the \".\" character",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested({
      'button.primary': 'red',
    });
  "#
);
