use stylex_swc_plugin::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  ModuleTransformVisitor,
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
  },
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
        stylex.attrs(styles[variant]);
        const styles = stylex.create({
            [0]: {
                color: 'red',
            },
            [1]: {
                backgroundColor: 'blue',
            }
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
  stylex_call_with_mixed_access,
  r#"
        import stylex from 'stylex';

        function MyComponent() {
            return (
                <>
                    <div {...stylex.attrs(styles.foo)} />
                    <div {...stylex.attrs(styles.bar)} />
                    <CustomComponent xstyle={styles.foo} />
                    <div {...stylex.attrs([styles.foo, styles.bar])} />
                </>
            );
        }

        const styles = stylex.create({
            foo: {
                color: 'red',
            },
            bar: {
                backgroundColor: 'blue',
            }
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
  stylex_call_with_composition_of_external_styles,
  r#"
        import stylex from 'stylex';
        stylex.attrs([styles.default, attrs]);
        const styles = stylex.create({
            default: {
                color: 'red',
            },
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
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries,
  r#"
        import stylex from 'stylex';
        stylex.attrs(styles.default);
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
    "#
);
