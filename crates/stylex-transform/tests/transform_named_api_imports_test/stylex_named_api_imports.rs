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

/// Asserts the two spellings of one module compile to the same body.
///
/// The import line is the one line they are meant to differ in, so it is the
/// one line left out. That makes the check blind to a difference confined to an
/// import, which is true of no input here: the compiler writes one import of
/// its own, and it writes `var _inject2 = _inject;` beside it, so a spelling
/// that injected nothing still reads as different.
///
/// A comparison of two spellings would also hold if neither of them compiled,
/// so the namespace spelling is anchored first: it has to have injected
/// something for the comparison to say anything at all.
#[track_caller]
fn assert_spellings_agree_but_for_the_import(
  shape: &str,
  namespace: &str,
  named: &str,
  compile: impl Fn(&str) -> String,
) {
  let body = |input: &str| {
    compile(input)
      .lines()
      .filter(|line| !line.trim_start().starts_with("import "))
      .collect::<Vec<_>>()
      .join("\n")
  };

  let from_namespace = body(namespace);

  assert!(
    from_namespace.contains("_inject2("),
    "{shape} injected nothing under the namespace spelling, so there is nothing to compare:\n\
     {from_namespace}"
  );
  assert_eq!(
    from_namespace,
    body(named),
    "{shape} compiles to something else when it is read by name.\n\
     namespace:\n{namespace}\nnamed:\n{named}"
  );
}

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
