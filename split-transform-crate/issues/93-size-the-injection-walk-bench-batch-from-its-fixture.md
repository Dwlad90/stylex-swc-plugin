# 93 — Size the injection-walk bench batch from its fixture

**What to fix:** `injection_walk_bench.rs` measured every leg with
`BatchSize::SmallInput`, which sets `batch = ceil(iters / 10)`. Criterion builds
the whole batch before timing, so at the `array/21733` leg it held up to a tenth
of the iteration count of full module-body copies -- roughly 200k AST nodes each
-- plus the same number of returned outputs, all live at once. Criterion
reserves `SmallInput` for an input under about one percent of available memory.

The bench then swaps or runs out of memory on a 16 GB runner, and the pressure
peaks exactly where the measurement is taken.

**How it was answered.** The batch size is chosen from the fixture: a body above
`LARGE_BODY_STATEMENTS` (1,000) is built one per iteration, and a smaller one
keeps `SmallInput`. The measured window is unchanged -- set-up and the output
drop both still sit outside it.

`injection_queue_bench.rs:168-174` was read and is fine as written.

**Blocked by:** None.

**Status:** resolved

- [x] The large legs build one input per iteration
- [x] The measured window is unchanged
- [x] `cargo bench -- --test` passes every leg of both groups
