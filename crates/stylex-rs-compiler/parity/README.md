# CSS value parity harness

Runs a corpus of CSS declarations through `@stylexswc/rs-compiler` and through
a pinned `@stylexjs/babel-plugin`, and reports — per declaration — whether the
two produce the same class name, the same rule text, and the same style-object
shape.

A StyleX class name is a hash of the canonical declaration text, so that text
is a compatibility contract. Any setup that mixes the two compilers (SSR built
by one and client bundles by the other, cached HTML, an incremental migration,
snapshot tests written against either) breaks silently when they disagree: the
markup names a class the stylesheet does not define, and nothing errors.

This is a developer tool, not a test. It lives outside the Rust test suite so
`cargo test` never needs a Node toolchain. The runner is not wired into CI — it
needs a built `dist/` and a Node toolchain — but the harvester's own unit tests
are, under this package's `vitest` suite, and `parity:harvest:check` reports a
stale corpus without writing one.

## Running it

```sh
pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
pnpm run --filter=@stylexswc/rs-compiler parity
```

The harness loads the compiler from `dist/`, not from the Rust sources. **A
report is only about the last build.** Rebuild after touching a crate or the
verdicts are stale.

It exits non-zero for exactly one reason: an entry whose recorded `expected`
verdict no longer holds. A divergence with no expectation recorded against it is
information for a person to read, not a failure.

| Flag                    | Effect                                            |
| ----------------------- | ------------------------------------------------- |
| `--only-mismatches`     | print only the entries that disagree              |
| `--set <name>`          | limit to one corpus set; repeatable               |
| `--filter <substring>`  | limit to entries whose subject text contains it   |
| `--json <path>`         | also write the full machine-readable report       |
| `--font-size-px-to-rem` | enable the font-size conversion in both compilers |

## Reading a verdict

| Verdict                  | Meaning                                                                                                                                                                                                                                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `identical`              | Both accepted and agreed byte for byte.                                                                                                                                                                                                                                                              |
| `identical (empty)`      | Both accepted and emitted **nothing**. Agreement about no declaration at all, which is not evidence of parity — a property both compilers drop, or a fixture that failed to carry the value in, agrees just as loudly. Counted apart so a corpus that stops measuring anything shows up as a number. |
| `divergent`              | Both emitted the same properties but spelled a value differently — and therefore hashed a different class name. This is what the harness is for.                                                                                                                                                     |
| `structurally divergent` | Both accepted but emitted different properties, or a different number of them. The divergence is in shorthand expansion or property validation, not in value normalization.                                                                                                                          |
| `acceptance divergent`   | One accepted and the other rejected.                                                                                                                                                                                                                                                                 |
| `both reject`            | Both rejected, and complained about the same thing. What is compared is the sentence each wrote, with the text saying _where_ it happened removed — see **Comparing two refusals** below.                                                                                                            |
| `both reject (diverged)` | Both rejected, for reasons they word differently. A refused build hands the author nothing but the message, so two compilers stopping one for opposite reasons have diverged in the only behaviour a refused input has.                                                                              |

Both compilers receive the same module text and the same option object,
constructed once in `lib/compare.ts`. Option drift would surface as a
normalization divergence and send the reader chasing the wrong thing, so
options are never spelled out per subject.

Right-to-left rule text is compared but never printed. An RTL-only difference
is a value difference all the same, and without comparing it a verdict would
call such a pair identical; the left-to-right spelling shows what changed more
plainly, so that is what the report displays.

The **style-object shape** is the other half of an answer the rule text cannot
carry. A property whose value is `null` emits no CSS, so two compilers that
disagree about whether the property exists at all agree on every rule — and a
`null` is how an absent value is spelled, which is what unsets an earlier
declaration of the same property when two styles merge. `lib/style-object.ts`
reads each `$$css`-marked object literal out of the emitted module and records
which keys exist and, per key, whether it carries a class name or an absence.
Class names are replaced by a placeholder there, so a hash divergence is
reported once, by the half that already reports it. The shape is printed only
when it is what differs.

This is deliberately not a comparison of emitted JavaScript. The two compilers
print code differently — which consumed declarations they leave standing, how an
injection is wrapped, JSX spacing — so comparing the text would report a
divergence on every entry and say nothing about StyleX.

### Comparing two refusals

A verdict that asked only _whether_ both compilers rejected read two refusals
for opposite reasons as agreement, and that is most of what a corpus of
degenerate values holds. What blocked comparing the messages was never the
comparison but the decoration each compiler wraps its complaint in:

```text
[StyleX] a > color > Invalid pseudo or at-rule.
/abs/path/to/value.js: Invalid pseudo or at-rule.
```

