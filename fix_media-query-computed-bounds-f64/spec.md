# f64 numeric parity in the CSS value parser

Status: ready-for-agent
Issue: https://github.com/Dwlad90/stylex-swc-plugin/issues/1267

## Problem Statement

A team migrating off `@stylexjs/babel-plugin` is checking output byte for byte
so they can skip a QA pass. Two of the four rules in a four-breakpoint
`minHeight` object come out with different class names than the official
compiler, and the media query text differs in the computed upper bound:

```
BABEL  @media (min-width: 25rem) and (max-width: 28.799999999999997rem)  → xu5ieg8
RUST   @media (min-width: 25rem) and (max-width: 28.8rem)                → x1a3nan5

BABEL  @media (min-width: 28.81rem) and (max-width: 32.870000000000005rem) → x1t400y5
RUST   @media (min-width: 28.81rem) and (max-width: 32.870003rem)          → xnl65xh
```

Reproduced on this branch against `@stylexjs/babel-plugin@0.19.0` from
`node_modules`. The class name differs, so the two compilers cannot be mixed
across an SSR and client boundary, and the breakpoint itself lands at a
different width — this is behavioural, not cosmetic.

The reported divergence is one instance of a wider one. Every numeric CSS type
in `stylex-css-parser` stores `f32` where upstream holds a JavaScript `number`,
and every one of them prints through Rust's `Display` where upstream prints
through `String(number)`. Both halves are observable, because the printed
spelling feeds the class-name hash.

## Solution

Numeric CSS types hold `f64`, the width of the JavaScript `number` they mirror,
and print through `stylex_utils::number::to_js_string`, the existing ECMA-262
`Number::toString` port that the rest of the workspace already uses.

Then `28.81 - 0.01` is `28.799999999999997` here exactly as it is upstream, the
class name is `xu5ieg8`, and the same is true of every other value the parser
computes or prints rather than merely echoes.

## Root cause

Three independent losses, all on the path from a parsed token to emitted text:

1. **Storage.** `Length.value` and its dozen siblings are `f32`. The token they
   are built from already carries `f64`, so the narrowing happens at
   construction — `Length::new(value as f32, unit)`.

2. **Emission.** Media query canonicalization already does its interval
   arithmetic in `f64` (deliberately — see `constraint_interval`), but hands the
   result back through `Length::new(lower as f32, …)`, discarding it. This is
   the specific line #1267 reports.

3. **Formatting.** `Display` uses Rust's `{}`, which never switches to
   exponential form, so a value JavaScript spells `1e+21` is spelled with
   twenty-two digits here. Two hand-rolled `format_number` helpers go further
   and *round* to six decimals, which no upstream code path does.

## User Stories

1. As an engineer migrating a codebase from the Babel plugin, I want a computed
   media query upper bound to carry the same digits Babel emits, so that the
   class name matches and I can skip a QA pass.
2. As an engineer migrating a codebase, I want a breakpoint at a fractional
   `rem` value to land at the same width under both compilers, so that my layout
   does not shift when I switch compilers.
3. As an engineer serving CSS from a build and class names from an SSR render, I
   want both compilers to agree on the hash, so that the two halves can be
   mixed during a staged migration.
4. As an author writing `@media (min-width: 28.81rem)`, I want the upper bound
   the last-media-query-wins transform derives for the preceding range to be
   exactly `next - 0.01` in double precision, so that adjacent ranges neither
   overlap nor leave a gap the official compiler would not have.
5. As an author writing four or more overlapping breakpoints, I want every
   derived bound in the chain to match, not just the ones that happen to fall on
   round numbers, so that adding a fractional breakpoint does not silently
   change the output of its neighbours.
6. As an author writing a strict range query such as `(width > 400.5px)`, I want
   the 0.01 nudge computed in double precision, so that the `min-`/`max-` pair
   it rewrites to matches the official compiler.
7. As an author writing a `transform: matrix(...)` with more than six decimal
   places of precision, I want every argument printed in full, so that the
   compiler does not silently round my matrix.
8. As an author writing a `cubic-bezier(...)` easing curve with high-precision
   control points, I want the control points printed in full, so that my
   animation curve is the one I wrote.
9. As an author writing a `transition-duration` in milliseconds that the
   normalizer rewrites to seconds, I want the division done in double
   precision, so that the printed seconds value matches upstream.
10. As an author writing a fractional `Hz` frequency the parser rewrites to
    `KHz`, I want the division done in double precision.
