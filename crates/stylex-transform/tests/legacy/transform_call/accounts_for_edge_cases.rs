use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  using_stylex_in_a_for_loop,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_dev(true)
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_dev(true)
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_runtime_injection()
    .into_pass(),
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
