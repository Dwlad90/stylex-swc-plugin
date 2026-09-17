//! Style keys that hold a `var(--name)` reference, and keys that only look
//! like one.
//!
//! A key spelled as a variable reference names the custom property the
//! brackets wrap, so a variable set inside a `create` call is filed under
//! `--name` rather than under the text the author wrote. The question is asked
//! of the whole key, and the name class is narrow: a name holding a dash, an
//! underscore or a capital is no reference.
//!
//! Every other key keeps every character it was written with. A pseudo
//! selector, an at-rule and a property name can all carry a bracket beside a
//! non-space character, and reading one of those as a reference named the rule
//! by text cut out of its middle -- or, for a key shorter than the slice,
//! stopped the build.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0`
//! under the parity harness's options and agrees with it.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── A key that is a variable reference ──────────────────────────────

// The rule declares the custom property the reference names.
stylex_test!(
  a_variable_reference_key_declares_the_property_it_names,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { 'var(--x1a2b3)': 'blue' },
    });
  "#
);

// The conditions written under the reference are filed under the same
// property, so one key carries the whole set.
stylex_test!(
  a_variable_reference_key_carries_its_conditions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { 'var(--x1a2b3)': { default: 'blue', ':hover': 'green' } },
    });
  "#
);

// A reference written inside a condition is named there as well.
stylex_test!(
  a_variable_reference_key_inside_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { '@media (min-width: 100px)': { 'var(--x1a2b3)': 'blue' } },
    });
  "#
);

// ── A key that only looks like one ──────────────────────────────────

// A property name holding a bracket keeps every character. Reading it as a
// reference sliced from the fifth character to the last and stopped the build.
stylex_test!(
  a_property_name_holding_a_bracket_keeps_its_name,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { 'a)b': 'red' },
    });
  "#
);

// The same key carrying text outside ASCII. The slice was counted in bytes,
// so it cut through the character and stopped the build with a second report.
stylex_test!(
  a_property_name_carrying_text_outside_ascii_keeps_its_name,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { 'a)bé': 'red' },
    });
  "#
);

// A pseudo selector holds brackets, and stays a condition.
stylex_test!(
  a_pseudo_selector_holding_brackets_stays_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { ':not(.a)::before': { color: 'red' } },
    });
  "#
);

// So does an at-rule holding a bracket beside a comma.
stylex_test!(
  an_at_rule_holding_a_bracket_stays_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { '@media (min-width:1px),(max-width:2px)': { color: 'red' } },
    });
  "#
);

// A name holding a dash is outside the class the question is asked with, so
// the key is no reference to it. The shorthand expansion one layer down asks a
// wider question and names the property there.
stylex_test!(
  a_reference_naming_a_dashed_variable_is_named_one_layer_down,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { 'var(--my-colour)': 'red' },
    });
  "#
);

// The question is asked of the whole key rather than of a key that is one
// reference and nothing else, so a reference written inside an at-rule is
// named by text cut out of the middle of that at-rule. The rule this declares
// is nonsense, and it is the same nonsense the reference implementation
// declares -- which is why the question is left unanchored.
stylex_test!(
  a_reference_inside_an_at_rule_names_the_text_around_it,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { '@media (min-width: var(--x1a2b3))': { color: 'red' } },
    });
  "#
);

// A character JavaScript spells with two units counts as two, so the name is
// cut where the reference implementation cuts it. Counting characters named
// this key `(--x1a2b3)` and gave it a class name of its own.
stylex_test!(
  a_two_unit_character_before_a_reference_counts_as_two,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { '\u{1F388}var(--x1a2b3)x': 'red' },
    });
  "#
);
