//! The evaluator's recursion budget, and where it sits against upstream.
//!
//! The fold walks a nested expression recursively. With no bound of its own its
//! real limit was the thread's stack, and its failure a process abort --
//! `fatal runtime error: stack overflow` -- which gives a bundler no message, no
//! file, and no chance to finish the rest of the build. Two things changed: the
//! fold runs on a stack it grows for itself, so the arms no longer fail at wildly
//! different depths, and it counts its own levels so that crossing the ceiling is
//! an ordinary refusal.
//!
//! Every folding case below was measured against `@stylexjs/babel-plugin` 0.19.0
//! under the same options and agrees with it on class names and rule text. The
//! refusals are where the two part company, and each says which way:
//!
//! - Under the ceiling both fold, identically.
//! - Over it we refuse with a message naming the file and the key path, and
//!   upstream keeps folding until its own engine throws
//!   `RangeError: Maximum call stack size exceeded`. Our ceiling is the lower of
//!   the two, and a refusal at a documented depth is the point.
//! - The one shape where ours is *higher* is a dynamic style: upstream throws a
//!   `RangeError` on a shadowing parameter nested 576 deep, and this compiler
//!   folds it to the same single custom property it folds a shallow one to.
//!
//! **Two ceilings appear below.** The shipped default is sized for styles
//! somebody wrote, and the first section is the only one that runs under it. Every
//! section after that raises it through the `maxEvaluationDepth` option, because
//! its subject is how deep a fold *can* go -- a case asking about 317 levels on
//! the default would be asserting that the default refuses them, which is a
//! different claim and one the first section already makes.
//!
//! Two rules for a case in this file, both learned from getting them wrong:
//!
//! 1. **Every folded value has to encode its depth.** A tower of
//!    `(true ? x : 0)` folds to `x` whatever its height, so a test asserting
//!    that passes whether 158 levels folded or one did. Each shape below adds
//!    `+ 1` or a character per level, so the asserted class name is a hash over
//!    a value only the full descent produces.
//! 2. **The ceiling is in fold levels, not source levels.** A member read
//!    descends to the object and then to the value under the key, an array
//!    element costs the array as well, a `Math.max` that also adds costs both,
//!    and a parenthesis is unwrapped before the fold is asked at all. So the
//!    deepest accepted source nesting differs per shape, and each one is
//!    measured and pinned rather than derived.

use crate::utils::{
  prelude::*,
  source::nest_expression as nest,
  // Compile under the shipped default ceiling -- what a project gets with no
  // configuration at all. Shared with the other files that compile a whole
  // module and assert on its rules, so none of them can drift into compiling
  // under different options.
  transform::{fold_module as fold, stringify_js},
};

/// `MY_CONST` under `depth` levels of `+ 1`, the shape every arm is measured
/// against unless it is the arm under test. Folds to `5 + depth`.
fn arithmetic(depth: usize) -> String {
  nest("(", " + 1)", "MY_CONST", depth)
}

fn create(decls: &str, body: &str) -> String {
  format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      {}
      export const styles = stylex.create({{
        {}
      }});
    "#,
    decls, body
  )
}

/// Compile with the ceiling raised to [`RAISED`].
///
/// Every case that measures how deep a fold *can* go uses this. The default is
/// sized for hand-written styles, so a case asking about 317 levels has to say
/// so: left on the default it would be asserting that the default refuses them,
/// which is a different claim and one this file already makes.
fn fold_deep(input: &str) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    deep_theme_import_transform(tr.comments.clone(), RAISED)
  })
}

/// The ceiling the deep cases run under. Chosen well above every boundary they
/// pin and well below the depth at which the stages *around* the fold run out of
/// stack, so a case that crosses it reports rather than aborting.
const RAISED: usize = 320;

/// The message a refusal past the ceiling carries, minus the key path each site
/// prefixes to it.
const TOO_DEEP: &str = "Expression is too deeply nested to evaluate at compile time.";

// ──────────────────────────────────────────────
// The shipped default
//
// Everything after this section raises the ceiling, because its subject is how
// deep a fold can go. This section is the only one that leaves it alone, and so
// the only one that says what a project with no configuration actually gets.
//
// The default is sized for styles somebody wrote, not for the deepest input that
// could be folded: 29 levels of arithmetic, 28 links of a member chain. Both
// numbers are measured, and both are far past anything in this repo's fixtures --
// nothing outside this file and three depth probes in the evaluator's own suite
// spends more than a handful of levels.
// ──────────────────────────────────────────────

