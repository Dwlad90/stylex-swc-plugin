# 48 — Read an own-keys array receiver as the reference implementation does

**What to build:** `Object.keys`, `Object.values` and `Object.entries` over an
array the engine will not carry are answered by this compiler's own walk, in
`crates/stylex-evaluator/src/evaluate/helpers.rs`. That walk reads such a
receiver **past anything it cannot fold**, where the reference implementation
refuses the declaration. Decide which answer each shape gets, and make the
readings one.

Every row below is measured, this compiler against the reference implementation,
with the comparison script in `.scratch/engine-fold-reads-values/babel.mjs`:

| Receiver                    | This compiler | Reference implementation           |
| --------------------------- | ------------- | ---------------------------------- |
| `Object.keys([, 'a'])`      | `1`           | refuses: cannot resolve the code   |
| `Object.keys([/re/])`       | `0`           | refuses: `Unsupported expression`  |
| `Object.keys([[, 'a'],'b'])`| `1`           | refuses: cannot resolve the code   |
| `Object.keys([() => 1])`    | refuses       | `1`                                |
| `Object.keys([[['a']]])`    | `1`           | `1`                                |
| `Object.keys(() => 1)`      | `0`           | `0`                                |
| `Object.keys('abc')`        | `3`           | `3`                                |
| `Object.keys(1)`            | `0`           | `0`                                |

The reference implementation folds no array literal holding a hole in **any**
position, and this compiler agrees for the array on its own -- `[, 'a']` refuses
with the same sentence. Only the own-keys walk parts from it, because it reads
the receiver out of the syntax so that a hole has something to be absent from.

**Two defects, and the second is the one that reaches a stylesheet.**

1. **A key that is not there, or one that is.** A shape the reference
   implementation refuses is folded here, and the other way round for an element
   with no compile-time form. Both compilers refuse or fold the whole
   declaration, so what changes is whether a build stops.
2. **A shortened array.** A value that resolved to nothing two levels down is
   dropped out of the array it sits in, and the element above it keeps its key.
   So `Object.values` answers an array shorter than the source describes, which
   is CSS the author did not write. Pinned by
   `a_value_that_resolved_to_nothing_deeper_down_is_dropped_from_its_array` in
   `src/evaluate/tests/object_statics_over_a_declined_receiver_tests.rs`.

The second is the one to answer first, and there are two readings of it. A
nested array can carry a hole -- an array literal has a slot for one -- so the
walk could write what the source describes. Or the receiver is one the walk
cannot read whole and answers nothing, which is the rule the same suite already
records for an element with no expression form. What it must not do is answer a
list with one element quietly missing.

Note that `evaluate_result_vec_to_array_expr` in `evaluate/mod.rs` already
settles the same question for every other reader, and refuses: "No caller answers
a shorter array either." The own-keys walk is a second, divergent copy of that
conversion, in `normalize_js_object_method_nested_vector_arg`, and closing this
ticket should leave one.

**Blocked by:** None. Found by the seventh instalment of
[ticket 15](./15-cover-the-evaluator-crate.md), which covered these readings
rather than changing them.

**Status:** resolved

## Comments

**2026-09-13, resolved.** The own-keys walk no longer reads an array literal out
of the syntax. The receiver is the value the evaluator resolved, which an array
reaches having already been read the way every other reader reads it, so the
second copy of the conversion is gone: `normalize_js_object_method_array_arg`
and `normalize_js_object_method_nested_vector_arg` are both deleted and the walk
reads each element through `evaluate_result_as_expr`, exactly as
`evaluate_result_vec_to_array_expr` does.

**Defect 2 is closed.** No path can answer a list shorter than the source
describes. An element with no expression form -- at any depth -- makes the
receiver unreadable instead of being dropped out of the array around it.

**Defect 1 is closed, and the table now agrees with the reference
implementation row for row**, sentence included. Measured with
`.scratch/engine-fold-reads-values/babel.mjs` against
`@stylexjs/babel-plugin@0.19.0`:

| Receiver | Before | After | Reference implementation |
| --- | --- | --- | --- |
| `Object.keys([, 'a'])` | `1` | refuses: cannot resolve the code | refuses: cannot resolve the code |
| `Object.keys([/re/])` | `0` | refuses: `Unsupported expression` | refuses: `Unsupported expression` |
| `Object.keys([[, 'a'],'b'])` | `1` | refuses: cannot resolve the code | refuses: cannot resolve the code |
| `Object.keys([() => 1])` | refuses | `1` | `1` |
| `Object.keys([[['a']]])` | `1` | `1` | `1` |
| `Object.keys(() => 1)` | `0` | `0` | `0` |
| `Object.keys('abc')` | `3` | `3` | `3` |
| `Object.keys(1)` | `0` | `0` | `0` |

`['a', , 'b']`, `[, ,]` and `[, ['a']]` moved the same way, and so did the
parity corpus row `modules-1266-object-keys-of-an-array-with-a-hole`, which is
now `both-reject` rather than `acceptance-divergent`.

**The one deliberate parting.** A receiver reached past a declined fold that
holds a value with no expression form -- `sx.missing ?? [own]`,
`sx.missing ?? [() => 1]` -- refuses here and counts the slot there. This
compiler holds no function value, so it could answer `keys` and not `values`,
and the three spellings of one question must not answer it three ways. Refusing
never writes CSS the source does not describe. Pinned by
`an_element_with_no_expression_form_refuses_the_receiver`.

**Two more readings the review found, and closed with it.**

1. **An array in its second spelling read as no array at all.** The reader knew
   the evaluator's own list and not the literal a fold answers, so
   `Object.keys(Object.keys(stylex))` folded to the empty list -- the answer
   that is neither a refusal nor the truth, and the one the function-fold arm
   above it exists to stop. Both spellings are now read, and the three rows of
   `the_key_list_of_a_fold_as_the_receiver` are byte-identical to upstream,
   class name included.
2. **One element rule, at every depth.** The top level accepted any expression
   while a level down accepted only an array, an object, a literal or a name,
   so `Object.values(sx.missing ?? [own.fn])` wrote the fold's placeholder arrow
   into a style value and the same value one level down refused.
   `array_element_expr` is now the one rule, and
   `evaluate_result_vec_to_array_expr` reads through it too.
