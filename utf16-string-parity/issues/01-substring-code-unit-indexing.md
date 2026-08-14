# SubString indexes by scalar where upstream indexes by code unit

Status: ready-for-human

`crates/stylex-css-parser/src/base_types.rs` — `SubString` navigates with
`chars().nth()` and returns `Option<char>`:

```rust
pub fn first(&self) -> Option<char> {
  self.string.chars().nth(self.start_index)      // :50
}

pub fn get(&self, relative_index: usize) -> Option<char> {
  self.string.chars().nth(absolute_index)        // :59
}
```

Upstream `packages/style-value-parser/src/base-types.js` indexes the raw string,
i.e. by UTF-16 code unit, returning a one-code-unit string:

```js
get first(): string { return this.string[this.startIndex]; }          // :35
get(relativeIndex: number): string {
  return this.string[this.startIndex + relativeIndex];                // :39
}
startsWith(str) { ... this.string[this.startIndex + i] !== str[i] ... } // :26
```

So for input containing an astral scalar, every index past it is off by one, and
`first`/`get` hand back a whole scalar where upstream hands back a surrogate
half. `starts_with` and `into_string` share the same `chars()`-based model.

## Why it was not fixed with #1248

Two reasons:

1. **Architectural.** The fix changes `SubString`'s index model and the public
   return type (`Option<char>` → a code unit), which ripples through `first`,
   `get`, `starts_with`, and `into_string` together. That exceeded the
   "surgical fix" scope the #1248 work was held to.
2. **Latent.** `SubString` currently has no consumers — it is referenced only
   inside its own module and one doc test in `lib.rs`. Nothing reaches the
   divergence today.

It is filed rather than dropped because the type is public API of the crate: the
first real consumer inherits the bug, and it will present as a parser that
disagrees with upstream only on emoji-bearing values.

## Suggested approach

Store the haystack as `Vec<u16>` (or keep the `&str` and precompute a code-unit
index) so `start_index` / `end_index` mean the same thing they mean upstream,
and have `first`/`get` return `u16`. Decide deliberately what `into_string` does
with an unpaired surrogate — upstream would produce a lone surrogate in a JS
string, which Rust cannot represent in a `String`.
