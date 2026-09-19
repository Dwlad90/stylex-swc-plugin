# 73 — Read a variable-reference key the way the reference does

**What to settle:** `flatten_raw_style_object_logic` asks
`CSS_VALUE_SPLIT_REGEX` whether a style key is a `var()` reference and, on a
match, names the key by what it wraps:

```rust
let css_property_key: Cow<'_, str> = if CSS_VALUE_SPLIT_REGEX
  .is_match(&key)
  .unwrap_or_else(report_unmatched_key)
{
  Cow::Owned(key[4..key.len() - 1].to_string())
} else {
  Cow::Borrowed(key.as_str())
};
```

That expression is `(\))(\S)|(\")(\")` — a bracket followed by a non-space, or
two quotes. It is wrong in both directions.

**It misses the key it is named for.** `var(--x1a2b3)` ends at the bracket, so
no non-space follows it and the match fails. The key keeps the spelling
`var(--x1a2b3)` through the whole flattening — the key path, the conditional
test, and the name the rule is filed under. The reference implementation
(`preprocess-rules/flatten-raw-style-obj.js:47` of the Babel plugin) asks
`/var\(--[a-z0-9]+\)/` and names the key `--x1a2b3` at this point.

Nothing fails today because the strip also happens one layer down, in
`flat_map_expanded_shorthands`, which asks `starts_with("var(") &&
ends_with(')')` — the reference asks the same question in the same place. So
the *property* a rule is filed under is right, while the key path and the
conditional test still carry the unstripped spelling.

**It matches keys that are not variable references.** Any key holding a
bracket with a non-space after it matches.
`@media (min-width:1px),(max-width:2px)` is one, and so is
`:not(.a)::before`. Such a key is then named
`key[4..len - 1]`, which is text cut out of the middle of it. The slice is a
byte slice, so a key carrying a character outside ASCII in that range stops the
build rather than garbling the name.

**What has to be decided, and why it is not a one-line change:**

- Anchored or not. The reference asks an unanchored question and then slices
  `4..-1` regardless, so a `defineConsts` placeholder written inside a longer
  at-rule (`@media (min-width: var(--x1a2b3))`) is named by text cut out of its
  middle there too. An anchored `^var\(--[a-z0-9]+\)$` answers sensibly for that
  key and diverges from the reference on it. Measure the reference first.
- The character class. The reference spells `--[a-z0-9]+` here and
  `--[a-zA-Z0-9-_]+` in `stylex-first-that-works.js`. `IS_CSS_VAR` in
  `stylex-regex` already mirrors the second. A name holding a dash, an
  underscore or a capital is read as a variable reference by one and not by the
  other.
- What moves once the key is named. `is_conditional_key`, `normalize_key_path`,
  the name the rule is filed under, and the inner key a condition is rewritten
  with all read `css_property_key`. Each has to be measured against the
  reference on a `defineConsts` at-rule placeholder, which is the one input that
  reaches this branch on purpose.

Whatever is decided, write the source that reaches the branch: today it is the
last unexercised region of `flatten_raw_style_object.rs`, and it is what holds
the `flatten_raw_style_object_logic` gate group open.

**Blocked by:** nothing. Splits out of
[63](./63-cover-the-transform-shared-utils.md).

**Status:** resolved

- [x] The question the branch asks matches the reference, measured with
      `pnpm run parity:probe` on twelve keys -- see the comment below. The
      expression is now `var\(--[a-z0-9]+\)`, unanchored.
- [x] No key can stop the build by being sliced through a character. The name
      is cut in characters rather than in bytes.
- [x] `scripts/coverage-missing.sh -p stylex_transform` reports no unexercised
      region in `flatten_raw_style_object.rs`, and the
      `flatten_raw_style_object_logic` gate group is closed. The file reads
      100.00% of regions, functions and lines, and the crate is at 33 regions
      over thirteen files, from 36 over fourteen.

      **Restated against what is measured.** The file also gained two
      `coverage(off)` attributes on this branch, at `or_refuse_handled_template`
      and `report_unmatched_key`, so part of the 741-to-0 fall lands by
      exclusion rather than by test. Measured today the file is 228 regions with
      none missed, and those two functions are not among them. Each attribute
      names why its step is total, and the second one says outright that neither
      half of it is reachable from a source -- which is the claim a reader should
      weigh, not the percentage.