// What a real theme read looks like, at the depth a real one has. Every value
// here folds, and every class name is upstream's -- which is the claim that
// matters about the default: it is not in the way.
#[test]
fn an_ordinary_theme_read_folds_under_the_default_ceiling() {
  let output = fold(&create(
    "const theme = { colors: { primary: '#123456' }, space: { md: '8px' } };",
    "base: { color: theme.colors.primary, padding: theme.space.md, marginTop: 4 * 2 + 'px' },",
  ));

  assert!(output.contains(".x1tfn4g9{color:#123456}"));
  assert!(output.contains(".xe8ttls{padding:8px}"));
  assert!(output.contains(".x1xmf6yo{margin-top:8px}"));
}

#[test]
fn the_default_ceiling_folds_twenty_nine_levels_of_arithmetic() {
  let output = fold(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(29)),
  ));

  assert!(output.contains(".x13i6vqj{z-index:34}"));
}

#[test]
#[should_panic(expected = "At most 32 levels of nested evaluation are supported")]
fn the_default_ceiling_refuses_thirty() {
  fold(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(30)),
  ));
}

// A member read spends two levels, so the chain runs out one link earlier than
// half of arithmetic's depth rather than at half of it. Pinned at the default as
// well as at the raised ceiling, because this is the shape a theme object is and
// so the one whose default matters most.
#[test]
fn the_default_ceiling_folds_a_twenty_eight_link_member_chain() {
  let output = fold(&create(
    &format!("const t = {};", nest("{ a: ", " }", "'red'", 28)),
    &format!("base: {{ color: t{} }},", ".a".repeat(28)),
  ));

  assert!(output.contains(".x1e2nbdu{color:red}"));
}

#[test]
#[should_panic(expected = "Expression is too deeply nested")]
fn the_default_ceiling_refuses_a_twenty_nine_link_member_chain() {
  fold(&create(
    &format!("const t = {};", nest("{ a: ", " }", "'red'", 29)),
    &format!("base: {{ color: t{} }},", ".a".repeat(29)),
  ));
}

// ──────────────────────────────────────────────
// Raising and lowering it
//
// The ceiling is configuration, so the tests that follow depend on being able to
// move it. These are the cases that say it moves -- without them every boundary
// below could be pinning a constant that nothing reads.
// ──────────────────────────────────────────────

// The same input, either side of the same ceiling: 100 levels refuses at the
// default and folds when the option allows it. One input, two verdicts, one
// option -- so a ceiling that stopped being read would fail here rather than
// silently widening every boundary in the file.
#[test]
fn the_option_raises_the_ceiling() {
  let input = create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(100)),
  );

  let output = fold_deep(&input);

  assert!(output.contains("z-index:105"));
}

#[test]
#[should_panic(expected = "Expression is too deeply nested")]
fn the_same_input_refuses_without_the_option() {
  fold(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(100)),
  ));
}

// And downwards: a ceiling of four refuses an expression the default folds
// without noticing. The message quotes the configured number rather than the
// default, which is how an author reading it knows which knob to turn.
#[test]
#[should_panic(expected = "At most 4 levels of nested evaluation are supported")]
fn the_option_lowers_the_ceiling_and_the_message_follows_it() {
  let input = create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(10)),
  );

  stringify_js(&input, ts_syntax(), |tr| {
    deep_theme_import_transform(tr.comments.clone(), 4)
  });
}

// ──────────────────────────────────────────────
// The ceiling, one arm at a time
//
// Each pair pins the last depth that folds and the first that refuses. The
// folding half also pins the class name, because a fold that survived the budget
// still has to produce upstream's answer -- a ceiling is only worth having if
// everything under it is unchanged.
// ──────────────────────────────────────────────

#[test]
fn arithmetic_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(317)),
  ));

  assert!(output.contains(".x9potx3{z-index:322}"));
}

#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn arithmetic_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(318)),
  ));
}

