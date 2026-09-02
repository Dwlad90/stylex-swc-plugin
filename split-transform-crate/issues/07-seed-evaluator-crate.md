# 07 — Create the evaluator crate and seed it with the dependency-free leaves

**What to build:** Stand up the crate that will hold the general JavaScript
evaluator, and move into it the parts that can travel alone: the growable stack,
the binding lookup and the evaluation helpers all have **zero** internal
dependencies, and the evaluation cache depends only on the growable stack.

Splitting this from the bulk move that follows keeps that move purely mechanical
and gives the new crate a compiling, gated existence before ~9.5k lines land in
it. Roughly 1.8k lines move here.

The **deopt** helper deliberately stays behind for now: it reads the code-frame
builder, so it travels with the core in the next ticket.

**Blocked by:** 05 — Extract the diagnostics crate; 06 — Rename the nested-config crate.

**Status:** ready-for-human

- [x] The crate exists under the name freed by the rename and compiles.
- [~] The growable stack, binding lookup, helpers and evaluation cache live in it. — only the growable stack can travel; see the first comment.
- [x] Nothing moved in this ticket depends on anything still in the transform.
- [x] The transform reaches these through the new crate path, with no facade left behind.
- [x] The unit tests covering the moved code move with it.
- [x] The crate reaches zero uncovered lines and zero uncovered regions, or ships a temporary exclusion that ticket 08 removes.
- [x] The crate has a `CONTEXT.md` and a context-map row.
- [x] The crate is placed in the documented layer list.
- [x] Benches diffed against the baseline; no regression outside noise.

---

## Outcome

Landed as `40cc62311`.

`crates/stylex-evaluator` exists at layer 9 of the documented DAG, between
diagnostics and CSS processing. It holds `growable_stack` and the fourteen unit
tests that cover it, and it measures 100% of regions, lines and functions.

Moved: 210 source lines and 266 test lines. The transform reaches
`grown_per_level`, `grown_for_depth`, `carriable`, `nesting_of` and
`DEEPEST_CARRIED` through the crate path at six call sites, with no facade.
`claim_for` stays `pub(crate)`: it exists so the crate's own test can assert the
arithmetic instead of restating it, and it has no consumer across the boundary.

## Comments

### Three of the four seeds are not leaves — the state manager is why

The ticket counts four modules with no internal dependencies. There is **one**.

