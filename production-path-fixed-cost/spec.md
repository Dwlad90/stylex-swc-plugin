# The branch costs ~2-3% on the production path, and nobody knows where

Status: ready-for-human

Branch: `fix_dynamic-style-parameter-shadowing-an-imported-binding` (148 commits
on `develop`, merge-base `c83ac5cbd`). Measured with the paired revision harness
in `crates/stylex-rs-compiler/benchmark`, 57 fixtures, 5 rounds, balanced order.

> **Corrected once already.** An earlier version of this spec called the cost
> *fixed per transform*, on the grounds that a two-line module regressed by the
> same **percentage** as a 90 µs one. That inference is backwards: an equal
> percentage across sizes is what a *proportional* cost looks like — a fixed cost
> would fall away as the module grows. The size sweep below is the evidence, and
> it says proportional. Anything downstream of the old claim needs re-reading.

## Problem

| Group                    | Geometric mean vs `develop` |
| ------------------------ | --------------------------- |
| All 57 fixtures          | **−5.00%**                  |
| Production shapes (40)   | **+2.40%**                  |
| Development shapes (17)  | **−20.37%**                 |

The development side is why the branch exists: `Debug data - lotsOfStyles.js`
70.6 ms → 11.2 ms (−84.1%), the debug data prop −25.8%, `props and attrs (dev)`
−24.8%, debug and dev class names −24.1%. That is not in question.

The production side costs ~2-3% and **no cause has been established.**

## Shape of it: proportional, with outliers both ways

Production fixtures by size, with the delta beside them:

| size    | delta | fixture                              |
| ------- | ----- | ------------------------------------ |
| 72 µs   | +2.3% | `consts`                             |
| 110 µs  | +2.1% | `card`                               |
| 150 µs  | +4.7% | `Performance - Basic create`          |
| 159 µs  | −4.4% | `use-memo`                           |
| 270 µs  | −6.3% | `buttons-demo`                       |
| 289 µs  | +3.5% | `Feature - props and attrs`           |
| 716 µs  | −4.1% | `counter-with-dynamic-styles`         |
| 716 µs  | +4.8% | `typography`                          |
| 971 µs  | +9.8% | `Feature - dynamic styles`            |
| 1.44 ms | +3.3% | `page-tsx`                            |
| 2.78 ms | +9.6% | `Performance - Complex create`         |
| 4.15 ms | +1.0% | `Performance - Complex theme`          |

Two readings follow:

- **The delta does not fall with size**, so it is not a fixed per-transform
  overhead. It sits around +2-3% from 72 µs to 1.4 ms.
- **Some fixtures got faster** — `buttons-demo` −6.3%, `use-memo` −4.4%,
  `counter-with-dynamic-styles` −4.1%. Real work moved in both directions;
  this is not uniform overhead sitting on top of an unchanged compiler.
- **Two fixtures cost far more than the rest**: `Feature - dynamic styles`
  +9.8% and `Performance - Complex create` +9.6%. Both are `create`-heavy, and
  the dynamic-style path is what the branch rewrote for issue #1266. That may be
  a second, separate cost — see `03`.

One more fact that constrains where to look: **a module with no StyleX import at
all regresses** (+2.0%, measured directly through both `dist` exports on a
two-line module and a 25-line JSX module). Whatever it is, it is in code every
module runs, not in the `create` producer or the `props` consumer alone.

## Measurement noise, and what it means for method

Repeating the same comparison with the same two binaries moves a single
fixture's delta by several points: `Performance - Complex create` read +4.8% in
one run and +9.6% in the next; `Feature - props and attrs` has read +1.8%,
+2.9%, +3.8% and +4.3%. The *sign* is stable across runs and the group means are
stable to within about a point; individual rows are not.

So: do not chase this with more corpus runs. A criterion micro-bench around one
function is the tool — that is what `01` is.

## Ruled out

Each **measured**, not reasoned about. Do not re-test without new evidence.

| Candidate                                        | How it was eliminated                                                                     |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| `stacker::maybe_grow` per fold level             | Built the branch with the call removed; delta unchanged                                    |
| The binding/write pre-scan doing 4× the inserts  | Gated behind `has_import_paths`; the no-StyleX module never reaches it and still regresses |
| `StateManager::clone` per dynamic-style callback | Not on the no-StyleX path at all                                                          |
| Memo key `DefaultHasher` → xxh3-128              | Now measured by `01`, and the direction is right: 38-56% *faster* per call expression      |
| `typescript()` replacing `typescript::strip()`   | `strip()` *is* `typescript(Config::default())` in swc 53                                   |
| `verbatim_module_syntax: true` for JS inputs      | A `.ts` module regresses as much as a `.js` one, and the flag is `false` for `.ts`         |

