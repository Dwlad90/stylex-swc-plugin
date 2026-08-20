//! An import specifier whose exported name is a string.
//!
//! `import { "color-lg" as colorLg } from 'vars.stylex.js'` binds `colorLg` and
//! carries `"color-lg"` as the name of the export it reads. The two halves are
//! separate questions and this file asks both:
//!
//! - the **lookup**: a reference resolves through the binding the specifier
//!   introduces, and the *string* is what the theme variable is hashed from --
//!   so the local alias must contribute nothing to the CSS
//! - the **emit**: read with no member access the group carries no value form,
//!   and the refusal has to be a refusal rather than an absent property
//!
//! The shape matters out of proportion to how often it is written, because it is
//! the one spelling where the name a reference reads and the name an export
//! carries cannot be the same identifier: a string export name need not be a
//! valid identifier at all. A lookup keyed by the wrong one of the two answers
//! plausibly and wrongly -- it hashes a variable the theme file does not define.
//!
//! Measured against `@stylexjs/babel-plugin` 0.19.0 under `haste` resolution and
//! one source string, the parity harness's configuration; the corpus rows are
//! `modules-1266-a-string-named-specifier-*`. The corpus compares acceptance and
//! not wording, so every sentence below is pinned here instead. Where the two
//! compilers disagree the divergence is named at the test.

use crate::utils::prelude::*;

// ──────────────────────────────────────────────
// The lookup: the export name is what the variable is hashed from
//
// Every accepting case here agrees with `@stylexjs/babel-plugin` 0.19.0 on the
// class name and the rule text. That is the whole point of the group: the alias
// is a local convenience and the string is the contract.
// ──────────────────────────────────────────────

stylex_test!(
  a_member_read_through_a_string_export_name_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "colors" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: c.lg } });
  "#
);

// The export name spelled so it *cannot* be an identifier: a hyphen is legal in
// a string export name and illegal in a binding, so no lookup keyed on the
// exported name could ever match a reference to it.
stylex_test!(
  a_string_export_name_that_is_not_an_identifier_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as colorLg } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: colorLg.x } });
  "#
);

// The shortest export name the grammar allows, and the one a name-keyed lookup
// is likeliest to read as absent rather than as empty.
stylex_test!(
  an_empty_string_export_name_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "" as blank } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: blank.x } });
  "#
);

stylex_test!(
  a_whitespace_only_string_export_name_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { " " as sp } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: sp.x } });
  "#
);

// The alias spelled like the export name. The redundant spelling is the control
// on the two above: it is the one case where keying the lookup either way gives
// the same answer, so it must not be the only case a test covers.
stylex_test!(
  a_string_export_name_equal_to_its_local_alias_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "zIndex" as zIndex } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { zIndex: zIndex.ten } });
  "#
);

// `default` names the default export through a *named* specifier, so the chain
// step that refuses a default import never sees it -- on either side. Both
// compilers resolve it as the export named `default` and hash that name.
stylex_test!(
  a_string_export_name_spelled_default_resolves_as_a_named_export,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "default" as d } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: d.x } });
  "#
);

// An export name spelled like a global the evaluator folds. The name is the
// export's and the reference is the alias's, so neither the globals step nor the
// import step can confuse the two -- unlike the reverse spelling below.
stylex_test!(
  a_string_export_name_spelled_like_a_global_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "NaN" as n } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: n.x } });
  "#
);

// The reverse: the *alias* takes a global's name over. `NaN` is a legal binding
// name, so this is the one shape where an import specifier and one of the
// globals name the same binding, and the import step answers first -- which is
// upstream's order and the single outcome the chain's reorder was not inert on.
stylex_test!(
  a_local_alias_spelled_like_a_global_resolves_to_the_import,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color" as NaN } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: NaN.x } });
  "#
);

// ──────────────────────────────────────────────
// How the export name is spelled
//
// A string export name can hold anything a string literal can, and the name is
// hashed rather than parsed -- so escapes, quotes and astral characters have to
// reach the hash as the characters they denote and not as the source spelled
// them.
// ──────────────────────────────────────────────

stylex_test!(
  a_string_export_name_carrying_an_escaped_quote_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "a\"b" as q } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: q.x } });
  "#
);

stylex_test!(
  a_string_export_name_carrying_an_escaped_backslash_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "a\\b" as bs } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: bs.x } });
  "#
);

stylex_test!(
  a_string_export_name_carrying_a_newline_escape_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "a\nb" as nl } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: nl.x } });
  "#
);

// An astral character, which is one code point and two UTF-16 code units. The
// hash reads code points, so the name has to survive the crossing intact.
stylex_test!(
  an_astral_string_export_name_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "😀" as e } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: e.x } });
  "#
);

stylex_test!(
  a_numeric_looking_string_export_name_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "0" as zero } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: zero.x } });
  "#
);

// A thousand characters. The length boundary is the point: the name is hashed
// whole, so a truncating or fixed-buffer read would show here and nowhere else.
stylex_test!(
  a_thousand_character_string_export_name_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" as big } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: big.x } });
  "#
);

