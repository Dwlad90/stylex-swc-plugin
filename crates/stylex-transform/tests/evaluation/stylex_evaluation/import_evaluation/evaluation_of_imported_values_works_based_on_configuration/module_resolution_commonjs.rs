use crate::utils::prelude::*;
use insta::assert_snapshot;
use swc_core::ecma::transforms::testing::test;

use crate::utils::transform::stringify_js;

fn tranform(input: &str) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    let cwd_path = std::env::current_dir().unwrap();

    let fixture_path = cwd_path.join("tests/fixture/consts");

    StyleXTransform::test(tr.comments.clone())
      .with_cwd(fixture_path.clone())
      .with_filename(fixture_path.clone().join("input.stylex.js").into())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_treeshake_compensation(true)
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        fixture_path.to_string_lossy().to_string(),
      )))
      .with_runtime_injection()
      .into_pass()
  })
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
