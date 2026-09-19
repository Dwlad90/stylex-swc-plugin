# 90 — Stop the copies that go straight to waste

**What to fix:** two copies made of values that are read no further.

**A fallback list copied before it is dropped.**
`flatten_raw_style_object.rs` owned the deduplicated `values` for one turn of
the loop and never read it after the branch, yet both arms copied it --
`values[0].clone()` and `values.clone()`. For a fallback list that is every
string of the declaration, copied for nothing.

Answered by taking the value out of the set rather than copying it out:
`swap_remove(0)` on the one-value arm and the set itself on the other. Read as a
`match` on the length, which says the two arms answer the same question.

**Every property of every namespace deep-copied to expand a shorthand.**
`evaluate_stylex_create_arg.rs` copied each property -- for a `create`
namespace, the whole style object -- so that `expand_shorthand_prop` could
rewrite a `Prop::Shorthand` into a `Prop::KeyValue`. Only a shorthand needs the
copy, and only a shorthand holds nothing but a name.

Answered with `expanded_shorthand_prop` in `stylex-ast`, which answers a
`Cow<'_, Prop>`: owned for a shorthand, borrowed for everything else.
`expand_shorthand_prop` now calls it and writes back only an owned answer, so
the rule lives in one place. Both reader sites take the borrowed form; neither
ever wrote to the property it had copied.

**The same copy at the three sites the review did not name.** A performance
review of the change found that the helper had been applied at two of the five
read-only sites. The other three take the property read-only as well, so each
is the same one-line swap:

- `stylex-ast/src/ast/convertors.rs`, `get_key_values_from_object` -- copied
  every property and then copied the pair it answered, so every property of
  every object crossed this reader twice. It has about 35 production call
  sites, `validate_theme_variables` among them, which makes it the widest of
  the five. One of the two copies is now gone.
- `stylex-evaluator/src/evaluate/nodes/object_expression.rs` -- the hottest,
  once per property of every object literal the evaluator folds.
- `stylex-structures/src/base_css_type.rs` -- copied the whole property list
  before walking it, and copied every property the inner search looked at, so a
  type declaration of n properties cost n copies of each of its n value
  subtrees. Both walks now read where the properties lie.

`flatten_raw_style_object.rs:403` was read and left alone: it writes
`inner_key_value.key`, so its copy is load-bearing and the helper does not
apply. The comment there already says so.

**Blocked by:** None.

**Status:** resolved

- [x] The fallback list is moved, not copied
- [x] Only a shorthand is copied, at all five read-only sites
- [x] The one site that writes to the property keeps its copy
- [x] `expand_shorthand_prop` and `expanded_shorthand_prop` state one rule once