`binding.rs`, `helpers.rs` and `cache.rs` each name `StateManager`, which the
spec pins to the transform ("Everything else stays: the state manager and the
remaining structures"). Each also reaches `deopt`, `evaluate_cached` or
`_evaluate`, all of which ticket 08 moves. Moving any of the three would either
drag the whole subtree along or point the new crate back at the transform, which
Cargo rejects.

**Why the count was wrong, and it is the same reason as in ticket 01.** All four
modules open with `use super::*;`. A scan of `use` lines sees no intra-subtree
import in any of them, because the glob is the import. The edges are in the
bodies, not in the header — `StateManager` in a signature, a bare
`evaluate_cached(...)` in a call. A measuring script that reads `use` lines
reports these four as leaves and is wrong three times out of four.

The growable stack is the genuine leaf. It names `stylex_structures` and
`swc_core` and nothing else, and every consumer of it is a caller rather than a
dependency it holds.

### The moved tests needed scaffolding that cannot move yet

`growable_stack_tests.rs` reads five helpers off `tests/source_evaluation.rs`: a
parser, a thread of a stated size, the two thread sizes and the nested literal.
None is about evaluation, but `source_evaluation.rs` as a whole is — it builds a
`StateManager` — so it stays.

The five are therefore carried in the new crate's own
`src/tests/scaffolding.rs`, and the transform keeps its copy for
`short_circuited_walk_tests`, `engine_fold_tests` and `applied_global_tests`,
which still need it. The two copies converge at ticket 09, when those suites
join the crate. The alternative — publishing test support across the crate
boundary behind a cargo feature — was rejected: no crate in this workspace
declares a feature today, and ticket 09 deletes the need for it.

### Ticket 08 is not achievable as written

Recorded here because this ticket is the tracer bullet for it, and the finding
above is what exposes it.

Every unit ticket 08 moves — the dispatcher, all node handlers, the engine fold,
deopt, the declaration check, the stylex functions — takes `&mut StateManager`.
`StateManager` is 3,019 lines in `crates/stylex-transform/src/shared/structures/`,
and the spec keeps it there and undecomposed on parity grounds. A directory move
therefore makes `stylex-evaluator` depend on `stylex-transform`: a cycle.

Both escapes are closed by the spec itself. Trait injection is refused —
"No trait or callback indirection was introduced on the evaluation path" —
because it lands on the compiler's hottest path. Decomposing the state manager
is refused on parity: "It stays one struct with its method surface unchanged,
because it corresponds to a single unit on the comparison side."

What the evaluate subtree names from the transform, in full:

| Item | Where it lives | Lines |
| --- | --- | --- |
| `StateManager`, `add_import_expression` | `structures/state_manager.rs` | 3019 |
| `EvaluationState` | `structures/state.rs` | 33 |
| `FunctionConfig`, `FunctionConfigType`, `FunctionMap`, `FunctionType` | `structures/functions.rs` | |
| `SeenValue` | `structures/seen_value.rs` | |
| `ThemeRef` | `structures/theme_ref.rs` | |
| `EvaluateResult` | `structures/evaluate_result.rs` | |
| `EvaluateResultValue` | `enums/data_structures/` | |
| `FunctionMapIdentifiers`, `FunctionMapMemberExpression` | `structures/types.rs` | |
| `convert_expr_to_str`, `expr_to_num` | `utils/ast/convertors.rs` | |
| six `utils/common` helpers | `utils/common.rs` | |

Everything below the first row is a small value type that could sink into
`stylex-structures` mechanically. The state manager is the one that cannot, and
it is the whole blocker. Ticket 08 needs re-scoping by a human before an agent
picks it up: either the state manager moves down with the evaluator, which keeps
it one struct with an unchanged surface but contradicts "everything else stays",
or the parity constraint on it is revisited. Neither is an agent's call.

### Benches: the pre-split baseline was destroyed mid-run, and the A/B is better

All seven targets ran against `pre-split` first: 35 measurements up, 17 down, 9
flat. Transform-level groups rose as a block -- `TransformDebugPath` +5.56% mean,
worst single `TransformDebugNamespacesPerCall/dev/32` at **+10.96%** -- while
evaluation groups fell as a block, `EvaluatePerfFixtures` -3.77% mean. A
systematic split like that is not thermal noise, and +10.96% is well outside the
+4.73% envelope ticket 05 recorded, so it was not accepted as layout on the
signature alone.

**Then `target/` was cleaned by something outside this work, and the `pre-split`
criterion baselines went with it.** The medians survive in
[`../baseline/bench-summary.txt`](../baseline/bench-summary.txt), but criterion
cannot import medians: the named baseline has to be re-measured at `e8887ab8f`
before any later ticket can diff against it again. **This is the one piece of
shared state this ticket leaves broken, and it blocks the bench criterion of
tickets 08 to 10.**

What replaced it is a stricter test. `HEAD` was checked out detached into a
second worktree sharing this one's target directory, benched with
`--save-baseline head-attrib`, and this change was then diffed against that --
same machine, same session, the two legs differing by this commit and nothing
else. It removes the drift of the four commits between `e8887ab8f` and `HEAD`,
which the pre-split diff could not separate out.

Three targets, 29 measurements. Everything shrank:

| Group | vs `pre-split` | vs `HEAD` (A/B) |
| --- | --- | --- |
| `TransformDebugPath` | +5.56% | **+0.37%** |
| `TransformDebugNamespacesPerCall` | +4.03% | +1.31% |
| `TransformConsumers` | +3.91% | +2.02% |
| `StructuralKey` | +3.10% | +1.93% |
| `SeenModuleSource` | +1.94% | +1.62% |
| `FullPipeline` | +2.67% | **-0.19%** |

Worst single delta fell from +10.96% to +4.30%. Most of the pre-split reading
was drift from earlier commits.

**The residual is layout, and one measurement in the A/B itself proves it.**
`StructuralKey/call/shallow` benchmarks `stable_hash_unspanned_call`, which lives
in `crates/stylex-utils/src/hash.rs`. That directory is byte-identical between
the two legs -- `diff -r` reports no difference -- and `stylex-utils` does not
depend on the new crate. It moved **+3.65%**, from about 58 ns to 60.4 ns. Two
nanoseconds. No line this change wrote, moved or dispatched can execute there, so
that delta is alignment and nothing else, and it sets the layout floor for the
run at roughly +4%. Every other measurement sits at or under that floor, the
worst being +4.30%.

> **[Ticket 16](../bench/ticket-16.md) re-read this floor and it is not a layout
> floor -- it is the measurement's own spread.** A control that rebuilds the
> baseline commit and measures it against its own saved baseline reads −2.0% to
> −3.0% on the median and up to +12.85% on one measurement, with nothing
> changed at all. So +3.65% on `StructuralKey/call/shallow` says only that one
> build per leg cannot resolve a few points. The conclusion below -- no
> regression -- stands; the number under it does not carry the meaning it was
> given, and it is not a ceiling later tickets can measure against.
> Ticket 16 also checked whether the `cdylib` explained the floor. It does not.

The mechanism is the one ticket 05 recorded: the bench links one rlib more than
the baseline binary did, and these targets build `-C lto -C codegen-units=1`, so
the whole program is a single codegen unit whose function placement shifts when
its membership changes. Confirmed here by inspection as well as by the control:
`git diff` over `crates/stylex-transform/src/**/*.rs` leaves only import lines
and one doc-link once the moved file is discounted, so **no executable line of
the transform changed**. There is no new work on any measured path for a
regression to consist of.

Verdict: no regression outside noise.
