use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_pass(PluginPass::test_default())
        .with_runtime_injection(),
    )
  })
}

stylex_test_panic!(
  invalid_export_not_bound,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.defineVars({});
  "#
);

stylex_test_panic!(
  invalid_export_not_bound_unbound,
  "defineVars() calls must be bound to a bare variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    stylex.defineVars({});
  "#
);

stylex_test_panic!(
  invalid_argument_none,
  "defineVars() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars();
  "#
);

stylex_test_panic!(
  invalid_argument_too_many,
  "defineVars() should have 1 argument.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({}, {});
  "#
);

stylex_test_panic!(
  invalid_argument_number,
  "defineVars() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars(1);
  "#
);

stylex_test_panic!(
  invalid_argument_string,
  "defineVars() can only accept an object.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars('1');
  "#
);

stylex_test_panic!(
  invalid_argument_non_static,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars(genStyles());
  "#
);

stylex_test!(
  valid_argument_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({});
  "#
);

stylex_test!(
  valid_export_separate_const_and_export_statement,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = stylex.defineVars({});
    export { vars };
  "#
);

stylex_test_panic!(
  invalid_export_re_export_from_another_file_does_not_count,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = stylex.defineVars({});
    export { vars } from './other.stylex.js';
  "#
);

stylex_test_panic!(
  invalid_export_renamed_re_export_from_another_file_does_not_count,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = stylex.defineVars({});
    export { vars as otherVars } from './other.stylex.js';
  "#
);

stylex_test_panic!(
  invalid_export_default_export_does_not_count,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = stylex.defineVars({});
    export default vars;
  "#
);

stylex_test_panic!(
  invalid_export_renamed_export_with_as_syntax,
  "The return value of defineVars() must be bound to a named export.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const vars = stylex.defineVars({});
    export { vars as themeVars };
  "#
);

/* Properties */

stylex_test_panic!(
  invalid_key_non_static,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      [labelColor]: 'red',
    });
  "#
);

/* Values */

stylex_test_panic!(
  invalid_value_non_static_variable,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      labelColor: labelColor,
    });
  "#
);

stylex_test_panic!(
  invalid_value_non_static_function_call,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      labelColor: labelColor(),
    });
  "#
);

stylex_test!(
  valid_value_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      cornerRadius: 5,
    });
  "#
);

stylex_test!(
  valid_value_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      labelColor: 'red',
    });
  "#
);

stylex_test!(
  valid_value_keyframes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      fadeIn: stylex.keyframes({
        '0%': { opacity: 0 },
        '100%': { opacity: 1}
      }),
    });
  "#
);

/* Function values */

stylex_test!(
  valid_value_same_group_function_reference,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () => `color-mix(${colors.text}, transparent 50%)`,
    });
  "#
);

stylex_test_panic!(
  invalid_function_value_parameterized,
  "Function values in defineVars() must be zero-argument and return a static value supported by defineVars().",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: (value) => value,
    });
  "#
);

stylex_test_panic!(
  invalid_function_value_non_static_body,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () => getColor(colors.text),
    });
  "#
);

stylex_test!(
  valid_function_value_returns_stylex_types,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () => stylex.types.color('red'),
    });
  "#
);

stylex_test!(
  valid_function_value_returns_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      textMuted: () => ({
        default: 'red',
        '@media (prefers-color-scheme: dark)': 'blue',
      }),
    });
  "#
);

stylex_test_panic!(
  invalid_same_group_reference_unknown_key,
  "Unknown same-group reference \"missing\" found while resolving \"textMuted\" in defineVars().",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () => colors.missing,
    });
  "#
);

stylex_test_panic!(
  invalid_same_group_reference_direct_cycle,
  "Cyclic same-group references in defineVars() are not allowed: textMuted -> textMuted.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () => colors.textMuted,
    });
  "#
);

stylex_test_panic!(
  invalid_same_group_reference_indirect_cycle,
  "Cyclic same-group references in defineVars() are not allowed: a -> b -> c -> a.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      a: () => colors.b,
      b: () => colors.c,
      c: () => colors.a,
    });
  "#
);

