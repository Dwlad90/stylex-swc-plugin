# UTF-16 string parity

Upstream StyleX is JavaScript, so every string operation in it is defined over
**UTF-16 code units**. Rust strings are UTF-8, and `char` is a Unicode scalar.
Where a port measures, indexes, or hashes a string, those three models disagree
for any input outside ASCII — silently, and only for non-ASCII input, which is
why it survives an all-ASCII test suite.

GitHub issue #1248 was one instance: `create_hash` hashed UTF-8 bytes where
`murmurhash2_32_gc` hashes UTF-16 code units masked to their low byte, so
`content: '•'` produced byte-identical CSS under a different class name and the
two compilers could not be mixed across SSR and client.

Fixed on branch `fix_utf16_hash_parity`:

- `create_hash` / `create_short_hash` — hash UTF-16 code units (`#1248`)
- `create_short_hash` — `to_base62` zero case returns `""` like `toBase62`
- `char_code_at` — index by UTF-16 code unit, return one code unit
- `dashify` — the borrowed fast path no longer skips the lowercasing
- `IS_CSS_VAR` — ASCII name class, since `\w` is Unicode-aware here

The issues in this directory are what that audit surfaced and deliberately did
**not** fix, either because the fix is architectural or because the divergence
sits outside the encoding class the audit was scoped to.

## Scope of the original audit

The encoding class (code units vs bytes vs scalars) swept across all crates,
plus a line-by-line re-read of `hash.rs` / `string.rs` against upstream
`hash.js` / `dashify.js`. Cleared as boundary-safe: `DASHIFY_REGEX` equivalence,
and the `val[4..val.len() - 1]` `var()` unwraps in
`convert_style_to_class_name`, `flat_map_expanded_shorthands`,
`flatten_raw_style_object`, and `evaluate_stylex_create_arg` (ASCII delimiters).
