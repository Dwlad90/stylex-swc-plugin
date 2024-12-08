use stylex_shared::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptionsParams},
  },
  StyleXTransform,
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  transforms_style_object_with_function,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (color) => ({
        backgroundColor: 'red',
        color,
      })
    });
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
    None
  ),
  adds_units_for_numbers_in_style_object_with_function,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (width) => ({
        backgroundColor: 'red',
        width,
      })
    });
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
    None
  ),
  transforms_mix_of_objects_and_functions,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (color) => ({
        backgroundColor: 'red',
        color,
      }),
      mono: {
        color: 'black',
      },
    });
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
    None
  ),
  transforms_styles_that_set_css_vars,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (bgColor) => ({
        '--background-color': bgColor,
      }),
    });
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
    None
  ),
  transforms_functions_with_nested_dynamic_values,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (color) => ({
        ':hover': {
          backgroundColor: 'red',
          color,
        },
      }),
    });
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
    None
  ),
  transforms_functions_with_dynamic_values_within_conditional_values,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (color) => ({
        backgroundColor: 'red',
        color: {
          default: color,
          ':hover': {
            '@media (min-width: 1000px)': 'green',
            default: 'blue',
          }
        },
      }),
    });
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
    None
  ),
  transforms_functions_with_multiple_dynamic_values_within_conditional_values,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (color) => ({
        backgroundColor: 'red',
        color: {
          default: color,
          ':hover': {
            '@media (min-width: 1000px)': 'green',
            default: 'color-mix(' + color + ', blue)',
          }
        },
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
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  transforms_shorthands_in_legacy_expand_shorthands_mode,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (margin) => ({
        backgroundColor: 'red',
        margin: {
          default: margin,
          ':hover': margin + 4,
        },
        marginTop: margin - 4,
      })
    });
  "#
);