#[test]
fn a_string_concatenation_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_STR = 'a';",
    &format!(
      "base: {{ content: {} }},",
      nest("(", " + 'b')", "MY_STR", 317)
    ),
  ));

  // The value is `a` followed by 317 `b`s, so the class name is a hash over the
  // whole tower: one level short would hash differently.
  assert!(output.contains(".x1370d3k{content:\""));
  assert!(output.contains(&format!("a{}", "b".repeat(317))));
}

#[test]
#[should_panic(expected = "base > content > Expression is too deeply nested")]
fn a_string_concatenation_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_STR = 'a';",
    &format!(
      "base: {{ content: {} }},",
      nest("(", " + 'b')", "MY_STR", 318)
    ),
  ));
}

// `-(- x - 1)` per level, which is `x + 1` with a unary minus at every level. A
// plain `-(x)` tower was the first version of this and could not fail: 317
// negations of 5 and one negation of 5 are both `-5`. Three nodes per level, so
// the ceiling arrives at a third of arithmetic's source depth.
#[test]
fn unary_negation_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("-(- ", " - 1)", "MY_CONST", 105)
    ),
  ));

  assert!(output.contains(".x1nh0kk4{z-index:110}"));
}

#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn unary_negation_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("-(- ", " - 1)", "MY_CONST", 106)
    ),
  ));
}

#[test]
fn a_conditional_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("(true ? ", " + 1 : 0)", "MY_CONST", 158)
    ),
  ));

  assert!(output.contains(".xg6sfce{z-index:163}"));
}

#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn a_conditional_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("(true ? ", " + 1 : 0)", "MY_CONST", 159)
    ),
  ));
}

// A logical operand is folded under a forked confidence, so it is the one arm
// where a refusal is ordinary rather than terminal. The budget still travels: a
// fork that cannot fold its operand loses the whole expression, which is what
// makes this refusal reach the `create()` call at all.
#[test]
fn a_logical_operand_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("(", " + 1 || 0)", "MY_CONST", 158)
    ),
  ));

  assert!(output.contains(".xg6sfce{z-index:163}"));
}

#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn a_logical_operand_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("(", " + 1 || 0)", "MY_CONST", 159)
    ),
  ));
}

#[test]
fn a_template_literal_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_STR = 'a';",
    &format!(
      "base: {{ content: {} }},",
      nest("`${", "}b`", "MY_STR", 317)
    ),
  ));

  // Each interpolation appends a character, so the folded value is the same `a`
  // plus 317 `b`s the concatenation arm produces -- and the same class name.
  assert!(output.contains(".x1370d3k{content:\""));
  assert!(output.contains(&format!("a{}", "b".repeat(317))));
}

#[test]
#[should_panic(expected = "base > content > Expression is too deeply nested")]
fn a_template_literal_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_STR = 'a';",
    &format!(
      "base: {{ content: {} }},",
      nest("`${", "}b`", "MY_STR", 318)
    ),
  ));
}

// A nested `Math.max` is the one shape in this file that does not answer to the
// configured ceiling at all. `Math` folds in the engine now, and the engine's
// parser recurses on the bare thread stack, so the fold carries a ceiling of its
// own -- 32 levels, which a `Math.max` that also adds spends two of per source
// level. It refuses at 17 where it folded 158 before the statics moved, and
// raising the project's ceiling does not move it.
//
// That is the second, lower limit ticket 11 exists to remove; the string and
// array surfaces have answered to it since their own tables were deleted, so
// this is `Math` joining a rule rather than a rule made for `Math`.
#[test]
fn a_builtin_call_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("Math.max(", " + 1, 0)", "MY_CONST", 16)
    ),
  ));

  assert!(output.contains(".x1jinmle{z-index:21}"));
}

#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn a_builtin_call_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("Math.max(", " + 1, 0)", "MY_CONST", 17)
    ),
  ));
}

// One level short of arithmetic's, because reading `.a` descends twice: to the
// object the read is against, and then to the value found under the key. The
// nesting is in the object as well as in the chain, so a chain that stopped
// early would read an object rather than the number and never reach CSS.
#[test]
fn a_member_chain_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    &format!("const o = {};", nest("{ a: ", " }", "1", 316)),
    &format!("base: {{ zIndex: o{} }},", ".a".repeat(316)),
  ));

  assert!(output.contains(".x1vjfegm{z-index:1}"));
}