// One export name bound twice. The variable belongs to the export name, so both
// reads write the same `var()` through different locals -- the check that the
// alias contributes nothing to the hash, stated as an equality rather than as
// two independent snapshots.
stylex_test!(
  one_string_export_name_bound_twice_resolves_to_one_variable,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color" as a, "color" as b } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: a.x, backgroundColor: b.x } });
  "#
);

// An export name that is well-formed UTF-16 and not well-formed Unicode: the one
// string an export name can hold that has no encoding a hash can read. Both
// compilers refuse. Upstream refuses it in the parser (`An export name cannot
// include a lone surrogate`) and this compiler where the name is decoded, so the
// outcome is shared and the sentence is not.
stylex_test_panic!(
  a_lone_surrogate_in_a_string_export_name_is_refused,
  "String value contains invalid UTF-8 encoding.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "\ud83d" as ls } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: ls.x } });
  "#
);

// ──────────────────────────────────────────────
// Where the member read is written
//
// Depth, a custom property, a vendor-prefixed property, a fallback array, a
// template, a dynamic style's body: the position changes what surrounds the
// value and not what the name resolves to.
// ──────────────────────────────────────────────

// Eight condition levels. The pseudo-classes nest alphabetically on purpose --
// nested out of that order the two compilers hash a different *selector*, for a
// reason that has nothing to do with resolution, which
// `.scratch/fix_dynamic-param-shadows-import/issues/19-three-nested-pseudo-classes-hash-differently.md`
// owns and one corpus row measures.
stylex_test!(
  a_string_export_name_member_read_eight_conditions_deep_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({
      w: {
        color: {
          default: 'red',
          '@media (min-width: 1px)': {
            default: 'red',
            '@supports (color: red)': {
              default: 'red',
              ':active': {
                default: 'red',
                ':first-child': {
                  default: 'red',
                  ':focus': {
                    default: 'red',
                    ':hover': { default: 'red', ':last-child': c.x },
                  },
                },
              },
            },
          },
        },
      },
    });
  "#
);

stylex_test!(
  a_string_export_name_member_read_on_a_custom_property_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { '--my-var': c.x } });
  "#
);

stylex_test!(
  a_string_export_name_member_read_on_a_prefixed_property_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { WebkitLineClamp: c.x } });
  "#
);

stylex_test!(
  a_string_export_name_member_read_in_a_fallback_array_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { position: [c.x, 'sticky'] } });
  "#
);

stylex_test!(
  a_string_export_name_member_read_in_a_template_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { boxShadow: `0 0 ${c.x} red` } });
  "#
);

stylex_test!(
  a_string_export_name_member_read_inside_a_dynamic_style_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ dyn: (a) => ({ color: c.x, opacity: a }) });
  "#
);

// Issue #1266's shape spelled through a string-named specifier: the parameter
// takes the local alias over while the static read beside it still resolves the
// import. The parameter wins, so one rule reads the theme variable and the other
// the inline custom property.
stylex_test!(
  a_dynamic_param_shadowing_a_string_named_specifier_still_compiles,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({
      w: { color: c.x },
      dyn: (c) => ({ color: c }),
    });
  "#
);

// The injected function map is keyed by the name a reference *spells*, and a
// string export name is how that name comes to differ from the export it binds.
// Called through its local alias the specifier still folds to `keyframes`.
stylex_test!(
  a_function_map_entry_imported_under_a_string_export_name_folds,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "keyframes" as kf } from '@stylexjs/stylex';

    export const styles = stylex.create({
      w: { animationName: kf({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
  "#
);

// The specifier nothing reads. It has to leave the module's CSS alone, which is
// the guard on the pre-scan that records every import local as a declared
// binding.
//
// The declaration survives, and that is upstream's answer too: measured on
// `@stylexjs/babel-plugin` 0.19.0, an unused theme import is kept in every
// spelling -- named, aliased, string-named, default, namespace -- because
// removing an import nothing reads is not this transform's job on either side.
// The elision that does happen here is `typescript_strip`'s, a later pass in the
// compiler's own pipeline that runs only for TypeScript syntax and drops the
// declaration for both spellings alike; `cargo test` runs the transform without
// it, which is why this snapshot carries what the transform produces rather than
// what the compiler ships. Issue 24 of this effort owns that pass.
stylex_test!(
  a_string_named_specifier_nothing_reads_changes_nothing,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: 'red' } });
  "#
);

// ──────────────────────────────────────────────
// Malformed CSS beside a resolved member read
//
// The value resolves before any CSS is parsed, so a malformed neighbour decides
// the outcome on its own terms -- and does so identically on both compilers,
// including where neither of them refuses.
// ──────────────────────────────────────────────

stylex_test_panic!(
  a_resolved_member_read_beside_an_unclosed_css_function_refuses_for_the_function,
  "Rule contains an unclosed function",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({
      w: { color: c.x, backgroundColor: 'rgb(0,0,' },
    });
  "#
);

// Neither compiler refuses an unterminated quote: it is a value neither parses,
// and both ship it verbatim. Recorded because the refusal above makes it look
// like malformed CSS is always caught here.
stylex_test!(
  a_resolved_member_read_beside_an_unterminated_quote_still_compiles,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({
      w: { color: c.x, content: '"unterminated' },
    });
  "#
);

stylex_test!(
  a_resolved_member_read_under_an_unclosed_attribute_selector_still_compiles,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({
      w: { color: { default: 'red', '[data-x': c.x } },
    });
  "#
);

// ──────────────────────────────────────────────
// The emit: the group itself, read where a value belongs
//
// The refusals `theme_reference_style_values` pins for an identifier-named
// import, reached instead through a string-named one. They are here rather than
// there because what is under test is the spelling of the specifier: this is the
// shape that compiled to nothing -- no rule, no error, no warning -- while
// everything written beside it compiled, and the string spelling is how a reader
// might expect the lookup to miss and the drop to survive.
// ──────────────────────────────────────────────

stylex_test_panic!(
  a_string_named_specifier_read_as_a_style_value_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as colorLg } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: colorLg } });
  "#
);

