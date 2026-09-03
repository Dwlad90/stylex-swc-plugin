# 31 — Settle the dead code the split exposed

**What to build:** A dead-code audit of the whole workspace, run against the
base branch for comparison, found the refactor itself remarkably clean: it
*removed* three dead-code allowances and four unused-import allowances and
added none, and there are no dead modules, no orphan non-test source files, no
dead enum variants and no unused crate dependencies in anything it touched. Out
of 1,863 public definitions cross-referenced against workspace-wide usage, two
items are newly dead and owed by this branch.

**The first is an over-widened visibility.** A declaration-framing helper in
the diagnostics crate was crate-private on the base branch and the split
widened it to public. It has no production caller anywhere in the workspace;
its only callers are two test files, one of them in a different crate. So the
item is public *solely so a cross-crate test can reach it*. The deadness
itself is pre-existing — it had no production caller before either — but the
public visibility is new, and it is the second concrete instance of the
uncalled public function that ticket 12 raised and left open. Note that a
library crate's dead-code lint never fires on a public item, so nothing will
ever warn about this.

**The second is an orphaned script target.** The shared script target that
every crate's `test` script used to invoke is now called by nothing: the base
branch had fourteen manifests pointing at it, and this branch has none, because
they were all rewritten to echo a skip line. Its fate depends on what ticket 21
decides — restoring the test scripts gives it a caller again, while keeping the
skip means it should go.

**Alongside the two, record what is deliberately not being fixed.** The audit
found seventeen pre-existing dead items, and the precedent set by ticket 12 is
that pre-existing deadness is recorded rather than fixed opportunistically.
Recording it is still worth doing, so the next reader does not re-derive the
list. Two parts of that inventory are worth calling out: eight items whose
leading-underscore names hide them from the dead-code lint entirely, which is a
blind spot rather than a decision; and four standing dead-code allowances, none
added by this branch, which the audit has already explained — one is
load-bearing because the only calls to the method it covers form a cycle the
compiler reads as dead, one covers a trait method with three implementations
and no caller at all, and two sit on public items where the lint cannot fire
anyway, making them vestigial no-ops.

**Blocked by:** 21 — the orphaned script target's fate follows from the
test-gate decision. The declaration-framing item overlaps ticket 28's
reshaping of the same crate's public surface, so whichever of the two lands
second should check the other's outcome rather than assume it.

**Status:** resolved

- [ ] The declaration-framing helper is dealt with: either deleted along with
      its two test call sites, or narrowed back to crate-private with the
      other crate's two assertions moved in beside it. **Neither.** Both cost
      more than they save, so it stays public and its doc comment now says
      what it is: the read half of a pair whose write half the evaluator calls
      in production. See the Comments
- [ ] No public item in the six new crates is public solely to satisfy a test
      in another crate. **One is:** `StateManager::for_test`, also widened by
      this branch. It is a test constructor three `stylex-transform` test files
      need, and its name says so, so it is kept and recorded
- [x] The orphaned script target is deleted, or given a caller, matching
      whatever ticket 21 settled. Deleted: ticket 21 kept the skip line
- [x] The pre-existing dead-code inventory is recorded rather than fixed:
      five uncalled public functions and the eight underscore-named items.
      The five match. The underscore-named items measure **twelve**, not eight
- [x] The four standing dead-code allowances carry the audit's answers, so a
      reader can tell the load-bearing one from the two no-ops without
      re-deriving it. Measured rather than read across: **three** are
      load-bearing and two are no-ops, and this branch added the third
- [x] Whether leading-underscore naming should keep hiding items from the
      dead-code lint is settled one way or the other — it is a lint blind spot
      the repo may not have chosen deliberately
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code

## Comments

**Resolved.** Every count below was measured on this branch, not carried over
from the audit. Three of them differ from it.

### The declaration-framing helper

`framed_declaration_of` stays public, and neither of the two options this ticket
named was taken. Both cost more than they save:

- Deleting it deletes real coverage. Its readers are three test files, and two
  of them are in `stylex-evaluator`: `check_declaration_test.rs` and
  `resolution_order.rs` assert *which* binding a refusal frames, measured
  against upstream 0.19.0. Nothing else asserts that.
