# 37 — No bench covers the debug path, and none of them set `GLOBALS`

Status: `resolved`
Blocked by: None

Two findings from the code review of 34 and 36. They are filed together because
they are the same gap seen from two sides: this repo cannot currently measure the
path that 36 made 4.5x faster, and the benches it does have may be measuring a
panicking code path.

**36's headline number is unregressable.** Its Answer claims 722.4 ms to 162.0 ms
at 400 `stylex.create` calls, and nothing committed reproduces it. The slices were
cut into a scratch directory and the harness was temporary. Worse, the one
end-to-end benchmark that could cover it —
`crates/stylex-rs-compiler/benchmark` — pins `dev: false` in
`benchmark/lib/config.ts`, and 35 established that `dev=false` is exactly the
configuration where this cost does not appear. So the 4.5x is real, measured, and
completely unguarded: the next change to that path reports as nothing.

**No in-repo bench sets `GLOBALS`.** `parse_and_normalize_program` calls
`Mark::new()`, which panics outside `GLOBALS.set`. Grepped: `GLOBALS` appears only
in `stylex-rs-compiler/src/lib.rs` and in the evaluator's own
`evaluate_tests.rs`. Neither `benches/evaluate_bench.rs` nor
`benches/evaluate_depth_bench.rs` sets it, and `evaluate_bench` runs real fixtures
through `evaluate`, which can reach the code-frame path. Any code-frame work those
benches touch is therefore timing a panic and its unwind rather than the work —
which is precisely the mistake that inflated 35's first attribution by 3.6x. The
numbers those benches report for everything _else_ are still good; the concern is
that a regression in the code-frame path would be invisible, or worse, would read
as an improvement.

**Why this matters more than a missing benchmark usually would.** The debug path
is the one a developer waits on. It is on by default in every dev build (`dev`
implies `debug`), it was 30x the cost of a production build before 36 and is still
5.8x after, and the largest term left in it — the `KeySpanFinder` whole-program
walk — is still quadratic in file size. That is a regression waiting to happen
with no gate in front of it.

- [x] Commit a fixture large enough to show the curve. The slices used were cut
      from `apps/rollup-large-example/lotsOfStyles.js` at 50 / 100 / 200 / 400
      `stylex.create` calls; the smallest that still shows it is the one worth
      committing, not the largest
- [x] Bench the debug path end to end with `dev` **on**, inside `GLOBALS.set`, and
      wire it into the existing `[[bench]]` discovery so `pnpm bench` picks it up
- [x] Decide whether `benchmark/lib/config.ts` should gain a `dev: true` variant
      rather than switching, since both configurations are worth watching and
      they have different shapes
- [x] Audit the two existing benches for the `GLOBALS` gap and set it where the
      code under test can reach `Mark::new()`. Re-read their numbers afterwards:
      any that moves was measuring a panic
- [x] State in `guidelines/PERFORMANCE.md` that a bench touching the transform
      must set `GLOBALS`, so the next one does not repeat this

## Answer

**The debug path now has a bench, and neither existing bench was measuring a
panic.**

`crates/stylex-transform/benches/transform_debug_bench.rs` transforms a
committed fixture end to end with `dev` on and with `dev` off, at three sizes,
all inside `GLOBALS.set`. `pnpm bench` picks it up with no wiring: the bench
script discovers `[[bench]]` names out of `Cargo.toml`.

| creates | lines | `dev`    | `dev=false` | penalty | µs per create |
| ------- | ----- | -------- | ----------- | ------- | ------------- |
| 25      | 512   | 5.87 ms  | 1.89 ms     | 3.1x    | 235           |
| 50      | 864   | 10.60 ms | 3.04 ms     | 3.5x    | 212           |
| 100     | 1 886 | 25.12 ms | 6.57 ms     | 3.8x    | 251           |

Flat per create across four times the size, which is the curve 36 left behind
and the thing a regression would bend. 25.1 ms at 100 creates against 36's
32.5 ms, on a different machine and a different harness -- an earlier run on a
busier machine gave 27.0 ms for the same code -- so the shape is the claim here
rather than the constant.

**36's number, reproduced from committed code.** The bench was checked out onto
`90574a29e` -- the commit *before* `d86ea224f` removed the per-lookup deep clone
-- and run there against the same fixture, so this is one harness measuring both
sides rather than two harnesses being compared:

| creates | dev before | dev after | speedup | prod before | prod after |
| ------- | ---------- | --------- | ------- | ----------- | ---------- |
| 25      | 8.95 ms    | 5.87 ms   | 1.5x    | 1.86 ms     | 1.89 ms    |
| 50      | 20.09 ms   | 10.60 ms  | 1.9x    | 3.04 ms     | 3.04 ms    |
| 100     | 66.83 ms   | 25.12 ms  | **2.7x**| 6.54 ms     | 6.57 ms    |

The speedup grows with the file, which is what removing a superlinear term looks
like as opposed to shaving a constant. Per create, `dev` before was 358, 402 and
668 µs -- climbing -- against 235, 212 and 251 after. The `dev` penalty over
`dev=false` was 4.8x, 6.6x and 10.2x, growing with size; it is now 3.1x, 3.5x
and 3.8x. Production is unchanged within noise at all three sizes, which is the
check that the fix touched only the debug path.

This is the first time that claim has been reproduced from committed code. 36's
own figures came from a scratch harness that no longer exists, which is what
this ticket was filed about; the table here is what a future change to the path
will be compared against. The magnitudes differ from 36's (1.5x/2.3x at 50/100
there, 1.9x/2.7x here) because the machine and the harness differ -- the shape
is what agrees.