// The key path names every `a` it walked through before giving up, so the
// expectation matches the tail rather than repeating 317 of them.
#[test]
#[should_panic(expected = "a > Expression is too deeply nested")]
fn a_member_chain_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    &format!("const o = {};", nest("{ a: ", " }", "1", 317)),
    &format!("base: {{ zIndex: o{} }},", ".a".repeat(317)),
  ));
}

// Two short of arithmetic's: a spread descends to the object it copies, and the
// read that follows spends the level a member read spends.
#[test]
fn a_spread_chain_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    &format!("const o = {};", nest("{ ...", " }", "{ a: 3 }", 315)),
    "base: { zIndex: o.a },",
  ));

  assert!(output.contains(".xzkaem6{z-index:3}"));
}

// The spread's own key is the last thing the path names, because the read that
// could not be folded is the one under it.
#[test]
#[should_panic(expected = "base > zIndex > a > Expression is too deeply nested")]
fn a_spread_chain_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    &format!("const o = {};", nest("{ ...", " }", "{ a: 3 }", 316)),
    "base: { zIndex: o.a },",
  ));
}

// Parentheses are unwrapped before the fold is asked, so they cost nothing at
// all: 512 of them fold where 318 additions do not. Pinned because it is the one
// nesting the ceiling deliberately does not count, and a reader measuring the
// budget against source text would otherwise call this a bug.
#[test]
fn parentheses_are_unwrapped_before_the_ceiling_is_consulted() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", nest("(", ")", "MY_CONST", 512)),
  ));

  assert!(output.contains(".x1u8a7rm{z-index:5}"));
}

// ──────────────────────────────────────────────
// Where in the style the depth sits
//
// The value is not the only place an expression can be nested. Each of these
// reaches the fold through a different part of the `create()` argument, and the
// refusal has to name that part -- a message that said only "too deeply nested"
// would leave an author reading a 300-line generated file with nowhere to look.
// ──────────────────────────────────────────────

#[test]
fn a_pseudo_class_value_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {{ default: 1, ':hover': {} }} }},",
      arithmetic(316)
    ),
  ));

  assert!(output.contains(".x1vjfegm{z-index:1}"));
  assert!(output.contains(".x1q5tbnt:hover{z-index:321}"));
}

#[test]
#[should_panic(expected = "base > zIndex > :hover > Expression is too deeply nested")]
fn a_pseudo_class_value_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {{ default: 1, ':hover': {} }} }},",
      arithmetic(317)
    ),
  ));
}

#[test]
fn a_media_query_value_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {{ default: 1, '@media (min-width: 100px)': {} }} }},",
      arithmetic(316)
    ),
  ));

  assert!(output.contains("@media (min-width: 100px){.x17xxns9.x17xxns9{z-index:321}}"));
}

#[test]
#[should_panic(expected = "@media (min-width: 100px) > Expression is too deeply nested")]
fn a_media_query_value_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {{ default: 1, '@media (min-width: 100px)': {} }} }},",
      arithmetic(317)
    ),
  ));
}

// A computed key is folded through its own evaluation, which starts fresh. The
// budget lives on the per-file state rather than on the evaluation's, precisely
// so a nested fold cannot hand itself a new allowance while the frames it would
// spend are still standing -- so the key answers to the same ceiling the value
// does. The refusal names the namespace only, because the key it would have
// named is the thing that could not be read.
#[test]
fn a_computed_key_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;\n      const KEY = 'zIndex';",
    &format!(
      "base: {{ [{}]: MY_CONST }},",
      nest("(", " + '')", "KEY", 317)
    ),
  ));

  assert!(output.contains(".x1u8a7rm{z-index:5}"));
}

#[test]
#[should_panic(expected = "base > Expression is too deeply nested")]
fn a_computed_key_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;\n      const KEY = 'zIndex';",
    &format!(
      "base: {{ [{}]: MY_CONST }},",
      nest("(", " + '')", "KEY", 318)
    ),
  ));
}

// A custom property is the one key whose name reaches CSS as written, so the
// refusal names it as written too -- `--depth`, not a camel-cased reading of it.
#[test]
fn a_custom_property_value_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ '--depth': {} }},", arithmetic(317)),
  ));

  assert!(output.contains(".xsrk0h2{--depth:322}"));
}

#[test]
#[should_panic(expected = "base > --depth > Expression is too deeply nested")]
fn a_custom_property_value_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ '--depth': {} }},", arithmetic(318)),
  ));
}

