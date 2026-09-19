# 78 — Build the default marker once for the file

**What went wrong:** The default marker is two strings, an index map and two
counted pointers, and it reads only the class name prefix, which is fixed for
the file. Three paths asked for it and each built its own:

| Path                                                            | Built                                                                   |
| --------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `build_runtime_function_map`, per `create` and per `atoms` call | once per `defaultMarker` import name, and once per `stylex` import name |
| `stylex_merge`, per `props`-family call                         | read from the memo, then copied once per name                           |
| `transform_default_marker_call`, per `defaultMarker()` site     | once, from the options                                                  |

A memo for it was added by
[64](./64-cover-the-transform-call-handlers.md), and only `stylex_merge` read
it.

**Blocked by:** None.

**Status:** resolved

- [x] One reader answers the marker, and all three paths ask it.
- [x] The marker is shared, not copied, into every registration.
- [x] The compiled output is unchanged, compared against the reference plugin
      rather than argued.
- [x] The full workspace suite and the coverage gate stay green.

## Comments

### The answer the repository already held

`FunctionConfigType` had the same problem one variant away, and had already
answered it: `EnvObject(Rc<...>)` carries the `env` object shared rather than
copied, because it is read-only after it is built. The marker is the same shape
of value, so it took the same answer. `FunctionConfigType::IndexMap` carries
`Rc<FlatCompiledStyles>`, `CacheState` keeps the marker as one, and each
registration is a count, not a copy.

### What it removed beside the copies

Two of the three paths read the marker through `stylex_default_marker`, which
wraps the values in `NestedStringObject`, and then unwrapped it again with
`as_values().unwrap_or_else(|| panic)`. The enum has two variants, and **every
one of its call sites knew which one it had**: nine sites wrapped a value and
the reader matched it straight back out. Its nested arm also copied each
namespace's whole value map to recurse.

It is replaced by two named writers -- `convert_values_to_ast` and
`convert_namespaces_to_ast` -- which is the second answer
[the repository ranks](../../../guidelines/stack/RUST.md): give the step a type
with exactly the states its producer can build, so the refusal has nowhere to
live. `NestedStringObject`, `as_values`, the `stylex_default_marker` wrapper and
the `EXPECTED_COMPILED_STYLES` message all lost their last reader and are gone.

`runtime_function_map.rs` reads 100% of regions now, from 95.89%, and the
crate's unexercised regions went from 83 to 80. None of the three was closed by
a test; each was a guard over an object the compiler had built a line earlier,
which is the class [71](./71-close-the-guards-over-a-compiler-built-object.md)
takes.

### What is saved, and what is not claimed

Per `create` call: the registrations are the number of `defaultMarker` names
plus the number of `stylex` names, and each cost a full build and a deep copy --
about ten small allocations each. Per `props` call: the copies alone, about
three maps. Per compiled `create` output: one map copy per namespace, from the
writer.

[The policy](../../../guidelines/PERFORMANCE.md) is that only a paired release
comparison blocks, and cross-run noise is about 16%. A dozen small allocations
per call is far under that, so this is recorded as an allocation reduction and a
simpler shape. **No speed-up is claimed**, and none was measured.

### Equality with the reference plugin was measured

`pnpm run parity` over the whole corpus exits 0. `pnpm run parity:probe` on the
shapes this change touches:

| Shape                                                                    | Result    |
| ------------------------------------------------------------------------ | --------- |
| `defaultMarker()` twice in one file                                      | identical |
| the marker through a named import, alone and with a style                | identical |
| the marker through the namespace, alone and with a style                 | identical |
| the marker under two import names in one file                            | identical |
| the marker with `create` over two namespaces                             | identical |
| `createTheme` against an imported variable group                         | identical |
| `defineVars` with an at-rule, and `defineConsts`, in a `.stylex.js` host | identical |

### The review

Four passes ran: standards and performance before the second half, standards and
adversarial correctness after it. The correctness pass read the old and the new
writer side by side and reports the same four arms, the same iteration order and
no call site given the wrong writer. What the standards passes found is folded
in: the free reader was renamed `shared_default_marker_values`, because it had
the same name as the memo it calls; one comment was repeated in two files and is
now written once; a comment said the marker is built on the first
`defaultMarker()` in the file, where a `create` call before it usually builds it;
and a test named for every prefix exercised one, so it is gone and the three that
name their prefix look the class up by name rather than by position.
