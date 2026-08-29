# 36 — A benchmark prices the input that folds nothing

**What to build:** A fixture of many call expressions that fold nothing, so the
cost the guard imposes on ordinary input is measured before it is optimised.

**Why it comes first.** The registered fixture is `engine-fold.js`, and it is all
folds. Every performance claim on this branch is therefore measured against input
that folds — which is the minority of real input by a wide margin, and the case
where the fold's own cost dominates and hides everything around it. Two measured
regressions, 37 and 38, land entirely on input that folds nothing; against the
current corpus, both are invisible.

The performance policy's rule that a fixture must change what the compiler emits
still applies, and this one does: the module compiles on the merge base and on
this branch to the same CSS, which is the point. What it must contain is call
expressions — `stylex` calls, member calls on values that do not resolve, calls
inside dynamic style functions — since the cost being priced is paid per
`CallExpr` the evaluator visits, whether or not anything folds.

**It is a measurement ticket, not a fix.** Nothing here changes compiler
behaviour. Its output is a number the three tickets it blocks are held to.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] A production/development fixture pair whose module compiles identically on
      the merge base and on this branch, containing many call expressions and no
      foldable call
- [x] The paired comparison runs against the merge base without failing the base
      subject — confirmed empirically before registering, as the policy requires
- [x] The baseline number is recorded, so 37, 38 and 39 have something to move
- [x] String concatenation and template interpolation are represented, since
      that is where 38 is paid

## Answer

**`perf_fixtures/engine-fold-miss.js`, registered as `Feature - calls that do
not fold` and its `(dev)` twin.** 111 call expressions, 69 rules, and not one
call an engine can answer. Every non-`stylex` call carries a leaf the guard
cannot resolve — the parameter of the dynamic style function it is written
inside — so the walk descends the whole expression before refusing. The ten
namespaces are named for where the refusal is decided: a global callee, a
conversion global, a method on a named receiver, a chain that only refuses at
its last link, an array element, a callback body, a template the parameter is
interpolated into, a concatenation, a theme member beside a parameter, and a
callback nested in a callback.

**The calls had to be stacked, not spread.** The first draft wrote one call per
declaration and measured the wrong thing: each value a dynamic namespace holds
costs a custom property and an `@property` rule, which is work the guard has
nothing to do with, and stripping every call out of it moved 3.14 ms to 0.45 ms —
the calls were 8 % of the fixture, so halving the guard would have shown as 4 %
and passed under the 10 % warn threshold. Stacking the calls inside a few values
holds that overhead flat while the count grows: 4.76 ms against 1.08 ms for the
same namespaces with the calls taken out, so what the fixture reports is now
about three quarters the cost of visiting call expressions.

**One shape had to come out, because the base subject refuses it.** A `filter`
whose receiver is a `map` the base cannot resolve threw `Expr is not a literal`
on the merge base while compiling here — the failure
`guidelines/PERFORMANCE.md` describes, which takes the whole leg down before a
measurement. The first bisect blamed the conversion global written beside it in
the same chain; asked on its own, `Number(value).toFixed(2)` inside a callback
compiles identically on both, and so does a `map` on an unresolvable receiver.
Both are in the fixture. Every shape was checked against both subjects one
namespace at a time, and the note in the fixture header now names only the one
that failed.

**The baseline, on `x86_64-apple-darwin` (M1 Max, Node 24.11.0), 10 rounds,
seed 1.** The merge base is `6d1cceac9`, the branch `e48e882ee`:

| fixture | merge base p50 | branch p50 | ratio (lower, upper) |
| --- | --- | --- | --- |
| Feature - calls that do not fold | ~6.30 ms | ~4.72 ms | 0.747 (0.736, 0.761) |
| Feature - calls that do not fold (dev) | ~6.74 ms | ~5.06 ms | 0.752 (0.747, 0.755) |

**The branch is a quarter faster on input that folds nothing, not slower.** So
37 and 38 are not regressions against the merge base on this fixture — whatever
they cost is already more than paid for by the rest of the branch. What they
have to move is the *branch* number: 4.72 ms and 5.06 ms, measured
branch-before against branch-after with the same command. These are darwin
numbers and cross-run noise there is ~34 %, so they are a local baseline for a
paired same-process run, not a threshold anything is held to in CI.

**How to reproduce.** The base subject is a package directory holding the merge
base's `dist`; `.scratch/bench-base` is one, staged from the `develop` worktree
so `dist/index.js` resolves this repo's `node_modules`:

```
node --import tsx/esm benchmark/bench-revisions.ts \
  --base ../../.scratch/bench-base --candidate . \
  --base-label merge-base --candidate-label branch \
  --rounds 10 --fixture "calls that do not fold"
node --import tsx/esm benchmark/bench-verdict.ts \
  --primary benchmark/results/revisions-raw-stats.v1.json
```

**The fixture guards its own claim.**
`the_no_fold_benchmark_fixture_holds_no_foldable_call` reads the file, collects
every call expression in it and evaluates each from a thread holding no engine,
failing if any of them builds one. It catches the way the mistake is actually
made — a call written out of literals. A call that would only fold through a
name the module binds still refuses there, and the paired comparison is what
catches that one. The count it holds the fixture to is 100, against 111 today —
the fixture reports the guard's cost only while the calls are most of what it
does, and a rewrite that quietly halved them would leave the number meaning
something else.

**The baseline lives here and nowhere else.** `.scratch` is never committed, so
what 37, 38 and 39 measure against is the command above re-run on their own
machine, not a recorded artifact. That is the tracker's convention rather than
an oversight: a darwin number from one laptop is not something CI could hold
anything to.
