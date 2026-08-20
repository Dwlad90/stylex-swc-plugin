//! Edge cases around a dynamic parameter that shadows an imported binding.
//!
//! The shadowing chain in `js/evaluate/binding.rs` decides, for every reference
//! in a `create()` call, whether the name means the import or the parameter that
//! took it over. `dynamic_styles.rs` pins the shapes the chain was written for.
//! This file asks the same question where the surrounding value is hostile --
//! malformed CSS, an unknown pseudo-class, a parameter list the validator
//! refuses, extreme nesting -- so a refusal that belongs to the value cannot be
//! mistaken for a refusal that belongs to the shadowing, and neither can move
//! without a test noticing.
//!
//! Every accepting case below was measured against `@stylexjs/babel-plugin`
//! 0.19.0 under the same options and agrees with it on class names and rule
//! text. Where the two disagree the divergence is named at the test.

use crate::utils::{prelude::*, source::nest_expression, transform::stringify_js};

// ──────────────────────────────────────────────
// The value is malformed or unrecognized CSS
//
// A dynamic parameter reaches CSS generation as a custom property rather than as
// a value, so the value half of these is whatever the *other* key holds. What is
// being asked is that the parameter still resolves to the parameter while the
// value beside it is being refused or passed through.
// ──────────────────────────────────────────────

// An unclosed CSS function is refused before the shadowing is ever asked about,
// and the message names the rule rather than the reference. Babel refuses the
// same input with `Rule contains an unclosed function`.
stylex_test_panic!(
  an_unclosed_css_function_beside_a_shadowing_param,
  "Rule contains an unclosed function",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { color: 'rgb(0,0,' },
      dyn: (zIndex) => ({ zIndex }),
    });
  "#
);

// A media query cut off mid-condition is refused as a query, not as a value.
stylex_test_panic!(
  a_malformed_media_query_around_a_shadowing_param,
  "Invalid media query syntax",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (zIndex) => ({
        zIndex: {
          default: zIndex,
          '@media (min-width:': zIndex,
        },
      }),
    });
  "#
);

// An unterminated quote inside a value is not malformed to either compiler: the
// value is a string to StyleX and the quote is a character in it. Both emit the
// same rule, doubled quote and all.
stylex_test!(
  an_unterminated_quote_in_a_value_beside_a_shadowing_param,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { content: '"unterminated' },
      dyn: (zIndex) => ({ zIndex }),
    });
  "#
);

// An unknown pseudo-class is not validated against a list -- it is carried into
// the selector as written, and the parameter under it still becomes an inline
// custom property named from the key path.
stylex_test!(
  an_unknown_pseudo_class_under_a_shadowing_param,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (zIndex) => ({
        zIndex: {
          default: zIndex,
          ':hoverr': zIndex,
        },
      }),
    });
  "#
);

// A colon with no pseudo-class after it, and an at-rule with no condition.
// Neither is validated: both reach the stylesheet as written, as `.x1xt2tkc:{…}`
// and a conditionless `@media{…}` -- text no CSS parser accepts. Babel 0.19.0
// emits exactly the same two rules under the same class names, so this pins an
// agreed-upon shortcoming rather than a divergence.
stylex_test!(
  a_bare_colon_and_a_conditionless_at_rule_around_a_shadowing_param,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      bare: (zIndex) => ({ zIndex: { default: zIndex, ':': zIndex } }),
      conditionless: (zIndex) => ({ zIndex: { default: zIndex, '@media': zIndex } }),
    });
  "#
);

// ──────────────────────────────────────────────
// Unicode and escapes
// ──────────────────────────────────────────────

// A parameter whose name is spelled with a unicode escape shadows the same
// import a plain-text spelling of it would: the parser folds the escape before
// the chain compares bindings.
stylex_test!(
  a_unicode_escaped_param_name_shadows_the_same_import,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { spacing as ünïcödé } from 'spacing.stylex.js';

    export const styles = stylex.create({
      plain: { padding: ünïcödé.md },
      escaped: (ünïcödé) => ({ padding: ünïcödé }),
    });
  "#
);

// A non-ASCII custom property as the key, driven by the shadowing parameter. The
// property name travels into both the rule and the name of the inline custom
// property that feeds it.
stylex_test!(
  a_non_ascii_custom_property_driven_by_a_shadowing_param,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (zIndex) => ({ '--dépth': zIndex }),
    });
  "#
);

