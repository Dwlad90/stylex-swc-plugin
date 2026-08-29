# 12 — The allocation ceilings become configuration

**What to build:** A repeat count no longer has to be written as a literal,
and a project that really produces large folds can say so.

```js
const n = 5;
content: 'x'.repeat(n)     // refused today, though 5 is trivially safe
content: 'x'.repeat(9e9)   // must refuse, naming the limit
```

Two ceilings exist because the engine bounds loop iterations, recursion and
stack — but not allocation. Growth inside a native builtin is not a counted
loop, so a typo folds, agrees with the language, and reaches gigabytes of
resident memory; one measured case peaked at 5.37 GB. A compiler that dies
there is worse than one that declines. A bounded string can also become one
element per code unit, which costs far more as a tree than it did as text, so
the second ceiling bounds the result.

Both stay. What changes is that they read a resolved count rather than
demanding a written one, and that they become project options with a
machine-level override — the same precedent already set for evaluation depth,
and for the same reason: the value that keeps a build reporting rather than
dying is a property of what a project generates, and the compiler cannot know
it.

The companion rule refusing an amplifying call whose receiver is itself a call
exists so per-link bounds cannot multiply across a chain. A receiver can now
be a resolved value, so that rule needs rechecking rather than carrying over.

**Blocked by:** 05.

**Status:** resolved

- [x] An amplifying call whose count resolves to a safe value folds, whether
      the count was written or named
- [x] An amplifying call past the ceiling refuses with a message naming the
      limit and the count asked for
- [x] Both ceilings are project options with a machine-level override,
      precedence matching the existing evaluation-depth setting
- [x] Both defaults are re-derived and each is stated in terms of what it
      costs, in resident memory and in tree size
- [x] The rule preventing per-link bounds from multiplying across a chain is
      rechecked against a resolved receiver and still holds
- [x] The corpus entries that pinned the two amplification divergences are
      re-measured

## What was built

The bound became **arithmetic on values** rather than a shape. The guard works
out how long a string the call would build and refuses when that is past the
ceiling: a count is read through the evaluator and then the language's own
`ToNumber`, so `'x'.repeat(n)`, `'x'.repeat(2 * 2)` and `'x'.repeat('3')` reach
the same bound `'x'.repeat(4)` does — and each of those was a divergence the
reference compiler folded.

**The chain rule did not survive as written.** "The receiver must not itself be
a call" was standing in for a product: `repeat` multiplies its receiver, so what
has to be bounded is `receiver length x count`. Rechecked against a resolved
receiver, the old rule had a hole it never covered — `const base =
'x'.repeat(999999); base.repeat(999999)` is a name, not a call, and reaches a
terabyte. The product closes it, keeps the chain case refused (a call receiver
has no length the guard can read without evaluating it, which is the thing being
prevented), and tightens `'xx'.repeat(600000)`, which used to be admitted and
refused only after building it. `padStart` and `padEnd` build to their count
whatever the receiver holds, so the count alone bounds them.

**Three ceilings, one rule.** `Ceiling` in `stylex-structures` carries the
precedence, the parse and the clamp; `maxEvaluationDepth`, `maxFoldedCharacters`
and `maxFoldedEntries` are each a four-line declaration of what they bound. Both
new ones reach the four call sites 06 left — text and entries, inward and
outward — through one `Ceilings` value carried beside the depth budget.

**The defaults, re-measured** (peak resident set, one declaration, debug build):

| units | peak    | per unit |
| ----- | ------- | -------- |
| 1e6   | 39.8 MB | 19 bytes |
| 4e6   | 99.0 MB | 19 bytes |
| 1e7   | 213 MB  | 19 bytes |
| 4e7   | 783 MB  | 19 bytes |

against a ~20 MB baseline: a code unit costs about 19 bytes while it is being
built, because the engine grows and copies rather than allocating the result
once. Entries were measured the same way, on `'x'.repeat(n).split('')`:

| entries | peak    | over baseline | per entry |
| ------- | ------- | ------------- | --------- |
| 10000   | 22.6 MB | 2.6 MB        | 260 bytes |
| 100000  | 38.5 MB | 18.5 MB       | 185 bytes |
| 500000  | 115 MB  | 95 MB         | 189 bytes |

The slope between the last two is 190 bytes per entry — a 24-byte slot and an
80-byte boxed expression in the tree, plus the engine's own array and the
evaluator's list beside it. So the shipped defaults are ~20 MB and ~2 MB at the
peak of one fold, and the limits (4e7 and 1e6) are the point past which a
ceiling stops turning a crash into a message.

**What it costs to read a count.** Nothing a fold used to pay.
`admit_amplification` returns before anything else for a method that is not one
of the three, which is every fold in the benchmark corpus. For the three, a
count and a receiver written out are answered by matching syntax — nothing is
evaluated. The only
new resolution is for an amplifying call whose count or receiver is *named*, and
that is a call which used to refuse outright: on the fold path the resolution is
memoised and `admit_value` would make it a moment later anyway; on the refusal
path it is one read on a build that is ending. No fold that folded before is
slower, which is why there is no benchmark leg for it — there is nothing to
compare against.

**Left out, deliberately.** Asking whether an input can still fold and allocate
past the ceiling turned up one that can: `Array(100000000).fill(0)` takes 34
seconds and 421 MB before the outward entry ceiling refuses it. A length set by
the *receiver* rather than by an argument is the same arithmetic read from the
other end, and widening the method list is a claim about which calls amplify
rather than about the bound they meet — so it is issue 18, not a checkbox here.

**Corpus.** `modules-06-unwritten-amplification-count` now reads `identical` and
lost its `expected` verdict. The two size divergences stay, with the ceiling
named so the note says how to move it. `modules-12-amplification-across-a-chain`
is new: the one length a readable count still cannot bound.

## Comments

### From 06

There is now a third place the entry ceiling is counted, and it is the same
number rather than a new one. `MAX_FOLDED_ENTRIES` bounded a folded array and a
folded object on the way *out*; 06 counts a resolved value against it on the way
*in* as well, across every name one fold carries, because an array a name holds
is copied into the engine element by element. `bound_value_has_too_many_entries`
is its refusal, and it names the binding rather than the method for the reason
`bound_value_too_large` does.

So this ticket has two inward counts to make configuration, not one: text, which
06 left as it found it, and entries. Both read the same constants the outward
direction reads, so one option moves all four call sites.
