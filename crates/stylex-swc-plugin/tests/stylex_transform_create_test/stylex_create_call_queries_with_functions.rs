use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
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
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
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
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
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
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
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
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
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