- Moving those assertions into `stylex-diagnostics` is not possible. They drive
  `check_ident_declaration`, and diagnostics sits below the evaluator in the
  crate graph.

The item is not public solely for a test. It is the read half of a pair whose
write half, `frame_declaration_of`, has a production caller in the evaluator
(`evaluate/deopt.rs:41`). A crate that lets a caller record a fact and never
read it back has half an interface. Its doc comment now says so, and says why
`get_span_from_source_code_impl` inlines the same two steps instead of calling
it: that path needs the expression key for the cache key as well, and hashing a
whole subtree twice was the cost ticket 27 removed.

### The second widened item this ticket did not catch

`StateManager::for_test` was `pub(crate)` on the base branch and is `pub` now.
It has no production caller, and its three readers are all tests in
`stylex-transform`. Unlike the helper above it is genuinely test scaffolding, so
narrowing it would delete a test constructor three files need. It is kept. The
name states what it is for, which is why it needs no further marking.

### The orphaned script target

Deleted. Ticket 21 chose one workspace run over per-crate runs, so the per-crate
`test` scripts keep their skip line, and `scripts/packages/test/index.sh` gains
no caller back. `coverage.sh` and `flamegraph.sh` in the same directory stay:
the crates still call them through `test:coverage` and `test:flamegraph`.

### The dead-code allowances -- measured, not read

The workspace holds fifteen `#[allow(dead_code)]` sites outside tests. Each was
removed and the workspace re-checked, so every verdict below is measured.

The five the audit named come first. Its split of "one load-bearing, one trait
method with no caller, two no-ops" is wrong: **three** are load-bearing and two
are no-ops.

| Site | Verdict |
| --- | --- |
| `pre_rule.rs` `PreRule::get_value` | Load-bearing. Warns without it. |
| `pre_rule.rs` `PreRule::equals` | Load-bearing. Warns without it. |
| `pre_rule.rs` `PreRules::equals` | Load-bearing. Warns without it. |
| `theme_ref.rs` `ThemeRefResult::ToString` | No-op. Public enum variant. |
| `core_stylex_options.rs` `enable_ltr_rtl_comments` | No-op. Public field. |

The ten the audit did not name sit in four crates this branch never opened.
They were measured all the same, because measuring costs nothing and the next
reader should not repeat it. Only one does work:

| Site | Verdict |
| --- | --- |
| `dimension.rs` `is_valid_dimension_unit` | Load-bearing. Warns without it. |
| `media_query.rs`, three sites | No-op. |
| `factories.rs`, four sites | No-op. |
| `legacy_expand_shorthands_order.rs` | No-op. |
| `styleq.rs` | No-op. |

Those ten keep no comment. They belong to crates outside this work, and adding
one to each is a fix rather than the recording this ticket asked for.

The three in `pre_rule.rs` form one cycle. `PreRules::equals` dispatches to
`PreRule::equals`, whose only implementation reached from outside is
`PreRuleSet::equals`, which calls `PreRules::equals` again; `get_value` closes
the same loop through `PreRuleSet::get_value`. No end of it has an outside
caller, so the lint reads all three as dead. Each site now carries its verdict.

`PreRules::equals` is new on this branch, which the audit read as adding no
allowance. It arrived with the change of `equals` from `&dyn PreRule` to
`&PreRules`.

### Two requirements declined, not met

Two boxes above stay unticked. Both are decisions, not oversights.

`framed_declaration_of` and `StateManager::for_test` are each public with no
production reader. Checkbox 2 forbids exactly that, and the diff does not
satisfy it. Deleting either deletes tests that nothing else covers -- upstream
parity on which binding a refusal frames, and a state constructor three
transform test files build on. The cost of obeying the checkbox is higher than
the cost of the two public items, so the checkbox is answered "no" and the
answer is written down here.

Anyone who reopens this should read the two sections above first, not restart
from the audit.

### Pre-existing deadness, recorded and not fixed

Five uncalled public functions, all public and uncalled before this branch:

| Item | Crate |
| --- | --- |
| `is_vars_leaf` | `stylex-nested-config` |
| `convert_ident_to_expr` | `stylex-state` |
| `StyleXError::with_location` | `stylex-macros` |
| `StyleXError::with_key_path` | `stylex-macros` |
| `StyleXError::with_source_location` | `stylex-macros` |

