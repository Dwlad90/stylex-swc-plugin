use std::path::PathBuf;

use stylex_structures::{named_import_source::RuntimeInjection, stylex_options::ModuleResolution};
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
      StyleXTransform::test(tr.comments.clone())
        .with_filename(input.clone().into())
        .with_dev(true)
        .with_treeshake_compensation(true)
        .with_unstable_module_resolution(ModuleResolution::haste(None))
        .with_enable_minified_keys(false)
        .with_enable_debug_class_names(true)
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
      StyleXTransform::test(tr.comments.clone())
        .with_filename(input.clone().into())
        .with_dev(false)
        .with_treeshake_compensation(true)
        .with_unstable_module_resolution(ModuleResolution::haste(None))
        .with_runtime_injection_option(RuntimeInjection::Boolean(false))
        .into_pass()
    },
    &input,
    &output_prod,
    Default::default(),
  );
}
