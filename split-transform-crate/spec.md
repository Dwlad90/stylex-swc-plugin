# Split `stylex-transform` into smaller crates

Status: ready-for-agent

## Problem Statement

`stylex-transform` is the largest crate in the workspace by a wide margin —
558 `.rs` files, ~112k lines in total, ~32k lines of non-test source. It sits at
layer 8 of the documented crate DAG and holds three unrelated concerns at once:
the SWC visitor, the **state manager**, and the JavaScript evaluator. Its own
context glossary says as much, and `CONTEXT-MAP.md` has to carry a disclaimer
pointing readers away from `stylex-evaluator` and back to this crate for the
general evaluator.

Three consequences fall out of that:

1. **Nothing separates unrelated work.** A change to class-name generation lives
   in the same crate as import resolution and the visitors, with only module
   privacy between them — and the module graph does not even hold that line
   today: a util in the style-semantics layer reaches *up* into the visitor
   layer, and a structure and a util depend on each other in a cycle. The
   guidelines require a strict DAG where higher layers depend only on lower
   ones; inside this crate that rule is unenforced and already broken.

2. **The crate has no coverage floor.** The workspace coverage gate runs at zero
   uncovered lines and zero uncovered regions, and it explicitly excludes
   `stylex_transform`. The single biggest body of code in the repo is the only
   one nothing holds to a coverage standard, and it cannot realistically be
   brought under the gate as one 32k-line unit.

3. **Iteration is slow.** Touching one line of the state manager rebuilds all
   ~32k lines of source and the ~80k lines of tests that ride along with it.

Every other large concern in this workspace has already been carved out —
`stylex-css`, `stylex-structures`, `stylex-ast`, `stylex-atoms`. This crate is
the remainder that was never split.

## Solution

Extract the crate's **Rust-only machinery** — the parts with no counterpart in
the JavaScript implementation used for behavioural comparison — into focused
crates that each own exactly one concern and each join the coverage gate at 100%
region coverage. What remains is the visitor, the state manager and the
style-semantics layer: still substantial, but roughly a third smaller and
honestly described by its own name.

Four crates change hands:

- **stylex-state-index** — the candidate-index and key-span-index structures the
  state manager composes to answer "which declarator, which call, which span"
  in one hash probe instead of a scan. Pure lookup machinery.
- **stylex-diagnostics** — building a code frame for an error, and finding the
  declaration span an error should point at.
- **stylex-evaluator** — the general JavaScript evaluator: **confident** results
  and **deopt** expressions, the per-node evaluation, the **engine fold** and its
  **transport**, **applied globals**, the binding lookup and the evaluation cache.
- **stylex-nested-config** — the crate currently *named* `stylex-evaluator`,
  which does nested-config flattening and no evaluation at all, renamed so the
  name goes to the code that earns it and the disclaimer in `CONTEXT-MAP.md` can
  be deleted.

The work is strictly behaviour-preserving. It moves code; it does not change it.
The public entry points the NAPI compiler consumes are untouched, so the
JavaScript test suite that runs against the built `.node` addon acts as an
unchanged control for the whole exercise.

## User Stories

1. As a compiler maintainer, I want the JavaScript evaluator in its own crate, so
   that I can change how an expression folds without rebuilding the visitors.
2. As a compiler maintainer, I want the state manager's lookup machinery behind a
   crate boundary, so that its indices cannot be reached from anywhere that has
   no business touching them.
3. As a compiler maintainer, I want code-frame diagnostics in their own crate, so
   that error presentation evolves independently of what raises the error.
4. As a compiler maintainer, I want each new crate held to zero uncovered lines
   and regions, so that the coverage gate covers a larger share of the compiler
   than it does today.
5. As a compiler maintainer, I want the excluded-from-coverage surface to shrink
   from ~32k lines to ~20k, so that the exclusion is a bounded exception rather
   than a hole.
