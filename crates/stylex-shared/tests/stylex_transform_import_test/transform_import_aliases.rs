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
  not_throw_when_named_import,
  r#"
        import { foo } from "@stylexjs/stylex";

        foo('bar');
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
  support_stylex_imports_with_types,
  r#"
      import stylex, { StyleXStyles } from "@stylexjs/stylex";
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
  support_stylex_imports_with_types_v2,
  r#"
      import stylex, { type StyleXStyles } from "@stylexjs/stylex";
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
  support_stylex_imports_and_types,
  r#"
      import stylex from "@stylexjs/stylex";
      import { StyleXStyles } from "@stylexjs/stylex";
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
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
