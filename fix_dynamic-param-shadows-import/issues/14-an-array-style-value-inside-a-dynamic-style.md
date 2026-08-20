# 14 — An array style value inside a dynamic style

Status: `resolved`
Blocked by: None

**What was found:** Every array written as a style value *inside a dynamic
style's body* aborts the build with `Style value must evaluate to a static
expression.` The reference implementation compiles them.

```js
export const styles = stylex.create({ dyn: (h) => ({ height: ['1px', '2px'] }) });
```

The same array in a static namespace compiles. The difference is the consumer:
a dynamic style's body is walked by `evaluate_partial_object_recursively`
(`shared/utils/core/evaluate_stylex_create_arg.rs`), and its two style-value
positions read the evaluated value through `as_expr()`. An array evaluates to
`EvaluateResultValue::Vec`, which has no expression form, so the value aborts
there rather than being folded to an `Expr::Array` the way
`object_expression.rs` folds it for a static namespace via
`evaluate_result_vec_to_array_expr`.

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration — `haste` resolution, one source string. Every row below is
`ERR Style value must evaluate to a static expression.` on this compiler:

| input, inside `create({ dyn: (h) => ({ … }) })` | Babel 0.19.0 |
| --- | --- |
| `height: ['1px', '2px']` | `.x…{height:1px;height:2px}` |
| `margin: [1, 2]` | `.x…{margin:1px;margin:2px}` |
| `height: F` where `const F = ['1px','2px']` | `.x…{height:1px;height:2px}` |
| `height: { default: ['1px','2px'] }` | `.x…{height:1px;height:2px}` |
| `height: []` | no rule, no error |
| `height: [null, '2px']` | `.x…{height:2px}` |
| `height: [, '2px']` | the hole makes it dynamic: `height:var(--x-height)` |
| `height: [['1px'], '2px']` | `A style array value can only contain strings or numbers.` |
| `height: [{a:1}, '2px']` | `A style array value can only contain strings or numbers.` |
| `height: [undefined, '2px']` | `A style array value can only contain strings or numbers.` |
| `height: [true, '2px']` | `A style array value can only contain strings or numbers.` |

Two shapes already agree, and are the reason this went unnoticed:
`stylex.firstThatWorks('1px','2px')` and `[...xs, '2px']` — the first answers an
expression rather than a `Vec`, the second is refused before the array is folded.

Not fixed under ticket 08, which is about a folded function map at the same two
positions. The overlap is one input — `height: [stylex, '1px']` with the
parameter shadowing the namespace import, where the reference implementation
reads `A style array value can only contain strings or numbers.` and this
compiler cannot reach that message until the `Vec` case is folded. Recorded in
the corpus as divergent, with this ticket named as what closes it.

The shapes above are the work: an empty array, a `null` element, a hole, a
nested array, and an `undefined` element each have their own answer upstream,
and none of them is the array's element list read straight through.

- [x] Every row above agrees with the reference implementation
- [x] The two positions no longer abort for an array
- [x] Corpus entries for each shape, with the verdict each is known to read

## Answer

Two changes, at two seams, because the table had two causes in it.

### The fold: one arm, at the seam ticket 08 built

`materialize_style_value` in `core/evaluate_stylex_create_arg.rs` already stood
between an evaluated value and a style value, folding a function map into the
object it stands for. An evaluated array wanted exactly the same treatment, so
it is one arm: `EvaluateResultValue::Vec` goes through
`evaluate_result_vec_to_array_expr` — the *same* function
`nodes/object_expression.rs` uses for a static namespace, made `pub(crate)`
rather than copied — and refuses with `ILLEGAL_PROP_ARRAY_VALUE` when an element
has no array-element form.

What an array may hold is then nobody's decision at the value position. It is
namespace validation's, from the folded `Expr::Array`, in both compilers: a
nested array, an object, `undefined`, a boolean, an arrow, a shadowed namespace
parameter and a theme object all read `A style array value can only contain
strings or numbers.` — the sentence upstream gives, which this position could
not reach while the array aborted ahead of validation.

That covers ten of the eleven rows. The eleventh was not about the position.

### The hole: the array itself was answering one element short

`height: [, '2px']` was not aborting for want of an expression form. The
evaluator skipped the hole, so `[, '2px']` folded to a one-element array, and
folding it would have emitted `height: 2px` — a value the source does not
describe, which is worse than the abort it replaced. The row says upstream makes
it dynamic, and the reason is the interesting part: upstream evaluates element
*paths*, and a hole's path carries no node, so it falls to the guard that reports
`PATH_WITHOUT_NODE`. We already had that constant, with upstream's exact words,
used by the evaluator's cache. So `nodes/array_expression.rs` now refuses a hole
with it, and the two compilers say the same sentence for the same input.

The refusal travels with the value, which is what makes it the right seam:

- inside a dynamic style's body a refusal is not an error — the value falls to
  the runtime as `var(--x-height)`, which is what upstream emits there, so the
  row reads `identical`
- in a static namespace both compilers refuse, where this one used to emit
  `height: 2px`
- read through a binding, likewise — `const F = [, '2px']` refuses wherever `F`
  is read, where the count used to come out short

Three shapes outside the ticket moved with it, all in the same direction:
`{ ...[, 1] }` now refuses for the hole rather than for a shifted key,
`A.length` through a binding to a holey array refuses instead of answering one,
and `[, 1][0]` reports the hole rather than the index.

### `[, 1].length` still answers two, and deliberately

A hole is a slot the language counts, so `[, 1].length` is two — upstream
crashes on it, and answering two is the divergence in this compiler's favour that
`member_length_tests.rs` was written to pin. Refusing the array would have taken
it away, so the count is read off the source *before* the receiver is evaluated:
`holey_receiver_length` in `nodes/member_expression.rs`, gated on a receiver
literal that actually carries a hole, so no shape that folds today takes a new
path. A spread still refuses through the same `written_slot_count`, because one
written element standing for however many a spread holds is not a count either
reading can give.

The corpus entry for it keeps `acceptance-divergent`, with the note rewritten:
the array refuses, the count does not.

### Measured

Every row of the table, plus twenty-odd shapes beyond it, against
`@stylexjs/babel-plugin` 0.19.0 under the harness's own options. Every ticket row
now agrees. Twenty-four corpus entries carry the verdicts, each with `expected`
recorded so a regression reports as a changed verdict rather than as silence, and
the whole corpus reports `changed 0` over 976 subjects.

Three divergences the measurement turned up are *not* array questions and are
filed rather than fixed, each with its corpus row:

- [25](./25-an-absent-value-in-a-dynamic-style-loses-its-marker.md) — an absent
  value in a dynamic style's body loses the marker that unsets a merged
  declaration. `height: null` shows it without an array anywhere.
- [26](./26-nan-and-infinity-as-a-style-array-element.md) — `NaN` and `Infinity`
  as array elements. The static namespace refuses them identically, so it is the
  array check and not the position; upstream's two answers differ from each
  other and one of them is `height: Infinitypx`.
- [27](./27-an-index-read-off-an-array-refuses.md) — an index read off an
  evaluated array, already named at its site as its own scope, which the fold
  made visible in a second position.

One divergence is an existing ticket's: an array eight conditions deep hashes a
different class name because the two compilers order nested pseudo-classes
differently ([19](./19-three-nested-pseudo-classes-hash-differently.md)). The
declaration text agrees, and the test asserts that half.

Tests: `tests/array_hole_tests.rs` for the hole and everything beside it that
must not move — a trailing comma, the ordering of a spread against a hole, a
hundred arrays deep, ten thousand holes, and each `length` spelling — and
`tests/transform_stylex_create_test/array_style_values.rs` for the two positions,
the refusals, non-ASCII and escaped elements, a custom property, a vendor-prefixed
property, an unclosed CSS function, a thousand elements, and eight condition
levels. Workspace green, `cargo clippy --workspace --all-features --all-targets`
clean, node suite 64 of 64.

## Comments

### Review, both axes

Standards and spec review over `01c16e3a8...HEAD`. Three findings acted on, two
recorded as answered.

**The corpus row for ticket 08's overlap input had gone stale, and no harness
could say so.** `modules-1266-shadowed-namespace-inside-an-array` still read
"this compiler still reads `Style value must evaluate to a static expression.`"
and still named this ticket as what would close it. Both halves were false after
the fold, and its verdict — `both-reject` — is exactly the one that compares
acceptance rather than wording, so nothing failed. Corrected, pointing at the
test that pins the text.

**The AST-level `length` test was a fourth private copy of it**, in the file
whose `classify_lookup` exists because three copies had drifted into three
diagnostics for one mistake. Routed through `convert_member_prop_to_string`,
which also settles a spelling the copy refused: a template folding to `length` is
the same property in the language.

**`Option<Option<_>>`** — outer "this arm applies", inner the evaluator's answer,
with nothing saying so. Now `holey_receiver_elems`, answering what it found, with
the count and the spread refusal spelled where the neighbouring arms spell
theirs. Two test gaps came out of the split: a non-literal receiver counted by
its evaluated elements, and a key that only looks like `length`.

**The coverage gate does not reach these files.** `scripts/packages/test/coverage.sh`
exits early for `stylex-transform` and the workspace script excludes it, so the
three uncovered lines review flagged are not the standards breach they would be
elsewhere. Two are gone with the refactor above; the third is the holey-spread
guard in `object_expression.rs`, kept deliberately — `written_slot_count`'s
spread arm sets the precedent for a bounds guard behind a refusal, and both now
say at the site that they are guards rather than live paths.

**Scope, recorded rather than trimmed.** The hole refusal is row seven of the
ticket's own table, and `holey_receiver_length` exists to stop that row taking
`[, 1].length` down with it — a divergence in this compiler's favour that
`member_length_tests.rs` was written to pin. Three shapes outside the table moved
with it, each toward upstream and each now under test: `{ ...[, 1] }`,
`A.length` through a holey binding, and `[, 1][0]`.
