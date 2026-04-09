use crate::utils::prelude::*;

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
  invalid_use_not_bound,
  "create() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          stylex.create({});
        "#
);

stylex_test_panic!(
  invalid_argument_none,
  "create() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create();
        "#
);

stylex_test_panic!(
  invalid_argument_too_many,
  "create() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({}, {});
        "#
);

stylex_test_panic!(
  invalid_argument_non_static,
  "create() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create(genStyles());
        "#
);

stylex_test!(
  valid_argument_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({});
        "#
);
