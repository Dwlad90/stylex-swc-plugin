# 01 — `"abc".length` evaluates to `"abc"`

Status: ready-for-agent
Phase: Phase 2

**What to build:** A member access of `length` on an evaluated string answers
its length.

```js
stylex.create({ x: { content: "abc".length } });
```

The reference implementation folds this to `3`, emitting
`.xxxxxxx{content:3px}`. This compiler answers `"abc"` — the receiver itself —
and emits `content:"abc"`. It does not error. It ships a wrong CSS value.

That makes this the most severe finding of the investigation even though it is
not what [#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265)
reported: a build error stops the line, a wrong value does not.

This is the member-expression path, not the call path — `length` is a property,
not a method — so it lives in `nodes/member_expression.rs` and is independent
of the panic/deopt work in 02.

`length` is measured in UTF-16 code units, not bytes and not scalars:
`"\u{1F600}a".length` is `3`. `crates/stylex-utils/src/string.rs` already
counts in code units for `charCodeAt`; reuse that view rather than introducing
a second convention.

Array `length` follows the same rule and the same reasoning; today
`["a","b"].length` raises `This evaluation result type is not yet supported`.

Independent of 02 — that work is the call path, this is the member path — so
it does not gate the regression fix and must not delay it. It changes emitted
CSS and a generated class name, so it needs its own release note.
