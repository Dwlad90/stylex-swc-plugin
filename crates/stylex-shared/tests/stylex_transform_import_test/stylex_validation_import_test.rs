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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  ignores_valid_imports,
  r#"
        import * as stylex from '@stylexjs/stylex';
        import {foo, bar} from 'other';

        export default stylex.create({
            foo: {
                color: 'red'
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
  ignores_valid_requires,
  r#"
        const stylex = require('@stylexjs/stylex');
        const {foo, bar} = require('other');

        export default stylex.create({
            foo: {
                color: 'red'
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
  named_declaration_export,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
            foo: {
                color: 'red'
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
  does_nothing_when_stylex_not_imported,
  r#"
        export const styles = stylex.create({
            foo: {
                color: 'red'
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
  named_property_export,
  r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
        export {styles}
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
  default_export,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export default stylex.create({
            foo: {
                color: 'red'
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
  default_export_with_parenthesis,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export default (stylex.create({
            foo: {
                color: 'red'
            },
        }));
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
  module_export,
  r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
        module.export = styles;
    "#
);