Neither wrapper can be hard-coded away — this compiler's carries the
evaluator's key path, which is the authored object's own keys, and upstream's
carries an absolute file path — so `lib/refusal.ts` derives it, from the marker
this compiler brands every diagnostic with and from the filename the harness
itself handed both compilers. It strips only what says _where_: the marker and
the breadcrumbs, the `-->` location line, the repaired rule text a CSS refusal
carries, and upstream's code frame. What survives is the complaint, newlines
included — several diagnostics are two lines in both compilers, and the second
carries the advice.

Every rule is pinned in `__tests__/refusal.test.ts`, including a refusal with no
prefix at all and one whose complaint contains a colon. The reduced sentence is
carried on the outcome as `sentence`, beside the message as it was thrown: the
verdict compares the sentence, the human report prints it, and `--json` keeps
the raw message as the evidence for both.

The upstream version is held by the lockfile, not by an exact range in the
catalog. The subject block prints the version actually resolved, so a report is
attributable either way — but after a `pnpm update`, read that block before
comparing against an older report.

### Comparing where a refusal points

Stripping the text that says _where_ is what makes two messages comparable, and
it leaves the position unmeasured — so a diagnostic naming a line that is correct
as written reads as agreement here. A refused build hands an author two things,
and that is the second one.

`pnpm parity:positions` compares it, over `corpus/positions.json`: one subject
per branch of the reference-resolution chain, each refused by both compilers with
the same sentence so that the position is all that is left to disagree about.
Verdicts are `identical`, `divergent`, `no-position` — one side stopped without
saying where — and `not-refused`. An entry may pin a known divergence with
`expected`, and the run exits non-zero when an entry's verdict is not what it
expects, in either direction.

```sh
pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
pnpm run --filter=@stylexswc/rs-compiler parity:positions
```

Each subject runs in a **child process**, because the two positions arrive in
different channels: upstream throws its `@babel/code-frame` excerpt inside the
message, while this compiler writes a code frame to stderr and throws the
sentence alone. Node cannot redirect its own file descriptor 2 and a native write
goes straight to it, so capturing that frame means being the parent of the
process that wrote it. `lib/position.ts` parses both, and every shape either
compiler produces is pinned in `__tests__/position.test.ts` — a parser that
silently misreads a frame would turn the whole set green.

The subject is written to `parity/__fixture__/positions.js` while the run lasts,
because this compiler locates a refusal in the file it names rather than in the
string it was handed. That path is git-ignored.

## The corpus

Four checked-in JSON files under `corpus/`, loaded in this order, with
duplicate subjects collapsed onto the first entry seen. `positions.json` is a
fifth, read only by `parity:positions` — its subjects ask where a refusal points
rather than what either compiler emitted, so running them through the value
comparison would report nine refusals nobody wrote them to measure:

- **`reported.json`** — the six divergences reported in issue #1256, one entry
  per illustrating value. Hand-written.
- **`modules.json`** — whole modules rather than declarations, for the
  questions a declaration cannot ask. Hand-written; see below.
- **`edge.json`** — non-ASCII content, escape sequences, URL bodies containing
  CSS-looking syntax, comments, importance annotations, empty values, and
  unclosed constructs. Hand-written.
- **`harvested.json`** — the CSS declarations the Rust test suites carry, as
  far as the scan below recognizes them. **Generated — do not edit.**

Adding a case means editing one of the three hand-written files. Entries take
an optional `note`, which the report prints next to a mismatch, and an optional
`expected` naming the verdict the entry is known to read.

An `expected` verdict is how a divergence someone has already looked at is told
apart from a new one. While it holds, the report marks the entry `(expected)`
and `--only-mismatches` leaves it out. When it stops holding — in either
direction — the entry is listed under **Verdicts that changed**, counted, and the
run exits non-zero, so a divergence that quietly goes away is as loud as a new
one: an entry recording a divergence that no longer happens has stopped measuring
what it was written for. `note` says _why_; `expected` is what the harness
checks, and the only thing it fails on.

### Module subjects

A declaration entry is `{ id, property, value, origin }` and is wrapped in the
smallest module that carries it. A module entry is `{ id, label, source,
origin }` and is handed to both compilers verbatim. Which one an entry is comes
from whether it carries a `source`, so nothing has to be written down twice and
the generated `harvested.json` stays free of a field whose value never varies.

