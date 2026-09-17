//! A StyleX API reached through a named import rather than the namespace.
//!
//! Two things are read by name rather than through `stylex.<name>`, and each
//! one has its own list to be on. A call reaches a handler only when the
//! cycle's import-kind list names the API, and a helper written inside a
//! producer's argument folds only when that producer registered the helper's
//! local name in its function map. A namespace import answers for every API at
//! once, so either list can lose an entry without any namespace case noticing.
//!
//! Each case therefore compiles both spellings of one module and compares what
//! each printed, rather than recording what one of them printed. A snapshot of
//! the named spelling alone says nothing: a call the compiler never reached
//! prints as the author wrote it, which is a perfectly ordinary-looking
//! module.

use crate::utils::prelude::*;

use crate::utils::transform::{compiled_module, compiled_theme_module};

// Each producer that is bound to a declarator: the call has to be recognised
// under its imported name, or the module comes out holding a call the compiler
// never read and the CSS it declares is never injected.
#[test]
fn a_keyframes_call_compiles_under_its_imported_name() {
  assert_spellings_agree_but_for_the_import(
    "a keyframes call",
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const fade = stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } });
    "#,
    r#"
      import { keyframes } from '@stylexjs/stylex';
      export const fade = keyframes({ from: { opacity: 0 }, to: { opacity: 1 } });
    "#,
    compiled_module,
  );
}

#[test]
fn a_position_try_call_compiles_under_its_imported_name() {
  assert_spellings_agree_but_for_the_import(
    "a positionTry call",
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const fallback = stylex.positionTry({ positionAnchor: '--anchor', top: '0' });
    "#,
    r#"
      import { positionTry } from '@stylexjs/stylex';
      export const fallback = positionTry({ positionAnchor: '--anchor', top: '0' });
    "#,
    compiled_module,
  );
}

#[test]
fn a_view_transition_class_call_compiles_under_its_imported_name() {
  assert_spellings_agree_but_for_the_import(
    "a viewTransitionClass call",
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const transition = stylex.viewTransitionClass({ old: { opacity: 0 } });
    "#,
    r#"
      import { viewTransitionClass } from '@stylexjs/stylex';
      export const transition = viewTransitionClass({ old: { opacity: 0 } });
    "#,
    compiled_module,
  );
}

// The helpers each producer registers in its own function map. A helper that is
// not registered does not refuse -- the argument simply stops folding -- so the
// comparison is what says the entry is there.
#[test]
fn keyframes_folds_a_first_that_works_call_under_its_imported_name() {
  assert_spellings_agree_but_for_the_import(
    "firstThatWorks inside keyframes",
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const fade = stylex.keyframes({
        from: { display: stylex.firstThatWorks('grid', 'flex') },
        to: { display: 'block' },
      });
    "#,
    r#"
      import { keyframes, firstThatWorks } from '@stylexjs/stylex';
      export const fade = keyframes({
        from: { display: firstThatWorks('grid', 'flex') },
        to: { display: 'block' },
      });
    "#,
    compiled_module,
  );
}

#[test]
fn view_transition_class_folds_keyframes_and_first_that_works_under_their_imported_names() {
  assert_spellings_agree_but_for_the_import(
    "keyframes and firstThatWorks inside viewTransitionClass",
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const transition = stylex.viewTransitionClass({
        old: {
          animationName: stylex.keyframes({ from: { opacity: 1 }, to: { opacity: 0 } }),
          display: stylex.firstThatWorks('grid', 'flex'),
        },
      });
    "#,
    r#"
      import { viewTransitionClass, keyframes, firstThatWorks } from '@stylexjs/stylex';
      export const transition = viewTransitionClass({
        old: {
          animationName: keyframes({ from: { opacity: 1 }, to: { opacity: 0 } }),
          display: firstThatWorks('grid', 'flex'),
        },
      });
    "#,
    compiled_module,
  );
}

// The nested variable group builds the map that `keyframes`, `positionTry` and
// `types` share, so one case answers for all three entries at once.
#[test]
fn nested_define_vars_folds_keyframes_position_try_and_types_under_their_imported_names() {
  assert_spellings_agree_but_for_the_import(
    "keyframes, positionTry and types inside unstable_defineVarsNested",
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const vars = stylex.unstable_defineVarsNested({
        motion: { fade: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
        anchor: { fallback: stylex.positionTry({ positionAnchor: '--anchor', top: '0' }) },
        palette: { accent: stylex.types.color('red') },
      });
    "#,
    r#"
      import { unstable_defineVarsNested, keyframes, positionTry, types } from '@stylexjs/stylex';
      export const vars = unstable_defineVarsNested({
        motion: { fade: keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
        anchor: { fallback: positionTry({ positionAnchor: '--anchor', top: '0' }) },
        palette: { accent: types.color('red') },
      });
    "#,
    compiled_theme_module,
  );
}

/// A name taken from the StyleX source that is no API of it.
///
/// The source is an import path either way -- the module does import StyleX --
/// and the name binds nothing the compiler answers for, so a call written
/// against it reaches the runtime as the author wrote it. The API imported
/// beside it still compiles, which is what says the unknown name was passed
/// over rather than taking the whole import with it.
#[test]
fn a_named_import_that_is_no_api_binds_nothing() {
  let output = compiled_module(
    r#"
      import { create, notAnApi } from '@stylexjs/stylex';
      export const styles = create({ base: { color: 'red' } });
      notAnApi({ base: { color: 'red' } });
    "#,
  );

  assert!(
    output.contains("notAnApi({"),
    "the call on the unknown name is left where it was written:\n{output}"
  );
  assert!(
    output.contains("_inject2("),
    "the API imported beside it still compiled:\n{output}"
  );
}

// A name bound twice, once to an API and once to the namespace.
//
// This is not a module any bundler builds: the language forbids one name in two
// declarations, and `@babel/parser` stops on it, so the reference implementation
// never reads the source and there is nothing to compare with. The SWC parser
// reads it, so this compiler has to answer something.
//
// The name is registered for the API first, and it carries that one function
// rather than the namespace's fold of many. The registration that follows has
// no fold to write into and leaves the API where it is.
//
// The call inside the style is what shows which registration won. It folds to a
// keyframes name, so the `@keyframes` rule is injected and `animationName`
// carries the class that reads it. Had the namespace overwritten the API, the
// same call would fold to nothing.
stylex_test!(
  a_name_bound_to_an_api_and_to_the_namespace_keeps_the_api,
  |tr| build_test_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import { keyframes as stylex } from '@stylexjs/stylex';
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { animationName: stylex({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
  "#
);
