use std::{path::PathBuf, rc::Rc};

use stylex_swc_plugin::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::{
  common::{chain, comments::SingleThreadedComments, FileName, Mark},
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::{base::resolver, testing::test_fixture},
  },
};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
  let output = input.parent().unwrap().join("output.js");

  test_fixture(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    &|_| {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();

      let mut config = StyleXOptionsParams {
        dev: Some(true),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        ..StyleXOptionsParams::default()
      };

      chain!(
        resolver(unresolved_mark, top_level_mark, false),
        ModuleTransformVisitor::new_test_styles(
          Rc::new(SingleThreadedComments::default()),
          &PluginPass {
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
}