// A folded function map read where a variable's value belongs. `keyframes` is
// registered for a `defineVars` call too, so the identifier step folds a
// reference to it, and the static object evaluator materializes the fold as the
// object it stands for -- which reaches this consumer as an object with no
// `default` key, and is refused for that.
//
// The sentence is the reference implementation's, byte for byte: an object with
// no `default` key is refused for the shape it is, before anything looks at
// what it holds. Looking at the values first answered a name the author wrote
// with a sentence about zero-argument functions, because the object a folded
// function map materializes to carries one in every value slot.
// The plain shape of the same rule, with no fold involved: an object value
// carrying at-rules and no `default`. The sentence names the top-level variable
// and not the nested key the recursion is standing on, which is what upstream
// names too.
stylex_test_panic!(
  an_object_value_with_no_default_key_is_refused,
  "Default value is not defined for cornerRadius variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      cornerRadius: { '@media (min-width: 600px)': '8px' },
    });
  "#
);

// The same object one level down. The top-level key is what is named, because
// that is the variable an author would go looking for.
stylex_test_panic!(
  a_nested_object_value_with_no_default_key_names_the_top_level_variable,
  "Default value is not defined for cornerRadius variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      cornerRadius: {
        default: '4px',
        '@media (min-width: 600px)': { '@supports (display: grid)': '8px' },
      },
    });
  "#
);

// A fold buried under an at-rule, rather than written at the top level. The
// reference implementation recurses through every branch of a value, so the
// level that lacks a `default` is refused wherever it sits; checking only the
// top level left this one reading the sentence about zero-argument functions,
// which is the defect the reorder is for.
stylex_test_panic!(
  a_nested_folded_function_map_is_refused_for_its_missing_default,
  "Default value is not defined for a variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_filename(swc_core::common::FileName::Real("vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::haste(None))),
  r#"
    import { defineVars, keyframes } from '@stylexjs/stylex';

    export const vars = defineVars({
      a: { default: '1px', '@media (min-width: 600px)': keyframes },
    });
  "#
);

// An empty object carries no `default` either, and is refused for that rather
// than compiling to a variable with no value.
stylex_test_panic!(
  an_empty_object_value_is_refused_for_its_missing_default,
  "Default value is not defined for cornerRadius variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({ cornerRadius: {} });
  "#
);

// A zero-argument arrow is still expanded and still refused for its parameters
// where it has them, so the reorder did not move the function check off the
// shapes it owns.
stylex_test_panic!(
  a_parameterized_arrow_beside_an_object_value_still_reads_the_function_sentence,
  "Function values in defineVars() must be zero-argument and return a static value supported by defineVars().",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      cornerRadius: { default: '4px' },
      other: (value) => value,
    });
  "#
);

stylex_test_panic!(
  a_folded_function_map_read_as_a_variable_value_is_refused_for_its_missing_default,
  "Default value is not defined for a variable.",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_filename(swc_core::common::FileName::Real("vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::haste(None))),
  r#"
    import { defineVars, keyframes } from '@stylexjs/stylex';

    export const vars = defineVars({ a: keyframes });
  "#
);

// A theme reference read as a variable's value. `defineVars` evaluates its
// object through the same evaluator a `create` namespace goes through, so the
// refusal that stopped the silent drop reaches here too -- and both compilers
// refuse, with their own words: upstream reads `Default value is not defined for
// a variable.` because the group folds to an object with no `default` key.
// Recorded as `modules-1266-a-theme-object-as-a-define-vars-value`.
stylex_test_panic!(
  a_theme_reference_read_as_a_variable_value_is_refused,
  "Only static values are allowed inside of a defineVars() call.",
  |tr| theme_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const vars = stylex.defineVars({ a: zIndex });
  "#
);

// The member read beside it, which is how one theme is meant to build on
// another, and agrees with the reference implementation on the rule text.
stylex_test!(
  a_member_read_off_a_theme_import_is_a_valid_variable_value,
  |tr| theme_module_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const vars = stylex.defineVars({ a: zIndex.ten });
  "#
);
