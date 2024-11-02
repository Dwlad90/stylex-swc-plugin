
use rustc_hash::FxHashMap;
use stylex_shared::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  StyleXTransform,
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
// TODO: This needs a different message. It mentions stylex.create right now.
#[should_panic(expected = "stylex.keyframes() can only accept an object.")]
fn only_argument_must_be_an_object_of_objects_null() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_force_runtime_injection(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
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
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_force_runtime_injection(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();

    let mut defined_stylex_css_variables = FxHashMap::default();

    defined_stylex_css_variables.insert("bar".to_string(), "1".to_string());

    config.defined_stylex_css_variables = Some(defined_stylex_css_variables);

    config.runtime_injection = Some(true);

    StyleXTransform::new_test_force_runtime_injection(
      tr.comments.clone(),
      &PluginPass::default(),
      Some(&mut config),
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
