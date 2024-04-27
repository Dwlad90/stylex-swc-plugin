use std::path::PathBuf;

use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::{
  common::{chain, FileName, Mark},
  ecma::{
    parser::{Syntax, TsConfig},
    transforms::{base::resolver, testing::test_fixture},
  },
  plugin::proxies::PluginCommentsProxy,
};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
  let output = input.parent().unwrap().join("output.js");

  test_fixture(
    Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
    }),
    // &|tr| {
    &|_| {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();

      let mut config = StyleXOptionsParams::default();

      config.dev = Option::Some(true);
      config.treeshake_compensation = Option::Some(true);

      chain!(
        resolver(unresolved_mark, top_level_mark, false),
        ModuleTransformVisitor::new_test_styles(
          PluginCommentsProxy,
          PluginPass {
            cwd: Option::None,
            filename: FileName::Real("/app/pages/Page.tsx".into()),
          },
          Option::Some(config)
        ) // ModuleTransformVisitor::new_test(tr.comments.clone())
      )
    },
    &input,
    &output,
    Default::default(),
  );
}
