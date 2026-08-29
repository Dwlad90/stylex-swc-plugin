# 01 — Price the engine fold

**What to build:** A baseline for what the fold costs, so every later ticket
has a number to move rather than an argument to have.

Nothing in the benchmark corpus currently writes a method call the fold can
take, which is why the paired comparison that introduced the engine reported
every ratio between 0.989 and 1.017 — the engine was never built, so nothing
paid. After this work folds fire on any named receiver, and the transport adds
work per fold. Both need pricing before they change.

Two instruments, for two different reasons. A criterion benchmark prices cold
start, a warm fold and the printing round trip; it has no constraint on what
the merge base can compile. A registered fixture pair prices the fold inside a
real transform, in both configurations, and feeds the paired comparison that
gates a release.

The fixture is the delicate half. A fixture must compile on the revision
*before* the change as well as on this branch, because the paired comparison
sanity-checks every fixture on both subjects before timing anything — a
fixture only this branch compiles fails the base subject and takes the whole
leg down before a single measurement. Confirm empirically which receiver
shapes the merge base folds before registering anything.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] A criterion benchmark prices cold start, a warm fold and the printing
      round trip, and runs inside the global scope the transform requires
- [x] That benchmark panics unless the fold produced the value it exists to
      time — a refusal, a deopt and a memo hit are all fast, and a curve that
      flattens because the work stopped happening must not read as a win
- [x] A production and development fixture pair is registered for a receiver
      shape verified to compile on the merge base, and the paired comparison
      passes on both subjects
- [x] The performance fixture header states why the named-receiver shapes are
      absent, as the dynamic-styles fixture already does for its own omission
- [x] Measured cold start, warm fold and per-fixture ratios are recorded in
      this ticket's Answer section as the baseline

## Answer

Two instruments, as the ticket asked, plus the empirical check that decided the
fixture's shape.

### The criterion benchmark

`crates/stylex-transform/benches/engine_fold_bench.rs`, two groups over one
table of legs — a string method, a callback the engine invokes per element, a
chain that folds at every link, and an answer that comes back as an array and so
runs the outward half of the bridge.

`EngineFoldRoundTrip` is a pair per leg: `engine/<leg>` hands a warm context the
source the fold would print, and `fold/<leg>` puts the same expression through
the evaluator. The second is the warm fold; the gap between them bounds what the
fold adds over the JavaScript. One pair rather than a separate warm group,
because a warm fold measured alone and a warm fold measured next to the engine
are the same measurement — the first draft had both, and the duplicate legs
disagreed by 4%.

`EngineFoldColdStart` builds a fresh context per iteration, with the same
loop-iteration limit the fold's own engine carries, and evaluates a leg. One leg
rather than four: measured per shape they came back within 8% of each other,
114-126 µs, against warm engine costs spanning 2.3-10.7 µs, so the number is
context construction and not the JavaScript. Four columns would have implied a
leg-dependence that is not there.

Every leg asserts before it times, on both sides. The fold side asserts through
the evaluator that the fold produced the recorded answer, which also warms the
thread-local engine so the group times warm folds rather than one cold one and
many warm ones. The engine side asserts the context answered that same value —
a context that came back answering `undefined` for everything would otherwise
report as a very fast cold start. One recorded answer per leg covers both,
because the folded value is rendered the way the language renders it.

### Measured — Apple M1 Max, `x86_64` unmeasured

Criterion defaults, median of 100 samples, `aarch64-apple-darwin`. Measured on
both subjects of the comparison — the merge base `6d1cceac9` and this branch —
by copying the bench into the merge-base worktree, so the baseline is a pair
rather than a single column.

Cold start, the same leg on both: **114.18 µs** on the merge base, **113.09 µs**
on this branch.

| Leg | Warm fold (`fold/`) | Engine alone (`engine/`) | Round trip adds, at most |
| --- | --- | --- | --- |
| string | 3.92 µs | 2.28 µs | 1.64 µs (1.72x) |
| callback | 14.67 µs | 10.60 µs | 4.07 µs (1.38x) |
| chain | 8.68 µs | 5.96 µs | 2.72 µs (1.46x) |
| array-answer | 7.81 µs | 5.15 µs | 2.66 µs (1.52x) |

