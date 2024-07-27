use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::ImportSources, plugin_pass::PluginPass,
    stylex_options::StyleXOptionsParams,
  },
  ModuleTransformVisitor,
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
      import_sources: Some(vec![ImportSources::Regular(
        "custom-stylex-path".to_string(),
      )]),
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    };

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      &PluginPass::default(),
      Some(&mut config),
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