## Comments

### What was decided, and what measured it

**Unanchored, and the reference name class.** The expression is
`var\(--[a-z0-9]+\)` asked of the whole key, and the name is the key without
its first four characters and its last one. An anchored question was measured
and dropped: a reference written inside a longer at-rule
(`@media (min-width: var(--x1a2b3))`) is named by text cut out of the middle of
that at-rule, and the reference implementation declares exactly the same
nonsense rule for it -- same class name, same declaration. Anchoring would have
diverged on the one key that tells the two spellings apart.

**Counted in units, which is what JavaScript counts.** Because the question is
unanchored, a key can carry text outside ASCII at either end of the reference.
The old slice was counted in bytes and stopped the build on such a key.
Counting characters instead does not stop the build and still names the key
differently from the reference, because JavaScript spells some characters with
two units: `🎈var(--x1a2b3)x` is named `r(--x1a2b3)` there and was named
`(--x1a2b3)` here, with a class name of its own. The name is cut in units now,
and that key agrees.

One case is left over and cannot be closed: a cut that falls between the two
units of one character. The reference gives one half of it -- a lone surrogate,
which this compiler cannot carry in a name -- so the whole character is kept
instead. `var(--x1a2b3)🎈` and `abc🎈var(--x1a2b3)x` are the two shapes, and the
rule the reference declares for each carries a lone surrogate in its property
name.

**The named key is now read where the reference reads it.** Four sites read the
authored key where the reference reads the named one: the key path a scalar
normalises, the key path a template literal normalises, the question that
decides whether an object value is a condition, and the name a condition files
its rules under. They all read the named key now. The fallback-list arm keeps
the authored key, because the reference asks that one arm differently.

No measured output moves. A key is named only where it holds a reference, and
each of the four sites was probed on such a key.

### The twelve keys, probed against the Babel plugin

`pnpm run parity:probe`, `@stylexjs/babel-plugin@0.19.0`, on the rebuilt addon.
Ten agree -- same CSS, same class name, same printed module. The last two are
the surrogate case above, which cannot agree.

| Key                                   | Before                                      |
| ------------------------------------- | ------------------------------------------- |
| `var(--x1a2b3)`                       | agreed; named one layer down                |
| `var(--x1a2b3)` carrying conditions   | agreed                                      |
| `var(--x1a2b3)` inside an at-rule     | agreed                                      |
| `a)b`                                 | **stopped the build**: byte index out of bounds |
| `a)bé`                                | **stopped the build**: byte index inside a character |
| `:not(.a)::before` holding a condition | agreed                                     |
| `@media (min-width:1px),(max-width:2px)` | agreed                                   |
| `var(--my-colour)`                    | agreed; outside the name class              |
| `@media (min-width: var(--x1a2b3))`   | agreed; both cut the name out of the middle |
| `🎈var(--x1a2b3)x`                    | agreed, once the cut is counted in units    |
| `var(--x1a2b3)🎈`                     | **diverges**: the reference names a lone surrogate |
| `abc🎈var(--x1a2b3)x`                 | **diverges**: the same, at the other end    |

A tenth key was probed before the change and is not in the table, because the
harness has no case for it: `:not(.a)::before` carrying a string rather than a
condition. It was read as a reference, named `(.a)::befor`, and declared a rule
under that name with a class name of its own. It is a condition again now, and
a condition carrying a string declares nothing here -- which is what the key
`:hover` already does, and a divergence from the reference that predates this
ticket and is left where it stands.

### Where the tests are

Eight unit tests in
`crates/stylex-transform/src/shared/utils/core/tests/flatten_raw_style_object_tests.rs`
and ten compiled modules in
`crates/stylex-transform/tests/transform_stylex_create_test/variable_reference_keys.rs`,
whose snapshots carry the class names the probe recorded.

`CSS_VALUE_SPLIT_REGEX` is gone. It had one caller, and nothing else asked the
question it was named for.
