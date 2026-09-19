# 92 — Cross a constant that is one JavaScript value directly

**What to fix:** `stylex-rs-compiler/src/utils/metadata.rs` wrote every constant
that is not a number through a `serde_json::Value` round trip. The constant
arrives as JSON text, is parsed into a `FlatCompiledStylesValue`, then
`as_json_value()` rebuilds a `serde_json::Value` tree of the same shape, which
`env.to_js_value` walks a third time -- three allocations and two tree walks to
set one property for the common case, a constant that is text.

**How it was answered.** The three kinds that are one JavaScript value each
cross directly: a number through `create_double`, text through `create_string`,
a boolean through `get_boolean`. Only an object and a list build a JSON tree,
and only those two pay for it.

Covered by `index.spec.ts`, which already asserts
`[null, 1, 'red', true, 0]` crosses -- one input per arm, the fallback
included. This crate is permanently excluded from the Rust coverage gate, so the
JavaScript suite is the only reader of these arms and it already held them.

**Blocked by:** None.

**Status:** resolved

- [x] Text, a number and a boolean cross without building a JSON tree
- [x] An object, a list and `null` are unchanged
- [x] `index.spec.ts` covers all four arms
