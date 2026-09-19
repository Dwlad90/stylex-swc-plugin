# 71 — Close the guards over an object the compiler built

**What to build:** The 34 regions under `src/transform/stylex/` that guard the
shape of an object this compiler produced a few lines earlier. No source
reaches any of them, and each one is the second reading of a question the
producer had already answered.

**Where they are:**

| File                                     | Regions | The guard                                                                                                                                                                |
| ---------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `create_call/dynamic_style_functions.rs` | 19      | over the compiled style object: a key that is neither an identifier nor a string, a property that is not a key-value, a member of the list that is not a property at all |
| `define_vars_call/helpers.rs`            | 15      | over the evaluated variable group: the same three, plus a private name read as a key and two fall-throughs of the cycle walk                                             |

**Why a test cannot close them.** Both objects are built by the step above.
`dynamic_style_functions` reads the object `stylex_create_set` returned, whose
keys the compiler wrote itself. `collect_keys_and_dependencies` and
`normalize_define_vars_functions` read the object `evaluate` folded, and the
evaluator rebuilds every object it folds under plain key-values with named keys.
So a spread, a method, a computed key and a private name are all shapes the
producer cannot hand on -- which is the first answer
`guidelines/stack/RUST.md` asks for, measured rather than argued, and it comes
back empty.

**What the answer has to be.** The second answer in that list: give the step a
type with exactly the states its producer can build, so the refusal has nowhere
to live. Concretely, the producers answer a list of named properties, and both
readers want to walk that list. A type carrying `(Atom, Expr)` pairs -- built
once where the object is produced, refused once there if it ever cannot be --
removes every one of these guards and the second walk with them.

**Why it is not part of [64](./64-cover-the-transform-call-handlers.md).** The
larger of the two lives inside a 335-line function nested nine levels deep,
whose middle is the rewrite of the dynamic entries. Reshaping what it reads
means reading that function first, and the reshape touches what the compiler
emits for every dynamic style. That is a change to judge on its own, not a pass
alongside twenty test files.

**Two more of the same class, from ticket 65.** `retain_object_props` and
`retain_style_props` in `transform/visit_mut/visit_mut_var_declarator.rs` read
the object `convert_object_to_ast` wrote, and guard it for a spread and for a
key with no compile-time text. Ticket 65 answered them with cases that build
the objects at the function's own boundary, and asserted the invariant at the
producer, so they are off the gate. They are the same question this ticket
takes, and the same reshape closes them -- count them in when it is scheduled.

**Blocked by:** [64](./64-cover-the-transform-call-handlers.md) — the
measurement, and the four questions it already asked once.

**Status:** done

- [x] `scripts/coverage-missing.sh -p stylex_transform` reports no unexercised
  region in the two files this ticket names: both read 100.00% of regions,
  functions and lines, and the report holds no "Regions the coverage gate
  counts" section at all. 36 regions remained under `src/transform/stylex/`,
  from 71, held by the five tickets below.

  **Closed at the end of the branch.** All five landed, and the workspace gate
  reads 28,858 regions with none missed -- 6,096 of them in `stylex-transform`.
  The figures are in
  [67](./67-remove-the-transform-coverage-exclusion.md). The remaining guards
  over a compiler-built object are the `coverage(off)` sites, each naming why
  its step is total; `arrow_body_expr` in
  `transform_stylex_define_vars_call/helpers.rs` is one of them, and it cites
  `a_function_value_with_a_block_body_is_refused` as the case that would start
  failing if the evaluator ever folded one. They are held by
  [63](./63-cover-the-transform-shared-utils.md),
  [64](./64-cover-the-transform-call-handlers.md),
  [69](./69-inject-the-rules-an-array-bound-create-declares.md),
  [70](./70-read-first-that-works-inside-a-keyframes-step.md) and
  [73](./73-read-a-variable-reference-key-the-way-the-reference-does.md).
- [x] Each guard removed names the producer that holds the invariant instead,
      and that producer refuses the shape where it is built.
- [x] The compiled output for every dynamic style is unchanged, compared
      against the recorded snapshots rather than argued. Every snapshot in the
      workspace suite is unchanged, and three dynamic shapes were compared
      against `@stylexjs/babel-plugin` with `pnpm run parity:probe`.
- [x] The full workspace suite stays green. 6216 tests, from 6189, and
      `pnpm run test:coverage:workspace` stays at 100.00%.

## Comments

### What closed the 36 regions

**The compiled style object is handed on as a named list.** `js_to_ast.rs`
answers `NamedProp { key: &str, value }` instead of an `ObjectLit`:
`compiled_namespaces` for the namespaces and the properties under each,
`namespaces_to_object` and `named_props_to_object` to write the object where
one is wanted. `apply_dynamic_style_functions` walks that list, so the three
questions it used to ask about each property -- a key that is neither an
identifier nor a string, a property that is not a key-value, a member of the
list that is not a property -- have nowhere to live. The invariant is asserted
at the producer, by `writes_every_prop_as_a_key_value_under_a_name`.

The 335-line function came apart with it: `dynamic_namespace_fn` is the arrow
one dynamic namespace becomes, `dynamic_styles_of_namespace` the styles it
reads, `joined_class_paths` the paths they are keyed by. Nine levels of nesting
became three, and the two functions take the state they write to rather than
the whole transform, so neither is generic any more.

**The hoist answers the name it made.** `hoist_to_module_level` returns an
`Ident`, so the reader that needs the name no longer reads it back out of an
expression and refuses a shape the hoist cannot answer.
`stylex_merge` and the atoms compiler wrap it back into an `Expr` at their own
boundary.

