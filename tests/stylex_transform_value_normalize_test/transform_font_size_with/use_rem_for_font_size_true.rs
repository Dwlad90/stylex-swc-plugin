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
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_font_size_from_px_to_rem,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          fontSize: '24px',
        },
        bar: {
          fontSize: 18,
        },
        baz: {
          fontSize: '1.25rem',
        },
        qux: {
          fontSize: 'inherit',
        }
      });
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
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_font_size_from_px_to_rem_even_with_calc,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          fontSize: 'calc(100% - 24px)',
        },
      });
    "#
);