// A backslash escape inside a CSS value survives beside a shadowing parameter --
// it is part of the declaration text the class name hashes.
stylex_test!(
  a_css_escape_in_a_value_beside_a_shadowing_param,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (zIndex) => ({ fontFamily: '"My\\ Font"', zIndex }),
    });
  "#
);

// ──────────────────────────────────────────────
// The parameter list itself
// ──────────────────────────────────────────────

// A rest, a destructured and a defaulted parameter are all refused by the
// dynamic-style validator before resolution runs, whether or not the name they
// bind shadows an import. Byte-identical to Babel's refusal.
stylex_test_panic!(
  a_rest_param_shadowing_an_import,
  "Only named parameters are allowed in Dynamic Style functions",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (...zIndex) => ({ zIndex }),
    });
  "#
);

stylex_test_panic!(
  a_destructured_param_shadowing_an_import,
  "Only named parameters are allowed in Dynamic Style functions",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: ({ zIndex }) => ({ zIndex }),
    });
  "#
);

// The default value reads the import the parameter beside it does not shadow, so
// the refusal has to come from the parameter's shape rather than from the read.
stylex_test_panic!(
  a_defaulted_param_reading_an_import,
  "Only named parameters are allowed in Dynamic Style functions",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (level = zIndex._10) => ({ zIndex: level }),
    });
  "#
);

// An arrow with no parameter at all: every reference in it means the import, and
// the style is static in everything but its emitted shape.
stylex_test!(
  a_dynamic_style_with_no_param_reads_the_import,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: () => ({ zIndex: zIndex._10 }),
    });
  "#
);

// Sixty-four parameters, two of them read. The unread ones take their names and
// contribute nothing: only the two that are read reach the stylesheet.
#[test]
fn sixty_four_params_beside_two_reads() {
  let params = (0..64)
    .map(|i| format!("p{}", i))
    .collect::<Vec<_>>()
    .join(", ");

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{
        dyn: ({}) => ({{ zIndex: p0, order: p63 }}),
      }});
    "#,
    params
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    theme_import_transform(tr.comments.clone())
  });

  // The two read parameters each get their own custom property; the other 62
  // contribute nothing. Each of the two names is spelled three times -- in the
  // rule, in its `@property` declaration, and in the object the function
  // returns -- so six mentions means two properties and not a third.
  assert!(output.contains(".xr3buco{z-index:var(--x-zIndex)}"));
  assert!(output.contains(".xuwbzjh{order:var(--x-order)}"));
  assert_eq!(output.matches("--x-").count(), 6);
}

// ──────────────────────────────────────────────
// The parameter read through something other than a bare reference
// ──────────────────────────────────────────────

// A member read and a call on the shadowing parameter both stay parameter reads:
// the whole expression travels into the inline custom property untouched, rather
// than the base name resolving to the import it shadows.
stylex_test!(
  a_member_read_and_a_call_on_a_shadowing_param,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      member: (zIndex) => ({ zIndex: zIndex._10 }),
      called: (zIndex) => ({ zIndex: zIndex() }),
    });
  "#
);

// A parameter that shadows a StyleX helper, called. The helper is looked up in
// `functions.identifiers` before the binding chain runs, so the call folds to the
// helper's value and the parameter is never consulted -- which is what upstream
// does with the same input.
stylex_test!(
  a_shadowed_helper_called_resolves_to_the_helper,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { firstThatWorks } from '@stylexjs/stylex';

    export const styles = stylex.create({
      dyn: (firstThatWorks) => ({ fontFamily: firstThatWorks('a', 'b') }),
    });
  "#
);

// The shadowing parameter as an *argument* to a helper: the helper still folds,
// and the parameter reaches the folded value as a custom property.
stylex_test!(
  a_shadowing_param_passed_to_a_helper,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { firstThatWorks } from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (zIndex) => ({ fontFamily: firstThatWorks(zIndex, 'serif') }),
    });
  "#
);

// Spread and a computed key both refuse. Babel refuses them too, with
// `Only static values are allowed inside of a create() call.` where we say
// `Referenced constant is not defined.` -- the outcome agrees, the text does
// not. Pinned here so the refusal cannot quietly turn into an emission.
stylex_test_panic!(
  a_shadowing_param_spread_into_the_style,
  "Referenced constant is not defined",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (zIndex) => ({ ...zIndex }),
    });
  "#
);