11. As an author writing a percentage the parser derives from a unitless number,
    I want the multiplication done in double precision, even where that prints a
    longer value than before.
12. As an author writing a colour in a precise space — `oklch`, `lab`, `oklab`,
    `lch` — I want each channel held and printed at double precision, so that
    the emitted colour is the one the official compiler emits.
13. As an author writing an `rgba`/`hsla` alpha, I want the alpha printed at
    double precision.
14. As an author writing a value at the edge of the double range, I want it
    spelled the way JavaScript spells it — `1e+21`, `1e-7`, `5e-324`,
    `Infinity`, and `0` for negative zero — rather than as a long decimal.
15. As a maintainer of this compiler, I want one place in the crate that turns a
    number into text, so that a future `Display` impl cannot reintroduce the
    divergence by reaching for `{}`.
16. As a maintainer, I want no `as f32` cast left on any path between a parsed
    token and emitted text, so that the class of bug #1267 belongs to is closed
    rather than fixed at one site.
17. As a maintainer reviewing this change, I want the commit that closes #1267
    to be separable from the prophylactic widening of the other types, so that
    one can be reverted without the other.
18. As a maintainer, I want the `constraint_interval` doc comment to stop
    asserting that emission narrows to `f32`, so that it does not mislead the
    next reader.
19. As a contributor adding a new numeric CSS type, I want the crate's
    `CONTEXT.md` to name the double-precision rule, so that I do not have to
    infer it from the surrounding code.
20. As the reporter of #1267, I want the exact input from the issue in the test
    suite with the class names Babel produces, so that a regression is caught
    here rather than in my stylesheet.

## Implementation Decisions

**Widen every numeric CSS type to `f64`.** Not only `Length`. The affected
types are `Length`, `Angle`, `Time`, `Frequency`, `Resolution`, `Calc`'s
dimension, `AlphaValue`, `Percentage`, `Number`, `Flex`'s fraction, and the
colour channel structs — `Rgba`'s alpha, `Hsl`/`Hsla`, `Lch`, `Oklch`, `Lab`,
`Oklab`. Also `MediaRuleValue::Number`, which sits on the same emission path as
`Length`. The field type changes; the field name does not.

Rejected: keeping `f32` and adding an exact side channel that only the media
query merge writes. It is the smaller diff, but it puts two sources of truth
behind one number and leaves the divergence reachable from every other caller.

**Print through `to_js_string`.** `stylex-css-parser` gains a dependency on
`stylex-utils`, which it does not have today. No cycle: `stylex-utils` depends
only on `stylex-regex`; `stylex-css-parser` depends only on `stylex-macros`.
Every `Display` impl in the crate that prints a number adopts the helper —
around fourteen of them — not only those on the media query path. A `{}` left
behind after the widening is a *new* divergence that the `f32` rounding was
previously masking, since Rust's `Display` never goes exponential.

**Delete both `format_number` helpers.** `transform_function.rs` and
`easing_function.rs` each hand-roll `(n * 1_000_000.0).round() / 1_000_000.0`
followed by a trailing-zero trim. Upstream interpolates the number directly
(`` `matrix(${this.a}, …)` ``), which is `String(number)`. Both helpers arrived
in the bulk "add all css parsers" commit with no accompanying rationale, issue,
or test asserting the rounding — they are porting artifacts, not decisions. They
are replaced by `to_js_string`, in their own commit, because their blast radius
(transforms and easing curves are common) is larger and more visible than
#1267's.

**Accept output that gets longer.** Three `Display` impls compute at print
time: `Time` and `Frequency` divide by 1000, `Percentage` multiplies a unitless
number by 100. At `f64` these produce upstream's exact results, which is
sometimes a longer string than the `f32` path produced — `0.07 * 100` is
`7.000000000000001`, not `7`. Where an existing expectation changes, the new
value is confirmed against Babel case by case before the expectation is
updated. A tidier string that upstream does not produce is the bug, not the
baseline.