## Found and fixed

`805c7d9d4` — `resolve_max_evaluation_depth` called `std::env::var` once per
options value, i.e. once per file, and `getenv` walks the environment comparing
entries. Measured at **~0.2 µs per transform** on a 34 µs module (35.58 → 35.37
µs); a `OnceLock` holds the read now. Too small to see at corpus resolution, and
not the cause of the +2.4%.

## Localized: none of the three, and not in the crate at all

`01` timed all three candidates on both revisions, with the same bench file
compiled by both trees. Every one is at parity or **faster** on this branch:

| candidate                              | delta on this branch    |
| -------------------------------------- | ----------------------- |
| The `Discover` walk, call-heavy module | -19 to -20%             |
| The `Discover` walk, call-free module  | -0.2%                   |
| The structural key per call expression | -38 to -56%             |
| `set_seen_module_source_code`          | unchanged               |
| `StateManager::new`                    | +29 ns, once per module |

The walk is faster *because* of the hasher this spec had ruled out, and for the
reason it gave: `add_call_expression` hashes every call expression in every
module, and xxh3 is much cheaper than `DefaultHasher` on one. `StateManager::new`
is the only candidate that costs more, and it costs four hundredths of one
percent of the 72 µs fixture.

A fourth leg was added to the same bench and says the same thing: the whole pass
chain a compile runs -- resolver, type stripping, StyleX, hygiene, fixer, printer
-- is 8-13% faster here, including on a module with no StyleX import.

### The cost is real, and only visible through the built `.node`

The loop is in `02`. Two package directories differing in one binary, four
modules importing no StyleX (two-line and 25-component, each as JavaScript and
as TypeScript), 4000 transforms per sample, subjects alternating inside a
process, seven processes, median across processes.

Same binary against itself reads -0.43% to +0.05%. This branch against
`c83ac5cbd` reads +1.53% to +1.95%, and all twenty-eight process medians are
positive.

So the same source, through the same passes, is faster compiled into a bench and
1.5-2.0% slower compiled into the `.node`. What is left between the two is the
napi boundary and the whole-binary layout of a fat-LTO, single-codegen-unit
build. Neither is a call site.

### And it is a ramp, not a commit

Five points across the 148 commits, each built and measured with that loop, put
the cost between one and three points from somewhere in the first half onward,
with no step and no correlation to binary size -- the quarter-way build is
*smaller* than `develop` and already pays it. A bisect would name whichever
commit it landed on, which is the false answer this spec already warns about.

### The two outliers are a second cost, and it is not the memo

`03` re-measured them at ten rounds and they hold. Counters on `evaluate_cached`
say the two that cost 9-11% more fold **exactly the same nodes** as `develop`,
with the same hits, misses and refusals, and refuse nothing on depth; the one
that got faster is the one whose counts moved. So the extra sits in what the
`create` transformation does around the fold, not in the fold.

### The measurement, repeated

57 fixtures, 10 rounds, paired against a `c83ac5cbd` build of the same package.
The spec's numbers reproduce:

| Group                   | Geometric mean vs `develop` |
| ----------------------- | --------------------------- |
| All 56 fixtures         | **-4.91%**                  |
| Production shapes (41)  | **+2.38%**                  |
| Development shapes (15) | **-22.28%**                 |

The production side is a tight band: everything but the two create-heavy
outliers sits between -6.0% and +5.6%, most of it between +2.3% and +3.0%, which
is the same envelope the no-StyleX modules pay. `Debug data - lotsOfStyles.js`
is 71.5 ms -> 11.3 ms, -84.2%.

`pnpm run --filter=@stylexswc/rs-compiler parity` over the same tree reports **0
changed verdicts over 1026 subjects**, which is the bar `02` set for any change
to per-module state. Nothing in this work changed what the compiler emits -- the
only committed artefact is a bench.

## Decision to take

Not the agent's to make:

- **Accept it.** +2.4% on production shapes against −20.4% on development ones
  and −84% on the debug-annotation path. Record the trade in
  `guidelines/PERFORMANCE.md` and close this.
- **Chase it.** `01` localizes with a micro-bench, `02` acts on what it finds,
  `03` asks whether the two `create`-heavy outliers are a second cost or noise —
  and its first step is to re-measure them, which is cheap.

## Tickets

| File                                                    | Status            |
| ------------------------------------------------------- | ----------------- |
| `01-localize-the-production-cost-with-a-micro-bench.md` | `needs-triage`    |
| `02-act-on-what-01-found.md`                            | `ready-for-human` |
| `03-the-two-create-heavy-outliers.md`                   | `ready-for-human` |

`01` is answered and its bench is committed. `02` and `03` are answered as far
as measurement takes them and now need the decision below, which is not an
agent's to make.