6. As a contributor, I want a one-line edit to the state manager to rebuild less,
   so that my iteration loop is shorter.
7. As a contributor, I want the crate DAG to actually forbid an upward
   dependency, so that a mistake is a compile error rather than a review catch.
8. As a contributor, I want the two existing illegal edges removed before
   anything moves, so that the module graph is provably acyclic while it is still
   one crate and easy to verify.
9. As a contributor, I want each extraction to be one commit, so that a
   regression bisects to a single move.
10. As a contributor, I want every commit independently green, so that no
    intermediate state is broken.
11. As a reviewer, I want move commits to contain only moves, so that I can
    review them by confirming nothing changed rather than by reading logic.
12. As a reviewer, I want snapshot churn isolated in its own commit, so that a
    path diff never hides a behaviour diff.
13. As a reviewer, I want the crate a change belongs to visible from its path, so
    that I know whether a diff can affect CSS output at all.
14. As a maintainer of behavioural parity, I want no function renamed, split,
    merged or reordered during a move, so that comparison against the reference
    behaviour stays line-for-line possible.
15. As a maintainer of behavioural parity, I want modules that have a counterpart
    to move whole or not at all, so that no boundary falls through the middle of
    a translated unit.
16. As a maintainer of behavioural parity, I want the state manager to remain one
    struct with one method surface, so that it keeps corresponding to a single
    unit on the other side.
17. As a maintainer of behavioural parity, I want restructuring confined to
    Rust-only machinery, so that anything with a counterpart is only relocated.
18. As a performance owner, I want the `Rc`-shared indices to stay `Rc`-shared,
    so that cloning the state manager for a dynamic style's callback does not
    become a deep copy that scales with file size.
19. As a performance owner, I want no dynamic dispatch introduced on the
    evaluation path, so that the fold stays as fast as it is today.
20. As a performance owner, I want the **engine fold** kept in the same crate as
    the evaluator nodes it is mutually recursive with, so that no trait
    indirection is inserted into the hottest path in the compiler.
21. As a performance owner, I want every criterion bench re-run against a
    recorded baseline before each commit lands, so that a slowdown is caught at
    the commit that caused it.
22. As a performance owner, I want the three benches that measure the evaluator
    to move with it and be re-baselined by hand, so that a per-crate baseline
    identity change is not mistaken for a regression.
23. As a release engineer, I want the NAPI compiler's dependency and entry points
    unchanged, so that no consumer-visible surface moves.
24. As a release engineer, I want the built addon rebuilt and the JavaScript
    suite re-run for every commit, so that a green Rust run never stands in for
    an untested addon.
25. As a release engineer, I want `Cargo.lock` regenerated with the commit that
    caused it, so that the lockfile never drifts.
26. As an agent working in this repo, I want every new crate to carry a
    `CONTEXT.md` and an entry in the context map, so that the domain vocabulary
    stays complete and navigable.
27. As an agent working in this repo, I want the layered DAG documentation
    updated with the new crates and their layers, so that the documented
    structure matches the real one.
28. As an agent working in this repo, I want the misleading note steering readers
    from `stylex-evaluator` to `stylex-transform` deleted, so that the crate
    names can be trusted.
29. As an agent working in this repo, I want no artifact to assert a porting or
    mirroring relationship with another implementation, so that the repo's own
    documentation stays self-describing.
30. As an agent working in this repo, I want the extraction order to follow real
    dependencies rather than convenience, so that no commit needs a temporary
    hack to compile.
31. As a test owner, I want a test's assertions never edited to accommodate a
    move, so that the suite's meaning is provably constant across the refactor.
32. As a test owner, I want unit tests to travel with the code they cover, so
    that each new crate can reach the coverage gate on its own.
33. As a test owner, I want the parity harvest chain accounted for, so that
    moving a Rust test file does not silently invalidate a generated fixture in a
    crate nobody touched.