**Re-derive the `f32` ceiling guard.** `constraint_interval`'s doc comment
justifies its `f64` arithmetic as comparison hygiene and states that emission
narrows to `f32` again; after this change that sentence is false and is
rewritten. The companion guard added in `5fb8dcfa1` ("refuse a ceiling the
boundary cannot represent") is re-examined against its own test: removed only
if the test still passes without it, kept if the representability limit was
guarding something that survives at `f64` scale. The finding is reported either
way rather than decided silently.

**Commit sequence.** Order matters, and it is the opposite of what it first
looks like. The formatter cannot be adopted before the widening: it takes a
double, and widening a single-precision field to a double exposes the rounding
error rather than hiding it — a single-precision `28.8` becomes
`28.799999237060547` when widened, where its own formatting prints `28.8`. So
every type is widened first, printing through Rust's default formatting, which
already produces upstream's exact digits for every realistic value. The shared
formatter lands afterwards and closes only the exponential-form and
negative-zero
edges.

1. `fix(stylexswc/css-parser)` — widen `Length` and `MediaRuleValue` to `f64`
   and
   drop the casts on the media query emission path. **This is the commit that
   closes #1267**, and the one its tests land in.
2. `docs(stylexswc/css-parser)` — rewrite the `constraint_interval` comment and
   record the outcome of the `5fb8dcfa1` re-derivation.
3. `fix(stylexswc/css-parser)` — widen the remaining dimension types to `f64`.
4. `fix(stylexswc/css-parser)` — widen the legacy colour channels to `f64`.
5. `fix(stylexswc/css-parser)` — widen the modern colour channels to `f64`.
6. `refactor(stylexswc/css-parser)` — add the `stylex-utils` dependency and
   print
   the dimension and number types through `to_js_string`.
7. `refactor(stylexswc/css-parser)` — print the colour types through
   `to_js_string`.
8. `fix(stylexswc/css-parser)` — delete `transform_function.rs`'s rounding
   helper.
9. `fix(stylexswc/css-parser)` — delete `easing_function.rs`'s rounding helper.
10. `docs(stylexswc/css-parser)` — add the double-precision rule to the crate's
    `CONTEXT.md` language section.

Broken down as ten tickets in `issues/`, one per commit, each declaring its
blocking edges.

## Testing Decisions

A good test here asserts emitted text and class names — the external, observable
output both compilers agree or disagree on — never a field's type or an
intermediate `f64`. The values asserted are the ones Babel actually produced,
captured from a side-by-side run, not values derived by reasoning about
floating point.

**Seams, highest first.** No new seam. Two existing ones carry this:

- `crates/stylex-transform/tests/` — the end-to-end `stylex.create` seam. This
  is the highest available seam and the one the issue is filed against: source
  in, CSS and class names out. The #1267 input goes here verbatim, pinning all
  four rules including `xu5ieg8` and `x1t400y5`. Class names are pinned
  deliberately: the hash is what the reporter cares about, and a text-only
  assertion would pass through a change that altered hashing.
- `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs`
  and its siblings — the existing unit seam, extended for the types the
  transform seam cannot reach cheaply, and for the JavaScript spelling edges
  (`1e+21`, `1e-7`, negative zero) that no realistic stylesheet reaches.

`stylex-utils`' `to_js_string` already has its own test module
(`crates/stylex-utils/src/tests/number_test.rs`); this change adds no tests
there, since it changes no behaviour of the helper.

**Prior art.** `.scratch/utf16-string-parity/spec.md` is the same shape of fix —
a whole class of encoding divergence closed at once, driven by a single reported
hash mismatch (#1248) — and its tests are the model to follow.

**Baseline.** The full suite runs before any line changes, so that every moved
expectation is attributable. Existing media query expectations use round
breakpoints (`1024px → 1023.99px`, `65em → 64.99em`), which print identically at
both widths, so churn is expected to be near zero; anything that does move is
listed with its Babel-confirmed value in the final report.

## Out of Scope

- Any numeric type outside `stylex-css-parser`. `stylex-css`, `stylex-ast`, and
  `stylex-transform` already use `f64` and `to_js_string`.
- `stylex_utils::number::to_js_string` itself. It is adopted, not modified.
- The last-media-query-wins algorithm, the DeMorgan distribution, and the
  interval merge. Their logic is unchanged; only the width of the numbers
  flowing through them and the spelling of the numbers coming out.
- `enableMediaQueryOrder` and its default.
- Performance tuning. `f64` doubles the width of these fields; the types are
  short-lived parse products, so no benchmark is expected to move. If one does,
  it is reported, not optimised away in this change.

## Further Notes

The reporter is filing these one at a time as they find them and has said they
expect duplicates to be closed. Closing the whole class rather than the single
reported site is a deliberate response to that: the goal is that the next
fractional-value divergence does not exist to be reported.

`to_js_string`'s own doc comment already states the principle this change
applies — "this rendering reaches both generated code and the class-name hash,
the spelling itself is observable and has to match, not merely round-trip to the
same `f64`". `stylex-css-parser` is simply the last crate that had not heard it.
