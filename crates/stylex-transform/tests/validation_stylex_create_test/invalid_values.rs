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
// which of the two diagnostics an author reads is not something a build can
// depend on. (The spec's non-goals reach the same conclusion, but only for the
// numeric receiver, so they are not what settles this one.)
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

// ── A literal that is neither a value nor an absence ────────────────
//
// `null` declares nothing, which is an answer; a boolean declares nothing
// *because it is not a style value at all*, which is a refusal. The two used to
// be told apart only where the value was written directly — under a condition a
// boolean compiled and the declaration silently vanished, which is a wrong
// build rather than a wrong message.
//
// Every message below is byte identical to the one the reference implementation
// gives for the same input, measured by running it. The one exception is marked
// at the test.

stylex_test_panic!(
  a_boolean_longhand_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: false } });
  "#
);

stylex_test_panic!(
  a_true_longhand_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: true } });
  "#
);

// A shorthand the specificity table refuses is still refused for its value
// first: the value validator runs before the property table, so the message is
// about the boolean, not about `borderTop`.
stylex_test_panic!(
  a_boolean_shorthand_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderTop: false } });
  "#
);

stylex_test_panic!(
  a_boolean_custom_property_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { '--x': false } });
  "#
);

stylex_test_panic!(
  a_boolean_vendor_prefixed_property_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { WebkitLineClamp: false } });
  "#
);

stylex_test_panic!(
  a_boolean_reached_through_a_binding_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const flag = false;

    const styles = stylex.create({ x: { color: flag } });
  "#
);

// The four condition kinds. A value under a condition is the same kind of
// value as one written directly, so each of these is refused the same way.

stylex_test_panic!(
  a_boolean_default_branch_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: { default: false, ':hover': 'red' } } });
  "#
);

stylex_test_panic!(
  a_true_default_branch_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: { default: true, ':hover': 'red' } } });
  "#
);

stylex_test_panic!(
  a_boolean_pseudo_branch_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: { default: 'red', ':hover': false } } });
  "#
);

stylex_test_panic!(
  a_boolean_media_branch_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: { default: 'red', '@media print': false } } });
  "#
);

stylex_test_panic!(
  a_boolean_attribute_branch_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: { default: 'red', '[data-x]': false } } });
  "#
);

// Nested a level down, where the refusal has to survive the recursion rather
// than only being reached at the top of a condition object.
stylex_test_panic!(
  a_boolean_nested_two_conditions_deep_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      x: { color: { default: 'red', ':hover': { default: false, '@media print': 'blue' } } },
    });
  "#
);

// A property inside a pseudo object reaches the refusal through the other
// recursion — the namespace walk rather than the conditional-styles walk.
stylex_test_panic!(
  a_boolean_inside_a_pseudo_object_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { ':hover': { color: false } } });
  "#
);

// The whole value of a condition key, rather than a property under it.
stylex_test_panic!(
  a_boolean_as_a_whole_condition_value_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { ':hover': false } });
  "#
);

// A fallback array holds values, so its message is the array one.
stylex_test_panic!(
  a_boolean_array_entry_is_not_a_style_value,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: [false] } });
  "#
);

stylex_test_panic!(
  a_boolean_beside_a_real_array_entry_is_not_a_style_value,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: ['red', false] } });
  "#
);

// A boolean beside a `null` in the same array: the `null` is an absence and
// refusing the boolean is still the answer, so the presence of a droppable
// entry must not make the array look empty and slip past.
stylex_test_panic!(
  a_boolean_beside_a_null_array_entry_is_not_a_style_value,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: [null, false] } });
  "#
);

// An array under a condition is refused with the non-array message, which is
// what the reference implementation reports there.
stylex_test_panic!(
  a_boolean_array_entry_under_a_condition_is_not_a_style_value,
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      x: { color: { default: ['red', false], ':hover': 'blue' } },
    });
  "#
);

// A regular expression under a condition. Refused, and the diagnostic is the
// one divergence in this group: the reference implementation refuses it during
// evaluation as `Unsupported expression: RegExpLiteral` rather than as an
// illegal value. Both compilers refuse the build, and which of the two
// diagnostics an author reads is not something a build can depend on. What is
// pinned here is that it is refused at all, where before this change it compiled
// and the declaration silently vanished.
stylex_test_panic!(
  a_regular_expression_under_a_condition_is_not_a_style_value,
  "Unsupported expression: RegExpLiteral",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: { default: /a/, ':hover': 'red' } } });
  "#
);

// An object's properties are enumerated with every array-index key first, in
// ascending numeric order, and every other key after them in insertion order.
// That order is the order the declarations reach the stylesheet, so it decides
// which of two rules at equal specificity wins.
//
// This compiler emitted them in pure insertion order, so `{ color, ...['a'] }`
// put `color` before `0` where the language, and upstream, put `0` first.
stylex_test!(
  an_index_key_is_enumerated_before_a_named_one,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ x: { color: 'red', ...['a', 'b'] } });
  "#
);

// A key is an array index only in its canonical decimal spelling, so `'00'`
// stays where it was written.
stylex_test!(
  a_non_canonical_numeric_key_keeps_its_place,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ x: { color: 'red', '00': 'z' } });
  "#
);

// A numeric key is spelled the way JavaScript spells the number, not the way
// Rust does: `1e21` names the property `1e+21`, and naming it
// `1000000000000000000000` changed both the declaration and its class name.
stylex_test!(
  a_large_numeric_key_is_spelled_as_javascript_spells_it,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ x: { 1e21: 'a' } });
  "#
);

// ==================== a spread inside a style value ====================
//
// Every spread earns one answer: `Unsupported expression: SpreadElement`.
//
// That is the reference implementation's, and it is uniform. Upstream
// evaluates each *element path* of an array, so a spread arrives as a
// `SpreadElement` node and falls to the terminal
// `UNSUPPORTED_EXPRESSION(path.node.type)` arm -- before any value validation,
// whatever the operand is, and whether or not the operand can be resolved.
// Measured against the installed 0.19.0 plugin across every shape below.
//
// This suite used to pin two other messages here, both from the value rule
// (`A style array value can only contain strings or numbers.` on a property,
// `A style value can only contain an array, string or number.` under a
// condition). Those were the messages our evaluator's unwrapping of
// `elem.expr` left the validator to produce, and an author comparing the two
// compilers on one input read a different sentence from each. The refusal now
// happens where upstream makes it, in `array_expression`, so the sentence
// agrees -- and so does the one shape that earned no sentence at all, a spread
// of a literal, which used to compile: `[..."ab"]` shipped `color:ab` where
// the language spreads two characters, and `[...1]` shipped `color:1` where
// the language throws.