34. As a test owner, I want a full baseline recorded before anything moves, so
    that "did this help" is answered with numbers rather than impressions.

## Implementation Decisions

### Boundary criterion

Boundaries are drawn **layered**, following the pipeline, and never cut through a
module that has a counterpart in the implementation used for behavioural
comparison. Such a module moves as a whole unit or stays put. Rust-only
machinery — code with no counterpart — may be restructured, and that is where
every new crate comes from. This matches how earlier splits were done:
`stylex-css` took whole coherent areas, never a slice through one.

Feature-vertical boundaries (one crate per `stylex.*` API) were rejected: every
API funnels through the same flatten → pre-rule → class-name pipeline, so
vertical crates would either duplicate it or all depend on a shared crate that
*is* the layered cut.

### New crates and their responsibilities

- **stylex-state-index** — the candidate index and the key-span index. The
  candidate index has no internal dependencies at all; the key-span index needs
  only the workspace hashing helper and AST helpers already owned by
  `stylex-ast`. The state manager keeps its fields and composes these types.
- **stylex-diagnostics** — code-frame construction and declaration-span lookup.
- **stylex-evaluator** — the entire evaluation subtree moved as one unit:
  per-node evaluation, binding lookup, deopt, the evaluation cache, the growable
  stack, the **engine fold** with its guard/amplification/**transport**/theme
  parts, and the declaration check. Carries the embedded JS engine dependency.
- **stylex-nested-config** — a pure rename of the crate currently occupying the
  `stylex-evaluator` name.

Everything else stays: the state manager and the remaining structures, the
style-semantics layer, the AST/CSS/common/object utilities, the validators, the
enums, and the whole visitor tree.

### The state manager is not decomposed

It stays one struct with its method surface unchanged, because it corresponds to
a single unit on the comparison side. Only the Rust-only machinery it *composes*
leaves. A four-way split into context/imports/bindings/output was considered and
rejected on parity grounds.

### The engine fold is not extracted separately

The fold and the evaluator's node handlers are mutually recursive — the call and
member handlers call into the fold, the fold imports back out of the evaluator in
several places, and the fold is a private module. Extracting it alone would
require inverting that edge with traits or callbacks on the compiler's hottest
path. The whole evaluation subtree therefore moves as one crate and the cycle
stays internal. Splitting the fold out later is a separate decision judged on its
own merits.

### Diagnostics takes an injected trait

The diagnostics code uses exactly nine state-manager methods, none of which have
a counterpart: filename lookup, seen-module source get/set, cached-span get/set,
key-span-index access, and the three framed-declaration methods. These are
declared as a trait owned by `stylex-diagnostics` and implemented by the state
manager. The trait is used at a diagnostic site, never on the evaluation path, so
the indirection costs nothing.

**Prior art:** `stylex-atoms` already takes its compilation utilities through an
injected `Compile` trait for exactly this reason — to avoid depending on the
transform, which would be a cycle. This follows the established pattern.

### The exported macros must move first

The crate's three exported macros expand to hard-coded paths rooted at the
defining crate, and those paths point at **three different destinations**: one
into diagnostics, one into the evaluator, and one into *both* the AST convertors
(staying) and the evaluator (leaving) within a single expansion.

Because an exported macro publishes at its defining crate's root, leaving them
where they are would force the evaluator crate to depend back on the transform —
a dependency cycle Cargo rejects. The first breakage lands at the diagnostics
extraction, not the evaluator one.

**Decision:** the macros move to `stylex-macros`, layer 1, which the context map
already describes as "the error and panic vocabulary every crate raises failures
through" — the correct owner. A layer-1 crate cannot name types in layers 5–8, so
the functions each macro calls are **passed in as macro parameters** rather than
named in the macro body; macros expand at the call site, so the call site's crate
supplies the paths. This is the same injection principle as the diagnostics
trait. These macros are Rust-only, so parity permits the change. It lands in its
own commit before any crate is created.

