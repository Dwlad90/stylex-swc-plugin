# 26 — Clear the documentation, layering and bookkeeping drift

**What to build:** The crate graph this work produced is genuinely acyclic and
strictly downward, but the documented layer ladder does not describe it: five
crates are documented far above their real depth, one of them seven rungs
high, so a reader concludes that CSS generation may call the evaluator. The
ladder should be generated from the manifests rather than drawn by hand, which
was the stated intent of the commit that deleted the previous hand-drawn
version.

Alongside that, this branch took several deviations from its own spec that are
each harmless and none of which is recorded. Two commits landed performance
optimisations the spec excludes from scope. A test file lost five helper
definitions to a new scaffolding module, where the spec permits editing only
import lines. Thirty snapshot headers were regenerated for staleness that
predates this work. Two functions were split during move commits, where the
spec forbids splitting — one of them specifically so the coverage tool could
instrument the halves separately, which is worth settling as a question in its
own right, since it happened in the same branch that exempted two crates from
that tool. A relocated macro stamps the caller's file and line into panic
messages, so every moved call site now reports a different location on
stderr — unavoidable, but an observable output change that no test asserts.

The spec's boundary criterion claims nothing with a counterpart in the
reference implementation was cut. Two readers were in fact split across
several crates. Neither has a counterpart, so no translated unit was severed
and no behaviour diverged — but line-for-line comparison against the reference
evaluator now spans four crates, and the next parity investigation should be
told that before it starts looking. (This paragraph said three when the ticket
was filed. The fourth is `stylex-diagnostics`, corrected below.)

Finally a handful of naming and comment drift, and one decision to record:
this branch made the only real test removal in 408 files, deleting a dead
path-resolution helper along with its eight tests and a workspace dependency.
The code was genuinely dead and the deletion is defensible; what is missing is
someone saying so on purpose.

**Blocked by:** 21

**Status:** resolved

- [x] The layer ladder matches the manifests, and a test holds it there. Every
      rung is the longest path measured off a crate's `[dependencies]` table,
      so no crate sits above its real depth and `stylex-css` no longer reads as
      being able to call the evaluator. The ladder is not _generated_ -- a
      generator script and its suite were written and removed at the author's
      request -- so instead `the_documented_ladder_matches_the_manifests` in
      `crates/stylex-rs-compiler/src/tests/crate_layers_tests.rs` reads the
      manifests and fails when the document drifts from them. It lives with the
      addon because the ladder is defined as what the addon links, and it runs
      under `cargo nextest run --workspace --all-features` with no new script,
      npm task or hook
- [x] Amendments record the two performance commits, the test-helper move, and
      the thirty refreshed snapshot headers
- [x] The two functions split during move commits are recorded:
      `build_code_frame_error` and `parse_and_normalize_program` each lost
      their error arm to a named reporter, and both are behaviour-neutral
- [x] Settled: a coverage tool does not decide a function boundary. A split
      needs a name and a contract of its own; a red region is answered with a
      test, an argument, or a recorded exemption
- [x] Recorded in the evaluator's glossary, which is where a parity
      investigation starts, and mirrored as a spec amendment. Four crates and
      not three -- `stylex-diagnostics` is the fourth, because the reference
      deopts on `binding.path`, so a refusal names the declaration's position
      and not the read's
- [x] Written up in the spec under **For the pull request description**. No
      pull request exists for this branch yet, so that section is the record;
      whoever opens the pull request copies the paragraph across. Opening it is
      not this ticket's work
- [x] `crates/stylex-state/docs/adr/0001` states it, names the alias, and says
      what would unlock a split
- [x] All three fixed: the transform now describes what it does in both
      manifests, the `as_string_key` example handles `None` instead of
      asserting it away, and `stylex-nested-config` groups its dependencies
      the way the other five crates do
- [x] `stylex_utils::types::type_of` is deleted, so the one `type_of` left is
      the evaluator's, which answers the language's `typeof`. The comparison it
      powered now reads the `PreRules` enum, so `&dyn PreRule` is gone from the
      workspace, and twenty new cases assert that the three `equals`
      implementations answer what the reference implementation answers
