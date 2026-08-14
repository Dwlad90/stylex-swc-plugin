# remove_quotes strips every outer quote and its Cow branch is a no-op

Status: needs-triage

`crates/stylex-utils/src/string.rs`:

```rust
pub fn remove_quotes(s: &str) -> Cow<'_, str> {
  let trimmed = s.trim_matches('"');

  if trimmed.len() == s.len() {
    Cow::Borrowed(s)
  } else {
    Cow::Borrowed(trimmed)
  }
}
```

Two things:

1. **`trim_matches` is greedy.** It strips *every* leading and trailing `"`, not
   one pair, so `"""a"""` becomes `a` and `""` becomes empty. If the intent is
   "unwrap one quoted string", this over-strips; if the intent really is "strip
   all outer quotes", the name should say so.
2. **The branch does nothing.** Both arms return `Cow::Borrowed`, and
   `Cow::Borrowed(trimmed)` is correct either way since `trimmed` borrows
   from `s`. The `if` can go, and with it the `Cow` — the function can
   return `&str`.

Callers: `crates/stylex-transform/src/shared/utils/common.rs:450,452`, both in
JSON-ish string handling.

## Why it was not fixed with #1248

Out of scope — it is not an encoding divergence, and it surfaced only because
the audit re-read `string.rs` line by line. There is also no obvious upstream
counterpart to check parity against (no `removeQuotes` in the babel-plugin
shared utils), so "what should this do" is a repo question rather than a parity
question.

## Suggested approach

Establish from the two call sites whether one pair or all quotes is wanted. If
one pair, `s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(s)`
expresses it. Either way, drop the dead branch and return `&str`.
