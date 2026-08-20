# 20 — A deep expression aborts the process where Babel throws

Status: `resolved`
Blocked by: None

**What was measured:** The evaluator walks a nested expression recursively, so
its limit is stack depth rather than a checked bound. Around a shadowing dynamic
parameter, `(((zIndex + 1) + 1) …)`:

| depth | Babel 0.19.0 | rs-compiler |
| --- | --- | --- |
| 256 | accepts, one custom property | accepts, one custom property |
| 384, 512, 768 | accepts | **512 aborts: `has overflowed its stack` / `fatal runtime error: stack overflow, aborting`** |
| 1024 | `RangeError: Maximum call stack size exceeded` | — |

Two differences, and the second is the one that matters:

1. Our ceiling is lower — somewhere in (256, 512] against Babel's in (768, 1024].
2. Our failure mode is a **process abort**, not a catchable error. Babel raises a
   `RangeError` a caller can catch and report against a file and a line. A
   `SIGABRT` gives a bundler nothing: no diagnostic, no file, and no chance to
   continue with the rest of the build. Every other refusal in this compiler is
   a panic carrying a StyleX message; this one is not a refusal at all.

Nesting this deep is not something a person writes, but it is something codegen
produces, and the failure is indistinguishable from a compiler crash.

**Why it is not part of ticket 10.** Found while measuring the boundary that
ticket's edge coverage asked for, and recorded there at
`two_hundred_and_fifty_six_levels_of_arithmetic_around_a_shadowing_param` — which
asserts the agreeing depth and stops, because a test that crosses the boundary
aborts the test binary rather than failing. Fixing it means giving the evaluator
a depth budget that refuses before the stack runs out, which is a change to the
evaluator's contract and wants its own decision.

**Not specific to the shadowing.** Measured through a shadowed parameter because
that is what was under test, but nothing in the mechanism depends on it — an
equally deep expression over a module-level `const` should abort the same way.
Worth confirming as the first step, since it decides whether this belongs to
this feature's tracker or the evaluator's.

- [x] Confirm the abort reproduces without any shadowing — it does, at the same
      depth, over a module-level `const` in a plain static value
- [x] Decide whether a depth budget refuses with a message, or the recursion
      becomes an explicit stack — a budget, *plus* a grown stack, because
      neither works alone;
      `crates/stylex-transform/docs/adr/0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md`
- [x] Pin the boundary as a test that can survive being wrong —
      `crates/stylex-transform/tests/transform_stylex_create_test/evaluation_depth_budget.rs`,
      62 cases

## Answer

**A counted ceiling on the fold, and a stack the fold grows for itself.** The
first half was the ticket's proposal; the second turned out to be what makes a
single ceiling possible at all, and the decision including its rejected
alternatives is ADR 0004.

`maxEvaluationDepth` bounds the levels `evaluate_cached` will descend, and it is
configuration rather than a constant: the option, then the
`STYLEX_MAX_EVALUATION_DEPTH` environment variable, then a default of **32**,
precedence in that order so a stray environment value cannot change what a
configured project compiles to. Zero and anything non-numeric read as unset,
because a ceiling of zero refuses the folds the compiler runs to do its own work.

The counter lives on the state manager, not on the evaluation state, because the
evaluation's confidence forks — a logical operand and a computed key each get
their own — while the stack it accounts for does not. Crossing it is a refusal,
not a panic: per ADR 0002 a panic in the evaluator claims a broken invariant no
author input can reach, and a deep expression is author input. It reaches an
author as a diagnostic anyway wherever a static value is required:

```
[StyleX] base > zIndex > Expression is too deeply nested to evaluate at compile time.
At most 32 levels of nested evaluation are supported.
```

**Why the default is 32 and not the highest safe number.** The value that keeps a
build reporting rather than aborting is a property of what a project generates,
which the compiler cannot know — so the shipped number covers styles somebody
wrote and anything past it says so out loud, with a documented knob for the
projects that generate more. Measured before choosing it: at 32 the fold takes
29 levels of arithmetic and a 28-link member chain, and across the whole
workspace exactly three tests change — `a_hole_a_hundred_arrays_deep_refuses`,
`a_deeply_nested_length_read_neither_overflows_nor_changes_answer` and
`a_deeply_nested_fold_still_folds`, all deliberate ~100-level probes, now run
under an explicitly raised ceiling. Nothing resembling authored CSS moved: a
`theme.colors.primary` / `theme.space.md` / `4 * 2 + 'px'` module folds to
byte-identical output against upstream.

**Why the budget alone was not enough.** Measured with the counter in and the
stack left as the thread's: the arms do not cost the same per level. A debug
build ran out between 32 and 64 levels of nested `Math.max`, where plain
arithmetic reached 384 — so a ceiling low enough to be safe for the worst arm
would have refused inputs upstream folds by an order of magnitude. Running the
descent inside `stacker::maybe_grow` (already in the graph via
`swc_ecma_parser`, which grows the stack for the same reason) removes the
per-arm variance, and the ceiling becomes a policy rather than a stack
measurement. `stacker` catches a panic on the grown stack and resumes the unwind
on the original one, so the StyleX diagnostic transport is unaffected.

