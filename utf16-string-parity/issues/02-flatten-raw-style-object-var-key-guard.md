# flatten_raw_style_object guards the var() key unwrap with the wrong regex

Status: needs-triage

`crates/stylex-transform/src/shared/utils/core/flatten_raw_style_object.rs:94`
decides whether to unwrap a `var(--x)` style key using `CSS_VALUE_SPLIT_REGEX`:

```rust
let css_property_key = if CSS_VALUE_SPLIT_REGEX.is_match(&key).unwrap_or_else(...) {
  key[4..key.len() - 1].to_string()
} else {
  key.clone()
};
```

`CSS_VALUE_SPLIT_REGEX` is `(\))(\S)|(\")(\")` — a value-splitting pattern for
finding adjacency, unrelated to variable keys. Upstream
`packages/@stylexjs/babel-plugin/src/shared/preprocess-rules/flatten-raw-style-obj.js:47`
uses:

```js
const key: string = _key.match(/var\(--[a-z0-9]+\)/)
  ? _key.slice(4, -1)
  : _key;
```

For a key spelled `var(--foo)` the upstream regex matches and the key becomes
`--foo`. Ours cannot match: the `)` is the last character, so `(\))(\S)` has no
following non-whitespace to consume, and there is no `"` pair. The key is left
as `var(--foo)`.

## Why it was not fixed with #1248

Out of scope. The #1248 audit was scoped to the UTF-16/UTF-8/scalar encoding
class; this is a plainly different regex being used for the job, not an encoding
mismatch. Changing it without understanding why it is there risked breaking a
behaviour the encoding fix had no business touching.

## Not confirmed as user-visible

The full suite passes, which suggests the case is covered downstream:
`flat_map_expanded_shorthands.rs:66` performs the same unwrap with a guard that
does work —

```rust
let key = if key.starts_with("var(") && key.ends_with(')') {
  key[4..key.len() - 1].to_string()
```

— so the key may simply be unwrapped one stage later. Triage should establish
whether the `flatten_raw_style_object` branch is dead, subtly load-bearing, or
genuinely wrong before anything changes.

Note also that upstream itself is inconsistent here: `flatten-raw-style-obj.js`
uses `[a-z0-9]+` while `preprocess-rules/index.js:41` slices unconditionally,
and `stylex-first-that-works.js:11` uses `[a-zA-Z0-9-_]+`. Whatever we do should
name which upstream site it mirrors.
