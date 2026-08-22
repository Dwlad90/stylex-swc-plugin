# 23 — An interpolation with no string form contributes nothing

Status: `resolved`
Blocked by: None

**What was found:** A template literal drops any interpolation the evaluator
cannot answer as a literal string, and declares the rest. Nothing says the value
went missing. Found while measuring
[18](./18-a-theme-object-read-as-a-style-value-is-dropped.md), whose template row
is one shape of it.

`nodes/template_literal.rs` pushes an interpolation's text only when the whole
chain holds -- evaluated, an `Expr`, a `Lit`, convertible to a string:

```rust
if let Some(expr) = exprs.get(i)
  && let Some(evaluated_expr) = evaluate_cached(expr, state, traversal_state, fns)
  && let Some(lit_str) = evaluated_expr.as_expr().and_then(|e| e.as_lit()).and_then(convert_lit_to_string)
{
  strng.push_str(&lit_str);
}
```

A value that is confident but carries no literal falls out of the chain and
contributes the empty string. The reference implementation interpolates whatever
JavaScript's coercion answers, so the two disagree about the text and therefore
about the class name.

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration -- `haste` resolution, one source string.

| input, inside `create({ … })` | Babel 0.19.0 | here |
| --- | --- | --- |
| `zIndex: \`${zIndex}\`` | `z-index:x1q8i56t` | `z-index:` |
| `color: \`${zIndex}red\`` | `color:x1q8i56tred` | `color:red` |
| `height: \`${stylex}px\`` (a folded map) | `height:[object Object]px` | `height:px` |

Both compilers emit nonsense; they disagree about which nonsense, and a class
name is a hash of that text. Recorded as
`modules-1266-theme-reference-coerced-in-a-template`.

The coercion this needs already exists and already answers each of these:
`evaluate_result_to_string_of` in `js/evaluate/helpers.rs` gives a theme
reference its `toString` (the var-group hash, which is upstream's answer) and an
entries map `[object Object]` (upstream's answer for the fold). The template
evaluator does not read it.

Two decisions before anything is written:

- Whether agreeing on nonsense is worth having. Every row above is a value no
  author intends, and refusing the interpolation is the other defensible answer
  -- but a class name is a compatibility contract, and this compiler emitting a
  *different* nonsense than upstream is the one answer that serves nobody.
- What the change reaches beside these three rows. The chain drops every
  interpolation with no literal form, not only these, so routing it through the
  coercion changes `${undefined}`, `${{}}`, `${[1,2]}` and a callback at once.
  Each needs measuring before the seam moves.

This is the third position of the same shape -- a value with no expression form
read where a value belongs -- after
[18](./18-a-theme-object-read-as-a-style-value-is-dropped.md) (the style value,
fixed) and [15](./15-the-function-map-read-where-it-is-not-a-map.md) case 4 (the
same template row for a folded map, still open). Closing this closes 15 case 4.

- [x] Each row above is measured for `${undefined}`, `${{}}`, `${[1,2]}` and a
      callback, not only for the three shapes found
- [x] The decision -- mirror the coercion, or refuse the interpolation -- is
      recorded with the reason
- [x] Whichever way it goes, the silent empty string is gone
- [x] Corpus entries carry the verdict each is known to read
- [x] 15 case 4 is closed or re-pointed here

## Answer

Mirror the coercion, except where the reference implementation's answer is an
artifact of its own internals. Landed as one commit.

### The measurement reached further than the three rows

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration, every row before and after:

| interpolation | Babel 0.19.0 | before | after |
| --- | --- | --- | --- |
| `${null}` | `anullb` | `ab` | agrees |
| `${undefined}` | `aundefinedb` | `ab` | agrees |
| `${true}` / `${false}` | `atrueb` / `afalseb` | `ab` | agrees |
| `${NaN}` / `${Infinity}` | `aNaNb` / `aInfinityb` | `ab` | agrees |
| `${{}}` / `${{a:1}}` | `a[object Object]b` | `ab` | agrees |
| `${[1,2]}` | `a1,2b` | `ab` | agrees |
| `${[{},{}]}` | `a[object Object],[object Object]b` | `ab` | agrees |
| `${[]}` / `${[null]}` | `ab` | `ab` | agrees |
| `${theme}` | `z-index:x1q8i56t` | dropped | agrees |
| `${stylex}` (the fold) | `a[object Object]b` | `ab` | agrees |
| `${() => 1}` | its own evaluator's closure source | `ab` | **refuses** |

`${null}` and `${true}` are the two the ticket did not name and the two that
matter most: they are ordinary literals, so the seam was not "a value with no
expression form" at all. It was a chain -- evaluated, an `Expr`, a `Lit`,
convertible -- that answered the empty string whenever any link failed. The
failure was silent in the worst way: no rule went missing, a *wrong* one was
emitted, and the class name hashed to match.

### The decision, and its one exception

Mirror. Every row above is a value no author intends, and the ticket's own
reasoning settles it -- a class name is a hash of the declaration text, so a
compiler emitting *different* nonsense than the one it interoperates with is the
answer that serves nobody. Refusing would have been defensible in the abstract
and is not available in practice: `${undefined}` and `${null}` are too ordinary
to start failing builds over.

The exception is the callback, and it is not a coercion question. Upstream's
answer there is the **source text of its own evaluator closure** -- `(...args)
=> { const identifierEntries = identParams.map(...` -- an internal artifact of
the compiler, hashed into a class name and written into a stylesheet. That is a
defect to leave upstream rather than reproduce. It refuses with the
not-a-string diagnostic, and the corpus row records `acceptance divergent` with
the reason.

### Two seams moved, not one

- **The template evaluator** reads `evaluate_result_to_js_string`, the bridge
  every other consumer of a folded value already read. `None` deopts instead of
  contributing nothing.
- **The folded function map** coerces as an object. It was classified as a
  function, so `${stylex}` refused where upstream writes `[object Object]` --
  and `nodes::object_expression` had already settled that a folded map is a
  plain object upstream, spreading its keys on exactly that reasoning. The two
  arms disagreed; they agree now. `FunctionConfig` and `Callback` stay
  functions, which is what they are.

  Both bridges that classify the value moved together, and the second one --
  `ToObject` -- was raised in review as unmeasured. It is measured now:
  `Object(stylex)` interpolated reads `x[object Object]y` on both compilers,
  where it used to refuse, and `Object(stylex)` as a style value and as a spread
  refuse identically on both. Pinned as
  `template_interpolation::the_folded_namespace_map_wrapped_in_object_takes_the_same_default`
  and as corpus `modules-the-folded-map-wrapped-in-object`.
  `helpers_tests::the_two_bridges_agree_a_function_map_is_an_object` fails if
  the two classifications part again, which is the drift that caused this.

  Measuring it also turned up a third reader that still disagrees:
  `Object.keys` of the same fold answers the empty list. Older than this change
  and only made visible by it, so it is filed as
  [40](./40-object-keys-of-a-fold-answers-an-empty-list.md) with a corpus row
  rather than patched.

### This closes 15 case 4's template row

15 recorded the fold-coerced-to-a-string family as a decided divergence and
pointed its template row here. That row now reads `identical`, as does the
theme-reference row beside it. The other three readings 15 recorded -- the
computed key, the concatenation, the ternary condition -- are untouched and
still 15's.

### Verification

`cargo test --workspace --all-features` 0 failed, `cargo clippy --workspace
--all-features --all-targets` clean, `cargo fmt` clean, `pnpm typecheck &&
pnpm lint:check && pnpm test` green. `parity` 0 changed verdicts over 1019
subjects; two entries moved to `identical` and five were added.
