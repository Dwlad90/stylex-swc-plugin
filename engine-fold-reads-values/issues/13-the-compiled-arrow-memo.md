# 13 — The compiled-arrow memo

**What to build:** A file with many style objects sharing one expression shape
parses it once.

The transport prints an expression and hands the text to the engine, so
printing and parsing dominate a warm fold. A file that writes the same shape a
thousand times pays for a thousand parses of identical text. Caching the
compiled function beside the engine on its thread-local, keyed by the printed
text, is the single largest performance lever in this effort.

It is a separate ticket rather than part of the transport for two reasons: its
value is only measurable once folds are common, and a change made for speed
should be priced on its own against a baseline rather than hidden inside a
change made for correctness.

The engine must still be built on first use and never before, and the memo
must not outlive the engine it belongs to.

**Blocked by:** 06.

**Status:** resolved

- [x] A distinct printed expression is compiled once per engine and reused for
      every later fold of the same text
- [x] The memo lives with the engine, is created no earlier than it is, and
      nothing leaks between folds or between files
- [x] The improvement is measured against the baseline recorded in 01 and the
      numbers are written into this ticket
- [x] The paired comparison passes on every fixture
- [x] Every behavioural test from the earlier tickets still passes, with no
      test modified — a memo that changes an answer is a defect

## Answer

What is memoised is the **compiled script**, not the compiled arrow the ticket's
title names. The change of unit is the whole of what the work found, and it is
what let the memo be a pure win rather than a trade.

### Why not the arrow

The first implementation did what the title says: print every fold as an arrow —
including one over no parameters, so that a bare expression had a function to
cache — and keep the compiled function object beside the engine. It worked, and
the repeated legs of the criterion benchmark moved 55-62%. It also cost 67% on
`fold-distinct`, the leg that folds a shape nobody has folded before, because
every expression that resolved no name now paid for a function object and a VM
frame it had never needed. That is most of the perf fixture: fifteen call sites,
fifteen distinct shapes.

Boa's `Context::eval` is exactly `Script::parse(src, None, self)?.evaluate(self)`.
So the parse can be lifted out of it and kept without touching either arm of
`apply`: the bare form still evaluates to its answer and the arrow form still
evaluates to a function the fold then calls, and what the memo changes is when a
source is parsed and nothing else. `print_fold` is unchanged, `apply` keeps the
branch it had, and the first-sight cost drops from +67% to +21%.

### Measured — Apple M1 Max, `aarch64-apple-darwin`

Medians, criterion defaults, measured against this branch's `HEAD` rather than
against ticket 01's merge base — eight tickets have landed on the fold since 01
was written, so 01's subject is no longer the revision immediately before this
change and a ratio against it would carry all nine.

The two baselines agree, which is what lets 01's table stand as the reference it
was written to be: warm folds read 3.91 against 01's 3.92 on `string`, 14.80
against 14.67 on `callback`, and 8.63 against 8.68 on `chain`. `array-answer` is
the one that moved, 8.02 against 7.81, and cold start reads 111.6 against
113.1.

| Leg | Before | After | Ratio |
| --- | --- | --- | --- |
| `fold/string` | 3.91 µs | 1.61 µs | **0.411** |
| `fold/callback` | 14.80 µs | 5.63 µs | **0.380** |
| `fold/chain` | 8.63 µs | 3.32 µs | **0.385** |
| `fold/array-answer` | 8.02 µs | 3.33 µs | **0.415** |
| `fold-distinct` | 5.14 µs | 6.19 µs | 1.206 |
| `engine/string` | 2.25 µs | 2.26 µs | 1.005 |
| `engine/callback` | 10.52 µs | 10.48 µs | 0.996 |
| `engine/chain` | 5.93 µs | 5.95 µs | 1.003 |
| `engine/array-answer` | 5.22 µs | 5.19 µs | 0.994 |
| cold start | 111.61 µs | 112.16 µs | 1.005 |

A warm fold is 2.4-2.6x faster. Every `engine` leg and cold start sit inside the
noise, which is what says the movement is the memo and not the machine.

`fold-distinct` is the price, and it prices the worst case rather than a likely
one: nothing is ever evicted, so tens of thousands of criterion iterations leave
the engine holding tens of thousands of compiled scripts to trace, where a real
build holds one per folded call site. The +21% is that growth as much as it is
the `String` key.

The `fold` legs are no longer bounded above by the `engine` legs beside them, and
the benchmark's module documentation now says so: the fold reuses a parse the
`engine` leg pays on every iteration, so the pair no longer does the same work
and `fold` is the faster half.

### The paired comparison

`bench:revisions` between a build of this branch's `HEAD` and a build of this
change, then `bench:verdict`. Every fixture passed; the two that move are the
two the fold reaches:

```
  Feature - engine fold                    point=0.749 lower=0.747 upper=0.753 status=pass
  Feature - engine fold (dev)              point=0.804 lower=0.802 upper=0.808 status=pass

Suite passed
```

25% off the production fixture and 20% off the development one, against 0.987 to
1.008 on all forty-odd others. Read it for what it measures: the harness
transforms one file many times, so every round after the first is folding shapes
the memo already holds. That is a watch-mode rebuild and a monorepo where files
share shapes, not a single cold compile of a single file — a cold compile of one
file gains nothing and pays `fold-distinct`, which is why the fixture number and
the criterion number both belong here.

### Vocabulary

`crates/stylex-transform/CONTEXT.md` gains a **fold memo** entry, which is what
the thing is called now that its unit is a script: the ticket's own
"compiled-arrow memo" is listed among the spellings to avoid, and neither the
code nor the tests use it.

### Tests

`crates/stylex-transform/src/shared/utils/js/evaluate/tests/engine_fold_tests.rs`
gains a section that counts compilations through a test-only reader on the memo,
because the answer alone cannot tell a hit from a miss — that is the point of a
memo and also why it needs a witness. One text is one entry however often it
folds; two texts are two; a thousand distinct call sites are a thousand and each
answers its own value; two spellings of one expression share an entry, because
the key is what this module printed rather than what was typed; a chain is one
entry rather than one per link; and a declined input still leaves no memo,
because it still leaves no engine.

`crates/stylex-transform/tests/transform_stylex_create_test/fold_memo.rs`
asserts the behaviour at the transform, in pairs of files that print the same
text and must not answer the same value. Same text with different values is
unreachable inside one file — a module cannot declare one name twice, and
`stylex.create` must be bound to a bare variable at the top level — so two
compiles on one thread is not a contrivance but the shape the risk actually has.
Every rule is measured output of `@stylexjs/babel-plugin` 0.19.0.

No earlier test was modified.