Cold start is ~113 µs and is paid once per thread, on the first fold — a file
with no foldable method call never builds the engine at all. A warm fold is
3.9-14.7 µs, of which at most 1.6-4.1 µs is what this compiler adds over the
JavaScript: the guard walk, printing the call to source, and converting the
answer back. *At most*, because the `fold` leg enters through `evaluate` and so
carries the evaluator's own entry cost alongside those three; nothing here tells
them apart. The overhead is roughly flat in absolute terms and so shrinks as a
proportion of the work the engine does, which is why the callback leg — the one
that runs real JavaScript per element — is the cheapest of the four in relative
terms.

Branch against merge base, same run conditions, as branch / merge base:

| Leg | Warm fold | Engine alone |
| --- | --- | --- |
| string | 0.966 | 0.987 |
| callback | 0.982 | 0.992 |
| chain | 0.993 | 0.988 |
| array-answer | **0.891** | 0.988 |

Cold start is 0.990. Three of the four warm folds and every engine leg sit
inside the noise, which is the expected reading: nothing on this branch touched
the print or the engine. The one real movement is `array-answer` at 0.891 —
ticket 04 answering an array as the evaluator's own value instead of building a
syntax node for it, worth 11% of that leg. The engine leg beside it is flat at
0.988, which is what says the movement is on this side of the bridge.

Reproducing the merge-base column needs one edit the committed bench does not
carry, because before ticket 04 an array answer came back as a syntax node
rather than as the evaluator's own value. Add this arm to `fold_text` in the
copy that runs there:

```rust
EvaluateResultValue::Expr(Expr::Array(array)) => {
  let rendered = array
    .elems
    .iter()
    .map(|elem| match elem {
      Some(elem) => match elem.expr.as_ref() {
        Expr::Lit(Lit::Str(string)) => convert_atom_to_string(&string.value),
        _ => String::from("?"),
      },
      None => String::from("?"),
    })
    .collect::<Vec<_>>()
    .join(",");

  Some(rendered)
},
```

It is not in the committed bench because on this branch it is unreachable, and
an unreachable arm is the smell the review of tickets 02-04 already deleted once.

### The fixture pair

`crates/stylex-rs-compiler/benchmark/perf_fixtures/engine-fold.js`, registered as
`Feature - engine fold` and `Feature - engine fold (dev)`. Fifteen rules over
four namespaces: string methods, array methods including two callback shapes,
chains, and two array answers read back as CSS fallback lists.

Its output was compared against the reference compiler on the same file and is
identical, class name for class name and declaration for declaration, on all
fifteen rules. Measured once, by hand — nothing pins it, because the corpus that
would is ticket 14's.

### Which shapes the merge base folds — measured, not assumed

The merge base is `6d1cceac9`, which already carries the fold, so the question
was which *receivers* it takes. Run against the merge base's own build, the
first draft of this fixture failed to compile:

```
[StyleX] chains > borderColor > Unsupported expression: CallExpression
```

`.split(' ').reverse().join(' ')` — the mutating method. Mutating methods only
fold as of `0a73a56c5` on this branch, so a fixture using one would have thrown
on the base subject and taken the whole leg down before a single measurement.
Both `reverse` calls were replaced, and the fixture header now records the
omission beside the named-receiver one, which the ticket already anticipated.
After that, both subjects emit the same fifteen rules.

### The paired comparison

`bench:revisions` against the merge-base build, then `bench:verdict`:

```
Paired verdict: merge-base vs branch
  Feature - engine fold                    point=0.994 lower=0.976 upper=1.013 status=pass
  Feature - engine fold (dev)              point=1.029 lower=1.005 upper=1.106 status=pass

Suite passed
```

Both ratios sit on 1.0, which is the expected reading and the point of the
baseline: the two subjects fold these shapes identically today, so the numbers
above are what ticket 05 and after have to move rather than an argument to have.
