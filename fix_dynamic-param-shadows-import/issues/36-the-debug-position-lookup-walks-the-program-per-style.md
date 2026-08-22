# 36 — The debug position lookup walks the program once per style

Status: `resolved`
Blocked by: None — 35 measured it.

**What was measured.** In 35. `add_source_map_data` is **98.6% of one transform**
of a 400-create file in a `dev` build (2 643 ms of 2 685), and its cost is
**quadratic in file size** — 54 ms, 212, 849, 2 643 for 50, 100, 200 and 400
`stylex.create` calls. With `dev` off the same file takes 28 ms.

**Why.** For each namespace key, `add_source_map_data` asks
`get_key_span_from_source_code` for the key's authored position, and that walks
the **whole memoized program** with a `KeySpanFinder` to locate one key. The
source parse is memoized (`get_memoized_frame_source_code`) and the resolved span
is cached per key (`state.cached_span`), so neither of those repeats — but the
_walk_ does, once per namespace key. A file that is one long list of styles pays
its own length once per style.

**Who pays.** Every dev build, and not obviously so: `dev` implies `debug`
(`stylex_options.rs:335`, and `structs/mod.rs:175` in the NAPI layer) and
`enable_debug_data_prop` defaults to `true`. The feature it pays for is the
`file:line` annotation on `$$css`.

**The shape of the fix.** One indexed pass instead of N walks. The finder already
knows what it is looking for — a namespace key inside a `stylex.create` object
argument — so a single visit of the memoized program can collect _every_
(call, namespace key) -> span pair into a map on the state, and each lookup
becomes a hash hit. That turns O(namespaces x file size) into O(file size), and
it does not change a single resolved position, which makes it verifiable against
the existing snapshots rather than a judgement call.

**Where it is not that simple.** The finder does more than match a key: it
carries `sibling_keys`, `namespace_value_keys`, a `target_lo`, and an
`ambiguous_best` flag, because two namespaces in one file can spell the same key
and the disambiguation is what makes the answer right. An index has to reproduce
that disambiguation rather than approximate it, which is the actual work here.

- [x] ~~Index every (call, namespace key) -> span in one pass~~ — **not the
      cost, and not done.** The walk is 69 ms of the 722; the deep clone was 247
      and the rest was the same clone paid twice. Left as the remaining work
      below
- [x] Every existing snapshot must be byte-identical afterwards — 27 test
      binaries, no failures, no snapshot changes
- [x] Re-measure the four slice sizes — 722 ms to **162 ms** at 400 creates, and
      the curve is now roughly linear
- [x] Re-take the whole 368 789-line `lotsOfStyles.js` — see below; the reason it
      was killed was memory, and the memory was this
- [x] `get_span_from_source_code` has the same shape — it shared the same clone,
      and shares the fix
- [ ] Consider, separately, whether `dev` should keep implying `debug`. Still a
      product question, still not needed here

## Answer

**The cost was a deep clone of the whole module, once per style, twice over.**
And the diagnosis this ticket was filed with was wrong — see the correction
below.

`get_memoized_frame_source_code` ended its cache-hit path with
`Some(Program::Module(cached_program.clone()))`. The parse was memoized and the
span was cached, but the _handing back_ was a full deep clone of the module AST,
and the debug-data path asks for it once per namespace key — then a second time
through the value-matching fallback. So a bigger module was both more clones and
a bigger clone, which is where the quadratic came from.

It now returns whether the state holds a program, and callers borrow the module
out of the state for their walk, with the borrow scoped so the span can still be
written back afterwards. `find_expression_span` takes `&Module`.

A second, smaller instance of the same shape: `load_code_frame_from_cache_for_state`
called `new_source_file` on every lookup, and the source map behind every
`CodeFrame` is a process-global `OnceLock` — so each lookup appended another copy
of the module's text to a map that never shrinks. On the 400-create file that was
~250 MB of duplicated source per module. `register_source_once` makes it once.
**That is what killed the full-file run**, not the time — the earlier guess of
"quadratic time" and the later guess of "not memory" were both half right.

| creates | lines | before   | after    | speedup  |
| ------- | ----- | -------- | -------- | -------- |
| 50      | 863   | 26.9 ms  | 17.8 ms  | 1.5x     |
| 100     | 1 886 | 74.0 ms  | 32.5 ms  | 2.3x     |
| 200     | 3 920 | 254.1 ms | 77.2 ms  | 3.3x     |
| 400     | 6 885 | 722.4 ms | 162.0 ms | **4.5x** |

Per line that is 20.6, 17.2, 19.7 and 23.5 µs — flat, where before it climbed
from 31 to 105. The `dev` penalty over `dev=false` drops from 30x to 5.8x.

## Correction to this ticket's own measurements

**35's numbers were inflated ~3.6x by the harness, not by the compiler.**
`parse_and_normalize_program` calls `Mark::new()`, which panics outside
`GLOBALS.set`. The real compiler sets it (`stylex-rs-compiler/src/lib.rs`); the
first attribution harness did not. So every lookup panicked, was swallowed by the
diagnostic panic boundary, and re-read and re-parsed the module for the next one —
because the memo it would have populated was never reached.

Re-measured inside `GLOBALS.set`, the honest figures are:

| claim                                   | as filed | corrected |
| --------------------------------------- | -------- | --------- |
| transform of the 400-create file, `dev` | 2 685 ms | 722 ms    |
| `dev` penalty                           | 107x     | 30x       |
| `add_source_map_data` share             | 98.6%    | ~85%      |

The conclusions survived the correction — it is still `dev`, still this function,
still superlinear — but the magnitudes did not, and a reader comparing against
this file needs to know which numbers came from which harness. Anything measuring
this path must set `GLOBALS`.

## Remaining work

The `KeySpanFinder` walk is still one whole-program visit per namespace key — 69
ms of the old 722, and the largest term left. It is genuinely O(namespaces x file
size), so it will dominate again on a file large enough. The index this ticket
was originally filed to build is the fix for it, and the disambiguation notes
above still apply. Worth its own ticket when a file gets big enough to need it.
