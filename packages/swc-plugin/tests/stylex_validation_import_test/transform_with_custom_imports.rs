use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::{ImportSources, NamedImportSource, RuntimeInjection},
    plugin_pass::PluginPass,
    stylex_options::StyleXOptionsParams,
  },
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();

    config.import_sources = Option::Some(vec![ImportSources::Regular("foo-bar".to_string())]);
    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();

    config.import_sources = Option::Some(vec![ImportSources::Regular("foo-bar".to_string())]);
    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();
    //{ from: 'react-strict-dom', as: 'css' }

    config.import_sources = Option::Some(vec![ImportSources::Named(NamedImportSource {
      from: "react-strict-dom".to_string(),
      r#as: "css".to_string(),
    })]);
    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();
    //{ from: 'react-strict-dom', as: 'css' }

    config.import_sources = Option::Some(vec![ImportSources::Named(NamedImportSource {
      from: "react-strict-dom".to_string(),
      r#as: "css".to_string(),
    })]);
    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
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
