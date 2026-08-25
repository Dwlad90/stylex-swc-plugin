# 06 — The prototype surface the reference implementation folds

Status: needs-triage
Phase: Deferred

**Blocked by:** 05

**What to build:** Static method calls on string, array and object literals
fold as they do upstream.

Not a regression — this gap shipped in `0.18.3` and `0.18.4-rc.1` and has never
been reported. It is recorded here so the measurements are not lost. Do not
start it before 05 answers how.

Measured across 70 expressions, reference implementation versus this compiler:

| receiver        | upstream folds                                              | this compiler folds       |
| --------------- | ----------------------------------------------------------- | ------------------------- |
| string literal  | 28 methods, `startsWith` … `normalize`, plus `.length`       | `concat`, `charCodeAt`    |
| numeric literal | none — all four **throw**, see `../spec.md` non-goals        | none, different message   |
| array literal   | 15, incl. `sort` `reverse` `reduce` `some` `flat` `at`       | `join`, `map`, `filter`   |
| object literal  | prototype methods: `hasOwnProperty`, `toString`, …           | own keys only             |
| globals         | `String` `Number` `Math` `Object` `Array`; rejects `random`, `assign`, `JSON`, `Boolean`, `parseInt` | **identical, no gap**     |

Agreed boundaries, if this is built by hand rather than via 05:

- **String (25):** `startsWith` `endsWith` `includes` `indexOf` `lastIndexOf`
  `slice` `substring` `substr` `at` `charAt` `charCodeAt` `codePointAt`
  `concat` `repeat` `padStart` `padEnd` `split` `replace` `replaceAll`
  `toLowerCase` `toUpperCase` `trim` `trimStart` `trimEnd` `length`.
  Excluded: `normalize`, `localeCompare`, `toLocale*` — these need ICU, which
  is a dependency decision of its own; `match` / `matchAll` / `search` — the
  reference implementation already errors on regex literals, so deopting is
  parity.
- **Array (16):** `includes` `indexOf` `lastIndexOf` `at` `slice` `concat`
  `join` `map` `filter` `find` `findIndex` `some` `every` `reduce` `flat`
  `length`. `evaluate_map` / `evaluate_filter` already invoke arrow callbacks,
  so the callback-taking ones are mostly reuse.
- **Object (6):** `hasOwnProperty` `toString` `valueOf` `isPrototypeOf`
  `propertyIsEnumerable` `toLocaleString`. `constructor` and `__proto__` deopt
  — they can only produce a value that is not a valid style, so folding them
  moves the error later and makes it worse.
- Unify the `EvaluateResultValue::Vec` and `Expr::Array` arms onto one dispatch
  **before** adding any method. They accept different sets today, which is why
  `["a","b"].map(x => x).join("-")` fails on the chained `join`. Two tables
  that must agree and are edited separately is the shape of the original bug.

**Deliberate divergences**, to be documented rather than quietly carried:

- Mutating methods keep deopting. The reference implementation folds
  `["a","b"].push("c")` to `3` and `["a","b"].sort().join("-")` to `"a-b"` by
  accident of reflection; `is_mutating_array_method` already refuses them.
  Matching that means implementing mutation semantics inside a pure evaluator
  to serve input nobody writes.
- Unpaired surrogates become `U+FFFD`. `"\u{1F600}a".slice(1)` is a lone low
  surrogate in JS; `Lit::Str`'s `Atom` is UTF-8 and cannot hold one.
  Substituting the replacement character keeps the declaration byte-identical
  to the reference implementation's — which itself becomes `EF BF BD` the
  moment the stylesheet is written to disk — and diverges only in the generated
  class name (`xn5tvdn` where upstream emits `xi08yer`), for input whose
  rendered output is a replacement character in both compilers. Deopting
  instead would fail a build that upstream completes. Representing the
  surrogate faithfully would mean the evaluator no longer carries values as SWC
  AST nodes, which is a rewrite of its core value type and needs its own spec.
  Pin the divergence in the corpus.

## 05's answer, and what it means here

05 is resolved: the engine matches the table (69/70 against Node, byte-identical
class names against the reference implementation across 80 rules), folds chains
for free, and costs nothing on the release gate. It grows the published artifact
by 5.6–6.1 MiB, which is accepted — that artifact is a build-time dependency, so
no consumer bundle carries it. What stops it today is that it **cannot be
resolved into this workspace at all**: `boa_engine` requires
`icu_normalizer ~2.0.0` where `icu_collator` needs `~2.3.0`.

So this ticket is not unblocked in practice. Either wait for boa to relax that
requirement upstream, or build the boundaries above by hand. Two of 05's findings
apply whichever way it is built:

- Locale-sensitive methods must stay excluded — the exclusion list above was
  right. `normalize` could move into scope only with an ICU dependency of its own.
- `"abc".charCodeAt(10)` folding to `NaN` emits `z-index:NaN`, and that is what
  the reference implementation emits too. Parity and a useful refusal disagree
  here; this ticket has to choose one and say why.
