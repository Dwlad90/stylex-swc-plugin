use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_pass(PluginPass::test_default())
      .with_runtime_injection()
  })
}

stylex_test_panic!(
  throws_must_be_assigned_to_a_variable,
  "unstable_createThemeNested() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    stylex.unstable_createThemeNested({__varGroupHash__: 'x568ih9'}, {});
  "#
);

stylex_test_panic!(
  invalid_argument_none,
  "unstable_createThemeNested() should have 2 arguments.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const theme = stylex.unstable_createThemeNested();
  "#
);

stylex_test_panic!(
  throws_must_have_exactly_2_arguments,
  "unstable_createThemeNested() should have 2 arguments.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const theme = stylex.unstable_createThemeNested({});
  "#
);

stylex_test_panic!(
  invalid_first_argument_non_static,
  "Only static values are allowed inside of an unstable_createThemeNested() call.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const theme = stylex.unstable_createThemeNested(genStyles(), {});
  "#
);

stylex_test_panic!(
  throws_first_arg_must_have_var_group_hash,
  "Can only override variables theme created with unstable_defineVarsNested().",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const theme = stylex.unstable_createThemeNested({}, {});
  "#
);

stylex_test_panic!(
  invalid_second_argument_number,
  "unstable_createThemeNested() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = { color: 'var(--color)', __varGroupHash__: 'x568ih9' };
    const theme = stylex.unstable_createThemeNested(vars, 1);
  "#
);

stylex_test_panic!(
  invalid_override_key_with_separator,
  "Key \"color.primary\" must not contain the \".\" character",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = { color: { primary: 'var(--color)' }, __varGroupHash__: 'x568ih9' };
    const theme = stylex.unstable_createThemeNested(vars, {
      'color.primary': 'blue',
    });
  "#
);
