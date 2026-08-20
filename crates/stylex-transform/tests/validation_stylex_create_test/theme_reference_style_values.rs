//! A theme reference read where a style value belongs.
//!
//! A `defineVars` group imported from another file evaluates to a theme
//! reference, which carries no expression form: the CSS a style value needs is
//! written by a *member* read off it (`zIndex.ten` is `var(--x1ew7r74)`), and the
//! group itself stands for the whole file. Read without that member access the
//! object evaluator used to answer "no value", and the caller read that as "no
//! property" -- so the declaration disappeared, with no rule, no error and no
//! warning. Every shape below compiled to nothing before the refusal below
//! existed.
//!
//! Measured against `@stylexjs/babel-plugin` 0.19.0 under `haste` resolution and
//! one source string, the parity harness's configuration. The reference
//! implementation folds the same import to an object its namespace validation
//! refuses -- it is not a plain object -- so the message is its own, and every
//! refusal here reads it. Where the two disagree the divergence is named at the
//! test.

use crate::utils::prelude::*;

// ──────────────────────────────────────────────
// The reported shape, and the positions around it
//
// A style value is refused by the same sentence wherever it is written: on a
// property, under a condition, on a shorthand, on a custom property. The key
// path in front of the sentence is this compiler's own addition and is why the
// expectations differ in length rather than in wording.
// ──────────────────────────────────────────────

stylex_test_panic!(
  a_bare_theme_import_read_as_a_style_value_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { zIndex: zIndex } });
  "#
);

// The sibling matters: it is the declaration that used to be emitted *alone*,
// which is what made the drop look like a compiling module rather than a bug.
stylex_test_panic!(
  a_bare_theme_import_beside_a_static_sibling_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { color: 'red', zIndex: zIndex } });
  "#
);

stylex_test_panic!(
  a_bare_theme_import_under_default_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { zIndex: { default: zIndex } } });
  "#
);

stylex_test_panic!(
  a_bare_theme_import_under_a_pseudo_class_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: { zIndex: { default: 1, ':hover': zIndex } },
    });
  "#
);

// A shorthand: the value is read before the shorthand expands, so the refusal is
// the value's and not the expansion's.
stylex_test_panic!(
  a_bare_theme_import_on_a_shorthand_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { margin: zIndex } });
  "#
);

// A custom property, which skips the property-name validation an authored
// longhand goes through and reaches the value with nothing else refused first.
stylex_test_panic!(
  a_bare_theme_import_on_a_custom_property_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { '--my-var': zIndex } });
  "#
);

// A dynamic style's body reads the same value through a different consumer --
// `evaluate_stylex_create_arg` rather than the object evaluator -- and used to
// report a sentence about a static expression, which named the evaluator's own
// vocabulary rather than the input. Both consumers now read the same sentence.
stylex_test_panic!(
  a_bare_theme_import_inside_a_dynamic_style_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (a) => ({ zIndex: zIndex, color: a }),
    });
  "#
);

stylex_test_panic!(
  a_bare_theme_import_under_a_condition_inside_a_dynamic_style_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (a) => ({ zIndex: { default: a, ':hover': zIndex } }),
    });
  "#
);

stylex_test_panic!(
  a_bare_theme_import_on_a_custom_property_inside_a_dynamic_style_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (a) => ({ '--my-var': zIndex, color: a }),
    });
  "#
);

// ──────────────────────────────────────────────
// Nesting
//
// A condition tree is walked to the leaf before the value is read, so depth
// changes the key path the message carries and nothing else. Both compilers
// refuse at the leaf.
// ──────────────────────────────────────────────

// The key path stops one key short of the leaf: the condition the value is
// written under is the one the message does not name, here and at every depth.
stylex_test_panic!(
  a_bare_theme_import_two_conditions_deep_is_refused,
  "w > zIndex > :focus > A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: {
        zIndex: {
          default: 1,
          ':focus': { default: 2, ':hover': zIndex },
        },
      },
    });
  "#
);

stylex_test_panic!(
  a_bare_theme_import_five_conditions_deep_is_refused,
  ":hover > :active > :focus > A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: {
        zIndex: {
          default: 1,
          '@media (min-width: 100px)': {
            default: 2,
            ':hover': {
              default: 3,
              ':active': { default: 4, ':focus': { default: 5, '@supports (color: red)': zIndex } },
            },
          },
        },
      },
    });
  "#
);

// ──────────────────────────────────────────────
// Hostile values beside the theme reference
//
// A value that cannot be written down is refused while the object is being
// evaluated, before any CSS is parsed -- so the theme reference is what the
// build stops on even when the declaration beside it is malformed. The
// reference implementation stops on the same one, for the same reason: its
// namespace validation runs before its CSS ever does.
// ──────────────────────────────────────────────

stylex_test_panic!(
  a_theme_reference_beside_an_unclosed_css_function_refuses_as_a_value,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: { color: 'rgb(0,0,', zIndex: zIndex },
    });
  "#
);

stylex_test_panic!(
  a_theme_reference_beside_an_unterminated_quote_refuses_as_a_value,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: { content: '"unterminated', zIndex: zIndex },
    });
  "#
);

stylex_test_panic!(
  a_theme_reference_under_a_media_query_cut_off_mid_condition_refuses_as_a_value,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: { zIndex: { default: 1, '@media (min-width:': zIndex } },
    });
  "#
);