// The sibling matters: it is the declaration that used to be emitted *alone*,
// which is what made the drop look like a compiling module rather than a bug.
stylex_test_panic!(
  a_string_named_specifier_read_beside_a_static_sibling_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as colorLg } from 'vars.stylex.js';

    export const styles = stylex.create({
      w: { backgroundColor: 'red', color: colorLg },
    });
  "#
);

// The alias spelled like a global, read bare. The import step answers before the
// globals step, so the sentence is the value's and not `UNINITIALIZED_CONST`.
stylex_test_panic!(
  a_local_alias_spelled_like_a_global_read_bare_is_refused_as_a_value,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color" as NaN } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: NaN } });
  "#
);

stylex_test_panic!(
  a_string_export_name_spelled_default_read_bare_is_refused_as_a_value,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "default" as d } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: d } });
  "#
);

// The key path in front of the sentence stops one key short of the leaf, here as
// at every depth.
stylex_test_panic!(
  a_string_named_specifier_read_bare_at_depth_is_refused,
  "w > color > @media (min-width: 1px) > :active > :focus > A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({
      w: {
        color: {
          default: 'red',
          '@media (min-width: 1px)': {
            default: 'red',
            ':active': { default: 'red', ':focus': { default: 'red', ':hover': c } },
          },
        },
      },
    });
  "#
);

stylex_test_panic!(
  a_string_named_specifier_read_bare_in_a_fallback_array_is_refused_as_an_array,
  "A style array value can only contain strings or numbers.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { position: [c, 'sticky'] } });
  "#
);

stylex_test_panic!(
  a_string_named_specifier_written_as_a_namespace_is_refused_as_a_namespace,
  "A StyleX namespace must be an object.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: c });
  "#
);

// The dynamic style's body reads the same value through the other consumer,
// `evaluate_stylex_create_arg` rather than the object evaluator. Both read the
// same sentence.
stylex_test_panic!(
  a_string_named_specifier_read_bare_inside_a_dynamic_style_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ dyn: (a) => ({ color: c, opacity: a }) });
  "#
);

// ──────────────────────────────────────────────
// The emit, where the two compilers disagree
//
// Each divergence below is the theme reference's rather than the string
// specifier's -- the same answer an identifier-named import gets in the same
// position, recorded once there and reached here through the other spelling. The
// corpus rows carry the measurement.
// ──────────────────────────────────────────────

// Upstream folds the group to an object with no own enumerable properties, so
// the spread contributes nothing and the namespace compiles empty; here the
// spread refuses because the group's properties cannot be enumerated.
stylex_test_panic!(
  a_string_named_specifier_spread_into_a_namespace_is_refused,
  "The spread argument's properties could not be read at compile time.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { ...c } });
  "#
);

// Upstream coerces the group to its hash and declares a property named after it,
// which is not a property. Refused here rather than reproduced.
stylex_test_panic!(
  a_string_named_specifier_read_as_a_computed_key_is_refused,
  "A style value can only contain an array, string or number.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { [c]: 1 } });
  "#
);

// Both refuse, and neither says the same thing: the argument check answers first
// here, the array check upstream.
stylex_test_panic!(
  a_string_named_specifier_passed_to_first_that_works_is_refused_as_an_argument,
  "Function argument must be a static expression.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    export const styles = stylex.create({ w: { color: stylex.firstThatWorks(c, 'red') } });
  "#
);

// A keyframes step. Upstream emits the step with the declaration missing and
// says nothing; here the value is refused, as every other shape with no value
// form in this position already is.
stylex_test_panic!(
  a_string_named_specifier_read_bare_in_a_keyframes_step_is_refused,
  "Only static values are allowed inside of a keyframes() call.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "color-lg" as c } from 'vars.stylex.js';

    const kf = stylex.keyframes({ from: { color: c }, to: { color: 'red' } });

    export const styles = stylex.create({ w: { animationName: kf } });
  "#
);