// A logical property is renamed and direction-resolved after the fold, so the
// depth is spent before any of that happens and the ceiling is the value's.
#[test]
fn a_logical_property_value_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_STR = '1px';",
    &format!(
      "base: {{ marginInlineStart: {} }},",
      nest("(", " + '')", "MY_STR", 317)
    ),
  ));

  assert!(output.contains(".xm2jcoa{margin-inline-start:1px}"));
}

#[test]
#[should_panic(expected = "base > marginInlineStart > Expression is too deeply nested")]
fn a_logical_property_value_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_STR = '1px';",
    &format!(
      "base: {{ marginInlineStart: {} }},",
      nest("(", " + '')", "MY_STR", 318)
    ),
  ));
}

// A shorthand under `legacy-expand-shorthands` turns one folded value into four
// declarations, so this is the case where the value crossing the ceiling is read
// by property expansion rather than emitted as-is. All four class names are
// upstream's, which is what says the expansion saw the whole tower and not a
// truncation of it.
//
// This is as close as the boundary gets to a vendor-prefixing case: this
// compiler emits no prefixes at all -- every test in
// `transform_polyfills_test/css_property_polyfills.rs` is `#[ignore]`d and
// `css_value_polyfills.rs` has none -- so there is no prefixed output for a
// depth to interact with. Expansion is the one-value-to-many-declarations path
// that does exist.
#[test]
fn a_shorthand_expansion_folds_at_the_deepest_accepted_nesting() {
  let input = create(
    "const V = '1px 2px';",
    &format!("base: {{ padding: {} }},", nest("(", " + '')", "V", 317)),
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    build_test_transform(tr.comments.clone(), |b| {
      b.with_runtime_injection()
        .with_max_evaluation_depth(RAISED)
        .with_style_resolution(StyleResolution::LegacyExpandShorthands)
    })
  });

  assert!(output.contains(".x4p5aij{padding-top:1px}"));
  assert!(output.contains(".x14vy60q{padding-inline-end:2px}"));
  assert!(output.contains(".x1j85h84{padding-bottom:1px}"));
  assert!(output.contains(".xyiysdx{padding-inline-start:2px}"));
}

#[test]
#[should_panic(expected = "base > padding > Expression is too deeply nested")]
fn a_shorthand_expansion_refuses_one_level_past_the_ceiling() {
  let input = create(
    "const V = '1px 2px';",
    &format!("base: {{ padding: {} }},", nest("(", " + '')", "V", 318)),
  );

  stringify_js(&input, ts_syntax(), |tr| {
    build_test_transform(tr.comments.clone(), |b| {
      b.with_runtime_injection()
        .with_max_evaluation_depth(RAISED)
        .with_style_resolution(StyleResolution::LegacyExpandShorthands)
    })
  });
}

// A style array spends a level on the array itself, so its ceiling is one under
// the same expression written bare. Both elements reach the same declaration,
// which is what makes the folded depth visible in the rule.
#[test]
fn a_style_array_element_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: [{}, 2] }},", arithmetic(316)),
  ));

  assert!(output.contains(".xatwel5{z-index:321;z-index:2}"));
}

#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn a_style_array_element_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: [{}, 2] }},", arithmetic(317)),
  ));
}

// `Array(n)` folds its length through the same descent, and the call plus the
// `.length` read off the result cost two more levels than the length alone. The
// fold's *other* size budget -- the one on how many elements `Array(n)` may
// produce -- is never reached here, because this ceiling arrives first.
#[test]
fn an_array_length_folds_at_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: Array({}).length }},", arithmetic(315)),
  ));

  assert!(output.contains(".x12lt65p{z-index:320}"));
}

#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn an_array_length_refuses_one_level_past_the_ceiling() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: Array({}).length }},", arithmetic(316)),
  ));
}

// ──────────────────────────────────────────────
// What the budget is spent on
// ──────────────────────────────────────────────

// Per expression, not per file: two siblings at 300 levels each fold, where a
// counter that accumulated across the object would refuse the second. Both class
// names are pinned so a leak would show up as a wrong hash rather than only as a
// refusal.
#[test]
fn the_budget_is_spent_per_expression_rather_than_per_file() {
  let output = fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {}, opacity: {} }},",
      arithmetic(300),
      arithmetic(300)
    ),
  ));

  assert!(output.contains(".xuyztwe{z-index:305}"));
  assert!(output.contains(".xzuca24{opacity:305}"));
}

