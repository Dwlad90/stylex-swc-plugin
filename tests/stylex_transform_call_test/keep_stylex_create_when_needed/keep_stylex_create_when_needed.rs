use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

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
  stylex_call_with_computed_key_access,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        [0]: {
          color: 'red',
        },
        [1]: {
          backgroundColor: 'blue',
        }
      });
      stylex(styles[variant])
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
  stylex_keeps_spaces_around_operators,
  r#"
      import stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        default: {
          margin: 'max(0px, (48px - var(--x16dnrjz)) / 2)',
        },
      });
      stylex(styles.default, props);
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
  stylex_call_with_composition_of_external_styles,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          color: 'red',
        },
      });
      stylex(styles.default, props);
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
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        default: {
          ':hover': {
            color: 'blue',
          },
          '@media (min-width: 1000px)': {
            backgroundColor: 'blue',
          },
        }
      });
      stylex(styles.default);
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
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries_within_props,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        default: {
          color: {
            ':hover': 'blue',
          },
          backgroundColor: {
            '@media (min-width: 1000px)': 'blue'
          },
        }
      });
      stylex(styles.default);
"#
);
