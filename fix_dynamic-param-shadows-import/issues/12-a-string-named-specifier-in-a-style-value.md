# 12 — A string-named import specifier read in a style value

Status: `resolved`
Blocked by: 07 — the fallback 07 deletes is what makes this reachable by the
aliased-away name, so the shape has to be re-measured after that lands.

**What to build:** A verdict on what a string-named theme import should do in a
style value.

```js
import { "color-lg" as colorLg } from 'vars.stylex.js';
export const styles = stylex.create({ wrapper: { color: colorLg } });
```

The reference implementation refuses with `A style value can only contain an
array, string or number.` We emit **nothing at all** — no rule, no refusal. A
property that silently declares nothing is the worst of the three outcomes: the
markup names a class the stylesheet does not define and nothing errors.

Measured while implementing 02, with no shadowing anywhere in the module. Found
because 02 needed to know whether the imported-name fallback was still live, and
the string-named arm is the half of it that is. Not in the spec's audit table.

Two things to separate, which is why this is its own ticket rather than a note on
07: whether the *lookup* should resolve `colorLg` at all, and why the emit path
answers with silence instead of either a rule or a refusal. 07 deletes the
fallback that resolves the aliased-away name `color-lg`; it does not explain the
silence for the local name.

- [x] The empty result is traced to the code that produces it, not inferred
- [x] A verdict: mirror the refusal, emit the rule, or state why silence is right
- [x] Corpus entry with the verdict it reads
- [x] Re-measured after 07, so the two changes are not confused for each other

## Comments

07 landed, and the re-measurement it was blocking on is half done: reading the
specifier by the name it was aliased *away* from no longer aborts, and refuses
the way the reference implementation refuses. That was a separate fault sharing
one lookup with this one.

This ticket's shape is untouched. `color: colorLg`, read by the *local* binding
`colorLg`, resolved through the local match before 07 and resolves through it
now -- 07 deleted only the imported-name arms. The empty result still stands, and
tracing it to the code that produces it is what remains.

## Answer

**Mirror the refusal -- and it already does.** The silence was one arm, and the
arm is gone: `EvaluateResultValue::ThemeRef(_) => None` in
`nodes/object_expression.rs`, deleted by e1836f1de for
[18](./18-a-theme-object-read-as-a-style-value-is-dropped.md). The caller read
that `None` as "no property", so the declaration was dropped; every arm of that
match now answers an expression or refuses, and a theme reference falls through
to `ILLEGAL_PROP_VALUE` -- `A style value can only contain an array, string or
number.`, byte for byte the reference implementation's sentence for the same
input. Traced by `git log -S` on the arm, not inferred from the outcome.

That closes this ticket's *emit* half without a change of its own. The string
spelling was never what produced the silence: `colorLg` resolved through the
local match before 07 and resolves through it now, and what happened after the
resolution was the theme reference's problem, shared with every other spelling.
This ticket's own question was whether the string spelling reached the same seam,
and it does.

### The lookup half, measured

**Resolve it.** The string is the export name, the alias is only the binding, and
the variable is hashed from the export name -- so `import { "colors" as c }` read
as `c.lg` has to write `var(--x38wx3q)`, the same variable a plain
`import { colors }` writes. Both compilers do. Measured across every spelling a
string export name admits and every position a member read can sit in, against
`@stylexjs/babel-plugin` 0.19.0 under the parity harness's configuration:

| export name | Babel 0.19.0 | here |
| --- | --- | --- |
| `"color-lg"`, not an identifier at all | `--x1vktwfk` | same |
| `""` | `--xs2r0fl` | same |
| `" "` | `--x19ch1gl` | same |
| `"0"` | `--xon846n` | same |
| `"default"` | `--x1t9dovf` | same |
| `"NaN"` | `--x6d9ph1` | same |
| `"a\"b"`, `"a\\b"`, `"a\nb"` | three distinct | the same three |
| `"😀"` (astral) | `--xardzau` | same |
| 1000 characters | `--xy7n7vi` | same |
| `"\ud83d"` (lone surrogate) | refuses in the parser | refuses at the decode |
| one export name, two aliases | one variable | same |

The alias contributes nothing to the hash on either side, which is the property
the two-alias row states as an equality rather than as two snapshots.

Three of those rows say something a reader would not guess:

- **`"default"` is a named specifier.** `import { "default" as d }` names the
  default export, and neither compiler's default-import step ever sees it -- the
  step keys on the specifier *kind*, and this is a `Named` one. Both resolve it
  as the export named `default` and hash that name. The refusal a reader would
  expect from [06](./06-measure-the-default-import-step.md)'s step is the one
  neither compiler gives; recorded rather than changed, because upstream is the
  contract.
