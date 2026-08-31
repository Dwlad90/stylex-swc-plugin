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

**Status:** ready-for-agent

- [ ] `common.rs` no longer exists under that name, or holds one concern.
- [ ] The stateless helpers sit below `stylex-state`, not in it.
- [ ] Each of the three uncalled functions is deleted or has a recorded reason
      to stay.
- [ ] Every crate that reads a moved helper reaches it downward; no facade.
- [ ] The full workspace suite stays green and coverage does not regress.
