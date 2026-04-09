use crate::utils::prelude::*;
use rustc_hash::FxHashMap;

stylex_test_panic!(
  local_variable_keyframes_object,
  "keyframes() can only accept an object.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
  r#"
          import stylex from 'stylex';
          const name = stylex.keyframes(null);
        "#
);

stylex_test_panic!(
  only_argument_must_be_an_object_of_objects_false,
  "Every frame within a keyframes() call must be an object.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
  r#"
          import stylex from 'stylex';
          const name = stylex.keyframes({
            from: false
          });
        "#
);

stylex_test!(
  only_argument_must_be_an_object_of_objects_valid_percentage,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
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
    StyleXTransform::test(tr.comments.clone())
      .with_pass(PluginPass::test_default())
      .with_defined_stylex_css_variables(defined_stylex_css_variables)
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
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
    StyleXTransform::test(tr.comments.clone())
      .with_pass(PluginPass::test_default())
      .with_defined_stylex_css_variables(defined_stylex_css_variables)
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
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
