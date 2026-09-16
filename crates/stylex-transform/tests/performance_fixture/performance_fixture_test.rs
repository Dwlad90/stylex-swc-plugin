use std::{fs, path::Path};

use stylex_structures::stylex_options::ModuleResolution;
use stylex_transform::StyleXTransform;

use crate::utils::{prelude::ts_syntax, transform::stringify_js};

fn tranform(input_path: &Path, input: &str) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_filename(input_path.to_path_buf().into())
      .with_dev(true)
      .with_treeshake_compensation(true)
      .with_unstable_module_resolution(ModuleResolution::haste(None))
      .with_enable_minified_keys(false)
      .with_runtime_injection()
      .into_pass()
  })
}

/// Transforms one fixture and compares the result with the recorded output.
fn assert_fixture(input_path: &Path) {
  let output_path = input_path
    .parent()
    .unwrap()
    .join(input_path.file_stem().unwrap())
    .with_extension("output.js");

  let input = fs::read_to_string(input_path)
    .unwrap_or_else(|error| panic!("cannot read {}: {}", input_path.display(), error));

  let output = tranform(input_path, &input);

  // Read the recorded output strictly. A default on a missing file compares the
  // result with an empty string, which turns a deleted fixture into a pass.
  let output_fixture = fs::read_to_string(&output_path)
    .unwrap_or_else(|error| panic!("cannot read {}: {}", output_path.display(), error));

  // An empty result against an empty recorded file is equal but says nothing,
  // so hold both sides to a result the transform actually wrote.
  assert!(
    !output.is_empty(),
    "{} transformed to nothing",
    input_path.display()
  );
  assert_eq!(output, output_fixture);
}

/// Checks that a simple theme and a complex theme both transform to their
/// recorded output.
///
/// The test used to assert a ratio between two wall-clock timings. A ratio
/// between two clock figures is set by the machine, not by the compiler: it
/// goes red on a busy host, and it moves again under coverage instrumentation,
/// which slows the two sides by different amounts.
///
/// Speed keeps a gate elsewhere. The benchmark suite has its own copy of each
/// of these two workloads, as `Performance - Basic theme transformation` and
/// `Performance - Complex theme transformation`, and gives each one a ceiling
/// in milliseconds that the budget check fails on
/// (`pnpm run --filter=@stylexswc/rs-compiler bench:budget`). Each ceiling is
/// an absolute figure, seeded from many runs on a pinned runner and given
/// headroom for the machine. That is what a speed gate needs, and what one
/// timing taken in a unit test on an unknown host cannot give. The benchmark
/// runs the production shape, so the dev shape this test uses has no speed
/// gate. The ratio gave it none either, because it compared dev with dev.
///
/// A figure the clock does not set — a node count, a declaration count, an
/// output length — would be repeatable, but it adds nothing here: the equality
/// in `assert_fixture` already pins both outputs byte for byte.
#[test]
fn stylex_transform_theme_fixture_test() {
  assert_fixture(Path::new("tests/performance_fixture/simpleTheme.js"));
  assert_fixture(Path::new("tests/performance_fixture/colorThemes.js"));
}
