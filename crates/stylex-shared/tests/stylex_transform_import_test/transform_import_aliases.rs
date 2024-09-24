use stylex_shared::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
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
  transform_import_aliases,
  r#"
        import foobar from '@stylexjs/stylex';
        const styles = foobar.create({
            default: {
                backgroundColor: 'red',
                color: 'blue',
                padding: 5
            }
        });
        styles;
    "#
);

#[test]
#[should_panic(expected = "Must be default import")]
fn throw_when_named_import() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_force_runtime_injection(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import { foo } from "@stylexjs/stylex";

            foo('bar');
        "#,
    r#""#,
    false,
  )
}

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
  can_import_with_a_different_name,
  r#"
        import foobar from '@stylexjs/stylex';
        const styles = foobar.create({
            default: {
                backgroundColor: 'red',
                color: 'blue',
                padding: 5
            }
        });
        styles;
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
  can_import_wildcard,
  r#"
        import * as foobar from '@stylexjs/stylex';

        const styles = foobar.create({
        default: {
                backgroundColor: 'red',
                color: 'blue',
                padding: 5
            }
        });
        styles;
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
  can_import_just_create,
  r#"
        import {create} from '@stylexjs/stylex';

        const styles = create({
            default: {
                    backgroundColor: 'red',
                    color: 'blue',
                    padding: 5
                }
            });
        styles;
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
  can_import_just_create_with_alias,
  r#"
        import {create as css} from '@stylexjs/stylex';

        const styles = css({
            default: {
                    backgroundColor: 'red',
                    color: 'blue',
                    padding: 5
                }
            });
        styles;
    "#
);
