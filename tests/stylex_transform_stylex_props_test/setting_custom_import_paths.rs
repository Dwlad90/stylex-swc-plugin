use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::{ImportSources, RuntimeInjection},
    plugin_pass::PluginPass,
    stylex_options::StyleXOptionsParams,
  },
  ModuleTransformVisitor,
};
use swc_core::{
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
  |tr| {
    let mut config = StyleXOptionsParams::default();
    //{ from: 'react-strict-dom', as: 'css' }

    config.import_sources = Option::Some(vec![ImportSources::Regular(
      "custom-stylex-path".to_string(),
    )]);

    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
    )
  },
  basic_stylex_call,
  r#"
        import stylex from 'custom-stylex-path';
        const styles = stylex.create({
            red: {
                color: 'red',
            }
        });
        stylex.props(styles.red);
"#
);