stylex_test_panic!(
  a_lone_spread_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['red'];

    const styles = stylex.create({ x: { color: [...fallbacks] } });
  "#
);

// The operand kinds. A literal is the one that used to fold to a value the
// source does not describe; the rest were already refused, but by the value
// rule and with the wrong sentence.
stylex_test_panic!(
  a_spread_of_a_string_refuses_rather_than_declaring_the_unspread_string,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: [..."ab"] } });
  "#
);

stylex_test_panic!(
  a_spread_of_a_number_refuses_rather_than_declaring_the_number,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: [...1] } });
  "#
);

stylex_test_panic!(
  a_spread_of_an_object_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: [...{ a: 1 }] } });
  "#
);

stylex_test_panic!(
  a_spread_of_a_call_result_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const tokens = { a: 1 };

    const styles = stylex.create({ x: { color: [...Object.keys(tokens)] } });
  "#
);

stylex_test_panic!(
  a_spread_of_an_empty_array_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: [...[], 'red'] } });
  "#
);

// An operand the evaluator cannot resolve is still a `SpreadElement` refusal,
// not the operand's own. That is why the refusal is made before the operand is
// evaluated: this case used to read `Unsupported expression: ArrayExpression`.
stylex_test_panic!(
  a_spread_of_an_unresolvable_value_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: [...unknownThing] } });
  "#
);

// The positions. A spread refuses the same way wherever a fallback chain is
// allowed to appear.
stylex_test_panic!(
  a_spread_on_a_custom_property_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['red'];

    const styles = stylex.create({ x: { '--x': [...fallbacks] } });
  "#
);

stylex_test_panic!(
  a_spread_inside_a_pseudo_object_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['red'];

    const styles = stylex.create({ x: { ':hover': { color: [...fallbacks] } } });
  "#
);

stylex_test_panic!(
  a_nested_spread_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['red'];

    const styles = stylex.create({ x: { color: [[...fallbacks]] } });
  "#
);

stylex_test_panic!(
  a_spread_under_a_condition_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['red'];

    const styles = stylex.create({
      x: { color: { default: [...fallbacks], ':hover': 'blue' } },
    });
  "#
);

// A spread on a shorthand, under a resolution that expands it -- the expansion
// runs after evaluation, so the spread still answers first.
stylex_test_panic!(
  a_spread_on_an_expanding_shorthand_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  |tr| build_test_transform(tr.comments.clone(), |b| b
    .with_runtime_injection()
    .with_style_resolution(StyleResolution::ApplicationOrder)),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['1px solid red'];

    const styles = stylex.create({ x: { borderTop: [...fallbacks] } });
  "#
);

// A spread beside a value the value rule would also refuse. The spread answers
// first in both positions, because evaluation runs before validation -- which
// is upstream's order too, measured on both orderings.
stylex_test_panic!(
  a_spread_before_a_boolean_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['red'];

    const styles = stylex.create({ x: { color: [...fallbacks, false] } });
  "#
);

stylex_test_panic!(
  a_boolean_before_a_spread_refuses_as_a_spread,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const fallbacks = ['red'];

    const styles = stylex.create({ x: { color: [false, ...fallbacks] } });
  "#
);

stylex_test_panic!(
  a_spread_beside_a_real_fallback_refuses_the_whole_chain,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: ['red', ..."ab"] } });
  "#
);

// A style value that reads a binding declared *later* in the module has no
// value to read at that point in the program, so inlining the initializer
// emits CSS for a value the runtime would never see. Declarations are
// collected module-wide, which is why the position has to be compared
// explicitly rather than falling out of the lookup.
stylex_test_panic!(
  binding_read_before_its_declaration_is_refused,
  "Referenced value is used before declaration",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { color: c } });

    const c = 'red';
  "#
);

// The refusal is about position, not about the reference: the same pair in
// program order still folds. Guards the check above from being satisfied by an
// evaluator that refuses every declared binding.
stylex_test!(
  binding_read_after_its_declaration_still_inlines,
  r#"
    import * as stylex from '@stylexjs/stylex';

    const c = 'red';

    const styles = stylex.create({ x: { color: c } });
  "#
);

// A shorthand is expanded into synthesized longhand properties, and the value
// they carry is the reference as authored — so the position is still there to
// compare and the refusal still lands. The reference implementation refuses
// this input too.
stylex_test_panic!(
  a_shorthand_value_read_before_its_declaration_is_refused,
  "Referenced value is used before declaration",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { margin: m } });

    const m = '4px';
  "#
);

// ──────────────────────────────────────────────
// A style value that reads a *default* import.
//
// A theme file is read through its named exports, so a default binding names a
// value from a file this compiler never evaluates. Resolving one as a theme
// reference emitted `var(--…)` for a variable the theme file does not define;
// `@stylexjs/babel-plugin` 0.19.0 refuses the same input, with the text these
// cases assert. Measured as `modules-1266-default-theme-import` in the parity
// corpus.
//
// The refusal is reached before the import path is resolved, so most of these
// need no module resolution configured. The one control that has to *succeed*
// does -- it gets the transform below.
// ──────────────────────────────────────────────

fn theme_import_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_filename(swc_core::common::FileName::Real("MyComponent.js".into()))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
      .with_runtime_injection()
  })
}

// The reported shape: a member read off a default theme import.
stylex_test_panic!(
  a_default_theme_import_read_in_a_style_value_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { color: tokens.color } });
  "#
);

// And the binding read bare, with no member access to fail on -- so the refusal
// is about the specifier and not about the property lookup.
stylex_test_panic!(
  a_default_theme_import_read_as_a_bare_value_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { color: tokens } });
  "#
);