// A depth far past the ceiling refuses exactly as one level past it does. This
// is the case the budget exists for: at 512 levels this input used to abort the
// process, which is why the boundary could not be pinned by a test at all --
// crossing it took the test binary down instead of failing.
#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn a_depth_far_past_the_ceiling_refuses_rather_than_aborting() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(512)),
  ));
}

// ──────────────────────────────────────────────
// Not the shadowing
//
// The abort was found while measuring a dynamic parameter that shadows an
// imported binding, and nothing in it depended on the shadowing. Both halves are
// pinned here: the plain module-level constant refuses, and the shadowing shape
// the abort was reported against now folds -- further than upstream manages.
// ──────────────────────────────────────────────

// The reported shape, at the depth it used to abort at. `zIndex` is the imported
// theme key and also the parameter, so the whole tower is a reference to the
// parameter; it collapses into the one inline custom property a bare `zIndex`
// collapses into, and buys no extra declarations for its depth.
//
// Upstream refuses this input: `@stylexjs/babel-plugin` 0.19.0 throws
// `RangeError: Maximum call stack size exceeded` from 576 levels up, where this
// compiler folds it. Ours is the higher ceiling in this one shape, and the
// divergence is an acceptance rather than a refusal.
#[test]
fn a_shadowing_dynamic_parameter_folds_far_past_the_ceiling() {
  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      import {{ zIndex }} from 'zIndex.stylex.js';
      export const styles = stylex.create({{
        dyn: (zIndex) => ({{ zIndex: {} }}),
      }});
    "#,
    nest("(", " + 1)", "zIndex", 576)
  );

  let output = fold_deep(&input);

  assert!(output.contains(".xr3buco{z-index:var(--x-zIndex)}"));
  assert_eq!(output.matches("@property --x-").count(), 1);
}

// The same depth over a module-level constant, with no parameter and no import
// to shadow, is the refusal. Together with the case above this is the answer to
// whether the abort belonged to the shadowing: it did not.
#[test]
#[should_panic(expected = "base > zIndex > Expression is too deeply nested")]
fn the_ceiling_does_not_depend_on_a_shadowed_binding() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(576)),
  ));
}

// ──────────────────────────────────────────────
// A refusal the depth did not cause
//
// A budget that fired first would rename every other refusal at depth, turning
// a diagnostic an author can act on into one about nesting. These pin which
// refusal wins, and each agrees with upstream on the message as well as on the
// verdict.
// ──────────────────────────────────────────────

// A regular expression is refused wherever it appears, and it is the seed here,
// so the fold reaches it before it reaches the ceiling. Upstream refuses with
// the same sentence.
#[test]
#[should_panic(expected = "base > zIndex > Unsupported expression: RegExpLiteral")]
fn a_regex_seed_under_the_ceiling_keeps_its_own_refusal() {
  fold_deep(&create(
    "",
    &format!("base: {{ zIndex: {} }},", nest("(", " + 1)", "/a/", 317)),
  ));
}

// And one level deeper it is still the regex, not the depth: the seed sits at the
// bottom of the tower, so a level added at the top does not put the ceiling in
// front of it. Upstream refuses this too, with the same message.
#[test]
#[should_panic(expected = "base > zIndex > Unsupported expression: RegExpLiteral")]
fn a_regex_seed_one_level_deeper_still_keeps_its_own_refusal() {
  fold_deep(&create(
    "",
    &format!("base: {{ zIndex: {} }},", nest("(", " + 1)", "/a/", 318)),
  ));
}

// A BigInt has no CSS spelling and no safe lossy conversion to one, so it is
// refused for what it is rather than for where it sits. Upstream agrees.
#[test]
#[should_panic(expected = "base > zIndex > Unsupported expression: BigIntLiteral")]
fn a_bigint_seed_under_the_ceiling_keeps_its_own_refusal() {
  fold_deep(&create(
    "",
    &format!("base: {{ zIndex: {} }},", nest("(", " + 1n)", "1n", 317)),
  ));
}