Most questions about this compiler are declaration questions, because a class
name is a hash of declaration text. A few are not: whether an expression the
evaluator cannot fold is _refused_ or _aborts the build_ is a fact about a
module, and it is what `corpus/modules.json` measures — the inputs reported in
[#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265), where a
method call inside a runtime `sx` condition failed the build.

The comparison is the same one: class names, rule text and style-object shape,
never the emitted JavaScript as text — `ModuleEntry` in `lib/types.ts` says why,
next to the type that would have to change to do otherwise. What a module
subject adds is the ability to ask whether a compiler _reached_ the rules at
all, which is the `acceptance divergent` verdict, and to ask a question a
declaration cannot spell: a `DeclarationEntry` value is a string that goes
through `JSON.stringify`, so a bare `null` or `false` can only be asked as a
module.

What a module subject **cannot** ask is anything about its own filename. Every
subject is handed the same one, deliberately — `haste` resolution and class
hashing both read it, so varying it per entry would vary the output for reasons
unrelated to the subject — and that filename is not a `*.stylex.js` one. A
`defineVars` call hashes the file that declares it, so such a subject refuses for
the filename in both compilers before the value under test is ever read: the
entry reports `both reject` and measures nothing. Ask that one where the filename
is a parameter, which is the Rust suites.

Most entries in that set carry an `expected` verdict, each with the `note` that
says why: some where upstream aborts and this compiler does not, some where
both reject, a few where both reject and word it differently, one where upstream
folds an indexed read this compiler refuses,
one where upstream reads a condition key as a property name and emits a key
named after a pseudo-class (a defect this compiler is not going to
reproduce), and the
shadowing shapes from
[#1266](https://github.com/Dwlad90/stylex-swc-plugin/issues/1266), which record
`identical` so a regression in binding-aware import resolution reports as a
changed verdict rather than as silence. The count is deliberately not written
here -- it went stale twice.

Not every entry in the set is a question about the evaluator. A row records that
the two compilers order three nested pseudo-classes differently and therefore
hash different class names, which a module is the smallest way to ask: a
condition tree is a style value, not a declaration. It was found by a guard
written for something else, which is the usual way a row like that arrives.

Other entries carry a `note` without an expectation, saying why a subject that
reads `identical` still earns one -- the shorthand rejection table having once
diverged, for instance.

### Regenerating the harvest

```sh
pnpm run --filter=@stylexswc/rs-compiler parity:harvest
# --check reports staleness and writes nothing:
pnpm run --filter=@stylexswc/rs-compiler parity:harvest -- --check
```

`harvest-corpus.ts` scans the test sources of `stylex-css` and
`stylex-transform` for seven literal shapes that carry a CSS declaration. Each
entry records the `<path>:<line>` it came from, so an unexpected one can be
traced back to the test that motivated it. Run this after adding tests that
carry CSS values.

| Shape | Written as                                                      | What is taken                                                  |
| ----- | --------------------------------------------------------------- | -------------------------------------------------------------- |
| 1     | `normalize_css_property_value("color", "red", &opts)`           | both literals                                                  |
| 2     | a case table looped through one property                        | the first element of each row, or each element of a flat array |
| 3     | `"* {{ transitionProperty: opacity; }}"`                        | the declaration inside the rule                                |
| 4     | `"*{color:red}"`                                                | the same, minified                                             |
| 5     | a `stylex.create` object in a transform fixture                 | every declaration in it                                        |
| 6     | `unchanged("color", "red")`, `same("color", "#ff0000", "#f00")` | the property and the **input** only                            |
| 7     | `rejects("width", &["*(", "/.5 *("], MESSAGE, &opts)`           | the property and every value in the slice                      |

Shapes 6 and 7 are the two worth knowing about, because what they _omit_ is
deliberate. A verdict case row carries the expected output and the reference
compiler's spelling after the input; a rejection table carries the diagnostic
the value is expected to fail with. Neither is harvested. Deriving an
expectation from the reference compiler is the whole point of the harness, so an
expectation already written down must never become an input to it — that would
have the corpus confirming what the tests already assert.

Two guards keep the scan honest, and both matter because a CSS value is
arbitrary text that can spell anything:

- Call sites are found on the **masked** source, in which every string literal
  is blanked to spaces. A fixture value that spells `unchanged("color", "x")` is
  data, not a call.
- The first two arguments of a shape 1 or 6 call must be **adjacent** — nothing
  but whitespace before the first literal, nothing but a comma between them. A
  call whose value argument is an identifier would otherwise pair its property
  with whatever literal came next, which is usually the expected output.

### The generation chain

Adding a Rust test that carries a CSS value invalidates a checked-in fixture in
a _different_ crate. The order is:

```text
Rust test sources
  -> pnpm --filter=@stylexswc/rs-compiler parity:harvest
       -> parity/corpus/harvested.json
            -> pnpm --filter=@stylexswc/postcss-value-parser generate:value-parser-cases
                 -> crates/postcss-value-parser/src/tests/cases.rs
```

`cases.rs` row order _is_ the corpus order, so anything that reorders the corpus
rewrites it wholesale. Both steps have a `:check` form that reports staleness
without writing, and the second runs as part of its package's `test` script.

The scan is a heuristic over Rust sources, so the corpus contains some values
that are not valid CSS — degenerate inputs to the whitespace-repair unit tests,
keys that are pseudo-selectors rather than properties. They are kept: they cost
two compiler runs each, and filtering them by guesswork risks dropping a real
divergence. They land in the `structurally divergent`, `acceptance divergent`,
`both reject` and `both reject (diverged)` buckets, which is why those are
counted apart from `divergent`.

## The generated corpus: `pnpm fuzz:shorthand`

The corpus above is curated — every entry came from a test file or from someone
writing it down. `fuzz-shorthand-split.ts` asks the same question over a
generated one, and it exists because one defect class is not reachable by
curation: a shorthand value is cut into parts, and where the cut falls depends
on which separator, which spacing and which token shape happen to sit next to
each other. Hand-picked probes twice reported a small remainder that a generated
alphabet then multiplied.

It differs from `pnpm parity` in three ways worth knowing before reading a
number from it.

- **It pins the style resolution.** Value splitting runs only under
  `legacy-expand-shorthands`; under the other two a `padding` reaches the
  stylesheet whole. A run left on the default compares two compilers that both
  never called the code, and reports agreement.
- **It reports its alphabet, not a score.** What a run can claim is the token
  classes it crossed. The class list, the joiner list and the property list are
  printed above the counts precisely so that a reader compares the two rather
  than quoting the count alone.
- **It has no expectations.** There is no `expected` verdict to change, so it
  cannot fail. It prints what the two compilers did and the reading is entirely
  a person's.

```sh
pnpm fuzz:shorthand                                     # summary
pnpm fuzz:shorthand --show 40                            # print divergent rows
pnpm fuzz:shorthand --property padding                   # one property
pnpm fuzz:shorthand --json parity/results/<name>.json    # full report
```

A row it reports is not yet a defect: the alphabet deliberately includes values
that are not valid CSS, and the same buckets that are counted apart in the
curated report are counted apart here for the same reason. Two groups are
deliberate refusals rather than bugs, and between them they account for every
`acceptance divergent` row the harness currently prints:

- **A value carrying a `;`** — the largest group by far. This compiler refuses
  one and the reference compiler does not, because the value reaches the
  stylesheet as the author's own bytes and a `;` would terminate the declaration
  the compiler is generating.
- **A value nested past 64 levels** — refused for the recursion budget, which is
  a stack the reference implementation does not have to answer for.

Neither has anything to do with where a value is cut.

## Checking a future upstream release

1. Bump `@stylexjs/babel-plugin` in this package's `devDependencies` and
   `pnpm install`.
2. Rebuild the compiler, then run
   `pnpm parity --json parity/results/<version>.json`.
3. The printed subject block states both versions, so a report is always
   attributable to a specific release.
4. Diff the new report's `divergent` set against the previous one. Anything new
   is an upstream normalization change this compiler has not adopted.

## Using a report to write test expectations

Never write or update an expectation by eye — a hand-edited expectation in this
area is just the bug re-encoded. Take it from `--json` output instead. Each
report entry carries, for both compilers:

- `classNames` — the generated class names, in emission order
- `rules` — the full LTR rule text
- `declarations` — the `property:value` text inside the braces, which is what
  value normalization produces and therefore what the value-normalization tests
  assert against
- `styleObjects` — the shape of each `$$css`-marked style object, which is where
  an absent value shows and the rule text does not

`entries[].babel.declarations` is the expectation; `entries[].rust` is what this
compiler produces today.

**A verdict with no declarations is not a verdict.** Both compilers expand or
drop shorthands such as `background` and `border`, so a subject written on one
produces an empty `declarations` array on both sides and the entry is reported
`identical` — agreement about nothing. Check that `entries[].rust.declarations`
and `entries[].babel.declarations` are non-empty before reading an `identical`
as evidence, and pick a longhand (`backgroundImage`, `boxShadow`) when writing
the subject. The exception is a subject whose whole point is a `null` value:
there `declarations` is empty on both sides by design, and `styleObjects` is
what the verdict rests on.

**One caveat when the expectation is for a value-normalization test.** The
harness runs the whole transform, and a few properties never reach value
normalization at all: `content`, `hyphenateCharacter` and `hyphenate-character`
are returned verbatim when their value already carries matching quotes, names a
content function, or is a content keyword. A verdict on one of those describes
the transform path, not the normalization seam, and the two disagree — `content:
"\2014 A"` survives the transform unchanged but has its escape resolved at the
seam. Pick a property that reaches normalization (`fontFamily` covers the same
string shapes) when sourcing an expectation for a normalization test.