// The control, and the reason the refusal is keyed to the specifier rather than
// to the declaration: one declaration carries both kinds, and the named half
// still resolves to a theme reference.
stylex_test!(
  a_named_theme_import_beside_a_default_one_still_resolves,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens, { colors } from 'colors.stylex.js';

    export const styles = stylex.create({ wrapper: { color: colors.primary } });
  "#
);

// A namespace specifier beside a default one, which is the other mixed shape the
// grammar allows. Both halves refuse now, and the sentence is what this case is
// for: a namespace specifier is refused as an undefined constant at the tail of
// the chain, so reading that sentence rather than the default specifier's proves
// the sibling reached its own arm. Refusing the default specifier must not start
// refusing whatever sits beside it *for the default specifier's reason*, and
// that is a sharper guard than the accepting snapshot this replaces.
stylex_test_panic!(
  a_namespace_theme_import_beside_a_default_one_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens, * as colors from 'colors.stylex.js';

    export const styles = stylex.create({ wrapper: { color: colors.primary } });
  "#
);

// The namespace specifier on its own, with no default beside it and no shadowing
// anywhere: the import kind is the whole reason for the refusal. Resolving one
// here used to fold `tokens.color` to a variable hashed from the *local alias*,
// which the theme file defines only when the alias happens to be spelled like
// the exported group -- so the same token read through a namespace import and
// through a named one produced two different variables, one of which nothing
// defines.
stylex_test_panic!(
  a_namespace_theme_import_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as tokens from 'colors.stylex.js';

    export const styles = stylex.create({ wrapper: { color: tokens.primary } });
  "#
);

// The spelling an author reaching for a namespace import would write: the group
// named, then the variable. It was refused before this change too, and by a
// different sentence -- the alias-hashed theme reference answered a string for
// `.colors`, and a second member read into a string is not a fold. Same refusal
// now as every other namespace read, from the specifier rather than from the
// shape of what the first hop returned.
stylex_test_panic!(
  a_namespace_theme_import_read_through_its_group_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as tokens from 'colors.stylex.js';

    export const styles = stylex.create({ wrapper: { color: tokens.colors.primary } });
  "#
);

// A namespace import of a file that is not a theme file at all. It read as a
// path-resolution failure before -- the wrong reason for the right refusal, and
// the same wrong reason the default-import case read before its own step landed.
// The specifier is answered before any path is resolved, so the module's
// extension no longer decides the sentence.
stylex_test_panic!(
  a_namespace_import_of_a_non_theme_file_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as colors from './colors.js';

    export const styles = stylex.create({ wrapper: { color: colors.primary } });
  "#
);

// ──────────────────────────────────────────────
// The namespace refusal under hostile input
// ──────────────────────────────────────────────
//
// The refusal belongs to the specifier, so nothing downstream of the value
// position should be able to change it -- not the spelling of the alias, not
// the shape of the member read, not the property it lands in, not how deep the
// conditions around it go. Every case below was measured against
// `@stylexjs/babel-plugin` 0.19.0 and refused there too; the sentence is
// asserted here because a corpus row compares acceptance and not wording.

// A non-ASCII alias. The lookup compares bindings rather than bytes, so a name
// no ASCII-only match could reach still resolves to its specifier and still
// refuses.
stylex_test_panic!(
  a_non_ascii_namespace_alias_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as ток from 'colors.stylex.js';

    export const styles = stylex.create({ w: { color: ток.primary } });
  "#
);

// One name written two ways: the specifier spells it with a unicode escape and
// the reference spells it plainly. They are one binding to the language, and
// the escape is gone before the lookup sees either -- so this is the shape a
// comparison of source bytes would have missed.
stylex_test_panic!(
  an_escaped_namespace_alias_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as \u0074okens from 'colors.stylex.js';

    export const styles = stylex.create({ w: { color: tokens.primary } });
  "#
);

// A member chain far longer than any theme file could answer. The chain is
// evaluated from its base and the base is what refuses, so the depth costs
// nothing and changes nothing.
stylex_test_panic!(
  a_deep_member_chain_off_a_namespace_import_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({ w: { color: t.a.b.c.d.e.f.g.h.i.j } });
  "#
);

// The member spelled as a computed string rather than as a property name. It
// is the same read, and the property spelling is decided after the base has
// already refused.
stylex_test_panic!(
  a_computed_member_off_a_namespace_import_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({ w: { color: t['primary'] } });
  "#
);

// Five conditions deep, mixing pseudo-classes with at-rules. The value is
// reached by the same evaluation at any depth, and the diagnostic names the
// path it took to get there.
stylex_test_panic!(
  a_namespace_import_read_at_condition_depth_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({
      w: {
        ':hover': {
          ':focus': {
            ':active': {
              '@media (min-width: 1px)': {
                '@supports (color: red)': { color: t.primary },
              },
            },
          },
        },
      },
    });
  "#
);

// A custom property, which takes a value the property validator never
// normalizes -- so this is the position where a refusal that came from
// validation rather than from the specifier would show as an acceptance.
stylex_test_panic!(
  a_namespace_import_read_as_a_custom_property_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({ w: { '--my-var': t.primary } });
  "#
);

// A vendor-prefixed property, and a shorthand that expands into several
// longhands. Both are positions where the value is handled by a path of its
// own after evaluation, and neither is reached.
stylex_test_panic!(
  a_namespace_import_read_in_a_prefixed_property_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({ w: { WebkitLineClamp: t.lines } });
  "#
);

stylex_test_panic!(
  a_namespace_import_read_in_an_expanding_shorthand_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({ w: { margin: t.space } });
  "#
);

// A property CSS does not define, holding the namespace read. The property is
// refused elsewhere for being unknown; the value is refused here first, which
// is what says the two refusals are ordered rather than racing.
stylex_test_panic!(
  a_namespace_import_read_in_an_unknown_property_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({ w: { 'not-a-real-prop': t.primary } });
  "#
);

// Inside a fallback chain, where the value is one candidate among several. A
// refusal of one candidate refuses the declaration; it does not silently fall
// back to the next.
stylex_test_panic!(
  a_namespace_import_read_in_a_fallback_chain_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({
      w: { color: stylex.firstThatWorks(t.primary, 'red') },
    });
  "#
);

