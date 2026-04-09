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
        .with_filename(FileName::Real(
          "/stylex/packages/src/vars/default.cssvars.js".into(),
        ))
        .with_debug(true)
        .with_enable_debug_class_names(true)
        .with_unstable_module_resolution(ModuleResolution {
          r#type: "commonJS".to_string(),
          root_dir: Some("/stylex/packages/".to_string()),
          theme_file_extension: Some("cssvars".to_string()),
        }),
    )
  })
}

stylex_test!(
  processes_tokens_in_files_with_configured_extension,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
  "#
);