// A value that folds fine and is then refused as CSS: 317 levels of appending
// the empty string to `rgb(0,0,` produce exactly `rgb(0,0,`, and the rule is
// what refuses. The message names the rule rather than the nesting, which is
// upstream's answer as well.
#[test]
#[should_panic(expected = "Rule contains an unclosed function")]
fn an_unclosed_css_function_folded_at_depth_is_refused_as_css() {
  fold_deep(&create(
    "const MY_STR = 'rgb(0,0,';",
    &format!("base: {{ color: {} }},", nest("(", " + '')", "MY_STR", 317)),
  ));
}

// Nested conditions are refused for repeating a condition, not for nesting: a
// `default` inside a `default` is the same key twice, and one level of it is
// already too many. So the shape a reader might reach for to measure the ceiling
// never gets near it -- in either compiler.
#[test]
fn one_level_of_condition_under_a_property_folds() {
  let output = fold_deep(&create("", "base: { zIndex: { default: 5 } },"));

  assert!(output.contains(".x1u8a7rm{z-index:5}"));
}

#[test]
#[should_panic(expected = "The same pseudo selector or at-rule cannot be used more than once")]
fn two_nested_conditions_are_refused_as_a_repeat_rather_than_as_depth() {
  fold_deep(&create(
    "",
    "base: { zIndex: { default: { default: 5 } } },",
  ));
}

// `NaN` and `Infinity` reach CSS as their JavaScript spellings, and 317 levels of
// arithmetic do not change either. Both class names are upstream's.
#[test]
fn nan_survives_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "",
    &format!("base: {{ zIndex: {} }},", nest("(", " + 1)", "NaN", 317)),
  ));

  assert!(output.contains(".x1uhybf7{z-index:NaN}"));
}

#[test]
fn infinity_survives_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("(", " + 1)", "Infinity", 317)
    ),
  ));

  assert!(output.contains(".xbdygrb{z-index:Infinity}"));
}

// A non-ASCII seed and an escaped non-ASCII appendix, concatenated 317 times.
// The value is asserted as well as the hash, so a fold that truncated or
// re-encoded what it carried through the budget check would be caught by the
// character count rather than only by a changed class name.
#[test]
fn a_unicode_escape_survives_the_deepest_accepted_nesting() {
  let output = fold_deep(&create(
    "const MY_STR = 'é';",
    &format!(
      "base: {{ content: {} }},",
      nest("(", " + '\\u2014')", "MY_STR", 317)
    ),
  ));

  assert!(output.contains(".xy003ii{content:\"é—"));
  assert!(output.contains(&format!("é{}", "—".repeat(317))));
}

// A refusal in a `keyframes()` call is reported by that call rather than by the
// key path, so the depth message is not the one an author sees. Pinned because
// it is the one measured place where the sentence changes: upstream folds this
// input, and a reader comparing the two needs to know the divergence is the
// ceiling and not the phrasing.
#[test]
#[should_panic(expected = "Only static values are allowed inside of a keyframes() call")]
fn a_keyframes_value_past_the_ceiling_is_refused_by_the_call() {
  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      const MY_CONST = 5;
      const fade = stylex.keyframes({{
        from: {{ zIndex: {} }},
        to: {{ zIndex: 0 }},
      }});
      export const styles = stylex.create({{ base: {{ animationName: fade }} }});
    "#,
    arithmetic(317)
  );

  fold_deep(&input);
}

// ──────────────────────────────────────────────
// Source the fold never gets to see
//
// A deep expression that does not parse is refused by the parser, at the depth it
// would otherwise have been folded at. Pinned because a ceiling that reported
// "too deeply nested" for a missing bracket would send an author looking for the
// wrong problem, and because these are the inputs where a recursive-descent
// parser is the thing most likely to break. Upstream refuses all three too, as
// `Unexpected token, expected ","`.
// ──────────────────────────────────────────────

#[test]
#[should_panic(expected = "Expected ','")]
fn an_unbalanced_bracket_at_depth_is_refused_by_the_parser() {
  let expr = format!("{}MY_CONST + 1{}", "(".repeat(317), ")".repeat(316));

  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", expr),
  ));
}

#[test]
#[should_panic(expected = "Expected ','")]
fn an_unterminated_quote_at_depth_is_refused_by_the_parser() {
  fold_deep(&create(
    "const MY_STR = 'a';",
    &format!(
      "base: {{ content: {} }},",
      nest("(", " + 'b)", "MY_STR", 317)
    ),
  ));
}