// The same file imported twice under two aliases. Neither resolves, and the
// first read is what stops the build -- recorded because the old resolution
// gave these two aliases two different variables for one token.
stylex_test_panic!(
  two_namespace_aliases_of_one_theme_file_are_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as a from 'colors.stylex.js';
    import * as b from 'colors.stylex.js';

    export const styles = stylex.create({
      w: { color: a.primary, backgroundColor: b.primary },
    });
  "#
);

// A dynamic parameter shadowing the namespace, with the import also read
// outside the dynamic style. The parameter still compiles to an inline style;
// the unshadowed read is what refuses, so the two halves are answered
// independently and the shadowing one is not dragged down with it.
stylex_test_panic!(
  a_namespace_import_read_beside_a_parameter_that_shadows_it_is_not_defined,
  "Referenced constant is not defined.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import * as t from 'colors.stylex.js';

    export const styles = stylex.create({
      s: { color: t.primary },
      d: (t) => ({ color: t }),
    });
  "#
);

// Nothing about the refusal is specific to a theme file. A default import of any
// module is a value this compiler cannot fold, and it read as a path-resolution
// failure before -- the wrong reason for the right refusal.
stylex_test_panic!(
  a_default_import_of_a_non_theme_file_is_refused_the_same_way,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import classnames from 'classnames';

    export const styles = stylex.create({ wrapper: { color: classnames } });
  "#
);

// A shorthand value is carried into synthesized longhand properties, so the
// refusal has to survive the expansion -- the same guard the used-before-
// declaration cases above keep.
stylex_test_panic!(
  a_default_theme_import_read_in_a_shorthand_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { margin: tokens.space } });
  "#
);

// The recursive value walk reaches the same reference through four levels of
// conditions, so a refusal that only fired at the top level would let this one
// through.
stylex_test_panic!(
  a_default_theme_import_read_in_deeply_nested_conditions_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens from 'tokens.stylex.js';

    export const styles = stylex.create({
      wrapper: {
        color: {
          default: 'red',
          ':hover': {
            default: 'blue',
            '@media (min-width: 600px)': {
              default: 'green',
              ':focus': tokens.color,
            },
          },
        },
      },
    });
  "#
);

// A computed key is evaluated too, so the same reference refuses from the key
// position rather than from the value position.
stylex_test_panic!(
  a_default_theme_import_read_as_a_computed_key_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { [tokens.color]: 'red' } });
  "#
);

// The refusal travels out of an operand rather than being swallowed by the
// operator, which is how a concatenated value reports the reference that caused
// it instead of a coercion failure.
stylex_test_panic!(
  a_default_theme_import_read_through_an_operator_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import tokens from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { width: tokens.width + 'px' } });
  "#
);

// A default import aliased to one of the folded globals is the one shape where
// the import step and the globals step name the same binding, and no syntax
// context keeps them apart. The import answers, as it does upstream.
stylex_test_panic!(
  a_default_import_bound_to_a_global_name_is_refused_as_the_import,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import NaN from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { width: NaN } });
  "#
);

// A non-ASCII binding name, and an escaped one that spells an ASCII name. Both
// are the same binding to the language as their unescaped spelling, so the
// refusal has to key off the binding rather than off the bytes.
stylex_test_panic!(
  a_default_theme_import_with_a_non_ascii_binding_name_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import цвета from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { color: цвета.основной } });
  "#
);

stylex_test_panic!(
  a_default_theme_import_with_an_escaped_binding_name_is_refused,
  "There was an error when attempting to evaluate the imported file.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import \u0074okens from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { color: tokens.color } });
  "#
);

// ──────────────────────────────────────────────
// A style value that reads the name an import was aliased *away* from.
//
// `import { spacing as sp }` binds `sp` and leaves `spacing` naming whatever it
// named before -- nothing, in these modules. The import lookup used to answer
// for that name anyway, so a reference to it resolved to a binding no scope
// holds. `@stylexjs/babel-plugin` 0.19.0 asks the scope for the binding a
// reference resolves to and never sees the aliased-away name at all, so it
// refuses the same inputs. Measured as
// `modules-1266-read-by-a-string-named-imports-imported-name` and
// `modules-1266-read-by-an-aliased-imports-imported-name` in the parity corpus,
// one entry per spelling.
//
// The string-named spelling is the half that was reachable: an identifier
// spelling carries the parser's syntax context and a reference carries the
// resolver's, so the two never compared equal, where a string carries no
// context to compare at all.
// ──────────────────────────────────────────────

stylex_test_panic!(
  a_string_named_imports_imported_name_is_not_a_binding,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "spacing" as sp } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { padding: spacing } });
  "#
);

// The identifier spelling of the same shape. It refused before this change too,
// for the reason above -- pinned so a lookup that went back to comparing symbols
// cannot make it resolve.
stylex_test_panic!(
  an_aliased_imports_imported_name_is_not_a_binding,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { spacing as sp } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { padding: spacing } });
  "#
);

// A member read off the aliased-away name, which is the shape a theme import is
// actually used in -- so the refusal is not an artifact of reading the binding
// bare.
stylex_test_panic!(
  a_member_read_off_an_aliased_away_import_name_is_refused,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "colors" as c } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { color: colors.primary } });
  "#
);

// A shorthand carries the value into synthesized longhands, so the refusal has
// to survive the expansion.
stylex_test_panic!(
  an_aliased_away_import_name_read_in_a_shorthand_is_refused,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "spacing" as sp } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { margin: spacing } });
  "#
);

// Four levels down the condition walk, where a refusal that only fired at the
// top level would let this one through.
stylex_test_panic!(
  an_aliased_away_import_name_read_in_deeply_nested_conditions_is_refused,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "spacing" as sp } from 'tokens.stylex.js';

    export const styles = stylex.create({
      wrapper: {
        padding: {
          default: '1px',
          ':hover': {
            default: '2px',
            '@media (min-width: 600px)': {
              default: '3px',
              ':focus': spacing,
            },
          },
        },
      },
    });
  "#
);

