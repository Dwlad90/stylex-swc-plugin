use std::path::PathBuf;

use stylex_structures::{
  named_import_source::RuntimeInjection,
  plugin_pass::PluginPass,
  stylex_options::{StyleXOptions, StyleXOptionsParams},
};
use stylex_transform::StyleXTransform;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test_fixture,
};

#[testing::fixture("tests/fixture/**/input.stylex.js")]
fn fixture(input: PathBuf) {
  let output = input.parent().unwrap().join("output.js");
  let output_prod = input.parent().unwrap().join("output_prod.js");

  test_fixture(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    &|tr| {
      let mut config = StyleXOptionsParams {
        dev: Some(true),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        enable_minified_keys: Some(false),
        enable_debug_class_names: Some(true),
        ..StyleXOptionsParams::default()
      };

      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass {
          cwd: None,
          filename: input.clone().into(),
        })
        .with_options(&mut config)
        .with_runtime_injection()
        .into_pass()
    },
    &input,
    &output,
    Default::default(),
  );

  test_fixture(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    &|tr| {
      let mut config = StyleXOptionsParams {
        dev: Some(false),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        runtime_injection: Some(RuntimeInjection::Boolean(false)),
        ..StyleXOptionsParams::default()
      };

      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass {
          cwd: None,
          filename: input.clone().into(),
        })
        .with_options(&mut config)
        .into_pass()
    },
    &input,
    &output_prod,
    Default::default(),
  );
}
