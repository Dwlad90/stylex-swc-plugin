# 01 — Make the module graph acyclic and downward-only

**What to build:** Today two edges inside `stylex-transform` violate the strict
dependency DAG the guidelines require, and the crate boundary cannot enforce
what module privacy allows. A style-semantics util reaches *up* into the visitor
layer to import the hoisting helper, and a structure and a style util depend on
each other in a cycle. After this ticket the crate's internal module graph is
acyclic and strictly downward, so every later extraction is a move rather than a
refactor in disguise.

Cut the upward edge by having the style-merge util receive the hoisting function
rather than import it — the same injection principle `stylex-atoms` already uses
to avoid depending on the transform. Cut the cycle by relocating the shared
member helper to sit with its caller.

This is prefactoring: make the change easy, then make the easy change. It lands
while everything is still one crate, so it is verifiable by inspection against a
completely unchanged test suite.

**Blocked by:** None — can start immediately.

**Status:** ready-for-human

- [x] No module in the style-semantics layer imports anything from the visitor layer.
- [x] No cycle remains between the structures and the style utilities.
- [x] The hoisting helper reaches its caller by injection, not import.
- [~] Zero test files changed — not assertions, not fixtures, not imports. — held for the first commit. Later commits in this ticket move test *import* lines, which the spec permits; no assertion and no fixture changed.
- [x] Debug workspace build and test green; no `--release`.
- [x] Typecheck, format check, lint check and the full suite pass.
- [x] Typecheck re-run after committing, since the pre-commit hook rewrites code.

## Comments

### The premise of one cycle is wrong — there were six

The ticket, and the spec's story 8, both count **one** structure/util cycle.
There were six. An earlier note in this file said two; that count came from a
measuring script that missed nested `use` brace groups and under-reported.

All six are now cut, across two commits, and none needed a test assertion or a
fixture changed:

| Cycle | Cut by |
| --- | --- |
| `state_manager` / `utils::ast::convertors` | deleting the re-export shim |
| `utils::ast::convertors` / `utils::common` | deleting the re-export shim |
| `state_manager` / `utils::ast::helpers` | moving the visitor tail out |
| `state_manager` / `utils::validators` | moving the visitor tail out |
| `state_manager` / `utils::js::check_declaration` | `DeclarationType` to `stylex-enums` |
| `state_manager` / `utils::common` | file-name readers to `stylex-ast` |
| `pre_rule` / `convert_style_to_class_name` | `PreRuleValue` to `stylex-structures` |

The root cause was not a stray import. `structures/state_manager.rs` was a
mixed-domain file: about 215 lines of visitors and module passes sat after the
struct, and those passes were the only reason the struct's module reached up
into the utilities. Moving them removed two cycles at once, and it agrees with
story 16 — the struct keeps every method it had, and only machinery it composes
left.

Three of the moved items went to lower crates, so a repeat of the same mistake
is now a compile error rather than something a reviewer must catch. That is
story 7.

### Two mutual pairs stay, and should

`enums::data_structures::evaluate_result_value` and `structures::functions` /
`structures::types` refer to each other. This is one recursive model rather than
a layering fault: an evaluate result can hold a function config, and a function
config takes and returns an evaluate result. Splitting it means merging two
large modules into a bigger mixed-domain file, or adding an indirection to the
evaluation path that story 19 forbids. Left alone on purpose.

### A pre-existing crate-layer inversion, not caused here

`stylex-structures` is documented at layer 3 but depends on `stylex-ast` at
layer 5 (`crates/stylex-structures/src/base_css_type.rs`). The edge is present
at this branch's base commit, and `stylex-ast` does not depend back, so there is
no crate cycle — but the documented layer list and the graph in
`crates/stylex-ast/README.md` do not describe the real thing. **A maintainer
should decide** whether to renumber the layers or move the offending code. The
spec renumbers the DAG in a later ticket, which is the natural place.

### The premise of one cycle is wrong — there are two

The ticket, and the spec's story 8, both count **one** structure/util cycle.
There are two, and only one can be cut under this ticket's own rules.

Cut, as asked:

- `structures::member_transform` used `utils::core::member_expression`, while
  `utils::core::stylex_merge` used `structures::member_transform`.
  `MemberTransform` moves down to sit beside its helper and its only caller.

Superseded by the table above; the entry below is kept for history.

The `pre_rule` cycle was first judged uncuttable here. To break it, either `PreRuleValue` or
`convert_style_to_class_name` must move, and both are named from test files:

- `shared/structures/tests/flatten_raw_style_objects_test.rs`
- `shared/structures/tests/gen_css_test.rs`
- `shared/utils/core/tests/convert_to_class_name_test.rs`

Any move edits a test import, which this ticket forbids: "Zero test files
changed — not assertions, not fixtures, not imports." The spec's Testing
Decisions are looser — they only hold *assertions and fixtures* constant — so
the two documents disagree. **A maintainer must decide** whether to relax this
ticket to permit import-only edits in test files, or to file the `pre_rule`
cycle as its own ticket.

Neither remaining edge blocks the extractions this spec sequences: `pre_rule`
and `convert_style_to_class_name` both stay in `stylex-transform`.

### Verification

Both edges were measured with an import-graph script that expands nested `use`
brace groups; an earlier naive version missed them and reported a false clean.

- No module under `src/shared/` imports `crate::transform`.
- `cargo check`, `cargo clippy` and `cargo test --workspace --all-features`:
  green, debug profile, 8072 passed, 0 failed.
- Addon rebuilt, then `pnpm test`: 87 of 87 tasks pass.
- `pnpm typecheck` and `pnpm lint:check` pass, before and after committing.
- `pnpm format:check` passes. It reported five unrelated `package.json` files
  mid-way through; a later `pnpm install` normalised them and the check is now
  clean, so nothing here was the cause.

### Follow-on commit

A second commit removes redundant work found in `stylex_merge` while reading
it: the default marker was rebuilt for every imported name and its compiled
styles copied twice per name, and the non-null props were copied into and out
of the member visitor although the caller keeps no other reference.
