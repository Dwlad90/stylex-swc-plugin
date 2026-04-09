use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test_transform};

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn must_be_bound_to_a_named_export() {
  test_transform(
    ts_syntax(),
    None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
    r#"
      import * as stylex from '@stylexjs/stylex';
      const marker = stylex.defineMarker();
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineMarker() should have 0 arguments.")]
fn no_arguments_allowed() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
    r#"
        import * as stylex from '@stylexjs/stylex';
        export const marker = stylex.defineMarker(1);
    "#,
    r#""#,
  )
}

#[test]
fn valid_export_direct_named_export() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
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
    "#,
  )
}

#[test]
fn valid_export_separate_const_and_export_statement() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
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
    "#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_re_export_from_another_file_does_not_count() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
    r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export { marker } from './other.stylex.js';
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_renamed_re_export_from_another_file_does_not_count() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
    r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export { marker as otherMarker } from './other.stylex.js';
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_default_export_does_not_count() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
    r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export default marker;
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_renamed_export_with_as_syntax() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        )))
        .into_pass()
    },
    r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = stylex.defineMarker();
        export { marker as themeMarker };
    "#,
    r#""#,
  )
}