Twelve underscore-named items with no caller, not eight:

| Item | Crate |
| --- | --- |
| `_check_directory` | `stylex-path-resolver` |
| `_get_directories` | `stylex-path-resolver` |
| `_get_directory_path_recursive` | `stylex-path-resolver` |
| `_set` | `stylex-state` |
| `_as_bool` | `stylex-state` |
| `_as_null` | `stylex-state` |
| `_as_computed_styles` | `stylex-transform` |
| `_get_property` | `stylex-transform` |
| `_get_pseudos` | `stylex-transform` |
| `_get_at_rules` | `stylex-transform` |
| `_new` | `stylex-transform` |
| `_as_styles` | `stylex-transform` |

`_check_directory` and `_get_directory_path_recursive` call only each other, so
the pair is dead as a whole. Two further underscore-named items are **live** and
belong on no list: `_evaluate` has a production caller in `evaluate/cache.rs`,
and `_assert_cache_send_sync` is a compile-time assertion a test calls.

### The underscore blind spot

Settled against the convention. The dead-code lint skips any name that starts
with `_`, so the prefix marks an item as deliberate for a reader and hides it
from the compiler at the same time. The second effect is not wanted: an item
that loses its last caller by accident then stays silent.

`guidelines/stack/RUST.md` now has an **Items Kept But Not Called** section that
says an item kept on purpose states so with `#[allow(dead_code)]`, writes down
why, and says whether the attribute does work at all. The twelve items above are
recorded as owed a rename, not renamed here -- renaming them makes the lint fire
across five crates this ticket did not otherwise open, which is a fix rather
than the recording this ticket asked for.

### The rule and this deferral met on two files

The **Items Kept But Not Called** section this ticket added to
`guidelines/stack/RUST.md` says "Rename one when you touch it". The deferral
above says the twelve are recorded rather than renamed. Those two read the same
on ten of the twelve and disagree on two files, which this branch did open:
`pre_rule.rs` and `pre_rule_set.rs`, both edited for the `equals` fix.

Settled in favour of the rule, because it is bounded to the files already open.
Five items renamed, each now carrying `#[allow(dead_code)]` and a note saying
why it is kept and that the attribute does work -- all five are `pub(crate)` in
a library crate, so the lint really does fire without it:

| Was | Now |
| --- | --- |
| `CompiledResult::_as_computed_styles` | `as_computed_styles` |
| `StylesPreRule::_get_property` | `property` |
| `StylesPreRule::_get_pseudos` | `pseudos` |
| `StylesPreRule::_get_at_rules` | `at_rules` |
| `PreRuleSet::_new` | `new` |

**The underscore was load-bearing on two of them,** which is worth knowing
before the remaining seven are renamed. `StylesPreRule` already has private
associated functions named `get_pseudos` and `get_at_rules` -- they take a key
path and select from it. Dropping the underscore alone would have been a
duplicate definition. The field readers are therefore named for the field, not
`get_*`. Whoever takes the other seven should check each for the same clash
rather than assume a mechanical rename compiles.

The remaining seven stay deferred. None of their files -- `js_to_ast.rs`,
`transform/mod.rs`, `tests/utils/transform.rs`, and the four outside
`stylex-transform` -- was opened by this branch, so the rule does not reach
them yet.

### The five renamed items now carry tests

They had none. No production code calls them, so a test is the only guard on
what they answer, and a rename with no guard is a rename nobody can check.
`shared/structures/tests/pre_rule_accessors_test.rs` holds nineteen cases: the
usual key path, an unusual one, and a limit case for each reader -- an empty
property name, a repeated key, a 10,000-segment key path, a 10,000-character
key, a non-ASCII key, and a 5,000-entry style list.

The tests do not change the dead-code verdict. A `#[cfg(test)]` caller is
invisible to the lint in a normal build, so all five attributes still do work.
Measured again after the tests landed: removing them returns the same three
warnings.

**One case failed first, and the failure is worth keeping.** `PreRuleSet::new`
was expected to agree with `PreRuleSet::create(vec![])`. It does not. `create`
collapses an empty list to a `NullPreRule`, so an empty `PreRuleSet` is a value
that no other route in the crate can build. That is the reason the function has
no caller, and the test now states it.

