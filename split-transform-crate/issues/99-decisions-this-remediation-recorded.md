# 99 — Decisions this remediation recorded

**What this holds:** the items of the second branch-wide review that were
answered with a decision rather than a change, and the open risk it could not
settle. Filed here for the reason [86](./86-decisions-the-remediation-recorded.md)
was: so a later reader finds the reasoning where the review is.

**Blocked by:** None.

**Status:** resolved

## The open risk, settled: `stale_coordinates()` does not false-positive

The review flagged the new staleness check in `scripts/coverage-missing.sh` as
an open risk. It refuses a whole report -- exit 3 -- when any measured `Code`
region starts on a blank line or on a line whose first token is `//`. The worry
was the false-positive rate: a macro-expanded region and a `#[derive]`-generated
region both carry coordinates that need not land on a line of real code, and
this workspace is dense with both.

Measured, on a clean target directory, as the review asked:

```
cargo +nightly llvm-cov clean --workspace
scripts/coverage-missing.sh -p stylex_css   →  exit 0, summary table reached
scripts/coverage-missing.sh                 →  exit 0, TOTAL 100.00%
```

Both runs reached the summary. The check does not refuse a legitimate report,
so it needs no change. This is the whole risk, and it is closed.

## A finding that would have broken the gate it was measured against

> **Superseded.** The reading below holds only while the reader stays inside
> `member_expression`. Moved out into a function a test can call, its refusals
> are reachable and covered, and the workspace run stays at 100%. Done in
> [100](./100-refuse-a-namespace-the-reader-cannot-name.md).

**N3 -- refusal diagnostics dropped for two shapes in `member_expression.rs`.**
The review asked for the two `None` arms of the namespace filter to be replaced
by the `stylex_panic!` they used to raise, and noted in the same breath that
this "adds two regions the gate cannot cover".

Not applied, and the note is the reason. The branch exists for the coverage
gate, and the review's own bar for any remediation is the workspace llvm-cov run
with `--fail-uncovered-regions 0`. The object those arms read comes from
`evaluate_with_functions` inside the same function, so no source and no unit
test can hand the reader a spread or a non-ident key -- which is exactly the
invariant [86](./86-decisions-the-remediation-recorded.md) established. A
refusal there could never be reached, never be covered, and would fail the gate
on every run.

The review's own preferred answer is option 2, making the shape
unrepresentable. That means the evaluator handing this reader a key type that is
already an ident, which is a change across the evaluator's whole result type for
one reader's benefit. Left for a ticket that wants it for its own sake.

What stands today: the invariant is asserted where the object is **written**, by
`every_written_object_carries_key_value_properties_only` in the evaluator, and
the reader's comment names `create_ident_key_value_prop` as what makes it hold.

## A finding that would have broken parity

**N5 -- `kebab_case` allocates twice for a non-ASCII name.** The review asked for
the Unicode lowering to move into the loop -- `kebab.extend(character.to_lowercase())`
-- so the second pass over the finished name disappears.

Not applied. The two spellings do not answer the same thing. `str::to_lowercase`
applies the Unicode final-sigma rule and `char::to_lowercase` cannot, because
only a reader of the whole name knows where a word ends:

| input  | runtime `toLowerCase` | `str::to_lowercase` | per character |
| ------ | --------------------- | ------------------- | ------------- |
| `ΑΣ`   | `ας`                  | `ας`                | `ασ`          |
| `aΣB`  | `aς-b`                | `aς-b`              | `aσ-b`        |

The runtime lowercases the whole name at once
(`str.replace(/([A-Z])/g, '-$1').toLowerCase()`), so the second pass is what
keeps this compiler agreeing with it. The suite already held the `aΣB` case; it
now also holds `ΑΣ`, `aΣ` and `ΣΑ` -- the last being the same letter inside a
word, where the rule answers the other way -- and the site carries a comment
saying why the pass stays. The cost is paid only by a name that is not ASCII,
which no property name a style object holds ever is.

## Two review claims that were checked and did not hold

The change set was reviewed twice before it landed, once for correctness and
once for performance. Both found the `to_posix_path` root defect recorded in
[95](./95-write-the-debug-file-name-with-one-separator.md). Two other claims
were measured and did not hold:

**"The dedupe moved when the variable group is read."** The performance review
read the surviving `validate_theme_variables` call as happening before the
overrides argument is evaluated, where the call it replaced happened after.
It does not. `git show HEAD:...transform_stylex_create_theme_call.rs` shows the
first call already sat after `evaluate(second_arg, &mut self.state, ...)` and
before the overrides were taken out of the result -- its own comment said so:
"Both arguments are already read at this point." The call did not move; only
its answer is kept. Nothing between the two positions writes to the state on
the path that reaches the producer.

**"The deleted directory was the only case exercising `props(defaultMarker())`."**
The correctness review read the deletion in
[87](./87-delete-the-marker-test-directory-that-never-ran.md) as losing that
shape. The live `transform_stylex_when_test/default_marker_transform.rs` holds
both spellings -- `props(defaultMarker())` and
`stylex.props(stylex.defaultMarker())` -- and `transform_stylex_when_test` is
registered in `tests/transform.rs`, so those two cases run. The deleted ones
never did. Registering them would have added a second snapshot of a shape
already covered.

## Two more review claims that were checked and did not hold

A third review, over the eleven commits this pass landed, reported no critical
defect. Two of its findings were measured and did not hold:

**"`is_css_var` was named but not shared."** The review read the new predicate
as one rule with two copies left open-coded, at
`evaluate_stylex_create_arg.rs:533` and `flat_map_expanded_shorthands.rs:103`.
Those two ask about a *key*, not a value, and the second already carries a
comment saying it is deliberately the wider of two key questions the compiler
asks, both of which the reference asks. The new predicate asks the value
question the reference asks in `convert-to-className.js:88`. Three questions
that read alike and are not the same question, so one helper would join what
the reference keeps apart. A note at `is_css_var` now says which is which, and
that it is not the `IS_CSS_VAR` expression either.

**"Upstream spreads a composed chain into characters."** The review read
upstream's `variableFallbacks` return as yielding a bare string from its
ternary, which a spread would take apart letter by letter, and called the Rust
a divergence. Upstream wraps it: `: [composeVars(...varValues)]`
(`convert-to-className.js:106`) is an array literal, so the spread yields one
element. The Rust answers one value, and so does the reference.

## What the whole pass cost the gate

Nothing. After every change above, the workspace run reports:

```
TOTAL   100.00%   100.00%   100.00%
✓ No uncovered regions — every measured source region is exercised.
```

## Comments

The first review recorded two false findings out of five reported as Critical
([86](./86-decisions-the-remediation-recorded.md)). This one recorded two
findings that were true but whose proposed remedies were not: one against the
gate the branch exists for, one against the runtime the compiler is measured
by. Both are cases where a remedy is worth measuring before it is applied.
