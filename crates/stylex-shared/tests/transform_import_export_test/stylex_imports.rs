#[cfg(test)]
mod stylex_imports_tests {
  use std::path::PathBuf;

  use stylex_shared::{
    StyleXTransform,
    shared::structures::{
      named_import_source::{ImportSources, NamedImportSource},
      plugin_pass::PluginPass,
      stylex_options::{ModuleResolution, StyleXOptionsParams},
    },
  };
  use swc_core::{common::FileName, ecma::transforms::testing::test};
  use swc_ecma_parser::{Syntax, TsSyntax};

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
    import_none,
    r#"
        export const styles = stylex.create({
          root: {
            color: 'red'
          }
        });
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
    import_non_stylex,
    r#"
        import {foo, bar} from 'other';
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
    require_non_stylex,
    r#"
        const {foo, bar} = require('other');
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
      },
      None
    ),
    import_wildcard_default,
    r#"
        import * as stylex from "@stylexjs/stylex";
        export const vars = stylex.defineVars({ bar: 'left' });
        const styles = stylex.create({
          root: {
            animationName: stylex.keyframes({
              from: { backgroundColor: 'yellow' },
              to: { backgroundColor: 'orange' },
            }),
            color: 'red',
            position: stylex.firstThatWorks('sticky', 'fixed')
          }
        });
        const theme = stylex.createTheme(vars, { bar: 'green' });
        stylex.props(styles.root, theme);
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
      },
      None
    ),
    import_wildcard_non_stylex,
    r#"
        import * as foo from "@stylexjs/stylex";
        export const vars = foo.defineVars({ bar: 'left' });
        const styles = foo.create({
          root: {
            animationName: foo.keyframes({
              from: { backgroundColor: 'yellow' },
              to: { backgroundColor: 'orange' },
            }),
            color: 'red',
            position: foo.firstThatWorks('sticky', 'fixed')
          }
        });
        const theme = foo.createTheme(vars, { bar: 'green' });
        foo.props(styles.root, theme);
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
      },
      None
    ),
    import_named,
    r#"
        import { create, createTheme, defineVars, firstThatWorks, keyframes, props } from "@stylexjs/stylex";
        export const vars = defineVars({ bar: 'left' });
        const styles = create({
          root: {
            animationName: keyframes({
              from: { backgroundColor: 'yellow' },
              to: { backgroundColor: 'orange' },
            }),
            color: 'red',
            position: firstThatWorks('sticky', 'fixed')
          }
        });
        const theme = createTheme(vars, { bar: 'green' });
        props(styles.root, theme);
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
      },
      None
    ),
    import_named_alias,
    r#"
        import { create as _create, createTheme as _createTheme, defineVars as _defineVars, firstThatWorks as _firstThatWorks, keyframes as _keyframes, props as _props } from "@stylexjs/stylex";
        export const vars = _defineVars({ bar: 'left' });
        const styles = _create({
          root: {
            animationName: _keyframes({
              from: { backgroundColor: 'yellow' },
              to: { backgroundColor: 'orange' },
            }),
            color: 'red',
            position: _firstThatWorks('sticky', 'fixed')
          }
        });
        const theme = _createTheme(vars, { bar: 'green' });
        _props(styles.root, theme);
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: Some(PathBuf::from("/stylex/packages/")),
        filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
      },
      // None
      Some(&mut StyleXOptionsParams {
        import_sources: Some(vec![ImportSources::Regular("foo-bar".to_string())]),
        unstable_module_resolution: Some(ModuleResolution {
          r#type: "haste".to_string(),
          root_dir: Some("/stylex/packages/".to_string()),
          theme_file_extension: None
        }),
        ..Default::default()
      })
    ),
    import_sources_string,
    r#"
        import * as stylex from "foo-bar";
        export const vars = stylex.defineVars({ bar: 'left' });
        const styles = stylex.create({
          root: {
            animationName: stylex.keyframes({
              from: { backgroundColor: 'yellow' },
              to: { backgroundColor: 'orange' },
            }),
            color: 'red',
            position: stylex.firstThatWorks('sticky', 'fixed')
          }
        });
        const theme = stylex.createTheme(vars, { bar: 'green' });
        stylex.props(styles.root, theme);
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: Some(PathBuf::from("/stylex/packages/")),
        filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
      },
      Some(&mut StyleXOptionsParams {
        import_sources: Some(vec![ImportSources::Named(NamedImportSource {
          r#as: "css".to_string(),
          from: "react-strict-dom".to_string(),
        })]),
        unstable_module_resolution: Some(ModuleResolution {
          r#type: "haste".to_string(),
          root_dir: Some("/stylex/packages/".to_string()),
          theme_file_extension: None
        }),
        ..Default::default()
      })
    ),
    import_sources_object,
    r#"
        import { css, html } from "react-strict-dom";
        export const vars = css.defineVars({ bar: 'left' });
        const styles = css.create({
          root: {
            animationName: css.keyframes({
              from: { backgroundColor: 'yellow' },
              to: { backgroundColor: 'orange' },
            }),
            color: 'red',
            position: css.firstThatWorks('sticky', 'fixed')
          }
        });
        const theme = css.createTheme(vars, { bar: 'green' });
        css.props(styles.root, theme);
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
      },
      None
    ),
    meta_only_import_default,
    r#"
        import stylex from "stylex";
        export const vars = stylex.defineVars({ bar: 'left' });
        const styles = stylex.create({
          root: {
            animationName: stylex.keyframes({
              from: { backgroundColor: 'yellow' },
              to: { backgroundColor: 'orange' },
            }),
            color: 'red',
            position: stylex.firstThatWorks('sticky', 'fixed')
          }
        });
        const theme = stylex.createTheme(vars, { bar: 'green' });
        stylex.props(styles.root, theme);
        "#
  );
}