stylex_test_panic!(
  a_theme_reference_under_an_unclosed_attribute_selector_refuses_as_a_value,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: { zIndex: { default: 1, '[data-x': zIndex } },
    });
  "#
);

// ──────────────────────────────────────────────
// How the name is spelled
//
// The refusal is about what the name resolves to, so no spelling of the import
// escapes it: a unicode identifier and a string specifier resolve to the same
// theme reference as a plain one. Both are how a name reaches the evaluator
// without an identifier the source ever spells.
// ──────────────────────────────────────────────

stylex_test_panic!(
  a_unicode_named_theme_import_read_as_a_style_value_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zÍndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { zIndex: zÍndex } });
  "#
);

stylex_test_panic!(
  an_escaped_string_specifier_read_as_a_style_value_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { 'zIndex' as zi } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { zIndex: zi } });
  "#
);

// A namespace import of the same file resolves to a theme reference here and to
// nothing at all upstream, which refuses it with `Referenced constant is not
// defined.` Both refuse; only the wording differs, and reaching upstream's would
// mean giving up a resolution that member reads rely on. What a namespace import
// of a theme file means is
// `.scratch/fix_dynamic-param-shadows-import/issues/11-refuse-a-namespace-theme-import.md`.
stylex_test_panic!(
  a_namespace_theme_import_read_as_a_style_value_is_refused_as_a_value,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { zIndex: t } });
  "#
);

// ──────────────────────────────────────────────
// Neighbouring positions, refused before this seam and still refused
// ──────────────────────────────────────────────

// A fallback chain holding the theme reference is refused as an array, by the
// array-specific message the reference implementation uses for the same input.
stylex_test_panic!(
  a_theme_reference_in_a_fallback_array_is_refused_as_an_array,
  "A style array value can only contain strings or numbers.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { zIndex: [zIndex, 1] } });
  "#
);

// A whole namespace written as the theme reference is refused one level up, by
// the namespace check, with the sentence both compilers use.
stylex_test_panic!(
  a_theme_reference_written_as_a_namespace_is_refused_as_a_namespace,
  "A StyleX namespace must be an object.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: zIndex });
  "#
);

// A computed key. Upstream coerces the theme reference to its group hash and
// declares a property named after it -- `.x12l9qay{x1q8i56t:1px}`, which is not
// a property at all. Refused here rather than reproduced.
stylex_test_panic!(
  a_theme_reference_read_as_a_computed_key_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { [zIndex]: 1 } });
  "#
);

// A spread of the theme reference. Upstream answers an empty namespace, because
// the object it folds to has no own enumerable properties; here the spread is
// refused because its properties cannot be read, which is the same answer stated
// rather than assumed. Recorded because it is the one shape where an empty
// namespace and a refusal are both defensible.
stylex_test_panic!(
  a_theme_reference_spread_into_a_namespace_is_refused,
  "The spread argument's properties could not be read at compile time.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { ...zIndex } });
  "#
);

// `firstThatWorks` given the theme reference. Both refuse; the sentence is the
// function argument's rather than the value's, because the call is evaluated
// before its result is ever read as a value.
stylex_test_panic!(
  a_theme_reference_passed_to_first_that_works_is_refused_as_an_argument,
  "Function argument must be a static expression.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: { zIndex: stylex.firstThatWorks(zIndex, 1) },
    });
  "#
);

// ──────────────────────────────────────────────
// The guards: what a member read must keep doing
//
// The refusal is about the group, not about the import. Every accepting case
// here agrees with `@stylexjs/babel-plugin` 0.19.0 on class names and rule text.
// ──────────────────────────────────────────────

stylex_test!(
  a_member_read_off_a_theme_import_still_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { zIndex: zIndex.ten } });
  "#
);

stylex_test!(
  a_member_read_off_a_theme_import_still_resolves_under_conditions,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: { zIndex: { default: zIndex.ten, ':hover': zIndex.twenty } },
    });
  "#
);

stylex_test!(
  a_member_read_off_a_theme_import_still_resolves_three_conditions_deep,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      w: {
        zIndex: {
          default: 1,
          '@media (min-width: 100px)': { default: 2, ':hover': zIndex.ten },
        },
      },
    });
  "#
);

stylex_test!(
  a_member_read_off_a_theme_import_still_resolves_inside_a_template,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { boxShadow: `0 0 ${zIndex.ten} red` } });
  "#
);

stylex_test!(
  a_member_read_off_a_theme_import_still_resolves_inside_a_fallback_array,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ w: { position: [zIndex.ten, 'sticky'] } });
  "#
);

stylex_test!(
  a_member_read_off_a_theme_import_still_resolves_inside_a_dynamic_style,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({
      dyn: (a) => ({ zIndex: zIndex.ten, color: a }),
    });
  "#
);

// A parameter that takes the import's name over: the parameter wins, so the
// value is the parameter's custom property and no theme reference is ever read.
// This is the shape the branch was opened for, and it must not start refusing.
stylex_test!(
  a_dynamic_param_shadowing_a_theme_import_still_compiles,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const styles = stylex.create({ dyn: (zIndex) => ({ zIndex }) });
  "#
);

// The group handed to the calls that take one. Neither reads it as a style
// value, so neither is touched by the refusal.
stylex_test!(
  a_theme_import_handed_to_props_still_compiles,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const p = stylex.props(zIndex);
  "#
);

stylex_test!(
  a_theme_import_overridden_by_create_theme_still_compiles,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const t = stylex.createTheme(zIndex, { ten: 'red' });
  "#
);
