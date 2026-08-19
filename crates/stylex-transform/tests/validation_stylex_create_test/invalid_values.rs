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
  "A style value can only contain an array, string or number.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { color: { default: /a/, ':hover': 'red' } } });
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
