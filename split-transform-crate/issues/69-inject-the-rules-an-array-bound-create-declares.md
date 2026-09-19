# 69 — Inject the rules an array-bound `create` declares

**What to fix:** A `stylex.create` call written inside a top-level array
compiles its class names and injects none of its rules. The module ships
elements carrying class names that no stylesheet defines.

**What comes out today.** Both modules below declare the same rule. Compiled
with runtime injection on:

```js
// bound to a name
import * as stylex from "@stylexjs/stylex";
export const all = stylex.create({ a: { color: "red" } });
```

```js
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
_inject2({ ltr: ".x1e2nbdu{color:red}", priority: 3000 });
export const all = { a: { kMwMTN: "x1e2nbdu", $$css: true } };
```

```js
// bound inside an array
import * as stylex from "@stylexjs/stylex";
export const all = [stylex.create({ a: { color: "red" } })];
```

```js
import _inject from "@stylexjs/stylex/lib/stylex-inject";
export const all = [{ a: { kMwMTN: "x1e2nbdu", $$css: true } }];
```

The second one is wrong twice. There is no `_inject2` call, so
`.x1e2nbdu{color:red}` is never injected and `kMwMTN` names a class with no
declaration. And the `import _inject` line is left behind without the
`var _inject2 = _inject;` that every use of it needs, so the prelude is half
written.

**Where it starts.** A top-level array holding a `create` call is an idiomatic
shape the validator accepts on purpose --
`is_bound_create_expr` in `crates/stylex-transform/src/shared/utils/validators.rs`
answers for it by span containment, and
`transform_stylex_create_test/program_level_positions.rs` has a case for it.
That case compiles with runtime injection off, which is why the missing rule
never showed.

`transform_stylex_create_call/mod.rs` reads the variable name the styles are
remembered under from `get_call_var_name`, which answers `None` for a call
inside an array -- the call initialises no declarator of its own. The
registration that injects the rules is behind `is_program_level && var_name`,
so an array-bound call takes neither branch.

**What was settled.** The rules are injected before the *statement* that holds
the call, which is where the name-bound spelling puts them and what the
reference implementation emits. Nothing is keyed to the array's own name: the
reference walks up from the call to the nearest statement under the program and
inserts before it, so the name the statement binds never enters the question.

The `"Function type"` refusal beside that branch is gone. `get_call_var_name`
reads the name off the declarator, so it answers a name only together with the
declarator it read it from -- the branch that refused when there was a name and
no declarator could not be entered, and the sentence could not be acted on.

**Status:** resolved

- [x] A `create` bound inside a top-level array injects every rule it declares.
- [x] The runtime-injection prelude is written whole, or not at all.
- [x] A case compares the array-bound and name-bound spellings of one module
      with runtime injection on.
- [x] The result agrees with the reference implementation, or the divergence is
      recorded with its reason.

## Comments

Found while closing [64](./64-cover-the-transform-call-handlers.md): the
`var_name.is_none()` path is one of the regions no test reaches, and reading
why led here.

### Where it was, and what the fix is

`decl_init_hashes` in `crates/stylex-state/src/state_manager.rs` names the
declaration a queued injection call belongs to, and it hashed a declarator
initializer only when that initializer was the style object itself. An
array-bound call leaves the array as the initializer and the object one level
down, so the hash never matched and `flush_pending_insertions` dropped the
call. The walk now looks through an array, at any depth, and through a
parenthesis at every level -- the second was found by
`parenthesised_spellings::every_shape_in_the_class_compiles_alike_in_both_spellings`,
which compares the bare and parenthesised spellings of the same module and
failed on the first half of the fix.

Measured against `@stylexjs/babel-plugin` 0.19.0 with `runtimeInjection: true`.
The two compilers print the same module for the array-bound spelling, the
name-bound one, an array holding two calls, a nested array, an array holding
values that are not styles, and the parenthesised array. A `keyframes` call
inside an array is compiled by neither.

Two shapes diverge and neither is this ticket's. A call inside an object
*inside* an array kept the defect this ticket closed, which is
[79](./79-inject-the-rules-an-object-in-an-array-declares.md), now resolved.
And `export const all = { x: stylex.create({ ... }) }`, with no array, is
refused here and compiled by the reference, which is a rule of the validator
rather than of this walk -- [80](./80-decide-what-binds-a-create-call.md).

### The walk costs less than it did

Two readings of the same function, both from the review:

- `flush_pending_insertions` hashed every top-level object and array
  initializer in the module even when no `BeforeDecl` item existed to match.
  That is every module compiled with runtime injection off -- the default --
  because the loop above drops them all. The walk now returns early on an empty
  bucket.
- The walk allocated a vector of hashes per module item, and a vector of
  declarator references beside it. Both are gone: one buffer is cleared and
  reused for the whole walk, and the declarators are read as a slice.

The walk through arrays is linear: every node is either stepped through or
hashed, never both, and the subtrees hashed do not overlap.

