# 68 — Stop the performance fixture asserting on the clock

**What to build:** the transform's performance fixture test, at
`crates/stylex-transform/tests/performance_fixture/`, asserts a ratio between
two wall-clock timings:

```rust
assert!(complex_time < simple_time * 23.0, …);
```

The assertion is not repeatable. The test is already recorded as flaky in a full
parallel run, so a red run says nothing about the change that produced it. A
coverage run makes it worse: instrumentation slows the two sides by different
amounts, and the ratio moves with it. Ticket 67 puts this crate on the coverage
gate, and a gate over a suite that is not repeatable is worth nothing.

The fixture comparison the same helper makes — `assert_eq!(output,
output_fixture)` — is the part with value and stays.

**Three ways to settle it.** Choose one and say why in the file:

1. Delete the timing assertion and keep the two figures as a printed line. The
   compiler already has a criterion benchmark suite, which measures speed on a
   quiet machine against a recorded budget; that is where a speed gate belongs.
2. Keep the assertion but skip it under instrumentation, read off
   `cfg(coverage)`. The gate then measures a different test from the one
   developers run, which is its own cost.
3. Replace the ratio with a figure the clock does not set — a node count, an
   allocation count, or the number of declarations produced.

**History.** The assertion was once unsatisfiable in a different way:
`as_millis` floors, so a sub-millisecond transform measured as exactly zero and
the ratio read `0.0 < 46.0`. That was corrected to a fractional millisecond,
which turned a test that measured nothing into a test that measures the machine.

**Found by:** ticket 62, which asked for this and was told to leave the test
alone.

**Blocked by:** None.

**Status:** resolved

- [x] `stylex_transform_performance_test` no longer asserts on wall-clock
      time -- the timing is gone entirely, and the test is renamed
      `stylex_transform_theme_fixture_test` to say so.
- [x] The file says which of the three ways was taken, and why
- [x] The fixture comparison stays
- [x] The full workspace suite stays green -- `cargo test --workspace
      --all-features` passes 9600 tests with no failure, and
      `cargo clippy --workspace --all-features --all-targets` and
      `cargo fmt --all -- --check` are both clean.

## Answer

Way 1 was taken, and taken one step further: the timing assertion is deleted,
and so is the timing itself. The ticket's way 1 offered to keep the two figures
as a printed line; review of the change showed that the line has no reader and
costs the suite its slowest pair of transforms, so it went too.

**Why the printed figures went with the assertion.** The same two workloads are
already benchmark fixtures. `crates/stylex-rs-compiler/benchmark/perf_fixtures/`
holds a copy of each file, registered in `benchmark/fixtures.v1.json` as
`Performance - Basic theme transformation` and `Performance - Complex theme
transformation`, and `benchmark/budget.json` gives each one a ceiling in
milliseconds that the budget check fails on -- 0.3881 ms and 6.955 ms, each
measured over 10 runs on a quiet CI machine. That is an absolute per-fixture
figure, which is what a speed gate needs. A figure printed into a captured test
log and read by nobody is not a second gate; it is the first gate's shadow.
Keeping the print also kept a warm-up pass, which doubled the work of the
heaviest fixture in the transform suite for the sake of that print.

**Why not way 3 -- a figure the clock does not set.** Any figure taken from the
result (node count, declaration count, output length) is already pinned by the
`assert_eq!(output, output_fixture)` that the test keeps: the two outputs are
compared byte for byte, so a derived count can only repeat what that equality
already says. It would add a second assertion with no second guarantee.

**Why not way 2 -- skip the assertion under `cfg(coverage)`.** It makes the
coverage gate stand over a different test from the one a developer runs, and it
keeps the flaky assertion in the local suite, which is the part
[ticket 62](./62-record-the-transform-coverage-baseline.md) reported.

**The assertion was not a ratio at all.** The line above it floored the simple
side -- `let simple_time = simple_time.max(2.0)` -- and the simple transform
measures 0.63 ms to 0.90 ms on this machine, always below the floor. The floor
therefore always applied, and the assertion read `complex_time < 46.0` on every
machine where the simple theme takes less than 2 ms. The simple side never
entered the comparison. What the test enforced was one absolute ceiling of
46 ms on the complex transform, on whatever host happened to run it.

The benchmark budget states the same relationship as two absolute ceilings,
0.3881 ms and 6.955 ms, and holds the complex side 6.6 times tighter than the
46 ms the test really enforced, on a pinned runner instead of an unknown one.
The intent of the test is not lost; it is gated harder, in the suite that can
measure it.

**Evidence the assertion was not repeatable.** During review the test printed
0.901 ms simple against 26.64 ms complex on this machine -- a ratio of 29.6,
which the deleted `complex_time < simple_time * 23.0` would have failed. Nothing
about the compiler had changed.

**Three more things came out with the assertion.**

- The `simple_time.max(2.0)` floor. It existed only to keep the ratio from
  dividing by a sub-millisecond figure, so it went with the ratio.
- `unwrap_or_default()` on the recorded output. A missing fixture file read as
  an empty string, and the test then compared the result with `""`. Deleting a
  fixture made the test pass instead of fail. The read is strict now and names
  the path it cannot read. Checked by hand: with `simpleTheme.output.js` moved
  away the test fails with `cannot read
  tests/performance_fixture/simpleTheme.output.js`. A `!output.is_empty()`
  guard stays with it, because an empty result against an empty recorded file is
  equal and says nothing.
- The names. The helper is `assert_fixture`, and the test is
  `stylex_transform_theme_fixture_test`, because neither one measures a time any
  more. The directory keeps the name `performance_fixture`: the fixtures are
  still the performance workloads, and the benchmark suite reads its copies
  under the same description.

**Left open, out of scope.** The two benchmark entries run the production shape,
while this test runs `with_dev(true)` and `with_runtime_injection()`. The dev
shape of these two workloads therefore has no speed gate. The deleted assertion
never guarded it either -- it compared dev against dev -- so nothing regressed
here, but a paired dev fixture would close it.

## Comments

Landed as `test(stylex_transform): compare the theme fixtures without timing
them`. The user was asked whether to keep the simple-against-complex comparison
as a gate and chose to keep it out of the unit test, on the evidence that the
floored assertion was never a ratio and that the budget already holds the same
relationship 6.6 times tighter.

[Ticket 67](./67-remove-the-transform-coverage-exclusion.md) lists this ticket
as a blocker. That blocker is now clear: the crate's suite holds no wall-clock
assertion, so the coverage gate can stand over it.
