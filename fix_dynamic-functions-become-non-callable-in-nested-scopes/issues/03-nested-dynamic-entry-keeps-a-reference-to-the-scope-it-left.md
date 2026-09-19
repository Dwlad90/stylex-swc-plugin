# 03 — A nested dynamic entry keeps a reference to the scope it left

**What to build:** Nothing in this compiler. This ticket records a defect that
the fix for ticket 01 makes reachable, the decision to leave the behaviour as it
is, and the report to raise with the maintainers of the specification.

**Status:** open — the report is drafted and not yet filed.

## What happens

A value that the compiler cannot fold becomes an inline style that holds the
authored expression as it is written. No rule says that expression may name only
the parameters of the dynamic entry, so an entry that reads a constant of the
enclosing scope carries that name with it. Ticket 01 hoists the compiled object
to the module level, and the name is not declared there:

```js
export function render(gap) {
  const styles = stylex.create({
    box: (value) => ({ width: value, margin: gap }),
  });
  return stylex.props(styles.box(1));
}
```

compiles to a module-level `const _styles` whose entry reads `gap`, which is a
parameter of `render`. The module still loads, because the body of the entry
runs only when it is called. The call is what fails, with
`ReferenceError: gap is not defined`, so the fault surfaces a long way from its
cause.

Before ticket 01 the same code failed differently, with
`styles.box is not a function`, because the entry was never rewritten into a
function. Both shapes are broken, so this is not a regression. It is a second
defect that the first one was hiding.

## Why the behaviour stays

The reference implementation of the specification hoists to the module level in
the same way and applies no scope check, so a diagnostic here would refuse code
that the specification accepts. The decision is to hold the behaviour and raise
the defect with the maintainers.

## What landed instead

- A rustdoc section on `hoist_styles_object` in
  `crates/stylex-transform/src/transform/stylex/transform_stylex_create_call/helpers.rs`
  that names the hazard, so the hoist does not read as scope-safe.
- The snapshot test
  `a_nested_dynamic_entry_keeps_a_reference_to_the_scope_it_left` in
  `crates/stylex-transform/tests/transform_stylex_create_test/dynamic_styles.rs`,
  which records the output as it is. The day the behaviour changes, the change
  is visible in that snapshot rather than in a user's build.

## Checklist

- [x] The hazard is documented where the hoist is written
- [x] A snapshot records the current output
- [ ] The report is filed with the maintainers of the specification
- [ ] The filed issue is linked here
