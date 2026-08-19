# 10 — What `null` and a boolean each mean as a style value

Status: done
Phase: Phase 2

**What was built:** one answer to "what may a style value be", reached from every
position a value can sit in, and a `null` that declares an absent value wherever
it is written rather than sometimes vanishing.

## The filing was wrong about the symptom

Filed as "`null` is rejected here and dropped upstream", from a table read off
`validators.rs:646`. Re-measured at `dc867a165` against
`@stylexjs/babel-plugin@0.19.0`, that does not reproduce: `Lit::Null` has been in
the accepted literal set of `validate_namespace` since `b945607c1`, so
`color: null` and `borderTop: null` both compiled, and both already agreed with
upstream. The line rejects a boolean, which upstream also rejects, with the same
message.

Re-measuring the whole neighbourhood found two divergences that are real, in the
opposite direction from the one filed.

| value                       | upstream                 | this compiler, before  |
| --------------------------- | ------------------------ | ---------------------- |
| `color: null`               | key present, `null`      | agreed                 |
| `color: false`              | rejects                  | agreed                 |
| `color: [null]`             | key present, `null`      | **no key at all**      |
| `color: { default: false }` | rejects                  | **compiles, drops it** |
| `color: { default: /a/ }`   | rejects                  | **compiles, drops it** |
| `{ ':hover': null }`        | key named for the pseudo | no key (see below)     |

The last two of the three marked changed which builds compile; the first changed
emitted output.

### An array carrying only `null` lost its property

`flatten_raw_style_object` skipped a `null` array element outright, so an array
holding nothing else registered no property and the key vanished. Upstream runs
the expansion for the element and then filters, so the property exists with an
empty value list and becomes a `NullPreRule` --
`flatten-raw-style-obj.js:103-117`.

The distinction matters because a key carrying `null` is what *unsets* an earlier
declaration of the same property on merge. A missing key leaves it standing. Both
emit no CSS, which is why the parity corpus could not see it -- see below.

`color: []` is the neighbouring case that legitimately has no key: there is no
property to declare an absence of. The two answers must stay apart.

### A value under a condition was held to a different standard

`validate_namespace` allowed `Str | Null | Num | BigInt`.
`validate_conditional_styles` allowed `Expr::Lit(_)` -- any literal at all. So a
boolean written as `{ color: false }` was refused and the same boolean written as
`{ color: { default: false } }` compiled with the declaration silently dropped,
on every condition kind: `default`, a pseudo-class, an at-rule, an attribute
selector, and nested arbitrarily deep. A regular expression behaved the same way.

Fixed by naming the set once -- `is_style_value_literal` in `validators.rs` -- and
reaching it from all four positions. Two spellings of "what a style value may be"
are exactly what let a boolean compile in one position and fail in the other.

The conditional array position now also refuses through the validator, with
`ILLEGAL_PROP_VALUE`, which is the constant upstream reports there
(`basic-validation.js:86`) rather than the array-specific one. Both compilers
refuse either way.

## What was decided, and against what

- **`null` is an absent value, everywhere.** It declares nothing, keeps its
  property, and drops out of a fallback chain without breaking the contiguity of
  the `var()` entries around it.
- **A boolean is not a style value.** Inside `create` it is refused. In
  `keyframes` and `positionTry`, which validate no values at all, it is dropped --
  and so is a `null`. Both compilers agree on all three, which is why the
  `Lit::Bool` arm in `flat_map_expanded_shorthands` stays: it is reachable, just
  not from `create`.
- The comment on that arm claimed upstream drops a boolean. Half right, and now
  says which half: dropped where nothing validates it, refused where something
  does.
- **A big integer moved to the refused side** (`c82f75fcd`). It was listed as
  allowed because nothing reaches it -- evaluation deopts on `BigIntLiteral`
  first, in every position, and still does. But the set is now the single answer
  to what a style value may be, and upstream's answer is `typeof val ===
  'number'`, which a big integer is not. Listing it as allowed left a divergence
  one change away from landing silently. No reachable behaviour changes.
- **The spread refusal in `validate_namespace` is gone** (`2c7f8eb8c`).
  Unreachable: the validator runs on an evaluated namespace, so a spread the
  evaluator resolved is already the value spread and the entry rule refuses that,
  while one it could not resolve deopts before validation runs. Established by
  sentinel over fifteen spread shapes with zero hits, eight of which are now
  tests. Removing it collapses the two array positions into one rule, which is
  the last of the four spellings this file had of it.

Not matched, deliberately: `{ ':hover': null }` and `{ '@media print': null }`.
Upstream treats the condition key itself as a property name and emits a key
hashed from `:hover` carrying `null`. Neither key can carry CSS, and neither can
collide with a real declaration made under the same condition, so it is inert --
reproducing a property named after a pseudo-class would bake an upstream defect
in rather than fix anything. Recorded in the corpus as structurally divergent,
with the reasoning, on the same footing as the non-goals in `spec.md`. File
upstream.

## The corpus could not ask the question

A `null` value emits no CSS, and the parity harness compared class names and rule
text only -- so both compilers "agreed" on every entry in this ticket whether or
not they agreed about the key. The harness now also reads the *shape* of each
`$$css`-marked style object: which keys exist, and per key whether it carries a
class name or an absence. Class names are replaced by a placeholder there, so a
hash divergence is still reported once, by the half that already reports it.

Not a comparison of emitted JavaScript, which would diverge on every entry for
reasons that are not about StyleX. Run over the whole 814-entry corpus the
extension changed exactly one verdict -- the deliberate divergence above.

## Coverage

- `tests/transform_stylex_create_test/absent_style_values.rs` -- 38 tests. Every
  position a `null` reaches, per style resolution; the fallback-chain and
  `var()`-contiguity cases; custom, vendor-prefixed, numeric and `content`
  properties; a hundred absences in one array; forty nested conditions; an
  absence beside an unclosed function, an unbalanced paren, a blank string, an
  `!important`, and non-ASCII property names.
- `tests/validation_stylex_create_test/invalid_values.rs` -- 19 refusals, each
  message measured against upstream rather than written by hand.
- `parity/corpus/modules.json` -- four module entries, since a declaration entry
  cannot carry a bare `null` through `JSON.stringify`.

## Release note

Changes emitted output and which builds compile:

- A fallback array carrying only `null` now keeps its property, unset, instead of
  dropping it. Merging behaviour changes for such a declaration.
- A boolean or a regular expression written as a style value under a condition is
  now a build error, where it previously compiled and silently declared nothing.
