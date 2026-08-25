//! A mutating method, and what it disqualifies.
//!
//! Sorting or pushing a list folds. It was refused here on the reasoning that
//! matching the reference compiler would carry mutation into an otherwise pure
//! evaluation. Measured, that reasoning does not survive: the reference
//! compiler does not refuse mutating methods at all. It folds them on any
//! receiver not reachable by name, and disqualifies the **binding** instead —
//! its mutation test walks a binding's references with no position check, so a
//! binding a mutating method touches is dead for the whole file in both
//! directions, including reads that come *before* the mutation.
//!
//! So the engine only ever mutates a temporary nothing can name afterwards,
//! which is unobservable, and the rule that does the work already exists in
//! binding resolution. This file is the pair: what folds now, and what the
//! binding rule still refuses. Every class name and rule text below is measured
//! output of `@stylexjs/babel-plugin` 0.19.0 under the same options.

use crate::utils::{
  prelude::*,
  transform::{create_module as module, fold_module as fold},
};

// ──────────────────────────────────────────────
// What folds
// ──────────────────────────────────────────────

/// The three shapes the divergence was reported as, each folded to the
/// reference compiler's own class name.
#[test]
fn a_mutating_method_on_a_value_written_out_folds() {
  let output = fold(&module(
    "",
    "transitionProperty: ['b', 'a'].sort().join(','), zIndex: ['a', 'b'].push('c'),",
  ));

  assert!(output.contains(".x1iq4t92{transition-property:a,b}"));
  assert!(output.contains(".xzkaem6{z-index:3}"));
}

/// The receiver is the result of another call rather than a written literal, so
/// no binding is involved and the mutation is unobservable for the same reason.
#[test]
fn a_mutating_method_on_an_intermediate_value_folds() {
  let output = fold(&module(
    "",
    "transitionProperty: 'b,a'.split(',').sort().join(','), content: ['a', 'b'].reverse().join('-'),",
  ));

  assert!(output.contains(".x1iq4t92{transition-property:a,b}"));
  assert!(output.contains(".x1y9cpk8{content:\"b-a\"}"));
}

// ──────────────────────────────────────────────
// What the binding rule refuses
//
// The mutation is what disqualifies the name, so every case here is the same
// value folding when it is written out and refusing when it is given a name the
// mutation reaches.
// ──────────────────────────────────────────────

/// A binding a mutating method touches stops folding — the read is below the
/// mutation, which is the ordering an author would expect to matter.
///
/// The sentence is the receiver's refusal reported at the call, which is what
/// the reference compiler writes for this input too: it names the node the
/// value was read through, and the mutated binding is the reason underneath it.
/// `a_mutated_binding_read_without_a_call_names_the_rule` below is the same
/// module with the call taken away, where both compilers name the rule instead.
#[test]
#[should_panic(expected = "base > transitionProperty > Unsupported expression: CallExpression")]
fn a_binding_a_mutating_method_touches_stops_folding() {
  fold(&module(
    "const parts = ['b', 'a'];\nparts.sort();",
    "transitionProperty: parts.join(','),",
  ));
}

/// And so does a read that appears *above* the mutation. The reference
/// compiler's mutation test has no position check, so the binding is dead for
/// the whole file; agreeing matters because a disagreement here emits a class
/// the other build never defines.
#[test]
#[should_panic(expected = "base > transitionProperty > Unsupported expression: CallExpression")]
fn a_read_before_the_mutation_stops_folding_too() {
  fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      const parts = ['b', 'a'];
      export const styles = stylex.create({
        base: { transitionProperty: parts.join(',') },
      });
      parts.sort();
    "#,
  );
}

/// Reassignment is the separate rule beside it, and refuses the same way.
#[test]
#[should_panic(expected = "base > transitionProperty > Unsupported expression: CallExpression")]
fn a_reassigned_binding_stops_folding() {
  fold(&module(
    "let parts = ['b', 'a'];\nparts = ['c'];",
    "transitionProperty: parts.join(','),",
  ));
}

/// The mutation reaches the binding through a member chain as well as through
/// the receiver itself, which is the shape a nested theme object is.
#[test]
#[should_panic(expected = "base > transitionProperty > Unsupported expression: CallExpression")]
fn a_mutation_further_down_a_member_chain_stops_folding() {
  fold(&module(
    "const theme = { parts: ['b', 'a'] };\ntheme.parts.sort();",
    "transitionProperty: theme.parts.join(','),",
  ));
}

/// The rule itself, read where no call stands between the author and it: both
/// compilers name the mutated binding. Without this the four cases above would
/// pin only that the value stopped folding, not that it stopped folding for
/// the reason this file is about.
#[test]
#[should_panic(expected = "base > flexGrow > Referenced value is not a constant")]
fn a_mutated_binding_read_without_a_call_names_the_rule() {
  fold(&module(
    "const parts = ['b', 'a'];\nparts.sort();",
    "flexGrow: parts.length,",
  ));
}

/// The guard on all of the above: an untouched binding folds. Without it every
/// refusal here could be satisfied by a compiler that resolves no binding at
/// all.
#[test]
fn an_untouched_binding_beside_them_still_folds() {
  let output = fold(&module(
    "const sep = ', ';",
    "transitionProperty: ['opacity', 'color'].map(p => p).join(sep),",
  ));

  assert!(output.contains(".x1mz1wvm{transition-property:opacity,color}"));
}
