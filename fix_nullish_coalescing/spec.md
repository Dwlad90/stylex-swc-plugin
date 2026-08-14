# Logical expression parity

GitHub issue [#1254](https://github.com/Dwlad90/stylex-swc-plugin/issues/1254).

Upstream StyleX has a distinct `LogicalExpression` AST node, evaluated over
JavaScript **values**, separately from and ahead of `BinaryExpression`. SWC has
no such node: `||`, `&&` and `??` arrive as `BinExpr` alongside `+` and `>>`.
The evaluator never grew the missing branch, so the three logical operators are
handled by whichever of the number path or the string path claims them first —
the number path coerces both sides through `ToNumber` and compares against
zero, and the string path refuses everything but `+`.

Worse, the missing branch is not merely absent: its body was transplanted onto
`BinaryOp::Add`, where upstream has a one-line `left + right`. The transplant
carried the two-sided confidence dance intact — including the shape that reads
one side's confidence off the wrong state — onto an operator that has no use
for it.

## Problem Statement

An author writes an ordinary statically-resolvable style value that guards a
token with a fallback:

```js
const radius = { s: '0.25rem' };
export const styles = stylex.create({
  a: { borderRadius: `0 0 ${radius.s ?? ''} ${radius.s ?? ''}` },
});
```

`@stylexjs/babel-plugin` folds this at compile time and emits
`border-radius: 0 0 .25rem .25rem`. This compiler fails the build with
`[StyleX] For string expressions, only addition is supported, got "??"`.

The reporter has this pattern in ten files. It is not exotic: `??` and `||`
guarding a design token against an absent value is the idiom the pattern
exists for, and there is no workaround short of rewriting every call site to
avoid an operator that the reference implementation accepts.

## Solution

The compiler folds `??`, `||` and `&&` in a style value exactly where the
reference implementation folds them, and produces the same CSS — including in
template literals, in direct style values, and where the winning operand is an
object or an array rather than a string.

Where the reference implementation declines to fold — because a side cannot be
resolved at compile time — this compiler declines identically, so the value
falls to the runtime as a
[dynamic style](../../crates/stylex-transform/CONTEXT.md) rather than failing
the build.

## User Stories

1. As a StyleX author, I want `` `${token ?? ''}` `` in a style value to
   compile, so that I do not have to rewrite working code to adopt this
   compiler.
2. As a StyleX author, I want `??` to fold to the same CSS class name the
   reference implementation produces, so that server-rendered and
   client-rendered markup agree when the two compilers are mixed.
3. As a StyleX author, I want `||` in a style value to fold, so that the
   operator I already use for fallbacks behaves like `??`.
4. As a StyleX author, I want `&&` in a style value to fold, so that a
   conditional style value resolves rather than failing the build.
5. As a StyleX author, I want `??` to fold in a direct style value
   (`borderRadius: radius.s ?? '1px'`), not only inside a template literal, so
   that the two spellings behave alike.
6. As a StyleX author, I want `??` to fold when the left side is `null`, so
   that an explicitly-absent token takes its fallback.
7. As a StyleX author, I want `??` to fold when the left side is `undefined` —
   a property that is simply missing from an object — so that an optional token
   takes its fallback.
8. As a StyleX author, I want a `??` whose left side is falsy but not nullish —
   `''`, `0`, `false` — to be refused by this compiler exactly where the
   reference implementation refuses it, so that the two builds of my source
   never disagree about which values fold.
9. As a StyleX author, I want `config ?? {}` and `list ?? []` to yield an
   object and an array, so that a guarded style object can still be spread or
   indexed by the surrounding fold.
10. As a StyleX author, I want a logical operator whose sides cannot be
    resolved at compile time to fall to the runtime as a dynamic style, so that
    guarding a function parameter keeps working exactly as it does today.
11. As a StyleX author using `??` on a dynamic style function parameter, I want
    today's emitted runtime code to be unchanged, so that this fix cannot
    regress the styles I already ship.
12. As a StyleX author, I want the diagnostic on a value that genuinely cannot
    fold to name the property it sits on, so that I can find it in a large
    style object.
13. As a StyleX author, I want `'1' + 2` in a style value to produce `"12"`,
    so that string concatenation follows JavaScript rather than silently
    producing the number `3`.
14. As a StyleX author, I want `'x' + true` and `'x' + null` to concatenate
    rather than fail the build, so that `+` accepts the same operands the
    reference implementation accepts.
15. As a maintainer of this port, I want the logical operators to live in one
    place rather than three, so that a parity fix lands once.
16. As a maintainer of this port, I want the evaluator to reach JavaScript
    coercion through a single crate, so that a second, weaker string conversion
    cannot drift from the first.
17. As a maintainer of this port, I want the file named for the binary
    expression to contain the binary expression's evaluation, so that the next
    reader finds it where the reference implementation puts it.
18. As a maintainer of this port, I want deopt reasons to read as the reference
    implementation's do, so that a divergence report can be compared line by
    line.
19. As a maintainer of this port, I want the reproduction matrix pinned as
    fixtures, so that a future refactor of the evaluator cannot quietly undo
    this.
20. As a maintainer of this port, I want the places where the reference
    implementation is itself wrong recorded rather than silently improved, so
    that "we disagree with upstream" is always a deliberate, documented state.

## Implementation Decisions

Settled through `/grill-me`; the operator behaviour is a 1:1 port, so where a
decision looks odd the reason is that upstream does it.

### The logical expression becomes its own evaluator node

A new node joins the evaluator's node set, mirroring the reference
implementation's `LogicalExpression` block. The binary-expression node
dispatches to it for `LogicalOr`, `LogicalAnd` and `NullishCoalescing` and
returns early, **before** the number-then-string fallback runs. This mirrors
upstream's dispatch order, where the logical node is tested ahead of the binary
one.

The node evaluates each side into its own
[confidence](../../crates/stylex-transform/CONTEXT.md) state, then selects an
operand. It returns the winning operand **verbatim** — upstream returns the
value, so an object, an array or a callback that wins stays what it is rather
than being flattened into a re-created string or number literal.

### The `??` guard is reproduced bug-for-bug

Upstream's nullish branch guards with `leftConfident && !!(left ?? rightConfident)`.
When the left side is falsy but not nullish — `0`, `false`, `''` — the `??`
yields that falsy value, `!!` turns it to `false`, and control falls through to
an unconditional deopt. So upstream **refuses to fold** `x ?? 5` when `x` is
`0`, even though both sides are confident. The intended guard was evidently
`left != null || rightConfident`.

This port reproduces the guard exactly, `!!` included, with a comment recording
that it is upstream's. Rationale: the stated goal is 1:1 logic parity, and
owning a divergence in the *permissive* direction is worse than inheriting the
restrictive one — a value this compiler folds and upstream refuses is a silent
CSS difference between two builds of the same source. Recorded as a deferred
follow-up below.

### `&&` with a falsy confident left side returns that value

Upstream returns `left` here, and then a later consumer crashes on it with an
unhandled `TypeError` rather than a diagnostic. A crash is not a behaviour
worth reproducing; the *evaluator* is unambiguous, so this port returns the
falsy operand faithfully and lets the existing downstream handling decide what
an empty value means — which recent work already settled as "a declaration whose
value spells nothing is dropped". Recorded as part of the same upstream report.

### Truthiness and nullishness join the coercion crate

[`stylex-js`](../../crates/stylex-js/CONTEXT.md) is where ECMAScript coercions
live, and it already answers `ToString`, `ToNumber` and `ToObject` over an
evaluated expression. It gains `ToBoolean` and a nullish predicate, following
the refusal contract its siblings already document: a value with no
compile-time form of that type is refused, and the caller deopts rather than
inventing one.

The evaluator's own value representation is bridged to it by a fourth sibling
alongside the three bridging helpers that already exist, not by a private
mapping in the new node. The variant partition is the one the object bridge
already draws — every variant the evaluator has of its own stands for an object
or a function, so all of them are truthy. The absent-value variant refuses, for
the same reason the object bridge records: it can mean "absent" or "unknown",
and the two would answer differently.

The crate dependency edge stays one-way — the coercion crate does not learn
about the evaluator's value representation.

### The duplicate implementations are deleted, not left as fallbacks

The number path's logical arms, and the ad-hoc `LogicalOr` early-outs in both
the number and string paths, have no counterpart upstream — the reference
implementation's `BinaryExpression` switch has no logical cases at all, because
those nodes never reach it. All of them go. After the new node lands, a logical
operator cannot arrive at either path, and the existing refusal on an
unrecognised operator is the correct shape for one that does.

### `+` is collapsed onto the shared coercion crate

The transplanted helper on `BinaryOp::Add` is deleted. Two measured divergences
go with it:

- Dispatch is "if either evaluated side is a string, concatenate", per
  JavaScript, rather than "if numeric coercion failed". This fixes `'1' + 2`
  yielding `3` where the language says `"12"`.
- The string side routes through the coercion crate's `ToString`, which already
  handles booleans, `null`, `undefined`, objects and arrays correctly. This
  fixes `'x' + true` and `'x' + null` failing the build.

This deliberately retires the evaluator's second, weaker string conversion for
this operator. Keeping two string conversions is the same duplication shape
that let the logical bug survive.

### The binary expression's evaluation moves to the file named for it

The number and string paths currently live in the AST-convertor module, though
their only non-test caller is the binary-expression node and upstream has them
inside its evaluator. They move to that node. This puts the whole
`BinaryExpression` switch in one place, keeps the bridging helpers' module-local
visibility honest rather than widening it, and stops the convertor module from
being a place where a second string coercion has to exist.

The move is landed as its own commit, ahead of any logic change, so the
behavioural diffs stay readable.

### Commit sequence

Five commits, in dependency order:

1. `refactor` — move the number and string paths into the binary-expression
   node, tests riding along. Pure move.
2. `feat` — `ToBoolean` and the nullish predicate in the coercion crate, plus
   the fourth bridging helper in the evaluator.
3. `fix` — the logical-expression node; delete the duplicate logical handling
   from both paths.
4. `fix` — collapse the transplanted `+` helper onto the shared coercion crate.
5. `test` — fixtures pinning the matrix.

## Testing Decisions

A good test here states one claim about **external behaviour**: StyleX source
in, CSS metadata out. It does not reach into the evaluator's value
representation or assert which internal path an operator took — those are
exactly the details this spec is rearranging, and a test coupled to them would
have to be rewritten by the very commit it is meant to guard.

**Primary seam — `stylex.create` snapshot fixtures.** One new fixture file
under the create-transform test suite, source-in / CSS-out, snapshots on disk.
Every case in the reproduction matrix below is reachable from here, which is how
the matrix was produced in the first place. Prior art, both parity work of this
exact shape: the non-ASCII hash parity fixtures, and the global builtin call
fixtures.

This is the only seam at which the new `??` / `||` / `&&` / `+` behaviour is
pinned. It is also how the rest of the evaluator is tested — no module under
the evaluator has inline tests today, deliberately.

**Second seam — the coercion crate's own tests.** `ToBoolean` and the nullish
predicate are new public API of a separately-published crate whose existing
coercions are all covered in its test module. They are tested there, at the
boundary the crate publishes.

**Migrated coverage, not new.** The existing unit tests on the number and
string paths cover operators that cannot be reached from real StyleX source —
comparison, bitwise, `in`. The move in commit 1 puts those functions behind
module-local visibility, so that coverage rides along into an inline test module
beside them. This is coverage preservation only; no new behaviour is pinned
there. The one existing logical test asserts the numeric coercion commit 3
deletes, and goes with it.

**The rebuild gate.** Any test that reaches the compiler through the Node
package exercises the built artifact, not the Rust sources. The package must be
rebuilt before the JS suite means anything.

### Reproduction matrix

Measured against `@stylexjs/babel-plugin@0.19.0` with `dev: false`,
`treeshakeCompensation: true`, `commonJS` module resolution. `radius = { s: '0.25rem' }`
unless stated.

| input | upstream | this compiler, before |
| --- | --- | --- |
| `` `0 0 ${radius.s ?? ''}` `` | `border-radius:0 0 .25rem` | refuses: only addition supported, got `??` |
| `` `0 0 ${radius.s \|\| ''}` `` | `border-radius:0 0 .25rem` | refuses: got `\|\|` |
| `` `${radius.s && 'red'}` `` | `color:red` | refuses: got `&&` |
| `borderRadius: radius.s ?? '1px'` | `border-radius:.25rem` | refuses: got `??` |
| `` `${radius.s ?? 'red'}` ``, `s: null` | `color:red` | refuses: unsupported expression |
| `` `${radius.s ?? 'red'}` ``, `s` missing | `color:red` | refuses: unsupported expression |
| `` `${radius.s ?? 'red'}` ``, `s: ''` | **refuses**: unknown error | refuses: got `??` |
| `flexGrow: radius.s ?? 5`, `s: 0` | **refuses**: unknown error | folds to `flex-grow:5` |
| `` `${radius.s && 'red'}` ``, `s: ''` | **crashes**: `TypeError` | refuses: got `&&` |
| `flexGrow: '1' + 2` | `flex-grow:12` | folds to `flex-grow:3` |
| `content: 'x' + true` | `content:"xtrue"` | refuses: unsupported expression |
| `content: 'x' + null` | `content:"xnull"` | refuses: unsupported expression |

Rows already matching, kept as regression coverage: `'a' + 'b'`, `1 + 'px'`,
`'solid' + ''`, `'' + 'solid'`, `1 + 2`, nested `+`, `+` inside a template
literal, `??` on a dynamic style function parameter.

Two rows need reading carefully, because "both fail" does not mean "already
agrees":

- **`s: ''` and `s: 0`** are the falsy-non-nullish quirk. Upstream refuses both;
  this compiler refuses the first for an unrelated reason (it refuses *every*
  `??`) and wrongly folds the second. After the fix it must refuse **both**, and
  refuse them by reaching upstream's guard — not by still lacking the operator.
  A fixture that only asserts "the build fails" would pass today and is
  therefore worthless here; the assertion has to be the diagnostic.
- **`'x' + true` / `'x' + null`** are where this port is wrong and is being
  fixed; **`s: 0`** is where upstream is wrong and this port is being made
  wrong to match.

## Out of Scope

- **Boolean, null and undefined stringification beyond `+`.** The coercion
  crate already answers these correctly; this spec routes the `+` operator to
  it. Every *other* site still reaching the evaluator's weaker string
  conversion is left alone. That is a wider migration with its own snapshot
  risk and belongs in its own effort.
- **The rest of the `BinaryExpression` switch.** Comparison, bitwise, `in` and
  `instanceof` are not audited here. The number path's `in` and `instanceof`
  arms in particular look nothing like upstream's, but nothing in issue #1254
  reaches them. One exception was forced and is recorded under
  [the closed risk below](#open-risk-the-snapshot-diff-from-commit-4): widening
  `+` to the shared `ToString` gave `null` and arrays a string, which would have
  turned the *refusal* for non-`Add` operators into a build failure. Testing the
  operator before converting either operand keeps `null - 1` and `[1, 2] * 2`
  deopting as they did, which is what holds this bullet true.
- **A committed differential runner against the reference implementation.** The
  matrix above was produced by a throwaway script. A permanent
  upstream-comparison harness is a genuinely useful thing for a port, but it is
  a tooling decision with its own scope — which corpus, which options matrix,
  how it fails CI — and must not ride in on a bug fix.
- **Changing what a dynamic style emits.** The runtime code emitted for a
  logical operator on a function parameter is unchanged; the existing fixtures
  for it are regression coverage, not a target.

## Further Notes

### Deferred follow-up: report the upstream defects

One issue against `facebook/stylex`, covering both defects in the same
`LogicalExpression` block, same root cause — value confused with truthiness:

1. The nullish guard's `!!(left ?? rightConfident)` deopts on a falsy
   non-nullish left, so `x ?? 5` fails to compile when `x` is `0`.
2. `&&` with a falsy confident left returns that value and crashes a downstream
   consumer with a bare `TypeError` rather than a diagnostic.

Text is drafted for review before anything is posted; filing is a deliberate act
and happens only on an explicit go-ahead.

### Open risk: the snapshot diff from commit 4

The coercion crate's `ToString` succeeds on strictly more inputs than the
conversion `+` uses today. A value that previously failed to resolve — and
therefore fell to the runtime as a dynamic style — may now fold statically.
That is the intended fix for the three measured rows, but it is not
*a priori* bounded to them.

The full suite diff after commit 4 must be read before that commit is called
correct. If it moves fixtures unrelated to `+`, that is a scope conversation,
not something to absorb into this branch.

**Closed by ticket 05 (`8ede5c368`): the risk did not materialise.** No existing
snapshot moved anywhere in the workspace. What the widened coercion did reach
was the *refusal* for non-`Add` operators, which is not a fold at all: `null`
and an array now have a string, so `null - 1` and `[1, 2] * 2` would have begun
failing builds they previously deopted. That is fixed at the source by testing
the operator before converting either operand, so those two go on deopting.

### Landed beyond the five commits

Three changes landed that the commit sequence above does not name. Each is
recorded here so the branch's shape stays legible rather than looking like
drift.

- **`void x` answers `undefined` rather than no value.** Reached through ticket
  08: `??` reads a confidently-absent operand as nullish, so `void 0 ?? 'red'`
  has to hand back a value or the caller turns the confident `None` into a
  deopt and fails a build it should have folded. `void` is a third spelling of
  `undefined` alongside `null` and a missing property, which stories 6 and 7
  already ask for; it rewrote one pre-existing unit test and moved no fixture.
- **A key an object does not carry reads as `undefined`.** This is story 7's
  mechanism rather than an addition to it. It has one consequence the stories do
  not state: a **bare** `obj.missing`, with no operator waiting for it, now
  reaches the style-value check and fails there instead of deopting to the
  runtime. The reference implementation fails the same input, wording it
  differently — which of the two refusals an `undefined` value earns is a
  pre-existing difference in the style-value check, not something this branch
  decides. Pinned by `a_bare_missing_property_is_rejected_as_a_style_value`.
- **The non-`Add` refusal became an `Err` rather than a panic.** Forced by
  ticket 05 and covered by the Out of Scope note above.

### Review outcomes

The two-axis review of the whole branch raised two points about behaviour that
were measured rather than taken:

- **Story 12 was already met, and is now pinned.** Diagnostics carry the
  property path already — a refused `??` reads `a > flexGrow > unknown error`,
  not a bare `unknown error`. The refusal fixtures asserted only the reason,
  and `should_panic(expected = …)` is a substring match, so nothing held the
  path in place. They now assert the path with it.
- **Array indexing is not inconsistent with the missing-key fold, because it
  does not work at all.** An array binding evaluates to the evaluator's own
  `Vec` variant, which the member-expression node has no arm for, so indexing
  one refuses at the catch-all whether the index is in range or not — on
  `develop` as much as here. There is no working array path for the object
  arm's `undefined` to disagree with. Teaching `Vec` to be indexed is its own
  scope; the `ArrayLit` arm carries a note saying so.

### Provenance

The transplant is what makes the root cause legible rather than speculative.
The helper bolted onto `BinaryOp::Add` carries the reference implementation's
`stateForLeft` / `stateForRight` / `deoptReason ?? 'unknown error'` structure
verbatim — a structure `+` has no use for, since upstream's `+` is a single
line. It also reads one side's confidence off the outer state rather than that
side's own, which is a copy-paste slip that only makes sense in code that was
moved. The logical branch was not forgotten; it was pasted onto the wrong
operator.
