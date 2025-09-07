use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
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
            const variantStyle = `${variant}Variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
            );
        }
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
            const variantStyle = `${variant}Variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
            );
        }
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
            const variantStyle = `${variant}_variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
            );
        }
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
            const variantStyle = `${variant}-variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
            );
        }
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
  stylex_call_props_with_override_dynamic_styles,
  r#"
        import stylex from 'stylex';

        const styles = stylex.create({
          'primary-variant': {
              color: 'red'
          },
          secondaryVariant: {
              color: 'blue'
          },
        });

        function TestComponent({ variant }) {
            return (
                <div {...stylex.props(styles.secondaryVariant, styles[`${variant}-variant`])} />
            );
        }
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    None
  ),
  stylex_call_props_with_renaming_dynamic_styles_prop,
  r#"
        import * as stylex from '@stylexjs/stylex';
        import { fonts as f } from '@stylexjs/open-props/lib/fonts.stylex';

        const styles = stylex.create({
          text: {
            color: 'hotpink',
          },
        });

        const variants = stylex.create({
          small: {
            fontSize: f.size1
          },
          big: {
            fontSize: f.size7
          }
        })

        export function Text2({ children, size: s }) {
          return <div {...stylex.props(styles.text, variants[s])}>{children}</div>;
        }

    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    None
  ),
  stylex_call_props_with_renaming_dynamic_styles_prop_and_conflict_import_name,
  r#"
        import * as stylex from '@stylexjs/stylex';
        import { fonts as foo } from '@stylexjs/open-props/lib/fonts.stylex';

        const styles = stylex.create({
          text: {
            color: 'hotpink',
          },
        });

        const variants = stylex.create({
          small: {
            fontSize: foo.size1
          },
          big: {
            fontSize: foo.size7
          }
        })

        export function Text2({ children, size: foo }) {
          return <div {...stylex.props(styles.text, variants[foo])}>{children}</div>;
        }
    "#
);
