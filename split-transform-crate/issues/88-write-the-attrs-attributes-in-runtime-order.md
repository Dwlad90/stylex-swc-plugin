# 88 — Write the attrs attributes in runtime order

**What to fix:** `attrs` wrote `class`, `data-style-src`, `style`. The runtime
writes `class`, `style`, `data-style-src`
(`packages/@stylexjs/stylex/src/stylex.js:190-201`). `FnResult::Values` is
printed by `convert_values_to_ast` in source order, so the emitted object
literal read differently from the object the runtime builds for the same input.

Measured against the runtime itself, because the reference's compiler does not
inline an `attrs` call and so has no order of its own to compare with:

```
attrs keys: [ 'class', 'style', 'data-style-src' ]
props keys: [ 'className', 'style', 'data-style-src' ]
```

`props_map` already matched. Only `attrs` diverged.

**How it was answered.** The `data-style-src` block moved below the `style`
block. The order is now stated in the function's own documentation, because it
is part of the printed code rather than an implementation detail.

No existing case held all three attributes at once, which is why the divergence
survived a full snapshot suite. One now does:
`stylex_call_writes_the_three_attributes_in_runtime_order` in
`transform_stylex_attrs_test/with_plugin_options.rs` -- a dynamic style is what
writes `style`, and `dev` is what writes `data-style-src`.

**Blocked by:** None.

**Status:** resolved

- [x] The three insertions read `class`, `style`, `data-style-src`
- [x] A case holds all three at once
- [x] The order is checked against the runtime, not inferred