// From the key position rather than the value position.
stylex_test_panic!(
  an_aliased_away_import_name_read_as_a_computed_key_is_refused,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "spacing" as sp } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { [spacing]: '1px' } });
  "#
);

// And out of an operand, so a concatenated value reports the reference rather
// than a coercion failure.
stylex_test_panic!(
  an_aliased_away_import_name_read_through_an_operator_is_refused,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "width" as w } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { width: width + 'px' } });
  "#
);

// A non-ASCII imported name, and an escaped spelling of an ASCII one. Both name
// the same export as their unescaped spelling, and neither binds anything here.
stylex_test_panic!(
  a_non_ascii_aliased_away_import_name_is_refused,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "цвета" as c } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { color: цвета } });
  "#
);

stylex_test_panic!(
  an_escaped_aliased_away_import_name_is_refused,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { "spacing" as sp } from 'tokens.stylex.js';

    export const styles = stylex.create({ wrapper: { padding: \u0073pacing } });
  "#
);

// A folded function map read where a style value belongs: `stylex` here is the
// map registered for the namespace import, `{ when }`, and not the parameter.
// Both compilers refuse it and `when` is the key they refuse on; this one used
// to refuse at the style-value consumer instead, with a sentence about a static
// expression.
stylex_test_panic!(
  dynamic_param_shadowing_the_stylex_namespace_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ dyn: (stylex) => ({ height: stylex }) });
  "#
);

// The namespace import under a local name, which is the only other spelling
// that can be shadowed -- the map is keyed by whatever the import binds, not by
// `stylex`.
stylex_test_panic!(
  dynamic_param_shadowing_an_aliased_stylex_namespace_is_refused_the_same_way,
  "Invalid pseudo or at-rule.",
  r#"
    import * as sx from '@stylexjs/stylex';

    export const styles = sx.create({ dyn: (sx) => ({ height: sx }) });
  "#
);

// A static property beside the dynamic one, which is the shape that reaches the
// consumer with something already collected. The refusal is the value's, not the
// namespace's, so it lands either way.
stylex_test_panic!(
  a_shadowed_namespace_beside_a_static_prop_is_still_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      wrapper: { color: 'red' },
      dyn: (stylex) => ({ height: stylex }),
    });
  "#
);

// Under a condition, and under a nested one. The value walk recurses per
// condition key, so a materialization that only happened at the top level would
// let these fall back to the old message.
stylex_test_panic!(
  a_shadowed_namespace_read_under_a_condition_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      dyn: (stylex) => ({ height: { default: stylex } }),
    });
  "#
);

stylex_test_panic!(
  a_shadowed_namespace_read_inside_a_pseudo_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      dyn: (stylex) => ({ ':hover': { height: stylex } }),
    });
  "#
);

// A shorthand, so the refusal survives being carried into synthesized longhands
// that have no authored position of their own.
stylex_test_panic!(
  a_shadowed_namespace_read_in_a_shorthand_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ dyn: (stylex) => ({ margin: stylex }) });
  "#
);

// The guard the materialization is written to keep: the map has to keep its own
// form where the identifier resolves, because `when` is read off it as a callee.
// Materializing at the identifier seam instead would compile this to nothing.
stylex_test!(
  when_read_as_a_callee_off_a_shadowed_namespace_still_resolves,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      dyn: (stylex) => ({
        color: { [stylex.when.ancestor(':hover')]: 'red', default: 'blue' },
      }),
    });
  "#
);

// The same callee off the unshadowed namespace, and off a bare `when` import.
// Neither goes through a parameter, and both read the map through the form the
// consumer no longer sees.
stylex_test!(
  when_read_as_a_callee_off_the_namespace_still_resolves,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      a: { color: { [stylex.when.ancestor(':hover')]: 'red', default: 'blue' } },
    });
  "#
);

stylex_test!(
  when_read_as_a_callee_off_a_bare_import_still_resolves,
  r#"
    import { create, when } from '@stylexjs/stylex';

    export const styles = create({
      a: { color: { [when.ancestor(':hover')]: 'red', default: 'blue' } },
    });
  "#
);

// A parameter that shadows the namespace but never reads it. The fold only
// happens where the name is read, so this one compiles -- upstream compiles it
// too, and a materialization that fired on the parameter list rather than on the
// value would break it.
stylex_test!(
  a_shadowing_param_that_is_never_read_still_compiles,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ dyn: (stylex) => ({ color: 'red' }) });
  "#
);

// A named import of a function-map entry, shadowed by a dynamic style's
// parameter. The reference implementation registers each of these names as the
// object `{ fn }`, so the parameter folds to that object and `fn` is the key
// namespace validation refuses on. This compiler registered them as function
// configs the identifier step had no value form for, so it deopted -- and a
// deopt inside a dynamic style is the inline-style path, which shipped
// `height:var(--x-height)` and an `@property` rule for a module the reference
// implementation refuses.
stylex_test_panic!(
  dynamic_param_shadowing_a_named_keyframes_import_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ height: keyframes }) });
  "#
);

stylex_test_panic!(
  dynamic_param_shadowing_a_named_first_that_works_import_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, firstThatWorks } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (firstThatWorks) => ({ height: firstThatWorks }),
    });
  "#
);

stylex_test_panic!(
  dynamic_param_shadowing_a_named_position_try_import_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, positionTry } from '@stylexjs/stylex';

    export const styles = create({ dyn: (positionTry) => ({ height: positionTry }) });
  "#
);

// A bare `when` import is the one entry of the family whose keys are not `fn`:
// the reference implementation registers the marker object itself, so the object
// the parameter folds to carries the marker names and `ancestor` is the key the
// refusal lands on. Same sentence either way, which is why the key set is worth
// pinning here rather than assuming.
stylex_test_panic!(
  dynamic_param_shadowing_a_bare_when_import_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, when } from '@stylexjs/stylex';

    export const styles = create({ dyn: (when) => ({ height: when }) });
  "#
);

