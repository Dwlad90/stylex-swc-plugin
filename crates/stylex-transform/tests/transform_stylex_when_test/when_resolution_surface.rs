//! Every route a `when` call takes to the function that answers it.
//!
//! `when` is not resolved like the other StyleX helpers. `keyframes`,
//! `firstThatWorks` and `positionTry` are each registered as a whole config
//! under their own name, so a reference finds one entry. `when` is registered
//! as a *map* under the namespace's name — the [folded function
//! map](../../CONTEXT.md) — and a call has to walk that map to reach the
//! function: the member step reads the entry in the map's own form, and the call
//! step calls what it finds. Materialize the entry into an object anywhere along
//! the way and the call step has nothing to call.
//!
//! That walk is the reason this file exists. The map's value type is read by the
//! member step, the call step, the spread arm and the own-keys classification,
//! and a change to it that keeps the *value* positions working can still break
//! the *callee* position — where the symptom is not a failed build but a
//! different class name, since a `when` key that resolved differently hashes a
//! different selector.
//!
//! So: all five methods, through every import spelling, plus the marker
//! argument that makes the call two-argument. Measured against
//! `@stylexjs/babel-plugin@0.19.0`. Runtime injection is on so each snapshot
//! records the selector beside the class name — the selector is what a
//! mis-resolved entry would move.

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

// ── All five methods, one spelling at a time ────────────────────────

// The whole map, reached by a named import. Two of the five were pinned before
// this file; `descendant`, `siblingAfter` and `anySibling` were reachable and
// unguarded, so a map that lost them would have failed nothing.
stylex_test!(
  every_when_method_through_a_named_import,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { when, create } from '@stylexjs/stylex';

    export const styles = create({
      c: {
        color: {
          default: 'black',
          [when.ancestor(':hover')]: 'red',
          [when.descendant(':focus')]: 'green',
          [when.siblingBefore(':active')]: 'blue',
          [when.siblingAfter(':checked')]: 'teal',
          [when.anySibling(':disabled')]: 'grey',
        },
      },
    });
  "#
);

// The same five off the namespace, which is the route that walks the map: the
// member step resolves `stylex.when` to the map entry and the call step calls
// it.
stylex_test!(
  every_when_method_through_the_namespace,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      c: {
        color: {
          default: 'black',
          [stylex.when.ancestor(':hover')]: 'red',
          [stylex.when.descendant(':focus')]: 'green',
          [stylex.when.siblingBefore(':active')]: 'blue',
          [stylex.when.siblingAfter(':checked')]: 'teal',
          [stylex.when.anySibling(':disabled')]: 'grey',
        },
      },
    });
  "#
);

// The string-key spelling of the same walk. One property in the language, so it
// must reach the same entry — a lookup that recognised only the identifier
// spelling answered the object the fold stands for instead, which is a fold that
// cannot be called.
stylex_test!(
  every_when_method_through_a_string_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      c: {
        color: {
          default: 'black',
          [stylex['when'].ancestor(':hover')]: 'red',
          [stylex['when'].descendant(':focus')]: 'green',
          [stylex['when'].siblingBefore(':active')]: 'blue',
          [stylex['when'].siblingAfter(':checked')]: 'teal',
          [stylex['when'].anySibling(':disabled')]: 'grey',
        },
      },
    });
  "#
);

// An aliased namespace and an aliased named import, so the map is found under a
// name the compiler did not choose.
stylex_test!(
  every_when_method_through_an_alias,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as sx from '@stylexjs/stylex';
    import { when as w } from '@stylexjs/stylex';

    export const styles = sx.create({
      namespaceAlias: {
        color: {
          default: 'black',
          [sx.when.descendant(':focus')]: 'green',
          [sx.when.anySibling(':disabled')]: 'grey',
        },
      },
      importAlias: {
        color: {
          default: 'black',
          [w.siblingAfter(':checked')]: 'teal',
          [w.ancestor(':hover')]: 'red',
        },
      },
    });
  "#
);

// ── The two-argument call ───────────────────────────────────────────

// A custom marker makes the call two-argument, which is what `when` alone among
// the helpers takes. Pinned on every method rather than on one, since the marker
// is threaded through the same entry the method name selects.
stylex_test!(
  every_when_method_with_a_same_file_marker,
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

    export const marker = stylex.defineMarker();

    export const styles = stylex.create({
      c: {
        color: {
          default: 'black',
          [stylex.when.ancestor(':hover', marker)]: 'red',
          [stylex.when.descendant(':focus', marker)]: 'green',
          [stylex.when.siblingBefore(':active', marker)]: 'blue',
          [stylex.when.siblingAfter(':checked', marker)]: 'teal',
          [stylex.when.anySibling(':disabled', marker)]: 'grey',
        },
      },
    });
  "#
);

// ── The walk under a nesting level ──────────────────────────────────

// The same call one level down, where the key path the fold is reached through
// is longer. A resolution that depended on being at the top of a value would
// pass every case above and fail here.
stylex_test!(
  a_when_call_nested_under_an_at_rule,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      c: {
        color: {
          '@media (min-width: 100px)': {
            default: 'black',
            [stylex.when.ancestor(':hover')]: 'red',
            [stylex.when.anySibling(':focus')]: 'green',
          },
        },
      },
    });
  "#
);

// Two properties reading the map in one namespace, and the namespace read for
// `create` in the same module. The map is shared, so a lookup that consumed the
// entry rather than reading it would answer the first caller and fail the second.
stylex_test!(
  the_map_is_read_more_than_once_per_module,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      first: {
        color: { default: 'black', [stylex.when.ancestor(':hover')]: 'red' },
        backgroundColor: { default: 'white', [stylex.when.ancestor(':hover')]: 'blue' },
      },
      second: {
        color: { default: 'black', [stylex.when.descendant(':focus')]: 'green' },
      },
    });
  "#
);
