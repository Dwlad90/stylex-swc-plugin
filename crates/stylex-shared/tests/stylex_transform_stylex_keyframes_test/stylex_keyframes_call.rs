use stylex_shared::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  empty_stylex_props_call,
  r#"
        import stylex from 'stylex';
        stylex.attrs();
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  converts_keyframes_to_css,
  r#"
        import stylex from 'stylex';
        export const name = stylex.keyframes({
            from: {
                backgroundColor: 'red',
            },
            to: {
                backgroundColor: 'blue',
            }
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  converts_keyframes_to_css_with_import_wildcard,
  r#"
    import * as stylex from 'stylex';
    export const name = stylex.keyframes({
        from: {
            backgroundColor: 'red',
        },

        to: {
            backgroundColor: 'blue',
        }
    });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  converts_keyframes_to_css_with_named_import,
  r#"
        import { keyframes } from 'stylex';
        export const name = keyframes({
            from: {
                backgroundColor: 'red',
            },

            to: {
                backgroundColor: 'blue',
            }
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  allows_template_literal_references_to_keyframes,
  r#"
        import stylex from 'stylex';
        export const name = stylex.keyframes({
            from: {
                backgroundColor: 'blue',
            },
            to: {
                backgroundColor: 'red',
            },
        });

        const styles = stylex.create({
            default: {
                animation: `3s ${name}`,
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  allows_inline_references_to_keyframes,
  r#"
        import stylex from 'stylex';

        const styles = stylex.create({
            default: {
                animationName: stylex.keyframes({
                    from: {
                        backgroundColor: 'blue',
                    },
                    to: {
                        backgroundColor: 'red',
                    },
                }),
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  generates_rtl_specific_keyframes,
  r#"
        import stylex from 'stylex';
        export const name = stylex.keyframes({
            from: {
                start: 0,
            },

            to: {
                start: 500,
            },
        });

        export const styles = stylex.create({
            root: {
                animationName: name,
            },
        });
    "#
);
