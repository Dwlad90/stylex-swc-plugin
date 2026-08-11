use crate::utils::prelude::*;

fn define_marker_transform(
  comments: std::rc::Rc<swc_core::common::comments::SingleThreadedComments>,
) -> impl swc_core::ecma::ast::Pass {
  build_test_transform(comments, |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/stylex/packages/vars.stylex.js".into(),
    ))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
  })
}

stylex_test_panic!(
  must_be_bound_to_a_named_export,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const marker = stylex.defineMarker();
  "#
);

stylex_test_panic!(
  no_arguments_allowed,
  "defineMarker() should have 0 arguments.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const marker = stylex.defineMarker(1);
  "#
);

stylex_test!(
  valid_export_direct_named_export,
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const marker = stylex.defineMarker();
  "#
);

stylex_test!(
  valid_export_separate_const_and_export_statement,
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const marker = stylex.defineMarker();
    export { marker };
  "#
);

stylex_test_panic!(
  invalid_export_re_export_from_another_file_does_not_count,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const marker = stylex.defineMarker();
    export { marker } from './other.stylex.js';
  "#
);

stylex_test_panic!(
  invalid_export_renamed_re_export_from_another_file_does_not_count,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const marker = stylex.defineMarker();
    export { marker as otherMarker } from './other.stylex.js';
  "#
);

stylex_test_panic!(
  invalid_export_default_export_does_not_count,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const marker = stylex.defineMarker();
    export default marker;
  "#
);

stylex_test_panic!(
  invalid_export_renamed_export_with_as_syntax,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const marker = stylex.defineMarker();
    export { marker as themeMarker };
  "#
);

// Each call is validated against its own declaration: an unexported marker is
// still an error when an exported one precedes it in the same file.
stylex_test_panic!(
  invalid_export_unexported_marker_after_an_exported_one,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const first = stylex.defineMarker();
    const second = stylex.defineMarker();
  "#
);

// A call in a nested scope is bound to a variable, just never to an export, so
// the missing export is what it is told about.
stylex_test_panic!(
  invalid_export_marker_in_a_nested_scope,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function useMarker() {
      const marker = stylex.defineMarker();
      return marker;
    }
  "#
);

stylex_test_panic!(
  invalid_export_marker_in_a_nested_scope_after_an_exported_one,
  "The return value of defineMarker() must be bound to a named export.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const first = stylex.defineMarker();
    function useMarker() {
      const second = stylex.defineMarker();
      return second;
    }
  "#
);

// Bound to a destructuring pattern rather than a plain identifier: here it is
// the variable that is wrong, not the export. Exported or not, and whichever
// pattern it is.
stylex_test_panic!(
  invalid_binding_destructured_marker,
  "defineMarker() calls must be bound to a bare variable.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const { marker } = stylex.defineMarker();
    export { marker };
  "#
);

stylex_test_panic!(
  invalid_binding_destructured_marker_export,
  "defineMarker() calls must be bound to a bare variable.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const { marker } = stylex.defineMarker();
  "#
);

stylex_test_panic!(
  invalid_binding_array_destructured_marker_export,
  "defineMarker() calls must be bound to a bare variable.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const [ marker ] = stylex.defineMarker();
  "#
);

// Not bound to a variable at all.
stylex_test_panic!(
  invalid_binding_bare_call_statement,
  "defineMarker() calls must be bound to a bare variable.",
  |tr| define_marker_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    stylex.defineMarker();
  "#
);