// The alias, which is the only other spelling that can be shadowed: the map is
// keyed by what the specifier binds locally, not by the exported name.
stylex_test_panic!(
  dynamic_param_shadowing_an_aliased_keyframes_import_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes as kf } from '@stylexjs/stylex';

    export const styles = create({ dyn: (kf) => ({ height: kf }) });
  "#
);

// An escaped spelling of the alias, which names the same local binding. The fold
// is keyed on the name the parser resolved, so the escape must not smuggle the
// reference past it -- `\u{6b}f` is `kf`, and it is how both the parameter
// and the reference are spelled here.
stylex_test_panic!(
  an_escaped_spelling_of_a_shadowed_keyframes_alias_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes as kf } from '@stylexjs/stylex';

    export const styles = create({ dyn: (\u{6b}f) => ({ height: \u{6b}f }) });
  "#
);

// A non-ASCII alias, shadowed. Nothing in the fold is ASCII-only, and a name
// that only differs outside the ASCII range must still match itself.
stylex_test_panic!(
  a_non_ascii_shadowed_keyframes_alias_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes as кадры } from '@stylexjs/stylex';

    export const styles = create({ dyn: (кадры) => ({ height: кадры }) });
  "#
);

// A static property beside the dynamic one, so the consumer is reached with
// something already collected.
stylex_test_panic!(
  a_shadowed_keyframes_import_beside_a_static_prop_is_still_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      wrapper: { color: 'red' },
      dyn: (keyframes) => ({ height: keyframes }),
    });
  "#
);

// Under a condition, and under three nested pseudo-classes. The value walk
// recurses per condition key, so a fold that only happened at the top level
// would let these fall back to the inline-style path.
stylex_test_panic!(
  a_shadowed_keyframes_import_read_under_a_condition_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({ height: { default: keyframes } }),
    });
  "#
);

stylex_test_panic!(
  a_shadowed_keyframes_import_read_under_three_nested_pseudo_classes_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({
        ':hover': { ':focus': { ':active': { height: keyframes } } },
      }),
    });
  "#
);

// A shorthand, so the refusal survives being carried into synthesized longhands
// that have no authored position of their own.
stylex_test_panic!(
  a_shadowed_keyframes_import_read_in_a_shorthand_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ margin: keyframes }) });
  "#
);

// One of several parameters, read after the others, so the fold cannot depend on
// being the first thing the body reads.
stylex_test_panic!(
  a_shadowed_keyframes_import_among_several_params_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (a, keyframes, b) => ({ width: a, height: keyframes, top: b }),
    });
  "#
);

// The same fold read twice. Evaluation is cached per expression, so a second
// read must answer the object rather than whatever the cache happened to keep.
stylex_test_panic!(
  a_shadowed_keyframes_import_read_twice_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({ height: keyframes, width: keyframes }),
    });
  "#
);

// Hostile CSS around the fold, all of which the reference implementation refuses
// with the same sentence: validation reaches the key it cannot read before
// anything parses the query text, the unclosed function or the unterminated
// quote, so the fold decides the refusal and the malformed CSS never gets a say.
stylex_test_panic!(
  an_unclosed_media_query_holding_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({ '@media (min-width: 100px': { height: keyframes } }),
    });
  "#
);

stylex_test_panic!(
  an_unclosed_css_function_beside_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({ height: keyframes, width: 'calc(1px' }),
    });
  "#
);

stylex_test_panic!(
  an_unterminated_quote_beside_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({ height: keyframes, content: '"abc' }),
    });
  "#
);

// An unknown pseudo-class and a bracket condition, both of which are conditional
// keys the walk recurses through rather than keys it refuses -- so the fold
// underneath is what the refusal names.
stylex_test_panic!(
  an_unknown_pseudo_class_holding_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ ':nope': { height: keyframes } }) });
  "#
);

stylex_test_panic!(
  a_bracket_condition_holding_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({ '[data-x]': { height: keyframes } }),
    });
  "#
);

// A custom property, which skips the property-name validation an authored
// longhand goes through and reaches the value walk with nothing else refused
// first.
stylex_test_panic!(
  a_custom_property_driven_by_the_fold_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ '--my-var': keyframes }) });
  "#
);

// A vendor-prefixed property, which is renamed on the way to the declaration.
// The fold has to be refused before that rename, or the refusal would name a
// property the author never wrote.
stylex_test_panic!(
  a_vendor_prefixed_property_driven_by_the_fold_is_refused,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ WebkitLineClamp: keyframes }) });
  "#
);

// Degenerate condition keys: an at-rule with nothing after it, an empty string,
// and a query carrying the brace that would open a block in authored CSS. None
// of them is a key the walk can read, and none of them gets to answer before the
// fold does.
stylex_test_panic!(
  an_at_rule_with_no_condition_holding_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ '@media': { height: keyframes } }) });
  "#
);

stylex_test_panic!(
  an_empty_condition_key_holding_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ '': { height: keyframes } }) });
  "#
);

stylex_test_panic!(
  a_media_query_holding_a_stray_brace_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (keyframes) => ({ '@media (min-width: 1px) {': { height: keyframes } }),
    });
  "#
);

// Characters a condition key is not expected to carry: a NUL, a zero-width
// space, a right-to-left override and an astral scalar. Each is a valid Rust
// `str` and a valid JavaScript string, so nothing rejects them as encoding
// before the walk reaches the fold -- which is the point, since a key that
// refused on its own bytes would hide whether the fold was reached at all.
stylex_test_panic!(
  a_nul_in_a_condition_key_holding_the_fold_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ '\u{0}': { height: keyframes } }) });
  "#
);

stylex_test_panic!(
  a_zero_width_space_in_a_condition_key_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ '\u{200b}': { height: keyframes } }) });
  "#
);

stylex_test_panic!(
  a_right_to_left_override_in_a_condition_key_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ '\u{202e}': { height: keyframes } }) });
  "#
);

stylex_test_panic!(
  an_astral_character_in_a_condition_key_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ '\u{1F3A8}': { height: keyframes } }) });
  "#
);

// The fold read where a static value belongs -- no shadowing, the import itself.
// The static object evaluator materializes the fold the same way the dynamic
// consumer does, so the refusal is namespace validation's and reads the
// reference implementation's sentence. It used to read `A style value can only
// contain an array, string or number.`, and before that `Function not found`,
// which named nothing a caller could act on.
stylex_test_panic!(
  a_named_keyframes_import_read_as_a_static_value_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { height: keyframes } });
  "#
);

