# 08 — A hole is dropped from an evaluated array

Status: needs-triage
Phase: Phase 2

**What to build:** An array hole survives evaluation as a value, so an evaluated
array has as many elements as the source wrote slots.

`nodes/array_expression.rs` evaluates elements with `iter().flatten()`, which
skips a hole rather than recording one:

```rust
for elem in arr_path.elems.iter().flatten() {
```

So `[, 1]` evaluates to a one-element `Vec` where JavaScript has two, and every
reader of an evaluated array inherits the short count:

- a binding — `const A = [, 1]; A.length` answers `1`
- `Object.keys([, 1])` — `helpers.rs` already carries an arm for
  `EvaluateResultValue::Null` as a hole, with a comment explaining that
  `Object.keys([, 1])` omits index zero. That arm is unreachable today, because
  a hole never arrives as `Null`
- `[, 1]` as a style array value

Found while implementing
[01](./01-length-on-a-string-evaluates-to-the-receiver.md), which works around
it: an array _literal_ receiver has its `length` counted off the AST, so
`[, 1].length` answers `2`, and only a non-literal receiver reads the short
count. That workaround is why the two counts are read from different places, and
closing this issue is what would let them be read from one.

**Not a regression, and not what anyone reported.** The reference implementation
does not fold `[, 1]` at all — it refuses the whole array with `Unexpected
error: Could not resolve the code being evaluated`, pinned as
`modules-length-on-an-array-with-a-hole` in the parity corpus. So this compiler
is already ahead on the input that surfaced it, and closing this issue widens
that lead rather than restoring parity.

**Decide before building** whether a hole is `EvaluateResultValue::Null` or a
variant of its own. `Null` is already what a confident element with no value
becomes (`unwrap_or(EvaluateResultValue::Null)`), so the two would be
indistinguishable — and `[, 1]` and `[undefined, 1]` are the same length but not
the same array to `Object.keys`. That is the question this ticket turns on.

Changes emitted CSS wherever a hole reaches a style array value, so it needs its
own release note.
