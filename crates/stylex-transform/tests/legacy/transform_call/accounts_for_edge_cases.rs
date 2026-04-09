use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments, customize: impl FnOnce(TestBuilder) -> TestBuilder) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(b.with_dev(true).with_runtime_injection())
  })
}

stylex_test!(
  using_stylex_in_a_for_loop,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
      import * as stylex from '@stylexjs/stylex';
      function test(colors, obj) {
        for (const color of colors) {
          obj[color.key] = stylex(color.style);
        }
      }
"#
);

stylex_test!(
  using_stylex_props_in_a_loop,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
      import * as stylex from '@stylexjs/stylex';
      function test(colors, obj) {
        for (const color of colors) {
          obj[color.key] = stylex.props(color.style);
        }
      }
"#
);

stylex_test!(
  trying_to_use_an_unknown_style_in_stylex,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_enable_debug_class_names(true)),
  r#"
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
      });
      stylex(styles.unknown);
"#
);

stylex_test!(
  trying_to_use_an_unknown_style_in_stylex_props,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_enable_debug_class_names(true)),
  r#"
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
      });
      stylex.props(styles.unknown);
"#
);
