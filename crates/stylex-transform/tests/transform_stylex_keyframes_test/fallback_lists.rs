//! `firstThatWorks` inside `stylex.keyframes`.
//!
//! The keyframes handler registers `firstThatWorks` under every local name an
//! import gave it, so the call folds in a step to the same fallback list it
//! folds to in `create`. What the two positions then do with that list is not
//! the same, and both answers below are measured output of
//! `@stylexjs/babel-plugin` 0.19.0 for the same input:
//!
//! - as a step **value** the list declares nothing. An animation step holds no
//!   condition and no fallback, so a list there is a value the step cannot
//!   write, and it drops the way every other non-string step value drops.
//! - as a step **key** the list is written, because a key is a selector list
//!   and a comma joins it. `firstThatWorks` reorders what it is handed, so the
//!   last value an author writes names the first step of the selector.
//!
//! The animation name is a hash of the steps, so a name that recurs between a
//! source with the list and a source without it is the claim that the
//! declaration is really gone rather than emitted empty. What drops is the
//! list, not the helper: a step value written as a plain array drops the same
//! way, which `css_keyframes.rs` records.

use crate::utils::prelude::*;

// `@keyframes x1t391ty-B{from{}to{display:block;}}` for both — the name a
// `from` step declaring nothing produces.
stylex_test!(
  a_fallback_list_in_a_step_declares_nothing,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fallback = stylex.keyframes({
      from: { display: stylex.firstThatWorks('grid', 'flex') },
      to: { display: 'block' },
    });
    export const omitted = stylex.keyframes({
      from: {},
      to: { display: 'block' },
    });
  "#
);

// A renamed import binds the same function to another name, and the step
// answers by what the name resolves to rather than by how it is spelled. The
// name is the `x1t391ty-B` the first case above proves is the empty step.
stylex_test!(
  an_aliased_fallback_list_in_a_step_declares_nothing,
  r#"
    import { keyframes as kf, firstThatWorks as pick } from '@stylexjs/stylex';
    export const fallback = kf({
      from: { display: pick('grid', 'flex') },
      to: { display: 'block' },
    });
  "#
);

// One import can bind the same function twice. Every name it bound reads, so
// the two animations below are the same animation.
stylex_test!(
  every_name_bound_to_a_fallback_list_reads_in_a_step,
  r#"
    import { keyframes, firstThatWorks, firstThatWorks as pick } from '@stylexjs/stylex';
    export const first = keyframes({
      from: { display: firstThatWorks('grid', 'flex') },
      to: { display: 'block' },
    });
    export const second = keyframes({
      from: { display: pick('grid', 'flex') },
      to: { display: 'block' },
    });
  "#
);

// Only the declaration holding the list drops. The step keeps its siblings and
// reaches the name a step declaring only that sibling produces.
stylex_test!(
  a_fallback_list_keeps_the_declarations_beside_it,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const dropped = stylex.keyframes({
      from: { display: stylex.firstThatWorks('grid', 'flex'), opacity: 0.5 },
      to: { display: 'block' },
    });
    export const sibling = stylex.keyframes({
      from: { opacity: 0.5 },
      to: { display: 'block' },
    });
  "#
);

// A list of one is still a list, so it drops for the same reason a longer one
// does rather than collapsing to the value it holds — `x1t391ty-B` again.
stylex_test!(
  a_fallback_list_of_one_value_in_a_step_declares_nothing,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const one = stylex.keyframes({
      from: { display: stylex.firstThatWorks('grid') },
      to: { display: 'block' },
    });
  "#
);

// A shorthand expands to longhands that each hold the list, so all of them
// drop and the step declares nothing rather than a partial expansion.
stylex_test!(
  a_shorthand_fallback_list_in_a_step_declares_nothing,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fallback = stylex.keyframes({
      from: { margin: stylex.firstThatWorks('10px', '1rem') },
      to: { margin: '0' },
    });
    export const omitted = stylex.keyframes({
      from: {},
      to: { margin: '0' },
    });
  "#
);

// The same list in a step key is written, not dropped: a key is a selector
// list, so `firstThatWorks('from', 'to')` names both steps, and it names them
// as `to,from` because the helper reorders what it is handed. This is the
// position that proves the registration is read.
stylex_test!(
  a_fallback_list_as_a_step_key_names_every_step,
  r#"
    import { keyframes, firstThatWorks } from '@stylexjs/stylex';
    export const fallback = keyframes({
      [firstThatWorks('from', 'to')]: { display: 'grid' },
      to: { display: 'block' },
    });
  "#
);

// A step key is not restricted to `from` and `to`, so a list of percentages
// names every one of them, reordered the same way: `100%,50%,0%`.
stylex_test!(
  a_fallback_list_of_step_percentages_names_every_step,
  r#"
    import { keyframes, firstThatWorks } from '@stylexjs/stylex';
    export const fallback = keyframes({
      [firstThatWorks('0%', '50%', '100%')]: { opacity: 0.5 },
    });
  "#
);

// A list long enough that no author would write it still folds and joins.
// Twenty steps in one key is the wide end of what the position accepts, and it
// answers the name upstream answers rather than truncating, reordering only
// part of the list, or failing.
stylex_test!(
  a_wide_fallback_list_as_a_step_key_joins_every_value,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const wide = stylex.keyframes({
      [stylex.firstThatWorks(
        '0%', '5%', '10%', '15%', '20%', '25%', '30%', '35%', '40%', '45%',
        '50%', '55%', '60%', '65%', '70%', '75%', '80%', '85%', '90%', '95%',
      )]: { opacity: 0.5 },
      to: { opacity: 1 },
    });
  "#
);