- [x] Restored on `import_specifier_declaring`, which is where the module-level
      scan reads an import. The comment travelled off the three nested import
      visitors that the move deleted as redundant, and this function -- which
      already existed -- is the only reader left
- [x] Recorded as deliberate dead-code removal, under **The one test removal**
      in the spec: both production copies, the eight tests and the
      `node-resolve` dependency went together, and `stylex-path-resolver`
      already answers the question
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code

## Comments

**Where each record landed.** Twelve criteria, four artefacts:

| Record                                       | Where                                                                               |
| -------------------------------------------- | ----------------------------------------------------------------------------------- |
| Layer ladder, corrected and guarded          | `guidelines/STRUCTURE.md`, checked by `crate_layers_tests.rs`                       |
| Perf commits, test helpers, snapshot headers | `spec.md`, three `> **Amended.**` blocks                                            |
| The two function splits                      | `spec.md`, under **New crates and their responsibilities**                          |
| Coverage may not decide a boundary           | `docs/adr/0003-a-coverage-tool-does-not-decide-a-function-boundary.md`              |
| Parity comparison spans four crates          | `crates/stylex-evaluator/CONTEXT.md`, **Comparing against the reference evaluator** |
| Panic location shift                         | `spec.md`, **For the pull request description**                                     |
| The state crate stays whole                  | `crates/stylex-state/docs/adr/0001-…`                                               |
| The one test removal                         | `spec.md`, **The one test removal**                                                 |

**Two findings that needed correcting as they were written.**

The ticket said the parity comparison spans three crates. It spans four:
`stylex-evaluator`, `stylex-declarations` and `stylex-state` hold the
evaluation, and `stylex-diagnostics` holds where a refusal is reported, since
the reference deopts on `binding.path`. The glossary entry names all four, and
the paragraph at the top of this file is corrected to four so a reader does
not carry the wrong number away from it.

The ticket reads the two function splits as one of them being made "so the
coverage tool could instrument the halves separately". Both are the error arm
of a function, lifted into a named reporter that chooses between a debug and a
warning message: `build_code_frame_error` → `warn_no_code_frame`, and
`parse_and_normalize_program` → `warn_unparseable`. Neither can be told apart
from a split made for testability by reading the code, which is why the
question is settled as a rule rather than as a verdict on these two.

**One code correction beyond the naming drift.** The type comparison
`type_of(other) == type_of(self)` did not merely use the wrong mechanism; it
could never answer `true`. One side named `&dyn PreRule` and the other named
the concrete struct, so `StylesPreRule::equals` was constant `false`.

`equals` now takes the `PreRules` enum rather than a trait object, which is
the real type the criterion asks for and removes `&dyn PreRule` from the
workspace. The field comparison had to come with the kind test: a kind test
alone would turn a constant `false` into a wrong `true` for any two styles
rules. `NullPreRule::equals` answered `false` for another null rule and
`PreRuleSet::equals` answered `true` for anything; all three now answer what
the reference implementation answers, and twenty cases assert it — including
that a member's `var(--…)` key is ignored, that a set of twenty thousand rules
compares to the last member, and that no pair of unlike kinds is equal. The
method still has no production caller, so nothing observable changes.

**Two corrections after review.** Both were raised when the work was handed
over, and both are now in the files rather than in the handover note.

The parity comparison spans four crates, not the three the ticket says. The
paragraph at the top of this file is corrected, so a reader does not carry the
wrong number away from it.

The ladder now has a guard. It was left hand-maintained and unchecked when the
generator was removed, which left the next crate added or moved free to
re-introduce the drift silently. A workspace test reads the manifests instead,
so no script, npm task or hook was added, and the check runs in the gate that
already runs. Sixteen further cases cover the two readers behind it -- a
package name that prefixes another, a dev dependency, a target table, a dotted
dependency, a cycle, a chain of five hundred crates, a wrapped list item, and a
document that states no ladder at all. The guard was proven by putting the
`stylex-evaluator` rung back to 11 and watching the test name both ladders.
