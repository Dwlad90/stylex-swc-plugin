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
#[should_panic(expected = "Rule contains an unclosed function")]
fn disallow_unclosed_style_value_functions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut config = StyleXOptionsParams::default();

      let mut defined_stylex_css_variables = FxHashMap::default();

      defined_stylex_css_variables.insert("foo".to_string(), "1".to_string());

      config.defined_stylex_css_variables = Some(defined_stylex_css_variables);

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut config),
      )
    },
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({default: {color: 'var(--foo'}})
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Unprefixed custom properties")]
fn disallow_unprefixed_custom_properties() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut config = StyleXOptionsParams::default();

      let mut defined_stylex_css_variables = FxHashMap::default();

      defined_stylex_css_variables.insert("foo".to_string(), "1".to_string());

      config.defined_stylex_css_variables = Some(defined_stylex_css_variables);

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut config),
      )
    },
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({default: {color: 'var(foo)'}})
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
    let mut config = StyleXOptionsParams::default();

    let mut defined_stylex_css_variables = FxHashMap::default();

    defined_stylex_css_variables.insert("foo".to_string(), "1".to_string());

    config.defined_stylex_css_variables = Some(defined_stylex_css_variables);

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  allow_defined_custom_properties_simple,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({foo: { color: 'var(--foo)' }});
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

    defined_stylex_css_variables.insert("foo".to_string(), "1".to_string());
    defined_stylex_css_variables.insert("bar".to_string(), "1".to_string());

    config.defined_stylex_css_variables = Some(defined_stylex_css_variables);

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  allow_defined_custom_properties_double,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({foo: { color: 'var(--foobar)' }});
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None,
  ),
  allow_undefined_custom_properties_regular,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({foo: { color: 'var(--foobar)' }});
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();

    let defined_stylex_css_variables = FxHashMap::default();

    config.defined_stylex_css_variables = Some(defined_stylex_css_variables);

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  allow_undefined_custom_properties_not_defined,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({foo: { color: 'var(--foobar)' }});
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

    defined_stylex_css_variables.insert("foo".to_string(), "1".to_string());
    defined_stylex_css_variables.insert("bar".to_string(), "1".to_string());

    config.defined_stylex_css_variables = Some(defined_stylex_css_variables);

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  allow_undefined_custom_properties_double_not_defined,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({foo: { backgroundColor: 'var(--foofoo)', color: 'var(--foobar)' }});
    "#
);
