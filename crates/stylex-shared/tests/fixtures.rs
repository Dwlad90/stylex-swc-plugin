use std::path::PathBuf;

use stylex_shared::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
  StyleXTransform,
};
use swc_core::{
  common::{chain, FileName, Mark},
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::{base::resolver, testing::test_fixture},
  },
};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
  let output: PathBuf;
  let output_prod: PathBuf;

  #[cfg(feature = "wasm")]
  {
    output = input.parent().unwrap().join("wasm_output.js");
    output_prod = input.parent().unwrap().join("wasm_output_prod.js");
  }

  #[cfg(not(feature = "wasm"))]
  {
    output = input.parent().unwrap().join("rs_output.js");
    output_prod = input.parent().unwrap().join("rs_output_prod.js");
  }

  test_fixture(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    &|tr| {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();

      let mut config = StyleXOptionsParams {
        dev: Some(true),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        gen_conditional_classes: Some(true),
        ..StyleXOptionsParams::default()
      };

      chain!(
        resolver(unresolved_mark, top_level_mark, false),
        StyleXTransform::new_test_force_runtime_injection(
          tr.comments.clone(),
          PluginPass {
            cwd: None,
            filename: FileName::Real("/app/pages/Page.stylex.tsx".into()),
          },
          Some(&mut config)
        )
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
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();

      let mut config = StyleXOptionsParams {
        dev: Some(false),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        runtime_injection: Some(false),
        gen_conditional_classes: Some(true),
        ..StyleXOptionsParams::default()
      };

      chain!(
        resolver(unresolved_mark, top_level_mark, false),
        StyleXTransform::new_test(
          tr.comments.clone(),
          PluginPass {
            cwd: None,
            filename: FileName::Real("/app/pages/Page.stylex.tsx".into()),
          },
          Some(&mut config)
        )
      )
    },
    &input,
    &output_prod,
    Default::default(),
  );
}
