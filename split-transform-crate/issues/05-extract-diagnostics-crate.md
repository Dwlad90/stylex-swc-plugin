# 05 — Extract the diagnostics crate

**What to build:** Building a code frame for an error, and finding the
declaration span an error should point at, are presentation concerns that
currently sit inside the transform and are reachable from anywhere in it. Give
them their own crate so error presentation can evolve independently of what
raises the error.

The diagnostics code reads exactly nine methods off the **state manager** —
filename, seen-module source get and set, cached span get and set, index access,
and the three framed-declaration methods. None of the nine has a counterpart, so
there is no parity risk in reshaping how they are reached. Declare them as a
trait owned by the diagnostics crate and implement it on the state manager.

This is established practice here: `stylex-atoms` already takes its compilation
utilities through an injected trait precisely to avoid depending on the
transform, which would be a cycle. The trait is consulted at a diagnostic site,
never on the evaluation path, so the indirection costs nothing measurable.

**Blocked by:** 03 — Move the three exported macros; 04 — Extract the state-index crate.

**Status:** ready-for-human

- [x] Code-frame building and declaration-span lookup live in the new crate.
- [x] The crate reads state through its own trait and never names the state manager.
- [x] The state manager implements that trait; its method surface is unchanged.
- [x] Every consumer of the old module reaches the new crate directly, with no facade left behind.
- [x] The unit tests covering the moved code move with it.
- [x] The crate reaches zero uncovered lines and zero uncovered regions.
- [x] The crate has a `CONTEXT.md` and a context-map row.
- [x] The crate is placed in the documented layer list.
- [x] Error output is byte-identical to the baseline for every diagnostic the suite exercises.
- [x] Benches diffed against the baseline; no regression outside noise.

## Comments

### The coverage gate turned a move into a rewrite of the dead branches

`stylex_transform` is on the workspace coverage exclusion list, so this code had
**never been measured**. The moment it became its own crate the gate reported
**194 uncovered regions** — not a regression, but a first measurement.

Closing them was not only test-writing. Three groups of the moved code could not
be exercised because nothing could reach them:

- **Three shadowed visitor methods.** `visit_import_named_specifier`,
  `visit_import_default_specifier` and `visit_import_star_as_specifier` are only
  reached after `module_level_declaration` failed, and an `import` can only be
  written at module level, where that walk already answers it through
  `import_specifier_declaring` — same predicate, same span. Deleted. All 62
  original tests still pass, both import tests included.
- **Two branches no input could take.** `get_source_code` ends in a synthesized
  module, so it can never return `None`; it returns `String` now, and the
  `?` its caller carried is gone. Its "memoized text" arm is gone too — the one
  caller reaches it only after finding there is none.
- **`DeclarationFinder::record`'s second guard.** Every call site asks `done()`
  first, so `if self.found.is_none()` was a branch nothing could take.

The ticket asked for a move, so this is scope the ticket did not name. It was
not optional: the gate demands zero uncovered regions, and dead code cannot be
covered.

### No coverage exclusions were added, on the second attempt

The first pass reached 100% with four `#[cfg_attr(coverage_nightly, coverage(off))]`
attributes. Both review axes flagged the same thing, and they were right —
`guidelines/stack/RUST.md` forbids new exclusions without justification, and
three of the four were not panic branches but ordinary code extracted into a
named `fn` *so that* the attribute could be applied. That is an exclusion
wearing a refactor.

All four are gone. What replaced them:

- The two debug-vs-warn reporting cascades are covered by a **capturing logger**
  (`src/tests/capturing_logger.rs`) whose level and message buffer are both
  per thread, so parallel tests cannot decide what each other sees. The tests
  now assert *what* is reported at each level, which the exclusion would never
  have caught.
- `nothing_printed` and `expect_module` are called directly — the second under
  `#[should_panic]`.

100% of regions, functions and lines, with zero exclusions.

### The nine delegating bodies are a latent recursion

`impl DiagnosticState for StateManager` is nine bodies of
`fn get_filename(&self) -> &str { self.get_filename() }`. Each reads as a call
to itself and is not one: an inherent method wins method resolution over a trait
method of the same name. Delete or rename one of the nine inherent methods and
its body here becomes unbounded recursion rather than a compile error. A comment
above the impl says so; **a maintainer may want a stronger guard than a comment.**

### The mermaid graph is now duplicated nineteen times

