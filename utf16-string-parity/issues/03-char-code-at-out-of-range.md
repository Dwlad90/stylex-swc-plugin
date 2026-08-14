# charCodeAt out of range panics where JS returns NaN

Status: needs-triage

`crates/stylex-transform/src/shared/utils/js/evaluate/nodes/call_expression.rs:1281`
folds `String.prototype.charCodeAt` and treats an out-of-range index as fatal:

```rust
let char_code = char_code_at(&base_str, *char_index as usize).unwrap_or_else(|| {
  stylex_panic!("String.charCodeAt() returned no result for the given index.")
});
```

In JavaScript, `"ab".charCodeAt(5)` is `NaN`, not an error. So a `stylex.create`
whose value calls `charCodeAt` past the end compiles under upstream — producing
`NaN` in the value — and raises here.

## Why it was not fixed with #1248

It is a behavioural question, not an encoding one, and it needs a decision
rather than a patch. The encoding half of `char_code_at` was fixed under #1248
(it now indexes by UTF-16 code unit); this is the remaining divergence in the
same function.

## The decision to make

Strict logic parity says fold to `NaN` and let it flow into the value, matching
upstream. Against that: `NaN` in a CSS value is meaningless output, and the
current panic tells the author immediately that their index is wrong, which is
arguably the more useful compiler. The `unwrap_or_else` also breaks the "handle
every case with `match`" rule in `CLAUDE.md`, so whichever way this goes the
call site wants rewriting.

Worth checking first whether upstream's evaluator actually reaches
`String.prototype.charCodeAt` with an out-of-range constant, or bails earlier —
if it deopts before folding, there is no parity obligation and the panic can
stay.
