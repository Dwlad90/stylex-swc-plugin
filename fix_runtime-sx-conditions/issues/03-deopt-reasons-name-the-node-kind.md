# 03 — A deopt reason names what it could not fold

Status: `resolved`
Phase: Phase 1

**Blocked by:** 02

**What to build:** `Unsupported expression: Unknown` stops being the answer.

The reference implementation reports
`errMsgs.UNSUPPORTED_EXPRESSION(path.node.type)` — `Unsupported expression:
CallExpression` for `["a","b"].filter(Boolean)`. This compiler already answers
`Unsupported expression: Unknown` for the same input, dropping the node kind.

Issue 02 converts roughly fifteen panics into deopts, which spreads that
vagueness. Inside `stylex.create()` a deopt still surfaces as a hard error, so
without this the change is a net regression in diagnostics for anyone whose
deopt lands in a static-required position: `The method 'startsWith' is not yet
supported in static evaluation` would become `Unsupported expression: Unknown`.

The reason is already threaded through `deopt`; it is the node-kind label that
is missing.

## Comments

### What the label was, and what it is now

`get_expr_node_kind` lives in `stylex-utils/src/swc.rs`, beside
`get_default_expr_ctx`, and answers the ESTree name of an expression node. The
match carries no wildcard arm, so a new SWC expression kind fails to compile
there rather than quietly reporting the wrong name.

Six sites asked `Expr::get_type` for the label. That reports the *value* an
expression would produce, which is `Unknown` for everything a static evaluation
cannot fold — so it was `Unknown` in every case it was ever printed, not just
for the one input the ticket quotes. The ticket estimated the vagueness would
*spread* with 02; it was already total.

The names are the ecosystem's rather than SWC's, because the label reaches the
author. Three places where the two ASTs disagree about node boundaries are
resolved towards ESTree: a logical operator is a `LogicalExpression` (SWC keeps
`&&`, `||` and `??` in `Expr::Bin`), an optional chain is named by its base
(`OptionalMemberExpression` / `OptionalCallExpression`, one variant in SWC), and
`super.x` is a `MemberExpression`. Two SWC nodes have no single ESTree spelling
and are documented at the function: `TsConstAssertion` carries both `x as const`
and `<const>x`, and `Invalid` is a parse failure with no node at all.

### Which node each site names, and why they are not all the same

Three of the six name the expression at the deopt path — the terminal arms in
`evaluate/mod.rs` (twice) and in `nodes/call_expression.rs`. Three name a value
that was folded on the way there: the method receiver in
`nodes/call_expression.rs`, the property receiver in
`nodes/member_expression.rs`, and the `typeof` operand in
`nodes/unary_expression.rs`.

That split is deliberate and is a divergence from the reference
implementation's uniform `path.node.type`. A refusal that happens *after* an
evaluation succeeded knows something the deopt path does not: `({ a: () => 1
}).a.b` reports `ArrowFunctionExpression`, where naming the node at the path
would say `Unsupported expression: MemberExpression` under a code frame already
showing a member expression — a restatement, not a diagnostic. Which half of
`a.b` the evaluator could not use is the part the author cannot see. Each site
comment says which node it names so the choice is not mistaken for drift.

### Two sites outside `evaluate/`

`utils/ast/convertors.rs::expr_to_num` reports through its `Result` rather than
aborting (issue 02), so its two `stylex_bail!` labels are author-facing:
`Math.abs({})` now says `ObjectExpression`. The remaining panic labels in the
same file were changed too — a panic message naming a value type is no more use
in a bug report than in a build error — and dropping `ExprExt` from that file
and from `evaluate/mod.rs` fell out of it.

### Measured against the reference implementation

Twelve messages are byte identical for the same input, measured by running
`@stylexjs/babel-plugin` rather than written by hand:

| input | both compilers |
| --- | --- |
| `content: this` | `Unsupported expression: ThisExpression` |
| `content: new Date()` | `NewExpression` |
| `content: import.meta` | `MetaProperty` |
| `content: function () {}` | `FunctionExpression` |
| `content: class {}` | `ClassExpression` |
| ``content: String.raw`a` `` | `TaggedTemplateExpression` |
| `width: 10n` | `BigIntLiteral` |
| `content: (1)()` | `CallExpression` |
| `content: typeof /a/` | `RegExpLiteral` |
| `content: (function () {}).name` | `FunctionExpression` |
| `content: counter++` | `UpdateExpression` |
| `content: (counter = 1)` | `AssignmentExpression` |

