use rustc_hash::FxHashMap;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

// stylex.keyframes() validation tests corresponding to JavaScript describe('[validation] stylex.keyframes()')

// Local variable keyframes object test

#[test]
#[should_panic(expected = "stylex.keyframes() can only accept an object.")]
fn local_variable_keyframes_object() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
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
    "#,
    r#""#,
  )
}

// Only argument must be an object of objects tests

#[test]
#[should_panic(expected = "stylex.keyframes() can only accept an object.")]
fn only_argument_must_be_an_object_of_objects_null() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
    r#"
      import stylex from 'stylex';
      const name = stylex.keyframes(null);
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Every frame within a stylex.keyframes() call must be an object.")]
fn only_argument_must_be_an_object_of_objects_false() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
    r#"
      import stylex from 'stylex';
      const name = stylex.keyframes({
        from: false
      });
    "#,
    r#""#,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::new(None, None),
      None,
    )
  },
  only_argument_must_be_an_object_of_objects_valid_percentage,
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
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::new(None, None),
      None,
    )
  },
  only_argument_must_be_an_object_of_objects_valid_from_to,
  r#"
    import stylex from 'stylex';
    const name = stylex.keyframes({
      from: {},
      to: {},
    });
  "#
);

// Allow defined CSS variables in keyframes

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

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::new(None, None),
      Some(&mut config),
    )
  },
  allow_defined_css_variables_in_keyframes,
  r#"
    import stylex from 'stylex';
    const styles = stylex.keyframes({
      from: {
        backgroundColor: 'var(--bar)',
      },
    });
  "#
);

// Allow undefined CSS variables in keyframes

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

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::new(None, None),
      Some(&mut config),
    )
  },
  allow_undefined_css_variables_in_keyframes,
  r#"
    import stylex from 'stylex';
    const styles = stylex.keyframes({
      from: {
        backgroundColor: 'var(--foobar)',
      },
    });
  "#
);
