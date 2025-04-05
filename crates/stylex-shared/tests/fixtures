use std::path::PathBuf;

use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
};
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
        gen_conditional_classes: Some(true),
        ..StyleXOptionsParams::default()
      };

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: None,
          filename: input.clone().into(),
        },
        Some(&mut config),
      )
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
        runtime_injection: Some(false),
        gen_conditional_classes: Some(true),
        ..StyleXOptionsParams::default()
      };

      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: None,
          filename: input.clone().into(),
        },
        Some(&mut config),
      )
    },
    &input,
    &output_prod,
    Default::default(),
  );
}