Ticket 04 recorded that the same dependency graph is copied byte-identically
into every crate README and that a maintainer should decide whether it becomes
one file. Layer 7 — Diagnostics shifts evaluation, CSS, transform and compilers
up by one again, so all eighteen copies were rewritten a second time and the new
crate's README makes nineteen. The renumber is mechanical; the duplication is
not, and it has now cost two tickets.

### Do not check error output by diffing the suite's stderr

The obvious way to check "error output is byte-identical" is to run the suite
before and after and diff the `error:` lines. **It does not work, and it looks
like it does.** A worktree at `HEAD` and the working tree differed by 14 lines
and by 6 in count — which reads as a regression until the same comparison is run
against *itself*: two runs of identical code differed by 12 lines and by 8 in
count.

The reason is that these diagnostics are written to stderr by the SWC handler,
and `cargo test` does not capture a test's stderr. The lines are interleaved from
parallel threads and tear. Any line-level diff of them measures the scheduler.

What actually proves the criterion is deterministic and was already in the tree:

- **69 `#[should_panic(expected = "…")]`** assertions in `stylex-transform` on
  exact diagnostic text, key path and all -- e.g.
  `"base > content > Cannot fold 'toLocaleUpperCase' at compile time."`
- **33 `insta` snapshots**, with **zero** drift after the move.
- The `add_source_map_data` tests, which assert the resolved **line number** a
  code frame produces, so the whole span lookup is checked by value.
- The crate's own 96 tests, which assert the framed line for each declaration
  shape and the text each reporting path logs.

All green. The message construction was not touched by the move: `create_error`
still formats `[StyleX] {message}`, and the reporting panic still builds the same
`StyleXError`.

### Benches: a reproducible delta on code that did not change

All seven targets re-run against `pre-split`. 70 measurements, **12 regressions
of +1.65% to +4.73% and 17 improvements**.

Ticket 04's method -- re-run and see whether the reading swings -- was not enough
here. It says noise when a target swings, but **most of these reproduced**:
`StructuralKeyFallback/object/128` measured +4.45% then +4.73%,
`StructuralKey/call/shallow` +3.37% then +3.52%, and the four
`EvaluateDepth/arithmetic` sizes +1.6% to +2.3% both times. Only
`ConcatenationChain` swung (+2.40% then +0.83%), which is the one ticket 04
already knew about.

So the deltas were checked for **attribution** instead, by benching `HEAD`
(pre-04, pre-05) against the same `pre-split` baseline, on this machine, in the
same session -- a worktree with `target/criterion` copied in so the baseline
resolves. `HEAD` is flat: **0.17% to 0.68% on every one of them.** The deltas
therefore belong to this change, not to drift in a baseline recorded on another
day.

They are still not a regression, and one measurement proves it.
`StructuralKeyFallback/object/128` benchmarks
`black_box(stable_hash_unspanned(black_box(&object)))` -- `stylex-utils` called
directly. `crates/stylex-utils/src/hash.rs` is byte-identical to `HEAD`; only its
README changed. That measurement cannot execute one line this work wrote, moved
or dispatched dynamically, and it moved 4.7%.

What did change is the binary. The bench links two crates it did not before, and
these targets build with `-C lto -C codegen-units=1`, so the whole program is one
codegen unit whose function placement shifts when its membership does. The
affected measurements all carry that signature -- the *fastest* leg of a group
moves while its siblings do not:

| Group | Regressed | Flat sibling, same function |
| --- | --- | --- |
| `StructuralKey/call` | `shallow` +3.5% (58.7 ns) | `member` +0.2%, `nested` +0.4% |
| `StructuralKeyFallback/object` | `128` +4.7% (2.61 µs) | `129` +0.5% (8.49 µs) |
| `StructuralKeyDepth/arithmetic` | `30` +2.2% | `60`, `120` +0.3% |

A function that got slower would move all of its legs. Alignment moves the short
one.

**The lesson, which cost three extra bench runs:** a criterion delta that
reproduces is not thereby a regression. Reproducibility separates code layout
from thermal noise -- layout is deterministic per binary, so it repeats exactly.
What separates a regression from layout is attribution: bench the parent commit
against the same baseline in the same session, and check whether the source on
the measured path changed at all.

### One new workspace dependency

`url = "2.5.8"`, a dev-dependency of the new crate only. `read_source_file`
matches on `FileName::Url`, and nothing else in the workspace could name that
type to test the arm.