**A `defineVars` group is read once.** `folded_style_object_lit` answers the
object the fold holds, and `VariableGroup::read` turns it into one named value
per variable. Both steps below it -- the dependency walk and the function-value
fold -- take that list, so neither asks what a property is again. The three
refusals live in `named_key_value` in `stylex-ast`, where a case can build a
spread, a method and a nameless key and read the sentence each gets.

**The cycle walk answers the cycle.** `find_cycle` returns the path it closed
instead of a flag, and `in_stack` carries each name's place on the stack rather
than a set the caller searched. The two fall-throughs went with the search, and
the walk no longer scans the stack for the back edge.

**The dependency reader is measured at its own boundary.** A member read is
written in five shapes and only some of them reach a `defineVars` group from a
source file -- a private name is legal inside a class body alone.
`dependency_visitor_test.rs` builds all five, so one instantiation runs the
whole reader.

**The sweep in `visit_mut_var_declarator` keeps its guard.** The object it
reads arrives through the program's own syntax tree in the finalize cycle, not
from a producer a step above, so no type can carry the promise that far. The
invariant stays asserted at the producer, and the two regions the gate counted
are closed by `an_entry_for_another_declaration_or_the_whole_variable_is_passed_over`:
the unit binary now drives the whole function, which is what the gate scores.

### The review round after the reshape

Three reviews ran over the work -- standards, spec and one on performance
alone. What was left after the first pass, and what came of it.

**The class-paths default is asserted where the paths are written.**
`joined_class_paths` answers the empty map for a namespace it holds no entry
for, and no reader reaches that answer, because `stylex_create_set` writes the
paths and the namespaces in one pass over the same names. The claim was in a
comment; it is now
`answers_class_paths_for_every_namespace_it_compiles`, which compiles a
namespace that declares nothing and reads both maps. That is the fourth answer
`guidelines/stack/RUST.md` ranks -- assert the invariant at the producer, where
a case costs no region. The reshape the review asked for instead, one map of
pairs out of `stylex_create_set`, would change what six other steps read to
close one unread branch.

**One place says what a function value is.** `VariableGroup::function_values`
answers the variables whose value is a function, and both the rule above it and
the dependency walk below read it. The walk also asks first whether the group
holds one at all: only a function body can name another variable, so a group
with none -- which is most of them -- no longer gathers the set of declared
names to compare against.

**The sentence a nameless key reads is recorded where it is decided.**
`VariableGroup::read` says that such a key is refused there, before the
missing-default rule looks at what the value holds, and that the rule below
always has a name to report because the group was named where it was read.

**Two findings were not taken.**

- The pair `folded_style_object` and `folded_style_object_lit` was read as one
  step delegating to another. Six producers pass the value on to a step that
  takes it, so the one line answers six call sites; writing the wrap out at
  each of them is the copy the pair removes.
- The four values `normalize_define_vars_functions` hands to
  `fold_function_value` were read as a type waiting to be born. They travel one
  hop, so a type for them would be the next smell on the same list. Sharing the
  refusal itself is the larger question -- eight readers report a value the fold
  would not answer, and seven of them already read it in one place -- and that
  is a change to judge on its own.

**A second round over the three fixes.** The parity case lost an assertion
that another case already makes, and it now reads the namespace count first,
so the check over the names cannot pass on an empty list. The note about the
sentence a nameless key reads stays at the rule that reports it and not at the
step above as well. The comment about how many groups hold a function value is
gone: `guidelines/PERFORMANCE.md` puts the noise floor far above one
allocation, so the step says what it skips and not what that is worth. The set
of declared names is sized up front, because a group can hold hundreds.
`joined_class_paths` reads each class name from the namespace rather than
copying it, so only the joined path is new text.

**On the `defineVars` half taking the first answer, not the second.** The spec
review read `named_key_value` as a guard relocated rather than removed.
`guidelines/stack/RUST.md` ranks proving the claim against a source and keeping
the guard **above** making the shape unrepresentable, and that is what this is:
a property that is a spread, a method or a key with no name is a shape any
object literal can hold, three cases build all three, and the sentence each one
gets is read. Making it unrepresentable would mean the evaluator answering a
named list for every object it folds, not only for a variable group.

### Three things an author reads differently

All three are shapes no source reaches, and they are recorded because the code
that stated them is gone.

- A function value with a parameter list that is not empty is refused on one
  rule now. The walk used to refuse a parameter the parser could read and let a
  placeholder through, and the step after it refused the placeholder on the
  same sentence, so an author read `invalid_define_vars_function_value` either
  way.
- A body that folds to something other than an expression reads the same
  sentence with a code frame, because it is now the same refusal as a body the
  fold declined. It had no frame before.
- A key with no name is refused where the group is read, on
  `KEY_HAS_NO_NAME`, before the missing-default rule looks at what the value
  holds. `MISSING_DEFAULT_VALUE_UNNAMED` lost its only reader and is deleted
  with it, and so is `create_prop_from_name`.

### What the reshape also paid for

Measured as removed work, not as a benchmark -- the deltas are far below the
noise `guidelines/PERFORMANCE.md` records.

- `get_key_values_from_object` left the dynamic path. It deep-copied every
  property of the whole compiled style object, and the namespace value was
  copied again after it.
- The per-property names are read as text, not copied: three `String`
  allocations per compiled declaration went.
- `normalize_define_vars_functions` no longer copies the folded object to
  rewrite it, and `collect_dependencies` borrows the names it collects.
- A dynamic style's path is joined out of the steps it holds, with no copy of
  each step, and a class name is hashed once per property rather than once per
  dynamic style.
- `NestedArrowDetector` was a second copy of `expr_contains_arrow` and is gone.
