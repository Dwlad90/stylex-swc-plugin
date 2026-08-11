use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_treeshake_compensation(true)
        .with_unstable_module_resolution(ModuleResolution::haste(None))
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  when_ancestor_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { when, create } from '@stylexjs/stylex';

    const styles = create({
      container: {
        backgroundColor: {
          default: 'blue',
          [when.ancestor(':hover')]: 'red',
        },
      },
    });

    console.log(styles.container);
  "#
);

stylex_test!(
  when_sibling_before_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { when, create } from '@stylexjs/stylex';

    const styles = create({
      container: {
        backgroundColor: {
          default: 'blue',
          [when.siblingBefore(':focus')]: 'red',
        },
      },
    });

    console.log(styles.container);
  "#
);

stylex_test!(
  when_functions_namespace_imports,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      container: {
        backgroundColor: {
          default: 'blue',
          [stylex.when.ancestor(':hover')]: 'red',
          [stylex.when.siblingBefore(':focus')]: 'green',
          [stylex.when.anySibling(':active')]: 'yellow',
          [stylex.when.siblingAfter(':focus')]: 'purple',
          [stylex.when.descendant(':focus')]: 'orange',
        },
      },
    });

    console.log(styles.container);
  "#
);

stylex_test!(
  when_functions_aliased_imports,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { when as w, create } from '@stylexjs/stylex';

    const styles = create({
      container: {
        backgroundColor: {
          default: 'blue',
          [w.ancestor(':hover')]: 'red',
          [w.siblingBefore(':focus')]: 'green',
        },
      },
    });

    console.log(styles.container);
  "#
);

// Mirrors the `named import of custom marker` case of the reference
// implementation's when-function suite: the selector must observe the
// marker's own generated class, not the default one.
stylex_test!(
  when_ancestor_with_imported_custom_marker,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_filename(
    swc_core::common::FileName::Real(
      format!("{}/test.js", std::env::current_dir().unwrap().display()).into()
    )
  )),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { customMarker } from 'custom-marker.stylex';

    const styles = stylex.create({
      foo: {
        backgroundColor: {
          default: 'blue',
          [stylex.when.ancestor(':hover', customMarker)]: 'red',
        },
      },
    });

    const container = stylex.props(customMarker);
    const classNames = stylex.props(styles.foo);

    console.log(container, classNames);
  "#
);

// A marker consumed by the same file that defines it, where it has already
// been replaced by its compiled `$$css` object before `create` evaluates it.
stylex_test!(
  when_ancestor_with_same_file_custom_marker,
  |tr| build_test_transform(tr.comments.clone(), |b| b
    .with_treeshake_compensation(true)
    .with_runtime_injection()
    .with_cwd(std::path::PathBuf::from("/stylex/packages/"))
    .with_filename(swc_core::common::FileName::Real(
      "/stylex/packages/vars.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution {
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    })),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const localMarker = stylex.defineMarker();

    export const styles = stylex.create({
      foo: {
        backgroundColor: {
          default: 'blue',
          [stylex.when.ancestor(':hover', localMarker)]: 'red',
        },
      },
    });
  "#
);

// A marker that evaluates to null or undefined is the same as no marker at
// all, so the selector falls back to the prefixed default marker rather than
// treating the value as an unresolvable marker.
stylex_test!(
  when_ancestor_with_null_marker,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      container: {
        color: {
          default: 'blue',
          [stylex.when.ancestor(':hover', null)]: 'red',
        },
      },
    });

    console.log(styles.container);
  "#
);

stylex_test!(
  when_ancestor_with_undefined_marker,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      container: {
        color: {
          default: 'blue',
          [stylex.when.ancestor(':hover', undefined)]: 'red',
        },
      },
    });

    console.log(styles.container);
  "#
);

// A second argument that evaluates to a plain object is neither a marker, a
// class name nor a compiled `$$css` style. The reference implementation lets
// it fall through to an unprefixed `default-marker` rather than rejecting it,
// so the compiler does too — while warning, since no element carries that
// class. Covers the fallback branch of `resolve_when_marker`.
stylex_test!(
  when_ancestor_with_unresolvable_marker,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      container: {
        color: {
          default: 'blue',
          [stylex.when.ancestor(':hover', { notAMarker: 1 })]: 'red',
        },
      },
    });

    console.log(styles.container);
  "#
);
