# 35 — Where the transform's time actually goes

Status: `resolved`
Blocked by: None

**Why this exists.** 29 measured the evaluator's memo key and found it quadratic
in depth. Chasing that led to a second measurement that matters more, and points
somewhere else entirely.

Instrumented over a 6 885-line slice of `apps/rollup-large-example/lotsOfStyles.js`
(400 `stylex.create` calls, release build), one transform of that file takes
**2 630 ms** and spends **112 737 node-visits** across 7 755 memo keys. Priced
off the depth benchmark — 9.15 µs for the ~481 node-visits of a 240-level tower,
so about 19 ns each — the entire structural-hash walk for that file is **~2 ms**.

Under 0.1%. So the thing 29 spent its time on is a rounding error on real input,
and nothing in this repo says what the other 99.9% is. That is the gap.

**What is already known, and is not enough.** `crates/stylex-rs-compiler/benchmark`
has `bench.ts`, `bench-revisions.ts` and `bench-verdict.ts`, which answer
_whether a change regressed_ against a budget. They do not answer _what the time
is spent on_ — a verdict is not an attribution, and a budget cannot tell you
which stage to look at.

**Why the obvious tool is awkward here.** `cargo flamegraph` is installed, and on
macOS its dtrace backend wants root (`--root`), which a headless run cannot
supply. So the first cut should be explicit stage timing, which is deterministic,
needs no privileges, and can be re-run by anyone; a sampling profile is the
second cut, once the timing says which stage is worth sampling.

**Care needed about what is being measured.** The 2 630 ms above came from a run
with per-node atomic counters compiled in, so it is an overestimate — the
attribution has to be taken again without them. The measurement also has to state
its options: that run had `dev` and runtime injection on, which is the expensive
configuration and not what a production build does.

- [x] Attribute one transform of the corpus slice to stages — **98.6% is one
      function**, `add_source_map_data`
- [x] Say whether the answer changes between `dev` on and off — it is the whole
      answer: 2 685 ms with `dev`, 28 ms without, and `dev` implies `debug`
- [x] Re-take it on the whole 368 789-line file — not needed, and it is not
      memory: the cost is quadratic in file size, which puts the full file at
      hours rather than at an out-of-memory kill
- [x] Whatever dominates gets its own ticket with the number attached — 36

## Correction — read this before the numbers below

**Every figure in the Answer below is inflated ~3.6x by the harness that produced
it.** `parse_and_normalize_program` calls `Mark::new()`, which panics outside
`GLOBALS.set`; the real compiler sets it, this attribution harness did not. So
every debug-position lookup panicked, was swallowed by the diagnostic panic
boundary, and left the memo it would have populated unset — making the next
lookup re-read and re-parse the module.

Re-measured inside `GLOBALS.set`: the 400-create file takes **722 ms**, not
2 685; the `dev` penalty is **30x**, not 107x; `add_source_map_data` is **~85%**,
not 98.6%. The conclusions all survive — it is `dev`, it is this function, it is
superlinear — but the magnitudes do not. 36 carries the corrected table, and
anything measuring this path must set `GLOBALS`.

## Answer

**One function, 98.6% of the transform, and it is quadratic in file size.**

Stage attribution over the 400-create slice (release build, no per-node
instrumentation this time), `dev` and runtime injection on:

| stage | time       | share |
| ----- | ---------- | ----- |
| read  | 0.1 ms     | 0.0%  |
| parse | 0.8 ms     | 0.0%  |
| visit | 2 682.7 ms | 99.9% |
| print | 0.8 ms     | 0.0%  |

Inside the visit, `transform_producers` is all of it, and inside that:

| probe                            | time       | calls |
| -------------------------------- | ---------- | ----- |
| `add_source_map_data`            | 2 643.5 ms | 400   |
| `stylex_create_set`              | 18.0 ms    | 400   |
| `evaluate_stylex_create_arg`     | 4.9 ms     | 400   |
| `other_injected_css_rules.clone` | 0.02 ms    | 400   |

The evaluator — the thing 29 was about — is 4.9 ms of 2 685. The memo key inside
it is ~2 ms of that.

**It is `dev`, not runtime injection.** All four combinations measured:
`dev=true` costs ~2 650 ms whether injection is on or off; `dev=false` costs
~29 ms either way. The connection is not obvious from the option name —
`stylex_options.rs:335` reads `.with_debug(options.debug.or(options.dev))`, and
the NAPI layer does the same at `structs/mod.rs:175`. So **`dev` implies
`debug`**, `enable_debug_data_prop` defaults to `true`, and every dev build pays
this.

**The shape is O(namespaces x file size).** Same slice cut four ways, `dev=true`:

| creates | lines | `add_source_map_data` | vs previous |
| ------- | ----- | --------------------- | ----------- |
| 50      | 863   | 54.4 ms               | —           |
| 100     | 1 886 | 212.2 ms              | 3.9x        |
| 200     | 3 920 | 849.1 ms              | 4.0x        |
| 400     | 6 885 | 2 643.5 ms            | 3.1x        |

Doubling the file quadruples the cost — the last row is 3.1x only because those
lines are 1.76x rather than 2x. `dev=false` over the same four is flat.

**Why.** `add_source_map_data` resolves each namespace key's authored position
through `get_key_span_from_source_code`, which walks the **whole memoized
program** with a `KeySpanFinder` to locate one key. The parse is memoized and the
result is cached per key, but the _walk_ is not shared: one full-program visit per
namespace key, so a file that is one long list of styles pays its own length once
per style.

Extrapolated, the full 368 789-line `lotsOfStyles.js` is ~53x this slice, which
at a quadratic puts it near two hours. That is why it was killed rather than
finishing — the earlier guess that it was memory was wrong.

**Not measured here, and worth knowing:** the same whole-program-walk shape is in
`get_span_from_source_code`, which is this function's fallback and also the
diagnostic path. A file with computed keys would take the fallback for every one
of them.
