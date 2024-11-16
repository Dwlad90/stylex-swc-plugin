use stylex_shared::{shared::structures::plugin_pass::PluginPass, StyleXTransform};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_props_with_camel_case_key,
  r#"
        import stylex from 'stylex';

        const styles = stylex.create({
          primaryVariant: {
              padding: '0',
              margin: '0',
              listStyle: 'none',
              display: 'grid',
              gridAutoFlow: 'column',
              width: '100%',
              justifyContent: 'flex-start',
              borderBottomStyle: 'solid',
              borderBottomWidth: '1px',
          },
        });

        function TestComponent({ variant }) {
            return (
                <div {...stylex.props(styles[`${variant}Variant`])} />
            );
        }
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_props_with_pascal_case_key,
  r#"
        import stylex from 'stylex';

        const styles = stylex.create({
          PrimaryVariant: {
              padding: '0',
              margin: '0',
              listStyle: 'none',
              display: 'grid',
              gridAutoFlow: 'column',
              width: '100%',
              justifyContent: 'flex-start',
              borderBottomStyle: 'solid',
              borderBottomWidth: '1px',
          },
        });

        function TestComponent({ variant }) {
            return (
                <div {...stylex.props(styles[`${variant}Variant`])} />
            );
        }
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_props_with_snake_case_key,
  r#"
        import stylex from 'stylex';

        const styles = stylex.create({
          'primary_variant': {
              padding: '0',
              margin: '0',
              listStyle: 'none',
              display: 'grid',
              gridAutoFlow: 'column',
              width: '100%',
              justifyContent: 'flex-start',
              borderBottomStyle: 'solid',
              borderBottomWidth: '1px',
          },
        });

        function TestComponent({ variant }) {
            return (
                <div {...stylex.props(styles[`${variant}_variant`])} />
            );
        }
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_props_with_kebab_case_key,
  r#"
        import stylex from 'stylex';

        const styles = stylex.create({
          'primary-variant': {
              padding: '0',
              margin: '0',
              listStyle: 'none',
              display: 'grid',
              gridAutoFlow: 'column',
              width: '100%',
              justifyContent: 'flex-start',
              borderBottomStyle: 'solid',
              borderBottomWidth: '1px',
          },
        });

        function TestComponent({ variant }) {
            return (
                <div {...stylex.props(styles[`${variant}-variant`])} />
            );
        }
    "#
);