#[test]
#[should_panic(expected = "Expected ','")]
fn an_invalid_token_sequence_at_depth_is_refused_by_the_parser() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!(
      "base: {{ zIndex: {} }},",
      nest("(", " + 1)", "MY_CONST ** * 2", 317)
    ),
  ));
}

// ──────────────────────────────────────────────

// The refusal carries the depth the fold was willing to spend, so the number in
// the message and the number in the code cannot drift apart without this
// failing.
#[test]
#[should_panic(expected = "At most 320 levels of nested evaluation are supported")]
fn the_refusal_names_the_raised_ceiling_it_crossed() {
  fold_deep(&create(
    "const MY_CONST = 5;",
    &format!("base: {{ zIndex: {} }},", arithmetic(318)),
  ));
}

// The message is stable enough to assert on in one place, so the constant above
// is the one every other refusal here matches a prefix of.
#[test]
fn the_refusal_message_is_the_one_the_other_cases_match() {
  assert!(
    stylex_constants::constants::evaluation_errors::expression_too_deep(320).starts_with(TOO_DEEP)
  );
}

// ──────────────────────────────────────────────
// The budget and the memo
//
// A depth refusal is the only refusal that depends on *where* a subtree sits
// rather than on what it says. The memo is keyed by a structural hash that
// carries no depth, so the two have to be kept apart: a subtree that refused
// because it was reached too deep must not answer for the same subtree written
// shallowly. The cases below are the ones that go wrong when they are not.
// ──────────────────────────────────────────────

/// The shared subtree. Ten levels folds well inside every ceiling here, and the
/// innermost ten levels of a taller `arithmetic` tower are structurally this
/// exact expression -- which is what puts both readings on one memo key.
const SHARED_DEPTH: usize = 10;

/// A dynamic style is where a refusal degrades instead of aborting: the value
/// becomes a custom property and the build carries on. That is what makes a
/// depth refusal *observable* beside a static namespace rather than ending the
/// compile before the static one is reached.
fn shared_subtree_in_both_orders(deep_first: bool) -> String {
  let deep = format!(
    "deep: (w) => ({{ zIndex: {} }}),",
    nest("(", " + 1)", "MY_CONST", 40)
  );
  let shallow = format!("shallow: {{ zIndex: {} }},", arithmetic(SHARED_DEPTH));

  let body = if deep_first {
    format!("{}\n{}", deep, shallow)
  } else {
    format!("{}\n{}", shallow, deep)
  };

  fold(&create("const MY_CONST = 5;", &body))
}

// A namespace that folds on its own folds wherever it is written. Before the
// memo learned to leave a depth refusal out, the ancestors of the refusal were
// recorded as unresolved under a key carrying no depth, so they answered for the
// shallow reading of the same subtree: writing the deep namespace first refused
// `shallow` outright, and a style that compiles perfectly well alone failed the
// build because of a sibling.
#[test]
fn a_depth_refusal_does_not_refuse_a_shallow_reading_of_the_same_subtree() {
  for (deep_first, order) in [(true, "deep first"), (false, "shallow first")] {
    let output = shared_subtree_in_both_orders(deep_first);

    assert!(
      output.contains(".x52sccv{z-index:15}"),
      "{order}: the shallow namespace folds on its own, but did not: {output}"
    );
  }
}

// The other direction is not symmetric, and deliberately so. The ceiling counts
// the levels the fold descends, and a memo hit descends none -- so writing the
// shallow namespace first warms the inner subtree and lets the deep one fold to
// `z-index:45`, where alone it refuses and becomes a custom property. Charging a
// hit for the height it skips was measured and refused two of the member-chain
// boundaries above, because the height a subtree records is the deepest the fold
// went anywhere under it rather than along the path a later read takes. Left as
// it is because it only ever folds *more*: upstream has no ceiling, so every
// case this decides differently is one upstream folds as well.
#[test]
fn a_warm_inner_subtree_lets_a_deeper_expression_fold() {
  assert!(
    shared_subtree_in_both_orders(false).contains("z-index:45"),
    "the deep namespace folds once its inner subtree is warm"
  );
  assert!(
    shared_subtree_in_both_orders(true).contains(".xr3buco{z-index:var(--x-zIndex)}"),
    "and refuses when it is the first thing folded"
  );
}
