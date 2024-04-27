use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection, plugin_pass::PluginPass,
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
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      ..StyleXOptionsParams::default()
    })
  ),
  keeps_used_styles,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
          color: 'blue',
        }
      });
      styles;
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      ..StyleXOptionsParams::default()
    })
  ),
  removes_unused_styles,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
          color: 'blue',
        }
      });
    "#
);
