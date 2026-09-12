use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_filename(FileName::Real("vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::haste(None))
        .with_class_name_prefix("x"),
    )
  })
}

// A function value written bare, which is the spelling the parenthesized pair
// below has to compile to exactly.
stylex_test!(
  a_function_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      size: () => '1px',
      textMuted: () => `color-mix(${colors.text}, transparent 50%)`,
    });
  "#
);

// Parentheses around the function are a node in this tree and none in the
// reference implementation's, so this has to compile to exactly what
// `a_function_value` above compiles to. The two snapshots read side by side,
// and a difference between them is the defect.
//
// It compiles today because both readers of this object see the one the
// evaluator rebuilt, which carries no parentheses at all -- not because either
// reader unwraps them. That is what makes the pair worth keeping: it is the
// producer that holds the property, and a change there would break this case
// rather than a stylesheet.
stylex_test!(
  a_function_value_written_inside_parentheses,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      size: (() => '1px'),
      textMuted: (() => `color-mix(${colors.text}, transparent 50%)`),
    });
  "#
);

stylex_test!(
  same_group_references_inside_function_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () => `color-mix(${colors.text}, transparent 50%)`,
      textSubtle: () => `color-mix(${colors.textMuted}, white 10%)`,
    });
  "#
);

stylex_test!(
  same_group_references_can_point_to_later_keys,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      textMuted: () => `color-mix(${colors.text}, transparent 50%)`,
      text: 'black',
    });
  "#
);

stylex_test!(
  nested_function_leaves_resolve_same_group_refs_inside_at_rules,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () => ({
        default: `color-mix(${colors.text}, transparent 50%)`,
        '@media (prefers-color-scheme: dark)': `color-mix(${colors.text}, white 20%)`,
      }),
    });
  "#
);

stylex_test!(
  function_values_can_reference_typed_vars,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      accent: stylex.types.color('red'),
      accentGlow: () => `color-mix(${colors.accent}, white 15%)`,
    });
  "#
);

stylex_test!(
  function_values_can_return_stylex_types,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      accent: () => stylex.types.color('red'),
    });
  "#
);

stylex_test!(
  function_values_can_return_nested_objects,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const colors = stylex.defineVars({
      accent: () => ({
        default: 'red',
        '@media (prefers-color-scheme: dark)': 'blue',
      }),
    });
  "#
);

stylex_test!(
  function_values_can_mix_same_group_and_same_file_references,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const base = stylex.defineVars({
      overlay: 'rgba(0 0 0 / 0.3)',
    });

    export const colors = stylex.defineVars({
      text: 'black',
      textMuted: () =>
        `color-mix(${colors.text}, ${base.overlay} 20%)`,
    });
  "#
);
