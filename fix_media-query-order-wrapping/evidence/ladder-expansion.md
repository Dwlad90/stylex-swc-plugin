# Ticket 05 — the coverage-suite ladder, re-derived

Ticket 02's table covers every `@media` key the last-media-query-wins transform
rewrites. One expectation had to change in ticket 05 that the table does not
carry, because it is not such a key: the coverage suite parses a media query
string directly rather than reaching it through a conditional value map, so it
has no `stylex.create` input for the emitted-CSS measurement to run on.

This is the run that justifies changing it.

- expectation:
  `crates/stylex-css-parser/src/tests/at_queries/media_query_coverage_test.rs`,
  the section formerly headed `DeMorgan distribution pruning`
- input: the query the transform builds for the first rung of a thirteen-rung
  ladder of disjoint ranges — `(min-width: 100px) and (max-width: 200px)`
  followed by twelve negated neighbours at `300..400`, `500..600`, and so on
- reference implementation: `@stylexjs/babel-plugin` 0.19.0, the version
  ticket 02 recorded

## What it pinned, and why that was wrong

The expectation was `@media (min-width: 100px) and (max-width: 200px)` — the
authored range, with every negation gone. That is the shortcut's output, not the
reference implementation's. The reference keeps each contradictory branch, so
its answer for the same input is a nest of disjunctions fifteen kilobytes wide.

## The equivalent module

`ladder-expansion-input.js`, beside this file, is the thirteen-rung ladder as a
`stylex.create` module. Run it through `probe`-style loading of `ref.cjs` and
read the first `@media` prelude, which is the key under discussion:

```sh
node -e "const {run}=require('./ref.cjs');const fs=require('fs');
  const p=run(fs.readFileSync('./ladder-expansion-input.js','utf8')).rules
    .flatMap(r=>(r?.[1]?.ltr??'').match(/@media[^{]*/g)??[]);
  const first=p[0].trim();
  console.log(first.length, first.split('not all').length-1);"
```

## The short ladder, for the readable half of the expectation

The corrected expectation is asserted twice: once at four negations, where the
whole result fits in a literal, and once at twelve, where it does not. The
four-rung run, same harness:

```text
@media ((min-width: 100px) and (max-width: 200px)) or (not all)
@media (min-width: 300px) and (max-width: 400px)
@media (min-width: 500px) and (max-width: 600px)
@media (min-width: 700px) and (max-width: 800px)
```

The first key is the one under test. One dead branch survives as `not all`
beside the authored range, which is the smallest visible form of the behaviour
ticket 05 restored. Both compilers emit exactly this, and all five class names
agree.

## What both compilers now emit

Byte for byte identical, and all fourteen class names agree:

| Measure                              | Reference | This compiler |
| ------------------------------------ | --------- | ------------- |
| first prelude, characters            | 15393     | 15393         |
| occurrences of `not all` in it       | 1023      | 1023          |
| occurrences of `(min-width: 100px)`  | 1         | 1             |
| class names differing, of fourteen   | —         | 0             |

The three numbers are what the corrected expectation asserts. A literal of that
length tells a reader nothing, while length, dead-branch count, and the single
surviving authored range all move under any change to the expansion.

## The one row ticket 02 flagged

`u03` — `basic_usage_nested_query_with_padding` — is **not** corrected here, and
that is the right outcome. Its flag is an artefact of how the table was
measured: a row is read through emitted CSS, at-rule sorting can reorder a
rule's nested preludes, and the expectation is a unit-seam one that sits before
that sort. The table's third column settles it — both compilers emit the same
CSS for that input and all six class names agree — so there is nothing for the
reference implementation to contradict. The expectation is untouched and still
passes.
