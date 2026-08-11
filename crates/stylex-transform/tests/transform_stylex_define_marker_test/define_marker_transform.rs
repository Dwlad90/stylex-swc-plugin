use crate::utils::prelude::*;
use std::path::PathBuf;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_cwd(PathBuf::from("/stylex/packages/"))
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution {
          root_dir: Some("/stylex/packages/".to_string()),
          theme_file_extension: None,
          ..ModuleResolution::common_js(None)
        }),
    )
  })
}

stylex_test!(
  member_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fooBar = stylex.defineMarker();
  "#
);

stylex_test!(
  multiple_marker_exports_in_one_file,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_filename(FileName::Real("/stylex/packages/markers.stylex.ts".into()))),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const firstMarker = stylex.defineMarker();
    export const secondMarker = stylex.defineMarker();
  "#
);

// Two declarators under one `export const`: `fill_top_level_expressions`
// records these from a loop over the declaration's `decls`, a different path
// to the one-statement-per-marker form above.
stylex_test!(
  multiple_marker_exports_in_one_declaration,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_filename(FileName::Real("/stylex/packages/markers.stylex.ts".into()))),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const firstMarker = stylex.defineMarker(), secondMarker = stylex.defineMarker();
  "#
);

stylex_test!(
  multiple_markers_exported_by_a_separate_export_statement,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_filename(FileName::Real("/stylex/packages/markers.stylex.ts".into()))),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const firstMarker = stylex.defineMarker();
    const secondMarker = stylex.defineMarker();
    export { firstMarker, secondMarker };
  "#
);

stylex_test!(
  named_import_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { defineMarker } from '@stylexjs/stylex';
    export const baz = defineMarker();
  "#
);
