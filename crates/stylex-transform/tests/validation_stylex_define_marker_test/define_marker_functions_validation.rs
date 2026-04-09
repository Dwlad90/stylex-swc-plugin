use crate::utils::prelude::*;
use swc_core::common::FileName;

stylex_test_panic!(
  must_be_bound_to_a_named_export,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
      import * as stylex from '@stylexjs/stylex';
      const marker = stylex.defineMarker();
    "#
);

stylex_test_panic!(
  no_arguments_allowed,
  "defineMarker() should have 0 arguments.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const marker = stylex.defineMarker(1);
    "#
);

stylex_test_transform!(
  valid_export_direct_named_export,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const marker = stylex.defineMarker();
    "#,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const marker = {
            x1allf69: "x1allf69",
            $$css: true
        };
    "#
);

stylex_test_transform!(
  valid_export_separate_const_and_export_statement,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export { marker };
    "#,
  r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = {
            x1allf69: "x1allf69",
            $$css: true
        };
        export { marker };
    "#
);

stylex_test_panic!(
  invalid_export_re_export_from_another_file_does_not_count,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export { marker } from './other.stylex.js';
    "#
);

stylex_test_panic!(
  invalid_export_renamed_re_export_from_another_file_does_not_count,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export { marker as otherMarker } from './other.stylex.js';
    "#
);

stylex_test_panic!(
  invalid_export_default_export_does_not_count,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export default marker;
    "#
);

stylex_test_panic!(
  invalid_export_renamed_export_with_as_syntax,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export { marker as themeMarker };
    "#
);
