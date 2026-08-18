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

// ── A refusal names the node kind it could not fold ─────────────────
//
// Inside `stylex.create()` a deopt is the build error, so the reason recorded
// by the evaluator is what the author reads.
//
// Eleven of the thirteen messages below are byte identical to the one the
// reference implementation gives for the same input, measured by running it
// rather than written by hand — which is what makes them worth pinning: a
// label is only useful if it is the label the ecosystem uses.
//
// The two that are not are marked at the test. Neither divergence is in the
// label: one input is rejected there with a different diagnostic entirely, and
// the other is not rejected there at all.

stylex_test_panic!(
  this_expression_value_names_its_node_kind,
  "Unsupported expression: ThisExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: this } });
  "#
);

stylex_test_panic!(
  new_expression_value_names_its_node_kind,
  "Unsupported expression: NewExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: new Date() } });
  "#
);

stylex_test_panic!(
  meta_property_value_names_its_node_kind,
  "Unsupported expression: MetaProperty",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: import.meta } });
  "#
);

stylex_test_panic!(
  function_expression_value_names_its_node_kind,
  "Unsupported expression: FunctionExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: function () {} } });
  "#
);

stylex_test_panic!(
  class_expression_value_names_its_node_kind,
  "Unsupported expression: ClassExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: class {} } });
  "#
);

stylex_test_panic!(
  tagged_template_value_names_its_node_kind,
  "Unsupported expression: TaggedTemplateExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: String.raw`a` } });
  "#
);

stylex_test_panic!(
  update_expression_value_names_its_node_kind,
  "Unsupported expression: UpdateExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let counter = 0;
    const styles = stylex.create({ x: { content: counter++ } });
  "#
);

stylex_test_panic!(
  assignment_expression_value_names_its_node_kind,
  "Unsupported expression: AssignmentExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let counter = 0;
    const styles = stylex.create({ x: { content: (counter = 1) } });
  "#
);

// A property read on a value with no properties. The label names the receiver
// rather than the member expression at the deopt path: the code frame already
// shows a member expression, and which half of `a.b` refused is the part the
// author cannot see.
//
// One of the two that diverge: the reference implementation rejects this input
// with `A style value can only contain an array, string or number.` — a
// different diagnostic, not a different label. Both compilers reject it, and
// the spec's non-goals establish that no build can depend on the wording.
stylex_test_panic!(
  a_property_read_on_a_function_names_the_receiver,
  "Unsupported expression: ArrowFunctionExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: ({ a: () => 1 }).a.b } });
  "#
);

// A call whose callee is not callable is named for the call, because that is
// the expression the author has to change.
//
// Byte identical upstream. Note this holds for a callee that is not callable
// at all; a callee that is the wrong *kind* of value diverges —
// `[1, 2].filter(1)` is `number 1 is not a function` there — so the input here
// is deliberately the former.
stylex_test_panic!(
  a_call_on_a_number_names_the_call,
  "Unsupported expression: CallExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: (1)() } });
  "#
);

// `typeof` folded its operand and has no answer for what it got.
stylex_test_panic!(
  typeof_a_regex_names_the_operand,
  "Unsupported expression: RegExpLiteral",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: typeof /a/ } });
  "#
);

// The label reaches the author from the numeric coercion too, which reports
// through a `Result` rather than through the evaluation state.
//
// The other of the two that diverge, and the wider one: the reference
// implementation does not reject this at all — it folds `-({})` to `NaN` and
// emits `width:NaNpx`. Refusing rather than writing `NaN` into a stylesheet is
// the judgement recorded in issue 02, not something this change introduced;
// what is pinned here is only that the refusal names the operand.
stylex_test_panic!(
  a_negated_object_names_the_operand,
  "Expression is not a number: ObjectExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { width: -({}) } });
  "#
);

// The reported position: a refusal inside the right operand of a logical
// operator keeps the operand's label rather than the operator's.
stylex_test_panic!(
  a_refusing_logical_operand_keeps_its_own_label,
  "Unsupported expression: NewExpression",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { content: 1 > 0 && new Date() } });
  "#
);

// An unparseable `@media` key is reported, not emitted verbatim into the
// stylesheet as a broken at-rule.
stylex_test_panic!(
  invalid_media_query_syntax_is_reported,
  "Invalid media query syntax",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { color: { '@media (((': 'red', default: 'blue' } } });
  "#
);
