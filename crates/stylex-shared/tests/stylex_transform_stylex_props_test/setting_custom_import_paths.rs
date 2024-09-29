use stylex_shared::{
  shared::structures::{
    named_import_source::ImportSources, plugin_pass::PluginPass,
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
      import_sources: Some(vec![ImportSources::Regular(
        "custom-stylex-path".to_string(),
      )]),
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection(
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
