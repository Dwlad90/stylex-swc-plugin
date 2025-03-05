use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_default_value() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            const styles = stylex.create({
                dynamic: (props = {}) => ({
                    color: props.color,
                }),
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_default_string_value() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            const styles = stylex.create({
                dynamic: (color = 'red') => ({
                    color,
                }),
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_object_arg() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            const styles = stylex.create({
                dynamic: ({ color }) => ({
                    color,
                }),
            });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_rest_arg() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            const styles = stylex.create({
                dynamic: (...rest) => ({
                    color,
                }),
            });
        "#,
    r#""#,
  )
}

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  dynamic_style_function_only_accepts_named_parameters_valid,
  r#"
    import stylex from "@stylexjs/stylex";
    export const styles = stylex.create({
        dynamic: (backgroundColor) => ({
            backgroundColor,
        }),
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
      PluginPass::default(),
      None,
    )
  },
  dynamic_style_function_only_accepts_named_parameters_valid_with_other_styles,
  r#"
    import stylex from "@stylexjs/stylex";

    const styles = stylex.create({
      size: (size: number) => ({ fontSize: (8 * size)+'px'}),
      count: {
        fontWeight: 100,
      },
      largeNumber: {
        fontSize: '1.5rem',
      },
    });

    const { className, style = {} } = { ...stylex.props(
      styles.count,
      styles.size(size),
      styles.largeNumber
    )}
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
      PluginPass::default(),
      None,
    )
  },
  dynamic_style_function_with_conditional_expression_fallback,
  r#"
    import stylex from "@stylexjs/stylex";

    const styles = stylex.create({
      fontSizeFallback: (size: number) => ({ fontSize: size ?? '1em' }),
    });

    const { className, style = {} } = { ...stylex.props(
      styles.fontSizeFallback(size),
    )}
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
      PluginPass::default(),
      None,
    )
  },
  dynamic_style_function_with_ternary_conditional_expression,
  r#"
    import stylex from "@stylexjs/stylex";

    const styles = stylex.create({
      fontSizeTernary: (size: number) => ({ fontSize: size < 10 ? '1em' : '2em' }),
    });

    const { className, style = {} } = { ...stylex.props(
      styles.fontSizeTernary(size),
    )}
  "#
);