### No re-export facades

The guidelines state plainly that each crate owns exactly one concern, with **no
re-export facades**. An earlier draft of this work proposed re-exporting every
moved module from the transform at its old path to avoid touching call sites;
that is rejected as a direct violation of the documented rule. Call sites update
to the new crate paths.

The consequence is that **import paths in test and bench files do change**. The
constraint is narrowed accordingly, and stated precisely below under Testing
Decisions: a test's *assertions and fixtures* are never edited to accommodate a
move; a test's *import lines* may be.

### The documented DAG is renumbered

New crates need layer assignments and the existing numbering shifts. The
state-index crate sits above AST foundations; diagnostics sits above state-index;
the real evaluator sits above diagnostics and below CSS processing and the
transform. The renamed nested-config crate keeps the layer its predecessor had.
The layer list, the context map row, and each new crate's `CONTEXT.md` are
updated as part of the commit that creates the crate — not as a follow-up.

### Manifest conventions

New crates match the existing ones exactly: workspace-inherited edition, licence,
repository, rust-version and version; doctests off; workspace lints; a
`description` field; Taplo-formatted. No `publish` key — no crate in this
workspace sets one. The workspace members glob already matches the new
directories, so the root manifest needs no edit.

### Sequencing

Three preparation commits, then one commit per crate:

1. **Cut the two illegal edges.** The util that reaches up into the visitor layer
   takes the hoisting function as a parameter instead of importing it; the
   structure/util cycle is resolved by moving the shared helper to sit with its
   caller. Done while still one crate, so it is provable by inspection against an
   unchanged suite.
2. **Capture the baseline** (see Testing Decisions).
3. **Relocate the exported macros.**
4. `stylex-state-index`.
5. `stylex-diagnostics`.
6. `stylex-nested-config` rename.
7. `stylex-evaluator`.
8. Snapshot regeneration, if any, mechanical and alone.

The evaluator goes last because it depends on both state-index and diagnostics.

## Testing Decisions

### What makes a good test here

This is a refactor with zero intended behaviour change, so the tests are not
being *written* — they are the **invariant**. A good test in this work is one
whose assertions and fixtures are byte-identical before and after, and which
still exercises the same code through the same entry point. Tests assert on
transform output — emitted JavaScript, injected CSS, metadata — never on which
crate a function now lives in. No test may be weakened, skipped or re-baselined
to make a move compile.

### Seams

The guiding rule is to prefer existing seams and use the highest one available.
**This work introduces no new seams.** Three already exist and all three are
preserved:

1. **The built NAPI addon, exercised by the JavaScript suite** — the highest seam
   in the repo, and the master control. The entry points the compiler consumes
   are frozen, so this seam is invariant by construction: if it stays green
   across every commit, no observable behaviour moved. The addon must be rebuilt
   before each run, because the JavaScript suite exercises the built artifact and
   not the Rust sources.
2. **The crate's integration test tree with its snapshots** — whole-module
   transform assertions. Unchanged in content. At risk only of snapshot *path*
   churn, which is quarantined to its own commit.
3. **Per-module unit tests living beside their code** — these travel with the
   module they cover, which is what lets each new crate reach the coverage gate
   independently. The key-span-index tests move with the index; the diagnostics
   tests move with diagnostics; the evaluation tests move with the evaluator.

The only permitted edit to any test file is its **import lines**, forced by the
no-re-export-facades rule. Assertions, inputs and fixtures are untouched.

### Modules under test

Each new crate is tested by the unit tests that move with it, and must reach zero
uncovered lines and zero uncovered regions. This is not optional or deferrable:
the coverage gate runs across the whole workspace, so **a new crate is gated the
moment it exists**. Each extraction commit either lands at full coverage or ships
a temporary exclusion that its own immediate follow-up removes. The gate ignores
test, bench and example directories, so only library source counts.

