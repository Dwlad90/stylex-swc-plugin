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
  invalid_export_not_bound,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.defineVars({});
        "#
);

stylex_test_panic!(
  invalid_export_not_bound_unbound,
  "defineVars() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          stylex.defineVars({});
        "#
);

stylex_test_panic!(
  invalid_argument_none,
  "defineVars() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars();
        "#
);

stylex_test_panic!(
  invalid_argument_too_many,
  "defineVars() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({}, {});
        "#
);

stylex_test_panic!(
  invalid_argument_number,
  "defineVars() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars(1);
        "#
);

stylex_test_panic!(
  invalid_argument_string,
  "defineVars() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars('1');
        "#
);

stylex_test_panic!(
  invalid_argument_non_static,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars(genStyles());
        "#
);

stylex_test!(
  valid_argument_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({});
        "#
);

stylex_test!(
  valid_export_separate_const_and_export_statement,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          const vars = stylex.defineVars({});
          export { vars };
        "#
);

stylex_test_panic!(
  invalid_export_re_export_from_another_file_does_not_count,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          const vars = stylex.defineVars({});
          export { vars } from './other.stylex.js';
        "#
);

stylex_test_panic!(
  invalid_export_renamed_re_export_from_another_file_does_not_count,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          const vars = stylex.defineVars({});
          export { vars as otherVars } from './other.stylex.js';
        "#
);

stylex_test_panic!(
  invalid_export_default_export_does_not_count,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          const vars = stylex.defineVars({});
          export default vars;
        "#
);

stylex_test_panic!(
  invalid_export_renamed_export_with_as_syntax,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          const vars = stylex.defineVars({});
          export { vars as themeVars };
        "#
);

/* Properties */

stylex_test_panic!(
  invalid_key_non_static,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({
            [labelColor]: 'red',
          });
        "#
);

/* Values */

stylex_test_panic!(
  invalid_value_non_static_variable,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({
            labelColor: labelColor,
          });
        "#
);

stylex_test_panic!(
  invalid_value_non_static_function_call,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({
            labelColor: labelColor(),
          });
        "#
);

stylex_test!(
  valid_value_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({
            cornerRadius: 5,
          });
        "#
);

stylex_test!(
  valid_value_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({
            labelColor: 'red',
          });
        "#
);

stylex_test!(
  valid_value_keyframes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({
            fadeIn: stylex.keyframes({
              '0%': { opacity: 0 },
              '100%': { opacity: 1}
            }),
          });
        "#
);
