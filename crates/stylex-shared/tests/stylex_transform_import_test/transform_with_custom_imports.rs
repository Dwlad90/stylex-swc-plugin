use stylex_shared::{
  shared::structures::{
    named_import_source::{ImportSources, NamedImportSource},
    plugin_pass::PluginPass,
    stylex_options::StyleXOptionsParams,
  },
  StyleXTransform,
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      import_sources: Some(vec![ImportSources::Regular("foo-bar".to_string())]),
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  handles_custom_default_imports,
  r#"
        import stylex from 'foo-bar';

        const styles = stylex.create({
            default: {
                    backgroundColor: 'red',
                    color: 'blue'
                }
            });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      import_sources: Some(vec![ImportSources::Regular("foo-bar".to_string())]),
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  handles_custom_as_all_imports,
  r#"
        import * as stylex from 'foo-bar';

        const styles = stylex.create({
            default: {
                    backgroundColor: 'red',
                    color: 'blue'
                }
            });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      import_sources: Some(vec![ImportSources::Named(NamedImportSource {
        from: "react-strict-dom".to_string(),
        r#as: "css".to_string(),
      })]),
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  handles_custom_named_imports,
  r#"
        import {css} from 'react-strict-dom';

        const styles = css.create({
            default: {
                    backgroundColor: 'red',
                    color: 'blue'
                }
            });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      import_sources: Some(vec![ImportSources::Named(NamedImportSource {
        from: "react-strict-dom".to_string(),
        r#as: "css".to_string(),
      })]),
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  handles_custom_named_imports_with_other_named_imports,
  r#"
        import {html, css} from 'react-strict-dom';

        const styles = css.create({
            default: {
                backgroundColor: 'red',
                color: 'blue',
            }
        });
    "#
);
