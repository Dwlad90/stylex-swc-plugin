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
          r#type: "commonJS".to_string(),
          root_dir: Some("/stylex/packages/".to_string()),
          theme_file_extension: None,
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
  named_import_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { defineMarker } from '@stylexjs/stylex';
    export const baz = defineMarker();
  "#
);