// The namespace import in the same position. It folds to the whole function map
// rather than to one config, which is the other half of what the static
// evaluator now materializes.
stylex_test_panic!(
  the_namespace_import_read_as_a_static_value_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ a: { height: stylex } });
  "#
);

// The guards the fold must not break. `types` is the one name of the family the
// reference implementation never registers for a create call, so the parameter
// stands and the module compiles to an inline style. This compiler does not
// register it for a create call either -- and now that the entries beside it
// refuse, that has to be the reason rather than a deopt that happened to agree.
stylex_test!(
  a_dynamic_param_shadowing_a_named_types_import_still_compiles,
  r#"
    import { create, types } from '@stylexjs/stylex';

    export const styles = create({ dyn: (types) => ({ height: types }) });
  "#
);

// `unstable_conditional` is the second such guard, and it was found by reading
// the reference implementation's registration rather than by report: it is a
// `{ fn }` entry like `keyframes`, but registered from `stylexConditionalImport`
// rather than for every create call, so nothing folds and the parameter stands.
// Both compilers compile this; a fold that fired on the whole family would
// break it.
stylex_test!(
  a_dynamic_param_shadowing_a_named_unstable_conditional_import_still_compiles,
  r#"
    import { create, unstable_conditional } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (unstable_conditional) => ({ height: unstable_conditional }),
    });
  "#
);

// `keyframes` called, not read: the call path resolves an identifier callee
// against the function map itself and never asks the identifier step for a
// value, so folding a config to an object cannot reach it.
stylex_test!(
  keyframes_called_through_a_named_import_still_resolves,
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    const fade = keyframes({ from: { opacity: 0 }, to: { opacity: 1 } });

    export const styles = create({ a: { animationName: fade } });
  "#
);

stylex_test!(
  first_that_works_called_through_a_named_import_still_resolves,
  r#"
    import { create, firstThatWorks } from '@stylexjs/stylex';

    export const styles = create({
      a: { position: firstThatWorks('sticky', 'fixed') },
    });
  "#
);

// A parameter that shadows one of these names but never reads it. The fold
// happens where the name is read, so this compiles -- and the reference
// implementation compiles it too.
stylex_test!(
  a_shadowing_keyframes_param_that_is_never_read_still_compiles,
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ dyn: (keyframes) => ({ color: 'red' }) });
  "#
);

// `defaultMarker`, measured and left as it is. It is the one entry of the family
// the reference implementation registers as a function rather than an object, so
// it refuses with `A style value can only contain an array, string or number.`
// Here the entry is an index map with no value form, and the sentence the build
// stops on names an internal shape rather than the input. Reaching upstream's
// sentence needs the namespace validator to refuse a value it currently passes
// over, which is a wider change than this seam -- recorded in
// `.scratch/fix_dynamic-param-shadows-import/issues/21-a-shadowed-default-marker-param-reports-an-internal-shape.md`.
// Pinned as it stands so the day it changes is visible.
stylex_test_panic!(
  a_dynamic_param_shadowing_a_named_default_marker_import_reports_an_internal_shape,
  "IndexMap values are not supported in this context.",
  r#"
    import { create, defaultMarker } from '@stylexjs/stylex';

    export const styles = create({
      dyn: (defaultMarker) => ({ height: defaultMarker }),
    });
  "#
);

// `when` read off a shadowed namespace parameter. The parameter folds to the
// namespace's map, and `when` read off that fold answers the marker config --
// which now materializes as the marker names, so the refusal is namespace
// validation's and reads the reference implementation's sentence.
//
// This was recorded in
// `.scratch/fix_dynamic-param-shadows-import/issues/15-the-function-map-read-where-it-is-not-a-map.md`
// as needing the when surface to carry its names rather than a change at the
// consumer. Measuring it disproved that: the consumer was enough, because the
// marker map behind the config already carries the names.
stylex_test_panic!(
  when_read_off_a_shadowed_namespace_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ dyn: (stylex) => ({ height: stylex.when }) });
  "#
);

// --------------------------------------------------------------------------
// The fold read where a style value belongs, in every position the static
// object evaluator can reach it, and in the ones it cannot. Every sentence
// below was measured against `@stylexjs/babel-plugin` 0.19.0 under the parity
// harness's configuration; the ones that differ from upstream's say so.
// --------------------------------------------------------------------------

// Depth: the object the fold materializes to is the value at whatever depth it
// was written, so the condition tree above it changes nothing about the refusal.
stylex_test_panic!(
  a_static_fold_under_a_condition_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ a: { height: { default: { ':hover': stylex } } } });
  "#
);

stylex_test_panic!(
  a_static_fold_eight_conditions_deep_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      a: {
        height: {
          ':hover': {
            ':focus': {
              ':active': {
                '@media (min-width: 1px)': {
                  '@supports (display: flex)': {
                    ':nth-child(2)': { ':first-child': { ':last-child': keyframes } },
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

// The property the fold is written on. A custom property and a vendor-prefixed
// one both reach namespace validation the same way a plain longhand does --
// neither name is read before the value is.
stylex_test_panic!(
  a_static_fold_on_a_custom_property_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { '--foo': keyframes } });
  "#
);

stylex_test_panic!(
  a_static_fold_on_a_vendor_prefixed_property_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { WebkitLineClamp: keyframes } });
  "#
);

// A shorthand, and a shorthand that expands into longhands. The expansion
// happens after the value is evaluated, so the fold is refused before either
// longhand exists.
stylex_test_panic!(
  a_static_fold_on_a_shorthand_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { margin: keyframes } });
  "#
);

stylex_test_panic!(
  a_static_fold_on_an_expanding_shorthand_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { borderWidth: keyframes } });
  "#
);

// A malformed condition key holding the fold. Neither an unclosed at-rule
// parenthesis nor an unclosed attribute-selector quote is refused as syntax
// before the walk reaches the value, which is the point: the fold is what stops
// the build, and both compilers stop on it.
stylex_test_panic!(
  a_static_fold_under_an_unclosed_media_query_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { height: { '@media (min-width: 1px': keyframes } } });
  "#
);

