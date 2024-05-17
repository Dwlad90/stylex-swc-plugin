use std::path::PathBuf;

use stylex_swc_plugin::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::{StyleXOptions, StyleXOptionsParams}},
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

      config.unstable_module_resolution =
        Option::Some(StyleXOptions::get_haste_module_resolution(Option::None));



      chain!(
        resolver(unresolved_mark, top_level_mark, false),
        ModuleTransformVisitor::new_test_styles(
          PluginCommentsProxy,
          &PluginPass {
            cwd: Option::None,
            filename: FileName::Real("/app/pages/Page.stylex.tsx".into()),
          },
          Option::Some(&mut config)
        ) // ModuleTransformVisitor::new_test(tr.comments.clone())
      )
    },
    &input,
    &output,
    Default::default(),
  );
}
