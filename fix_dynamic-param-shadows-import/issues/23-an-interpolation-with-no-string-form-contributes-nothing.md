# 23 — An interpolation with no string form contributes nothing

Status: `needs-triage`
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

- [ ] Each row above is measured for `${undefined}`, `${{}}`, `${[1,2]}` and a
      callback, not only for the three shapes found
- [ ] The decision -- mirror the coercion, or refuse the interpolation -- is
      recorded with the reason
- [ ] Whichever way it goes, the silent empty string is gone
- [ ] Corpus entries carry the verdict each is known to read
- [ ] 15 case 4 is closed or re-pointed here