Three inputs reject in both compilers with different text, each for a reason
already recorded elsewhere: `(5).toFixed(2)` (the spec's non-goal — the
reference implementation raises `Number.prototype.toFixed requires that 'this'
be a Number`), `[1, 2].filter(1)` (`number 1 is not a function` there,
`Unsupported expression: CallExpression` here), and `({ a: () => 1 }).a.b`
(`A style value can only contain an array, string or number.` there). The
spec's non-goals already establish that no build can depend on the text.

### Found on the way, not fixed here

A regex literal folds. `stylex.create({ x: { content: /a/ } })` compiles here
and deopts in the reference implementation with `Unsupported expression:
RegExpLiteral`. It is the mirror of the folds issue 06 tracks — a value folded
that upstream refuses, rather than refused where upstream folds — and it
changes emitted CSS, so it does not belong in a diagnostics change. Note that
`typeof /a/` *does* refuse here and matches upstream, so the divergence is in
the literal reaching a style value, not in the label.

### Tests

- `stylex-utils/src/tests/swc_test.rs` — every `Expr` variant, named from the
  syntax that produces it rather than from the variant it came from, so a
  mapping written off the variant names cannot agree with itself. Plus the
  logical/binary split, the optional-chain base, the six TypeScript kinds, the
  seven kinds that only appear nested, a parse failure, deep nesting, and
  non-ASCII and escaped source.
- `evaluate/tests/unsupported_shape_tests.rs` — one input reaching each of the
  six sites, with **exact** reason assertions rather than substring ones, since
  a label that loses the node kind passes a `contains` check. Plus the three
  logical operands, and the folds beside each refusal so "refuse everything"
  cannot pass.
- `tests/validation_stylex_create_test/invalid_values.rs` — thirteen
  author-facing diagnostics at the compiler seam, which is where a deopt reason
  becomes a build error.

Site coverage was confirmed by temporarily tagging each of the six labels with a
marker and reading which input carried which tag, rather than by assuming an
input reaches the arm its shape suggests. Two did not: `(function () {}).name`
refuses while *evaluating* the receiver, not at the member arm, and
`(() => 1).length` refuses on the property lookup instead. The member-receiver
arm needs a receiver that folds first — `({ a: () => 1 }).a.b`.

### One test defect, and what it says about asserting on a diagnostic

The two assertions that read an `expr_to_num` reason initially compared the
rendered string. That reason comes from `stylex_bail!`, which renders through
`StyleXError`'s `Display` — and that colours the `[StyleX]` prefix and the
message, but only when the process decides colour is wanted, which depends on
whether stderr is a terminal and on `NO_COLOR` / `CLICOLOR_FORCE`. The tests
therefore passed with output piped to a file and failed in a terminal.

Worth recording because the trap is general: any assertion on a
`StyleXError`-rendered string has it, and the obvious repair — pasting the
escape codes into the expectation — pins styling as though it were part of the
diagnostic and still only holds under one of the two colour modes. The styling
is stripped before comparing instead, and the stripper is pinned itself, since
the assertions are only as exact as it is. The `should_panic` tests at the
compiler seam were unaffected: they match a substring, and the whole message is
one coloured span.

### Review, and what it caught

