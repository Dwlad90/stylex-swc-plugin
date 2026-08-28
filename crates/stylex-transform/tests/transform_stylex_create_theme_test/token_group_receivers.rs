//! A `defineVars` group reaching a fold as a value rather than through one of
//! its members.
//!
//! A token group is this compiler's own value and has no JavaScript form, so it
//! used to stop every fold it appeared in. Its own `toString` is the exception:
//! the variable-group hash is read off the reference itself and mutates nothing,
//! so the group crosses into the engine as that string and the whole of
//! `Array.prototype` folds on it.
//!
//! The rule that keeps it honest is here too. A string has none of the group's
//! members, so an expression that reads a property as a value is handed back
//! rather than folded — the dispatch below resolves the member this compiler's
//! own way, which is a narrower answer rather than a wrong one.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0` with
//! the same options and the same file layout.

use crate::utils::prelude::*;
use swc_core::common::FileName;

fn virtual_app_path(rel: &str) -> String {
  format!(
    "{}/tests/__virtual__/app/{}",
    env!("CARGO_MANIFEST_DIR"),
    rel
  )
}

fn stylex_transform(comments: TestComments) -> impl Pass {
  let filename = virtual_app_path("src/components/Card.js");
  let root_dir = virtual_app_path("");

  build_test_transform(comments, move |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real(filename.clone().into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(root_dir)))
  })
}

// --- The group as an element -----------------------------------------------

// The three spellings of one array holding the group, which have to agree: a
// literal, the same literal behind a name, and the one `Array()` builds. Each
// joins to the variable-group hash, `x13pcrg7`.
stylex_test!(
  a_joined_token_group_reads_its_own_to_string,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    const named = [colors];
    export const styles = stylex.create({
      literal: { color: [colors].join('-') },
      named: { color: named.join('-') },
      built: { color: Array(colors).join('-') },
    });
  "#
);

// The separator is not the subject: the default join and `toString` answer the
// same hash, because one element joins to itself whatever sits between them.
stylex_test!(
  a_token_group_joins_the_same_without_a_separator,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      defaulted: { color: [colors].join() },
      stringified: { color: [colors].toString() },
    });
  "#
);

// The rest of the prototype, on a receiver holding the group: a chain that ends
// in a join, a callback that hands each element back, and the length, which
// never reads an element at all. `.x2hmdj7{color:x13pcrg7-a}` for the chain.
stylex_test!(
  the_array_prototype_folds_on_a_token_group,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      chained: { color: [colors].concat(['a']).join('-') },
      mapped: { color: [colors].map(x => x).join('-') },
      counted: { color: String([colors].length) },
    });
  "#
);

// More than one group in one array, and a group beside a plain value, so the
// join is shown to render each element through its own `toString` rather than
// through the array's.
stylex_test!(
  several_token_groups_join_element_by_element,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      repeated: { color: [colors, colors].join('-') },
      mixed: { color: [colors, 'solid', 1].join(' ') },
    });
  "#
);

// --- Where the string it crossed as would be read as the group --------------

// A member read off the *result* of a fold. `Object()` hands its argument back,
// so what the member lands on is the group itself — the fold hands the call back
// rather than answering an object, and the dispatch below resolves the member.
stylex_test!(
  a_member_read_off_a_folded_token_group_still_resolves,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      wrapped: { color: Object(colors).primary },
      indexed: { color: String([colors][0].primary) },
    });
  "#
);

// A member read inside a callback. The element the callback is handed is the
// string the group crossed as, which has no `primary`, so folding it would write
// `undefined` into a declaration. Refused instead.
//
// Upstream folds this to `var(--x17y9eti)`, holding the group as a live object
// its evaluator can read any member off. A written divergence, in the safe
// direction: a refused build never names a class the other build does not
// define, where a wrong value is silent.
stylex_test_panic!(
  a_member_read_inside_a_callback_is_rejected,
  "Cannot fold 'map' at compile time.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      root: { color: [colors].map(x => x.primary).join('-') },
    });
  "#
);

// --- What the ceilings still bound -----------------------------------------

// A group crossing inward is a string like any other and is counted like one, so
// the ceiling refuses it where it refuses any other carried value — before the
// join is built, and naming the binding whose value was too large rather than
// the join it was headed for. The ceiling is configurable; raising it past what
// the value needs folds the same source.
stylex_test_panic!(
  a_token_group_carried_past_the_ceiling_is_rejected,
  "Cannot carry the value of 'colors' into a fold.",
  |tr| {
    let filename = virtual_app_path("src/components/Card.js");
    let root_dir = virtual_app_path("");

    build_test_transform(tr.comments.clone(), move |b| {
      b.with_runtime_injection()
        .with_filename(FileName::Real(filename.clone().into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(root_dir)))
        .with_max_folded_characters(4)
    })
  },
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from '@design-system/tokens/src/colors.stylex';
    export const styles = stylex.create({
      root: { color: [colors, colors, colors].join('-') },
    });
  "#
);
