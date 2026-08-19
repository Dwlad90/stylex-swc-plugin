mod accounts_for_edge_cases;
mod keep_stylex_create_when_needed_test;
mod setting_custom_import_paths;
mod specific_edge_case_bugs;
mod stylex_transform_call_common_test;
mod with_contextual_styles_and_collisions;
mod with_plugin_options;

use crate::utils::prelude::*;

/// The options upstream pins once, at file level, in
/// `__tests__/legacy/stylex-transform-call-test.js` — they cover every
/// `describe` in it, and every module here is a port of one of those.
///
/// Ported per-file without it, these tests measure the *default* resolution
/// instead, and a shorthand then answers a different question than the one
/// upstream asked. Shared so a new module cannot be added without it.
pub(crate) fn legacy_call_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_style_resolution(StyleResolution::ApplicationOrder)
        .with_runtime_injection(),
    )
  })
}
