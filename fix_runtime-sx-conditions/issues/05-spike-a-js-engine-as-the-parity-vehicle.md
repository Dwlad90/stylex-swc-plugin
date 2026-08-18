# 05 — Spike: a pure-Rust JS engine instead of hand-written method folds

Status: needs-triage
Phase: Deferred

**What to answer:** Whether the prototype-surface gap in 06 should be closed by
embedding a JS engine rather than by hand-writing the methods.

The reference implementation does not enumerate methods. It reflects
(`evaluate-path.js:1007-1010`):

```js
const val = object.node.value;
func = val[property.node.name];
```

so it inherits all of `String.prototype`, `Array.prototype` and
`Object.prototype` for free. Any enumeration this compiler writes is finite by
construction, and method N+1 is the next issue — which is the shape of
[#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265) itself.

`boa_engine` is pure Rust, so no C toolchain is added and all seven NAPI
targets — including `x86_64-unknown-linux-musl` and `aarch64-pc-windows-msvc` —
keep cross-compiling. Where receiver and every argument are already static, the
values go to the engine and the result comes back.

**It does not solve surrogates.** Boa's strings are UTF-16 internally, but the
result must land in `Lit::Str`'s `Atom`, which is UTF-8. The unpaired surrogate
dies at this compiler's boundary, not the engine's. The engine buys method
coverage, not exactness — 06's U+FFFD rule still applies.

Answer with numbers, on a throwaway branch, before any of 06 is written:

- Binary-size delta on each of the seven published targets.
- Cold-start and per-call cost against the release gate. `bench:verdict` fails
  at a reproduced 1.20x; `bench:budget` is seeded on
  `x86_64-unknown-linux-gnu`. See `guidelines/PERFORMANCE.md`.
- Whether the engine's `String.prototype` / `Array.prototype` coverage actually
  matches the divergence table in 06.
- Behaviour of a `stylex.create` call folding ~50 values with the engine
  initialised lazily and reused.

A JS engine inside a compiler whose pitch is that it is faster than the
reference implementation has to be defended with this repo's benchmark harness,
not with an argument.