- **An alias may take a global's name over.** `import { "color" as NaN }` binds
  `NaN`, and the import step answers before the globals step -- so it resolves to
  the import on both sides, and read bare it is refused as a *value* rather than
  as an uninitialized const. This is the ordering
  [`binding.rs`](../../../crates/stylex-transform/src/shared/utils/js/evaluate/binding.rs)
  step 7 documents, reached through the one spelling where the two steps can name
  one binding.
- **A lone surrogate is the only export name with no hash.** Well-formed UTF-16,
  not well-formed Unicode. Both refuse and neither says the same thing: upstream
  in the parser (`An export name cannot include a lone surrogate`), here where
  the name is decoded (`String value contains invalid UTF-8 encoding.`). Outcome
  shared, wording not -- so the corpus row reads `both-reject` and the sentence is
  pinned in Rust.

### Where the divergences are, and whose they are

Read bare, a string-named specifier diverges in exactly the five positions an
identifier-named one already does -- a spread, a computed key, a `firstThatWorks`
argument, a keyframes step, a `createTheme` override. Every one is the theme
reference's divergence, decided and recorded by 18, reached here through the
other spelling. Nothing about the string spelling adds or removes one, and all
five carry a test; the `createTheme` override has the member read beside it as
the control, because the group handed to that call as its *first* argument is in
the position it belongs in and still resolves.

One divergence found while measuring is **not** this seam's:
`modules-1266-a-string-named-theme-member-eight-conditions-deep` first hashed a
different selector on the two sides, and the cause is
[19](./19-three-nested-pseudo-classes-hash-differently.md) -- four pseudo-classes
nested out of alphabetical order. The row now nests them alphabetically, as
ticket 09's depth guard does for the same reason, so it measures resolution and
19 keeps the ordering. Noted on 19 as a second reproducing shape.

### The unused specifier, checked against upstream

Asked because the snapshot for the specifier nothing reads keeps the import
declaration, and removing an unused import is the kind of thing a reader expects
this compiler to do. It does not, and neither does upstream: measured on
`@stylexjs/babel-plugin` 0.19.0, an unused theme import survives in every
spelling -- named, aliased, string-named, default, namespace -- and so does an
unused import of a file that is not a theme file. Both compilers agree, and the
declaration stays.

The elision that does happen here belongs to a different pass. `typescript_strip`
(`stylex-rs-compiler/src/lib.rs`) drops an import whose binding has no value
reference, and only for TypeScript syntax:

| module | identifier-named, unused | string-named, unused |
| --- | --- | --- |
| `value.js` | kept | kept |
| `value.ts` | dropped | dropped |
| `value.tsx` | dropped | dropped |

The two spellings are treated alike, so nothing about a string export name is
special here -- which is the question this ticket had to answer. `cargo test`
runs the transform without that pass, so the snapshot records what the transform
produces, which is also what upstream produces.
[24](./24-the-typescript-half-of-the-stripped-specifier.md) owns the pass.

### What landed

No production change -- the fault was closed by 18 at the same seam, and a second
fix would have been a second spelling of one refusal.

- `crates/stylex-transform/tests/validation_stylex_create_test/string_named_import_specifiers.rs`
  -- 40 tests, both halves in one file: the lookup pinned by snapshot on class
  names and variables verified against upstream row by row, the emit pinned by
  message, which the corpus cannot do
  ([17](./17-the-corpus-cannot-report-a-changed-refusal.md)). Covers the export
  name's spellings and length boundary, the positions a member read can sit in
  (depth eight, a custom property, a vendor-prefixed property, a fallback array,
  a template, a dynamic style's body), malformed CSS beside a resolved read
  (unclosed function, unterminated quote, unclosed attribute selector), and the
  specifier nothing reads.
- Fifteen corpus rows, `modules-1266-a-string-named-specifier-*` and one for the
  three escape spellings measured against each other, each carrying the verdict
  it reads so a regression reports as a changed verdict. Every row of the table
  above has one, so no claim about upstream in this ticket rests on a
  measurement that is not checked in and re-runnable. The whole `modules` set
  reports `changed 0` over 145 subjects.
- The harvest chain the new tests invalidated, regenerated per
  `crates/stylex-rs-compiler/parity/README.md`: the malformed-CSS neighbour test
  carries `backgroundColor: 'rgb(0,0,'`, which the harvester picks up, so
  `parity/corpus/harvested.json` gained a row and
  `crates/postcss-value-parser/src/tests/cases.rs` was regenerated behind it.
  Caught by review, not by the first pass.