**Confirmed without any shadowing.** 512 levels of `(MY_CONST + 1)` in a plain
static value aborted before the change and refuses after it. Nothing in the
mechanism depended on the shadowing, so this belonged to the evaluator; it is
recorded here because that is where it was found.

**Measured against `@stylexjs/babel-plugin` 0.19.0**, same options, on eleven
nesting shapes. Every depth both compilers accept produces identical class names
and rule text. The last accepted depth per shape, and what happens one level
past it:

Measured under a raised ceiling of 320, because the subject is how deep a fold
*can* go rather than where the shipped default sits:

Each shape adds `+ 1` or a character per level, so the folded value encodes the
depth and the class name is a hash over something only the full descent produces.
That matters: the first version of five of these used shapes that fold to the
same value at any height — a tower of `(true ? x : 0)` is `x` however tall — and
would have passed had the fold stopped after one level.

| shape | last folded | upstream at that depth |
| --- | --- | --- |
| `(x + 1)` arithmetic | 317 | folds, same hash |
| `(x + 'b')` concatenation | 317 | folds, same hash |
| `` `${x}b` `` template | 317 | folds, same hash |
| shorthand expansion into four longhands | 317 | folds, same four hashes |
| `o.a.a…` member chain | 316 | folds, same hash |
| `:hover` / `@media` value | 316 | folds, same hash |
| style array element | 316 | folds, same hash |
| `{ ...{ … } }` spread chain | 315 | folds, same hash |
| `Array(n).length` | 315 | folds, same hash |
| `(true ? x + 1 : 0)` conditional | 158 | folds, same hash |
| `(x + 1 || 0)` logical | 158 | folds, same hash |
| `Math.max(x + 1, 0)` call | 158 | folds, same hash |
| `-(- x - 1)` unary | 105 | folds, same hash |
| `(x)` parentheses | unbounded | folds, same hash |

The ceiling is in fold levels, and a source level is not always one: a member
read descends twice, a spread descends twice, an array element costs the array as
well, a conditional that also adds costs both, the unary shape spends three nodes
a level, and a parenthesis is unwrapped before the fold is asked. That is why the
numbers differ by shape, why each is pinned rather than derived, and why the
message says *nested evaluation* rather than naming source levels.

At the shipped default of 32 the same measurement gives 29 levels of arithmetic
and a 28-link member chain, both pinned as well.

**Where the two disagree.** Between our ceiling and upstream's, upstream folds
and we refuse with a message. Upstream's own ceiling is higher than this ticket
recorded: it folds 1024 levels of arithmetic and throws
`RangeError: Maximum call stack size exceeded` at 4096. Difference 1 in the
report stands, and is the accepted cost of difference 2 being fixed.

**One shape reverses.** The reported input — a dynamic parameter shadowing an
imported binding, 576 levels deep — now *folds* here, to the single custom
property a shallow one folds to, and upstream throws a `RangeError` on it from
576 up. Ours is the higher ceiling in the shape the ticket was filed against.

**What is left, and it is not the fold's.** A deep enough expression still
aborts, in the stages that recurse over it with no ceiling of their own —
parsing, visiting, printing, and the deep clone a refusal records. Measured in a
debug build on a 2 MiB thread: 768 levels with no `stylex` call involved is fine
and 1024 aborts; inside a `create()` call 576 refuses cleanly and 608 aborts. So
the fold's ceiling sits well under theirs by design, and the residue is filed as
28.

**One class of edge case has nothing to measure.** Vendor prefixing was asked
for and is not implemented in this compiler — all three cases in
`transform_polyfills_test/css_property_polyfills.rs` are `#[ignore]`d and
`css_value_polyfills.rs` has none — so no prefixed output exists for a depth to
interact with. Covered instead by the nearest path that does exist: a shorthand
under `legacy-expand-shorthands`, where one folded value at the boundary becomes
four longhand declarations, all four hashes upstream's.

**No measurable cost.** The complex-theme performance fixture over six runs:
10.35-11.22 ms with the change, 10.76-12.15 ms without. A stack segment is
allocated only for an expression that reaches the red zone, which nothing under
the default ceiling comes close to.

**One thing found on the way, filed as 29.** Asked whether the recursion should
become an explicit stack for speed; measured, and it should not. Fold cost is
about quadratic in depth with the output held constant — 1.7 ms at 60 levels,
54 ms at 480 — and what grows is not the call frames but the structural hash that
keys the memo, recomputed over the whole remaining subtree at every level. At the
default ceiling it is a small constant, which is why it is filed rather than
fixed.