Note that coverage tooling keeps only the best-covered instantiation of a
generic, so a generic helper can read as fully covered while one instantiation is
untested — worth remembering for the index crate, which is generic.

### Prior art

- The existing per-module unit tests inside the crate are the model for what
  moves with each extraction.
- The snapshot-based integration tests are the model for the untouched
  whole-transform assertions.
- `stylex-css` and `stylex-structures` are the model for what an extracted crate
  looks like once gated.

### Baseline

Recorded before any code moves, at a named commit, and kept alongside this spec:

- The full suite green — run directly, never piped into a pager or `tail`, since
  the exit code would then be the pager's.
- All seven criterion benches recorded. The three that measure the evaluator move
  with it in the final commit; criterion baseline identities are per-crate, so
  those three need a manual same-machine before/after rather than an automatic
  diff. The other four diff normally. Criterion benches are not CI-gated, so this
  is a local step.
- Coverage output saved, including the current exclusion list.
- Cold build and incremental check-after-touching-the-state-manager, timed.

### Verification per commit

Workspace build and test in **debug** — the fixture suite only guards the debug
profile, and fixtures that pass in debug can fail in release, so no `--release`.
Then typecheck, format check, lint check and the full suite; rebuild the addon
and re-run the JavaScript suite; re-run the benches against the baseline;
re-run coverage with the new crate included; confirm the compiler's dependency
and entry points are unchanged; regenerate the lockfile.

Two known traps: the pre-commit hook rewrites code, so typecheck must be re-run
*after* committing rather than only before; and one bench plus the performance
fixture test are wall-clock flaky, so a lone failure is re-run before being
believed.

### The harvest chain

Rust test sources feed a harvested parity corpus in the NAPI compiler package,
which in turn generates a committed fixture in `postcss-value-parser`. The
compiler package's pre-test step checks that fixture. Moving or editing a Rust
test file therefore invalidates a generated fixture in a crate this work does not
otherwise touch — expect it whenever test files move, and regenerate rather than
hand-edit.

## Out of Scope

- **Extracting the style-semantics layer** (the transformers and the core style
  utilities, ~5.6k lines). Both reference the state manager across many files;
  extracting them now would make it a public cross-crate type and leak its
  crate-private fields. Revisit only after establishing whether that surface
  narrows to options and context — if it does, a further crate becomes viable.
- **Splitting the engine fold out of the evaluator.** A separate decision, on its
  own merits, after the evaluator crate exists.
- **Decomposing the state manager into several structs.** Ruled out by parity.
- **Removing the transform's coverage exclusion entirely.** The visitor and
  orchestration layer is where coverage is genuinely hard; the goal is shrinking
  the excluded surface, not eliminating it.
- **Any behaviour change, bug fix, performance optimisation or idiomatic cleanup.**
  If a defect is noticed during a move, it is recorded and fixed separately.
- **Publishing the new crates anywhere.** Nothing in this workspace is published
  to a Rust registry.

## Further Notes

- The branch for this work already exists.
- The crate currently named `stylex-evaluator` was never a stalled extraction —
  it was created whole for the nested-API work and has only ever held
  nested-config flattening. Its name is simply wrong, and the context map carries
  an explicit note redirecting readers because of it. The rename deletes the need
  for that note.
- Two things are already broken today and are fixed by the first preparation
  commit regardless of whether the rest proceeds: the upward edge from the style
  utilities into the visitor layer, and the structure/util cycle. Both violate
  the documented DAG rule inside a crate boundary that cannot enforce it.
- The single largest risk in this work is the macro relocation, because it is the
  one place where code shape genuinely changes rather than merely moving. It is
  isolated in its own commit for that reason, and it lands before any crate is
  created so that a failure there stops the work early and cheaply.
- Expected end state: the transform crate drops from ~32k to ~20k lines of
  source; three new gated crates hold ~13.8k lines at full region coverage; the
  workspace's uncovered-by-policy surface shrinks by roughly a third.