Both axes independently found that the ticket's cited input never reaches a
node-kind label: `["a", "b"].filter(Boolean)` answers `Referenced constant is
not defined.` here *and* upstream, because `Boolean` is not a folded global in
either. The ticket's regression premise — that `The method 'startsWith' is not
yet supported` would become `Unsupported expression: Unknown` — was therefore
wrong; the requirement above it ("`Unsupported expression: Unknown` stops being
the answer") is what was implemented, and no label site uses `Expr::get_type`
any more.

Two defects, both a comment claiming more than its test does:

1. `names_a_logical_operator_as_one` asserted `Expression is not a number:
   ObjectExpression` while claiming to pin the `LogicalExpression` mapping — it
   passed whether the mapping answered `LogicalExpression` or
   `BinaryExpression`. The label turns out **not to be reachable as a deopt
   reason** at all: every site that names a node either has a dedicated binary
   arm ahead of it or names a folded value, which is never a `Bin`. Removed;
   the mapping is pinned where it is real, in `stylex-utils`.
2. The compiler-seam header claimed all thirteen messages were byte identical
   upstream. Re-measuring all thirteen gives **eleven**. The two exceptions are
   marked at the test, and neither divergence is in the label:
   `({ a: () => 1 }).a.b` is rejected upstream with a different diagnostic, and
   `-({})` is not rejected upstream at all (it folds to `width:NaNpx`). The note
   on the call test was wrong the other way — `(1)()` *is* identical; it is
   `[1, 2].filter(1)` that diverges.

Also from review, and applied: the `Unsupported expression: ...` frame is now
sourced from the constant that owns it rather than spelled at each of thirty
expectations, so the node kind is the only thing they state.

Also applied, each in its own commit:

- **The five remaining value-type labels** —
  `stylex-structures/src/base_css_type.rs` (three catch-alls),
  `stylex-ast/src/ast/convertors.rs` (`convert_lit_to_number`),
  `stylex-rs-compiler/src/utils/fn_parser.rs` (the napi bridge). Each carried
  the same `Unknown` defect and each is the same one-line substitution, so the
  migration off `Expr::get_type` for author-facing labels is now complete.
  `ExprExt` drops out of all three files. The second `fn_parser` panic is
  deliberately left alone: it reports a napi `ValueType`, not an expression, and
  naming that enum is already the useful answer.

  This also settles the scope question below. With the workspace consistent, the
  change is "every author-facing node label names the node kind", not "deopt
  reasons only with some panics along for the ride" — one rule, no site left
  reporting the value type.

  Coverage: the single existing test that reached any of them asserted the
  message *prefix* only, which is exactly what let the vague label sit there
  unnoticed. It now names the kind, and the two sibling catch-alls it left
  uncovered are pinned beside it. The napi sites need a live napi env and remain
  covered only by the JS suite.

- **`strip_ansi` replaced by `colored::control::set_override(false)`** in the
  `#[ctor]` the suite already runs. Reversing the earlier call: the concern was
  global mutation from a concurrent suite, but the override is set before any
  test starts and nothing in the suite asserts that colour is *present*, so
  there is no ordering hazard. It is the better fix because it protects every
  diagnostic assertion in the binary, including ones not written yet, where a
  helper only protects the assertions that remember to call it. Verified to beat
  the environment: the exact-equality assertions pass under a forced-colour run
  with no stripping, which they could not if the override were ignored. Drops
  twenty-five lines of hand-rolled escape-sequence handling and its own test.

- **Both `TsConstAssertion` spellings pinned.** `x as const` and `<const>x`
  collapse into one SWC variant, so one is necessarily named for the other; the
  choice is now asserted rather than only documented, and a future SWC split
  fails the test instead of silently relabelling.

- **The ticket's cited inputs pinned.** `["a", "b"].filter(Boolean)` and
  `"documentation".startsWith(lowerQuery)` are asserted to refuse by identifier
  and by method name, so the corrected premise lives in code. Without them a
  reader checking this work finds the cited inputs absent and may "restore" the
  label onto arms that never produced it.

- **`get_default_expr_ctx` removed.** It existed to feed `Expr::get_type`, and
  every caller was a diagnostic label this work relabelled, so nothing in the
  workspace called it any more — only its own six tests did, which is coverage
  of a function no build depends on. `ExprCtx` and `SyntaxContext` left the
  crate's imports with it and no documentation referenced it.

Noted and not applied:

- **Primitive Obsession on `-> &'static str`** — suppressed by the glossary,
  which defines Node kind as that string, and by `unsupported_expression` taking
  `&str`.

### Verification

`cargo test --workspace --all-features` (27 binaries, 0 failures) under
`CLICOLOR_FORCE=1`, `NO_COLOR=1` and neither,
`cargo check --workspace --all-targets`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all`,
`pnpm format:check`, `pnpm lint:check`. No JavaScript or TypeScript changed and
no JS test asserts on an evaluation diagnostic, so the `dist/*.node` suites are
unaffected.

### History

The branch carries six commits for this ticket. The `strip_ansi` helper the
colour fix originally introduced does not appear in any of them: the commit
adding it and the commit removing it were squashed, since their only net
contribution was the `colored` override and the two files it touches. The
rewrite was checked by tree hash rather than by eye — the reshaped history
produces byte-identical content to the pre-squash branch.
