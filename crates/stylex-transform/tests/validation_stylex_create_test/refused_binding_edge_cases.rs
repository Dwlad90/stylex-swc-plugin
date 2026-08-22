//! A refusal about a binding, under inputs chosen to break the machinery that
//! reports it.
//!
//! Reporting a refusal against the *declaration* means the diagnostic path now
//! searches the module for a name — see
//! `shared::utils::log::declaration_span` — so a refused build reads the source
//! a second time, on a path that only ever runs while something is already going
//! wrong. Every case here refuses, and what is asserted is that the sentence an
//! author is handed is still the right one: an input that made the position
//! lookup panic would replace it, and one that made the lookup answer the wrong
//! question would leave it in place while sending the reader elsewhere.
//!
//! The positions themselves are not observable from here — `stylex_test_panic!`
//! matches the message and a code frame is written separately — so they are
//! pinned in two other places: the frame's own suite, over fixtures whose lines
//! are asserted, and `parity/corpus/positions.json`, which compares them against
//! `@stylexjs/babel-plugin` 0.19.0. Every message asserted below was measured
//! against 0.19.0 too, so a case that changes here is a case that stopped
//! agreeing with upstream.

use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test_transform;

// ── malformed CSS behind a refused binding ──────────────────────────────────
//
// Evaluation runs before value normalization, so the binding refusal is what an
// author sees and the malformed value is never reached. That order is the point:
// a CSS lint reporting on a value the evaluator already refused would name the
// wrong problem.

stylex_test_panic!(
  a_reassigned_binding_carrying_an_unclosed_string_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let content = '"unclosed';
    content = '"also unclosed';

    const styles = stylex.create({ x: { content } });
  "#
);

stylex_test_panic!(
  a_reassigned_binding_carrying_an_unclosed_function_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let width = 'calc(100% - 8px';
    width = 'calc(50%';

    const styles = stylex.create({ x: { width } });
  "#
);

stylex_test_panic!(
  a_reassigned_binding_carrying_an_unclosed_comment_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let color = 'red /* unclosed';
    color = 'blue /* unclosed';

    const styles = stylex.create({ x: { color } });
  "#
);

stylex_test_panic!(
  a_reassigned_binding_carrying_a_brace_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let value = 'red } color: blue';
    value = 'green } color: blue';

    const styles = stylex.create({ x: { '--custom': value } });
  "#
);

stylex_test_panic!(
  a_reassigned_binding_carrying_an_important_flag_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let color = 'red !important';
    color = 'blue !important';

    const styles = stylex.create({ x: { color } });
  "#
);

// ── a vendor-prefixed property, and a custom property ───────────────────────

stylex_test_panic!(
  a_mutated_binding_read_by_a_vendor_prefixed_property_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const tokens = { clamp: 2 };
    tokens.clamp = 3;

    const styles = stylex.create({ x: { WebkitLineClamp: tokens.clamp } });
  "#
);

stylex_test_panic!(
  a_mutated_binding_read_by_a_custom_property_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const tokens = { gap: '4px' };
    tokens.gap = '8px';

    const styles = stylex.create({ x: { '--gap': tokens.gap } });
  "#
);

// ── depth, and the two ceilings around it ───────────────────────────────────

// A binding written to is refused for the write before its value's depth is
// ever measured, so a value past the CSS nesting ceiling still reports the
// write. The two ceilings themselves are pinned where they live:
// `stylex_css`'s value-nesting tests and the evaluation-depth suites.
stylex_test_panic!(
  a_reassigned_binding_whose_value_is_nested_past_the_css_ceiling_reports_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let width = 'calc(calc(calc(calc(calc(calc(1px))))))';
    width = 'calc(calc(calc(calc(calc(calc(2px))))))';

    const styles = stylex.create({ x: { width } });
  "#
);

// ── unicode, escapes, and names that are not ASCII ──────────────────────────

// The declaration search compares names, so a name spelled outside ASCII has
// to compare equal to itself — and the span it answers with has to be a
// character boundary of a source measured in bytes.
stylex_test_panic!(
  a_reassigned_binding_named_outside_ascii_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let σ = 'red';
    σ = 'blue';

    const styles = stylex.create({ x: { color: σ } });
  "#
);

// An escaped identifier is the same name to the language, so a declaration
// spelling it one way and a read spelling it another are one binding -- and the
// declaration search has to answer for the name, not for the text.
stylex_test_panic!(
  a_reassigned_binding_declared_with_an_escaped_identifier_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let \u0063olor = 'red';
    color = 'blue';

    const styles = stylex.create({ x: { color } });
  "#
);

stylex_test_panic!(
  a_reassigned_binding_whose_value_carries_css_escapes_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let content = '"\\2014 \\00a0"';
    content = '"\\2013"';

    const styles = stylex.create({ x: { content } });
  "#
);

