use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

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

// Using the same options as the fixtures test
fn default_transform_options() -> StyleXOptionsParams {
  StyleXOptionsParams {
    dev: Some(true),
    treeshake_compensation: Some(true),
    unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
    enable_minified_keys: Some(false),
    ..StyleXOptionsParams::default()
  }
}

// Helper function to measure transform performance
fn measure_transform_time(input_path: &Path) -> (String, f64) {
  let output_path = input_path
    .parent()
    .unwrap()
    .join(input_path.file_stem().unwrap())
    .with_extension("output.js");

  let start = Instant::now();

  test_fixture(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    &|tr| {
      let mut config = default_transform_options();

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: None,
          filename: input_path.to_path_buf().into(),
        },
        Some(&mut config),
      )
    },
    input_path,
    &output_path,
    Default::default(),
  );

  let duration = start.elapsed().as_millis() as f64;

  // Read the output file to return the transformation result
  let code = fs::read_to_string(&output_path).unwrap_or_default();

  (code, duration)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stylex_transform_performance_test() {
    // Paths to the theme files
    let simple_theme_path = PathBuf::from("tests/performance_fixture/simpleTheme.js");
    let complex_theme_path = PathBuf::from("tests/performance_fixture/colorThemes.js");

    // Warm up the transformer with a simple transform
    measure_transform_time(&simple_theme_path);

    // Measure performance of simple theme transform
    let (simple_result, simple_time) = measure_transform_time(&simple_theme_path);

    #[allow(clippy::explicit_write)]
    writeln!(
      std::io::stderr(),
      "Simple theme transform took: {}ms",
      simple_time
    )
    .unwrap();

    let simple_time = simple_time.max(2.0); // Ensure at least 1.0 ms

    // Measure performance of complex theme transform
    let (complex_result, complex_time) = measure_transform_time(&complex_theme_path);

    #[allow(clippy::explicit_write)]
    writeln!(
      std::io::stderr(),
      "Complex theme transform took: {}ms",
      complex_time
    )
    .unwrap();
    std::io::stderr().flush().unwrap();

    // Verify the results are non-empty
    assert!(
      !simple_result.is_empty(),
      "Simple theme transformation result should not be empty"
    );
    assert!(
      !complex_result.is_empty(),
      "Complex theme transformation result should not be empty"
    );

    // Verify performance expectation (complex should be less than 20x slower than simple)
    assert!(
      complex_time < simple_time * 20.0,
      "Complex theme transform took too long: {}ms (simple: {}ms)",
      complex_time,
      simple_time
    );
  }
}
