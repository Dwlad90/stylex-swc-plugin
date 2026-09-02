# 14 — Decompose `common.rs`

**What to build:** `crates/stylex-state/src/common.rs` is a grab-bag. Nineteen
functions across four unrelated concerns share one module and one name that
describes none of them. It arrived in `stylex-state` whole because
[ticket 08](./08-move-evaluator-core.md) moved it mechanically, and
[ticket 12](./12-extract-declarations-crate.md) takes the first
cut by lifting out the four declaration lookups.

Finish the job for what is left.

| Concern | Functions |
| --- | --- |
| State writers | `fill_top_level_expressions`, `fill_state_declarations`, `record_top_level_declarator`, `downcast_style_options_to_state_manager` |
| Stateless AST / object helpers | `type_of`, `prop_key`, `array_index_of`, `order_own_keys`, `remove_duplicates`, `assign_props`, `get_css_value`, `get_variable_names` |
| Serialization | `gen_file_based_identifier`, `serialize_value_to_json_string`, `js_object_to_json` |
| Declaration lookup | *taken by ticket 12* |

The stateless group names no `StateManager` at all, and several of them are read
by the evaluator, so they can sink below `stylex-state` entirely rather than
being reached upward through it. That is the change worth making; the writers
stay with the state they write.

**Three functions have no caller anywhere:** `array_index_of`,
`get_variable_names` and `record_top_level_declarator`. They are `pub`, so no
dead-code warning fires and nothing forces the question. Decide each on its
merits -- deleting is likely right, but confirm against the reference
implementation first in case one is a parity hook nothing has needed yet.

**Blocked by:** 12 — Extract the declarations crate.

**Status:** resolved

- [x] `common.rs` no longer exists under that name, or holds one concern.
- [x] The stateless helpers sit below `stylex-state`, not in it.
- [x] Each of the three uncalled functions is deleted or has a recorded reason
      to stay.
- [x] Every crate that reads a moved helper reaches it downward; no facade.
- [x] The full workspace suite stays green and coverage does not regress.

## What landed

`crates/stylex-state/src/common.rs` no longer exists. Its nineteen functions
went to four homes, each one a layer that already owned the concern:

| Went to | Layer | What |
| --- | --- | --- |
| `stylex-utils/src/identifier.rs` | 0 | `gen_file_based_identifier` |
| `stylex-utils/src/types.rs` | 0 | `type_of` |
| `stylex-ast/src/ast/objects.rs` | 3 | `prop_key`, `array_index_of`, `order_own_keys`, `remove_duplicates`, `assign_props` |
| `stylex-structures/src/base_css_type.rs` | 4 | `get_css_value` |
| `stylex-types/src/serialization.rs` | 5 | `serialize_value_to_json_string`, `js_object_to_json` |
| `stylex-state/src/state_writers.rs` | 9 | `fill_top_level_expressions`, `record_top_level_declarator`, `fill_state_declarations` |
| `stylex-state/src/state_manager.rs` | 9 | `downcast_style_options_to_state_manager` |

Every reader reaches its helper downward. No `pub use` facade was left behind.

**Why the serialization pair did not go to `stylex-utils`.** It needs
`stylex_panic!` and `JSON_REGEX`, and `guidelines/STRUCTURE.md` puts
`stylex-utils` at layer 0, "no internal dependencies". `stylex-types` is the
first layer above that already carries the serde stack and whose whole subject
is compiler output, so the pair went there. `gen_file_based_identifier` needs
nothing and stayed at layer 0, beside `create_key_hash`, which is its hashed
sibling.

**The three uncalled functions.** The ticket's premise was stale by the time it
ran. Only `get_variable_names` had no caller anywhere; it is **deleted**.
`array_index_of` is private and called by `order_own_keys`, and
`record_top_level_declarator` is private and called by
`fill_top_level_expressions` -- both were made private by earlier tickets. Both
**stay**, and neither is `pub`, so no dead-code question remains open.

**A parity bug the move exposed.** `array_index_of` read `+0` as array index 0,
because Rust's `u32::from_str` accepts a leading sign where JavaScript's
canonical-index rule does not. `{'+0': 1, '1': 2}` therefore enumerated `+0`
first, where Node answers `["1", "+0"]`. A digit-only guard now settles it, and
the guard also short-circuits every ordinary property name at its first byte.
Verified against Node for `+0`, `00`, `01`, `4294967294`, `4294967295` and the
empty key.

**Serialization rewritten around a dead arm.** The quote-sniffing guard
(`starts_with('"') && ends_with('"') && len > 2`) made the `from_str` failure
arm unreachable, and that arm is an uncovered region in a gated crate. Asking
`serde_json::from_str::<String>` directly answers "is this a JSON string" and
makes the other arm the live non-string path. Behaviour is unchanged, checked
by a throwaway differential test over 36 input shapes, of which the surviving
`shape_table_tests` keeps the useful ones. The body also moved out of the
generic into `render_json`: coverage counts a generic function once per type it
is called with, so one refusing type in a test left every other branch
unmeasured.

**Tests.** All 111 in `common_tests.rs` travelled to the crate that now owns
the code they exercise, and five convertor suites that had been sitting in the
state crate's test file went with them to `stylex-ast`. The duplicated
`make_*` scaffolding was hoisted -- eight helpers had been declared four times
between them -- and the `_extra_tests` / `_edge_tests` module names, which said
where a test was appended rather than what it covered, were folded into one
module per function. 24 tests were written to close the gaps the boundary
exposed: `order_own_keys` had no direct test at all, `get_css_value` had no
case for a `syntax` key with no `value` beside it, and the serializer had no
case for a value that refuses to serialize.

**Coverage.** The workspace gate reports **100%** of regions, lines and
functions with no new exemption. `stylex-state` stays on the exclusion list;
[ticket 11](./11-cover-the-state-crate.md) owns that.

**Orphaned dependencies.** `stylex-state` no longer reads `stylex_regex` at
all, and reads `serde_json` only from a test. The first is removed from both
manifests; the second moved to `[dev-dependencies]`, keeping its
`preserve_order` feature so the unified feature set is unchanged.

## Left for later, deliberately

- `get_css_value` moved verbatim. It clones `obj.props` and repeats the
  spread-refusal and shorthand-expansion read inside its own `find` closure,
  which is quadratic in the property count. The object it reads is a
  `{ syntax, value }` pair, so the cost is not measurable; reshaping it is a
  change to the function rather than to where it lives.
- `remove_quotes` only ever borrows, so `into_owned()` at the end of
  `render_json` always copies. One copy per `defineConsts` value is not a
  bottleneck, and avoiding it costs more clarity than it buys.