// A declaration sitting after a long run of multi-byte characters is the shape
// that turns a byte offset into a position inside a character.
stylex_test_panic!(
  a_reassigned_binding_declared_after_multibyte_text_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const label = 'λλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλ';
    let color = 'red';
    color = 'blue';

    const styles = stylex.create({ x: { color, content: label } });
  "#
);

// ── where the read sits ─────────────────────────────────────────────────────

// Nested pseudo and at-rule keys put the read several levels below the
// namespace, which is what the diagnostic's key path is built from. The
// refusal has to survive the nesting rather than be reported against the
// outermost key.
stylex_test_panic!(
  a_reassigned_binding_read_under_nested_rules_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let color = 'red';
    color = 'blue';

    const styles = stylex.create({
      x: {
        color: {
          default: 'black',
          '@media (min-width: 100px)': {
            default: 'grey',
            ':hover': { default: 'white', ':active': color },
          },
        },
      },
    });
  "#
);

// A module with many declarations ahead of the refused one: the declaration
// search walks the module in source order, and the answer must not depend on
// how much it walked past.
stylex_test_panic!(
  a_reassigned_binding_declared_below_many_others_is_refused,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const a1 = 1; const a2 = 2; const a3 = 3; const a4 = 4; const a5 = 5;
    const a6 = 6; const a7 = 7; const a8 = 8; const a9 = 9; const a10 = 10;
    const a11 = 11; const a12 = 12; const a13 = 13; const a14 = 14;
    const a15 = 15; const a16 = 16; const a17 = 17; const a18 = 18;

    let color = 'red';
    color = 'blue';

    const styles = stylex.create({ x: { color } });
  "#
);

// A name declared twice: once at module level and once in a block. The chain
// resolves bindings module-wide, so the outer declaration is the one the
// refusal is about, and the refusal is reached either way.
stylex_test_panic!(
  a_name_declared_twice_is_refused_once,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let color = 'red';
    color = 'blue';

    function unrelated() {
      const color = 'green';
      return color;
    }

    const styles = stylex.create({ x: { color } });
  "#
);

// ── refusals whose declaration is not a plain declarator ────────────────────

// A write is refused for the write, whatever kind of declaration the binding
// came from. Upstream asks whether a *binding* was written to, so every one of
// these answers `Referenced value is not a constant.` there; each was measured
// against 0.19.0, and `parity/corpus/positions.json` carries the positions.

stylex_test_panic!(
  a_destructured_binding_that_is_reassigned_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let { primary } = { primary: 'red' };
    primary = 'blue';

    const styles = stylex.create({ x: { color: primary } });
  "#
);

stylex_test_panic!(
  a_destructured_binding_whose_value_is_mutated_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const { theme } = { theme: { color: 'red' } };
    theme.color = 'blue';

    const styles = stylex.create({ x: { color: theme.color } });
  "#
);

stylex_test_panic!(
  an_array_destructured_binding_that_is_reassigned_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let [first] = ['red'];
    first = 'blue';

    const styles = stylex.create({ x: { color: first } });
  "#
);

// A reassigned `function` or `class` is refused for the write rather than for
// its declaration kind, because the write question is asked first -- upstream's
// 657 precedes the kind refusals at 685-690, and so does this chain's step 3.
stylex_test_panic!(
  a_reassigned_function_declaration_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    function paint() { return 'red'; }
    paint = 'blue';

    const styles = stylex.create({ x: { color: paint } });
  "#
);

stylex_test_panic!(
  a_reassigned_class_declaration_is_refused_for_the_write,
  "Referenced value is not a constant",
  r#"
    import * as stylex from '@stylexjs/stylex';

    class Paint {}
    Paint = 'blue';

    const styles = stylex.create({ x: { color: Paint } });
  "#
);

// A binding of a kind with no write recorded against it keeps its own refusal:
// the write steps are a probe first and a declaration question second, so a
// `function` nobody wrote to is still refused for being a `function`.
// A hoisted `function` read as a value, with the reference below it, is refused
// for its declaration kind — the refusal upstream reaches through the resolved
// declaration, which is why its frame names the `function` line.
stylex_test_panic!(
  a_function_declaration_read_as_a_value_is_refused_for_its_kind,
  "Unsupported expression: FunctionDeclaration",
  r#"
    import * as stylex from '@stylexjs/stylex';

    function color() { return 'red'; }

    const styles = stylex.create({ x: { color } });
  "#
);

// The same read from *above* the declaration is early instead, because the
// position comparison runs ahead of the declaration-kind refusals.
stylex_test_panic!(
  a_function_declaration_read_from_above_is_early_rather_than_unsupported,
  "Referenced value is used before declaration",
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({ x: { color: color() } });

    function color() { return 'red'; }
  "#
);

// A module binding that took a global's name carries no value to fold, so the
// reference is refused rather than answered with the global.
stylex_test_panic!(
  a_module_binding_that_took_a_global_name_is_refused,
  "Referenced constant is not initialized",
  r#"
    import * as stylex from '@stylexjs/stylex';

    let NaN;

    const styles = stylex.create({ x: { zIndex: NaN } });
  "#
);