stylex_test_panic!(
  a_static_fold_under_an_unclosed_quote_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { height: { "[data-x='": keyframes } } });
  "#
);

stylex_test_panic!(
  a_static_fold_under_an_astral_condition_key_is_refused_for_the_fold,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { height: { '\u{1F3A8}': keyframes } } });
  "#
);

// A condition key that is a lone surrogate. Upstream reads `Invalid pseudo or
// at-rule.` -- it holds the key as a JavaScript string and never has to write
// it down. This compiler refuses the key's encoding before the fold is reached,
// which is the same rule `char_code_at` and an object spread of a string
// already answer: no Rust string can hold a lone surrogate, and emitting a
// replacement character would write a selector the source does not describe.
stylex_test_panic!(
  a_static_fold_under_a_lone_surrogate_key_is_refused_for_the_key,
  "String value contains invalid UTF-8 encoding.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { height: { '\u{d800}': keyframes } } });
  "#
);

// Position, not depth: a property that would have compiled beside the fold does
// not save the namespace, and neither does the fold being one of many.
stylex_test_panic!(
  a_static_fold_beside_a_good_property_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { color: 'red', height: keyframes } });
  "#
);

stylex_test_panic!(
  twelve_static_folds_in_one_create_are_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({
      n0: { height: keyframes },
      n1: { height: keyframes },
      n2: { height: keyframes },
      n3: { height: keyframes },
      n4: { height: keyframes },
      n5: { height: keyframes },
      n6: { height: keyframes },
      n7: { height: keyframes },
      n8: { height: keyframes },
      n9: { height: keyframes },
      n10: { height: keyframes },
      n11: { height: keyframes },
    });
  "#
);

// The fold reached through a binding rather than written in place. The
// identifier step folds the reference wherever it resolves it, so the value
// position sees the same object.
stylex_test_panic!(
  a_static_fold_read_through_a_binding_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    const held = keyframes;

    export const styles = create({ a: { height: held } });
  "#
);

// The remaining names of the family in the static value position. A bare `when`
// import is the marker object, so its keys are the marker names; `positionTry`
// and `firstThatWorks` are the `{ fn }` wrapper -- and one key is enough for
// namespace validation either way.
stylex_test_panic!(
  a_bare_when_import_read_as_a_static_value_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, when } from '@stylexjs/stylex';

    export const styles = create({ a: { height: when } });
  "#
);

stylex_test_panic!(
  a_named_position_try_import_read_as_a_static_value_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, positionTry } from '@stylexjs/stylex';

    export const styles = create({ a: { height: positionTry } });
  "#
);

stylex_test_panic!(
  a_named_first_that_works_import_read_as_a_static_value_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, firstThatWorks } from '@stylexjs/stylex';

    export const styles = create({ a: { height: firstThatWorks } });
  "#
);

// The fold inside a fallback array. An array element is refused with the
// array's own sentence, which upstream gives too.
stylex_test_panic!(
  a_static_fold_inside_a_fallback_array_is_refused_as_an_array_element,
  "A style array value can only contain strings or numbers.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { height: ['1px', keyframes] } });
  "#
);

// The fold as a spread operand. Its keys are own enumerable properties of the
// object the reference implementation folds it to, so a spread copies them onto
// the style object -- where a `{ fn }` wrapper's function is refused as a value
// and the namespace's `when` object is refused as a condition. This used to
// contribute nothing and compile a style object the author did not write.
stylex_test_panic!(
  a_static_fold_spread_into_a_style_object_is_refused_for_its_function,
  "A style value can only contain an array, string or number.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { ...keyframes, color: 'red' } });
  "#
);

stylex_test_panic!(
  a_static_fold_spread_alone_is_refused_for_its_function,
  "A style value can only contain an array, string or number.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { ...keyframes } });
  "#
);

stylex_test_panic!(
  a_static_fold_spread_twice_is_refused_for_its_function,
  "A style value can only contain an array, string or number.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { ...keyframes, ...keyframes } });
  "#
);

stylex_test_panic!(
  a_bare_when_import_spread_into_a_style_object_is_refused_for_its_function,
  "A style value can only contain an array, string or number.",
  r#"
    import { create, when } from '@stylexjs/stylex';

    export const styles = create({ a: { ...when, color: 'red' } });
  "#
);

stylex_test_panic!(
  the_namespace_import_spread_into_a_style_object_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ a: { ...stylex, color: 'red' } });
  "#
);

stylex_test_panic!(
  a_static_fold_spread_under_a_condition_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { color: { ...keyframes, default: 'red' } } });
  "#
);

// The one shape a spread of the fold compiles: every key it contributed is
// written over by a property that follows. Both compilers emit the same
// declaration, from a property named after the wrapper key.
stylex_test!(
  a_static_fold_spread_then_overridden_declares_the_overriding_value,
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: { ...keyframes, fn: 'red' } });
  "#
);

// --------------------------------------------------------------------------
// The neighbours of this seam, measured and pinned as they stand. Each is a
// refusal both compilers reach or a divergence recorded in
// `.scratch/fix_dynamic-param-shadows-import/issues/15-the-function-map-read-where-it-is-not-a-map.md`.
// --------------------------------------------------------------------------

// The fold written where a whole namespace belongs. The namespace's own object
// carries `when`, whose value is an object, so validation reads its keys as
// conditions and refuses them -- and a `{ fn }` wrapper carries a function,
// which is refused as a value. Both are the sentences upstream gives, which it
// gives for the same two shapes.
stylex_test_panic!(
  the_namespace_import_written_as_a_namespace_is_refused_as_a_namespace,
  "Invalid pseudo or at-rule.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({ a: stylex });
  "#
);

stylex_test_panic!(
  a_named_keyframes_import_written_as_a_namespace_is_refused_for_its_function,
  "A style value can only contain an array, string or number.",
  r#"
    import { create, keyframes } from '@stylexjs/stylex';

    export const styles = create({ a: keyframes });
  "#
);