**What stands behind the numbers.** Three assertions, run by
`cargo test --benches` as well as by `cargo bench`:

- every slice holds exactly the number of `stylex.create` calls it says, and
  parses -- the slices are cut textually from the committed fixture, so a
  reformatted fixture fails rather than quietly benchmarking a smaller file;
- a `dev` transform resolved at least one `file:line` per create. A failed
  lookup degrades to `$$css: true`, and a bench timing failed lookups is fast,
  flat and meaningless -- indistinguishable from a win;
- a `dev=false` transform resolved none, so the two legs really are the
  presence and the absence of the debug path.

**The fixture.** `crates/stylex-transform/benches/fixtures/lotsOfStyles100.js`,
the first 100 `stylex.create` calls of
`apps/rollup-large-example/lotsOfStyles.js` -- 1 886 lines, 38 KB. The smallest
slice that still showed the effect, as the ticket asked; the 25- and 50-create
points are cut from it at bench startup into `CARGO_TARGET_TMPDIR`, because the
position lookup re-reads the module by filename and each size therefore needs a
file of its own.

Checked against the upstream Babel plugin, both ways round: 366 distinct rules
each, in the same first-occurrence order, and all 330 `$$css` annotations
byte-identical with `dev` on. Babel emits 852 rules to the compiler's 366
because it repeats rules it re-emits; the distinct sets are equal. `pnpm parity`
reports `changed 0`.

**Neither existing bench was timing a panic.** Both now set `GLOBALS`, and both
were re-measured either side of the change:

| benchmark                          | before   | after    |
| ---------------------------------- | -------- | -------- |
| `EvaluatePerfFixtures/colors`      | 35.95 µs | 37.13 µs |
| `EvaluatePerfFixtures/createTheme-complex` | 3.357 ms | 3.330 ms |
| `EvaluateDepth/arithmetic/240`     | 832.6 µs | 821.3 µs |
| `StructuralKeyDepth/arithmetic/240` | 6.526 µs | 6.429 µs |
| `StructuralKeyFallback/object/128` | 2.716 µs | 2.575 µs |

Nothing moved beyond the noise floor, and the two rows that moved most are pure
hashing, which cannot reach `Mark::new()` at all -- so that ~4-5% is machine
drift and it bounds what the rest of the table could be hiding. The gap was
real but latent: those fixtures never reached the code-frame path. The rule is
now written down rather than left to be rediscovered.

**`benchmark/lib/config.ts` gains the variant, not a switch.** A fixture opts
in with `"dev": true` in `fixtures.v1.json`; `fixtureStylexOptions` merges it,
and the runner uses that one function for both the sanity check and the timed
run, so a fixture cannot be validated under one configuration and timed under
another. `createStylexOptions` stays `dev: false`.

Switching it was the alternative and it is the wrong trade: `dev` and
`dev=false` differ by 3-4x on the same file, so flipping the shared option
moves every trend series in the repo at once onto a shape nobody had been
watching, and the two cannot be compared against each other afterwards. One
`dev` fixture is registered -- the same committed slice -- so the shape is
watched under its own name without doubling the matrix.

**Found while wiring it.** `bench-compare.ts`'s Babel subject ignored the
options the runner handed it and closed over the module-level ones instead. A
per-fixture override would have reached only the Rust side, which is not a
comparison. It now uses its argument.

## Found in review

Both axes came back with judgement calls rather than violations. Acted on:

- the `dev`-is-a-different-measurement rationale and the `GLOBALS` rationale
  were each restated in four places, which is four copies to drift. The policy
  now lives in `guidelines/PERFORMANCE.md` and the code cites it;
- `slice()` also created a directory, wrote a file and parsed it. Renamed
  `materialize_slice`, because the write is the point -- the lookup re-reads the
  module by filename;
- the annotation counter matched any string ending in `:<digits>` anywhere in
  the program. It now matches `$$css` property values, so a `grid-area` or a
  `url()` with a port cannot stand in for an annotation. Still passes, which is
  the interesting part: the strings it was counting really were the
  annotations;
- the size rationale cited "2.3x against 1.5x", which are 36's *speedups* at
  those sizes, next to this bench's 3-4x `dev` penalty. Two different ratios one
  paragraph apart; the comment now says which is which;
- the module doc now states what the timed region excludes -- no lex, no
  codegen, no CSS emission -- and why the lookup's own re-read belongs inside
  it;
- `fixtures.v1.json` points at the Rust bench's fixture, so re-cutting that
  fixture would reshape a JS trend series under an unchanged name. The size the
  name claims is now pinned by a test;
- `parseManifestEntry` and `loadAllFixtures` copied the optional `dev` with two
  different idioms twenty lines apart. Same shape in both now.

Declined: the review called the extra manifest-validation and filtering tests,
the `bench-compare.ts` options fix, and the second and third paragraphs of the
`PERFORMANCE.md` section scope creep. The tests were asked for; the
`bench-compare.ts` fix is a real bug this work would otherwise have papered
over; and "assert what the bench is measuring" is the other half of the same
lesson -- 36's numbers were wrong because nothing asserted it.

## What is still not covered

- `cargo test --workspace` does not run bench targets; `cargo test --benches`
  does. So the assertions above gate `pnpm bench` and an explicit
  `--benches` run, not the default test suite. Left as is rather than setting
  `test = true` on three `[[bench]]` targets, which would put ~20 s of
  criterion into every `cargo test`.
- The `KeySpanFinder` whole-program walk per namespace key is still
  `O(namespaces x file size)`. This bench is what will show it when a file gets
  big enough to make it dominate again; building the index is still 36's
  remaining work.
