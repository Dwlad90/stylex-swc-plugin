use std::collections::HashMap;

use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection, plugin_pass::PluginPass,
    stylex_options::StyleXOptionsParams,
  },
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

#[test]
// TODO: This needs a different message. It mentions stylex.create right now.
#[should_panic(expected = "stylex.keyframes() can only accept an object.")]
fn only_argument_must_be_an_object_of_objects_null() {
  swc_core::ecma::transforms::testing::test_transform(
    Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None,
      )
    },
    r#"
            import stylex from 'stylex';
            const name = stylex.keyframes(null);
        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "Every frame within a stylex.keyframes() call must be an object.")]
fn only_argument_must_be_an_object_of_objects_non_keyframe() {
  swc_core::ecma::transforms::testing::test_transform(
    Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None,
      )
    },
    r#"
            import stylex from 'stylex';
            const name = stylex.keyframes({
                from: true,
            });
        "#,
    r#""#,
    false,
  )
}

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
  ),
  only_argument_must_be_an_object_of_objects_valid,
  r#"
        import stylex from 'stylex';
        const name = stylex.keyframes({
            from: {},
            to: {},
        });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Option::None
  ),
  only_argument_must_be_an_object_of_objects_valid_filled,
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

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();
    //{ from: 'react-strict-dom', as: 'css' }

    let mut defined_stylex_css_variables = HashMap::new();

    defined_stylex_css_variables.insert("bar".to_string(), "1".to_string());

    config.defined_stylex_css_variables = Option::Some(defined_stylex_css_variables);

    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
    )
  },
  allow_defined_css_variables_in_keyframes,
  r#"
    import stylex from 'stylex';
    const styles = stylex.keyframes({
        from: {
            backgroundColor: 'var(--foobar)',
        },
    });
    "#
);
