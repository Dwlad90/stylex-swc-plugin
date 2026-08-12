use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

stylex_test_panic!(
  #[ignore],
  invalid_value_display_important,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { display: "block !important" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_background_position_top_left,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { backgroundPosition: "top left" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_color_red_blue,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderColor: "red blue" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_radius_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderRadius: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_style_solid_dashed,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderStyle: "solid dashed" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_width_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderWidth: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_inset_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { inset: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_inset_block_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { insetBlock: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_inset_inline_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { insetInline: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_flex_1_1_0,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { flex: "1 1 0" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_grid_1_1_0,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { grid: "1 1 0" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_margin_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { margin: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_margin_block_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { marginBlock: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_margin_inline_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { marginInline: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_outline_1px_solid_red,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { outline: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_overflow_hidden_visible,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { overflow: "hidden visible" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_padding_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { padding: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_padding_block_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { paddingBlock: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_padding_inline_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { paddingInline: "1px 2px" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_all,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { transitionProperty: "all" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_bottom,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { transitionProperty: "bottom" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_end,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { transitionProperty: "end" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_height,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { transitionProperty: "height" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_width,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { transitionProperty: "width" } });
  "#
);

// A binding that is rebound or mutated anywhere in the module no longer holds
// its declaration initializer at the `stylex.create` site. Inlining the
// initializer would bake a stale value into the generated CSS, so evaluation
// must bail out and report a non-constant reference instead.
stylex_test_panic!(
  reassigned_binding_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let color = 'red';
    color = 'blue';

    const styles = stylex.create({ x: { color } });
  "#
);

stylex_test_panic!(
  mutated_array_binding_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const spacing = [4, 8];
    spacing.push(16);

    const styles = stylex.create({ x: { gap: spacing[0] } });
  "#
);

stylex_test_panic!(
  object_assigned_binding_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const tokens = { color: 'red' };
    Object.assign(tokens, { color: 'blue' });

    const styles = stylex.create({ x: { color: tokens.color } });
  "#
);

stylex_test_panic!(
  binding_mutated_through_a_nested_member_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const theme = { colors: { primary: 'red' } };
    theme.colors.primary = 'blue';

    const styles = stylex.create({ x: { color: theme.colors.primary } });
  "#
);

// Parenthesised and optional-call write targets reach the same binding as
// their bare forms. Each of these silently produced stale CSS before the
// write-target walk was unified.
stylex_test_panic!(
  parenthesised_update_target_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let gap = 4;
    (gap)++;

    const styles = stylex.create({ x: { gap } });
  "#
);

stylex_test_panic!(
  parenthesised_object_assign_target_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const tokens = { color: 'red' };
    Object.assign((tokens), { color: 'blue' });

    const styles = stylex.create({ x: { color: tokens.color } });
  "#
);

stylex_test_panic!(
  optionally_called_mutating_method_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const spacing = [4, 8];
    spacing?.push(16);

    const styles = stylex.create({ x: { gap: spacing[0] } });
  "#
);

stylex_test_panic!(
  mutating_method_named_by_a_string_literal_is_not_a_constant_value,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const spacing = [4, 8];
    spacing['push'](16);

    const styles = stylex.create({ x: { gap: spacing[0] } });
  "#
);

// With the `sx` prop disabled the module pre-scan is deferred until the
// module is known to import stylex; the binding-write guard must still hold.
stylex_test_panic!(
  reassigned_binding_is_not_a_constant_value_with_sx_disabled,
  "Referenced value is not a constant",
  |tr| {
    build_test_transform(tr.comments.clone(), |b| {
      b.with_sx_prop_name(SxPropNameParam::Disabled)
        .with_runtime_injection()
    })
  },
  r#"
    import * as stylex from '@stylexjs/stylex';

    let color = 'red';
    color = 'blue';

    const styles = stylex.create({ x: { color } });
  "#
);

// The upstream evaluator has no BigIntLiteral case, so a BigInt value deopts
// as an unsupported expression rather than compiling to a `px` length.
stylex_test_panic!(
  big_int_value_is_an_unsupported_expression,
  "Unsupported expression: BigIntLiteral",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { width: 10n } });
  "#
);