stylex_test_panic!(
  a_shadowing_param_as_a_computed_key,
  "Referenced constant is not defined",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (zIndex) => ({ [zIndex]: 1 }),
    });
  "#
);

// ──────────────────────────────────────────────
// Nothing to emit
// ──────────────────────────────────────────────

// An empty object, and a property whose value is `null`. Both leave the dynamic
// style with no declaration to carry, and the import beside it is still read.
stylex_test!(
  a_shadowing_param_with_nothing_to_emit,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      empty: (zIndex) => ({}),
      nulled: (zIndex) => ({ zIndex: null }),
    });
  "#
);

// ──────────────────────────────────────────────
// Boundary conditions
//
// Generated rather than written out, because the point is the size. Both are
// measured against Babel 0.19.0 at the same sizes, which accepts them too.
// ──────────────────────────────────────────────

// 128 levels of nested conditions, every level reading the shadowing parameter.
// One rule and one custom property per level, and no stack exhaustion on the way
// down or back up.
#[test]
fn a_hundred_and_twenty_eight_nested_conditions_read_a_shadowing_param() {
  const DEPTH: usize = 128;

  let mut nested = String::from("zIndex");
  for level in (1..=DEPTH).rev() {
    nested = format!(
      "{{ default: zIndex, '@media (min-width: {}px)': {} }}",
      level, nested
    );
  }

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      import {{ zIndex }} from 'zIndex.stylex.js';
      export const styles = stylex.create({{
        wrapper: {{ zIndex: zIndex._10 }},
        dyn: (zIndex) => ({{ zIndex: {} }}),
      }});
    "#,
    nested
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    theme_import_transform(tr.comments.clone())
  });

  // One `@property` per level, plus one for the innermost leaf.
  assert_eq!(output.matches("@property --x-").count(), DEPTH + 1);
  // The innermost query is reached, so the descent completed.
  assert!(output.contains("@media (min-width: 128px)"));
  // And the import beside it is still a theme reference, not the parameter.
  assert!(output.contains(".x145lhke{z-index:var(--x1t53vvn)}"));
}

// A five-thousand-character value beside a shadowing parameter. The value is
// hashed whole -- `x1ahcjaz` is the hash of all five thousand characters, and
// asserting the class name is what makes that a claim rather than a hope.
#[test]
fn a_five_thousand_character_value_beside_a_shadowing_param() {
  let long = "x".repeat(5000);

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      import {{ zIndex }} from 'zIndex.stylex.js';
      export const styles = stylex.create({{
        wrapper: {{ content: '"{}"' }},
        dyn: (zIndex) => ({{ zIndex }}),
      }});
    "#,
    long
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    theme_import_transform(tr.comments.clone())
  });

  assert!(output.contains(&format!(".x1ahcjaz{{content:\"{}\"}}", long)));
  assert!(output.contains(".xr3buco{z-index:var(--x-zIndex)}"));
}

// 256 levels of arithmetic around the shadowing parameter fold to a single
// custom property in both compilers.
//
// This depth is the shadowing question; the depth *limit* is not, and it is not
// asked here. The evaluator's ceiling and what happens on either side of it --
// including this same shape at 576 levels, which upstream refuses and this
// compiler folds -- are pinned in `evaluation_depth_budget.rs`, because nothing
// in the limit turned out to depend on the shadowing.
#[test]
fn two_hundred_and_fifty_six_levels_of_arithmetic_around_a_shadowing_param() {
  const DEPTH: usize = 256;

  let expr = nest_expression("(", " + 1)", "zIndex", DEPTH);

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      import {{ zIndex }} from 'zIndex.stylex.js';
      export const styles = stylex.create({{
        dyn: (zIndex) => ({{ zIndex: {} }}),
      }});
    "#,
    expr
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    theme_import_transform(tr.comments.clone())
  });

  // The whole tower collapses into one inline custom property, exactly as a
  // single `zIndex + 1` does -- the depth buys no extra declarations.
  assert!(output.contains(".xr3buco{z-index:var(--x-zIndex)}"));
  assert_eq!(output.matches("@property --x-").count(), 1);
}
