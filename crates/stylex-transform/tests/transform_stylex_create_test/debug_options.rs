use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_debug(true)
        .with_enable_debug_class_names(true)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  adds_debug_data,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/html/js/components/Foo.react.js".into(),
    ))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      foo: {
        color: 'red'
      },
      'bar-baz': {
        display: 'block'
      },
      1: {
        fontSize: '1em'
      }
    });
  "#
);

stylex_test!(
  adds_debug_data_for_npm_packages,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into(),
    ))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      foo: {
        color: 'red'
      },
      'bar-baz': {
        display: 'block'
      },
      1: {
        fontSize: '1em'
      }
    });
  "#
);

stylex_test!(
  adds_debug_data_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/html/js/components/Foo.react.js".into(),
    ))
    .with_unstable_module_resolution(ModuleResolution {
      root_dir: None,
      theme_file_extension: None,
      ..ModuleResolution::haste(None)
    })
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      foo: {
        color: 'red'
      },
      'bar-baz': {
        display: 'block'
      },
      1: {
        fontSize: '1em'
      }
    });
  "#
);

stylex_test!(
  adds_debug_data_for_npm_packages_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/node_modules/npm-package/dist/components/Foo.react.js".into(),
    ))
    .with_unstable_module_resolution(ModuleResolution {
      root_dir: None,
      theme_file_extension: None,
      ..ModuleResolution::haste(None)
    })
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      foo: {
        color: 'red'
      },
      'bar-baz': {
        display: 'block'
      },
      1: {
        fontSize: '1em'
      }
    });
  "#
);

// ──────────────────────────────────────────────
// Dev class names follow `dev`
//
// `enableDevClassNames` defaults to the value of `dev`, so a dev build names
// each namespace after its file and variable unless the option turns it off.
// A dynamic entry keeps its name too, in the static fragment the compiler
// hoists for it.
// ──────────────────────────────────────────────

/// A dev build with no debug options set, reading the authored text so the
/// debug lines are the ones the author sees. Tree-shake compensation is on
/// because the reference output these snapshots were checked against keeps the
/// import. The tester's fields come by value, so the pass does not borrow the
/// tester it outlives.
fn dev_transform(
  comments: TestComments,
  source_map: std::sync::Arc<swc_core::common::SourceMap>,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, move |b| {
    customize(
      b.with_source_map(source_map)
        .with_dev(true)
        .with_treeshake_compensation(true)
        .with_filename(swc_core::common::FileName::Real("MyComponent.js".into())),
    )
  })
}

stylex_test!(
  a_dev_build_names_each_namespace_by_default,
  |tr| dev_transform(tr.comments.clone(), tr.cm.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: 'red' },
      dyn: (width) => ({ width }),
    });
  "#
);

stylex_test!(
  a_dev_build_names_no_namespace_when_the_option_is_off,
  |tr| dev_transform(tr.comments.clone(), tr.cm.clone(), |b| {
    b.with_enable_dev_class_names(false)
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: 'red' },
      dyn: (width) => ({ width }),
    });
  "#
);
