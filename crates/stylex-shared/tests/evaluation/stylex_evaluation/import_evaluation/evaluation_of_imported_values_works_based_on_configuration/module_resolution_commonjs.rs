use std::env;

use insta::assert_snapshot;
use stylex_shared::shared::structures::{
  named_import_source::RuntimeInjection,
  stylex_options::{StyleXOptions, StyleXOptionsParams},
};
use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

use crate::utils::transform::stringify_js;

fn tranform(input: &str) -> String {
  stringify_js(
    input,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      let cwd_path = std::env::current_dir().unwrap();

      let fixture_path = cwd_path.join("tests/fixture/consts");

      let mut config = StyleXOptionsParams {
        runtime_injection: Some(RuntimeInjection::Boolean(true)),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
          fixture_path.to_string_lossy().to_string(),
        ))),
        ..Default::default()
      };

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(fixture_path.clone()),
          filename: fixture_path.clone().join("input.stylex.js").into(),
        },
        Some(&mut config),
      )
    },
  )
}

#[test]
fn recognizes_ts_stylex_imports_when_resolving_js_relative_imports() {
  let input = r#"import stylex from 'stylex';
        import { MyTheme } from './input.stylex';
        const styles = stylex.create({
          red: {
            color: MyTheme.__varGroupHash__,
          }
        });
        stylex(styles.red);"#;

  let transformation = tranform(input);

  assert_snapshot!(transformation);
}
