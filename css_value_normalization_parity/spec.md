# Spec — Normalize CSS values through a ported value parser

**Status:** ready-for-agent

**Branch:** `fix_nullish_coalescing` (continuing on the current branch, by
request)

**Related:** GitHub issue #1256 — "CSS value normalization diverges from
`@stylexjs/babel-plugin`, changing class hashes (whitespace, hex, case, quotes,
exponents)"

**Upstream reference:** `~/Projects/Facebook/stylex` @ `5f51b2444` (the v0.19.0
release commit). Vendored library reference:
`postcss-value-parser` as resolved in that checkout (MIT).

**Upstream reference postcss-value-parser:** `~/Projects/Facebook/postcss-value-parser`

## Problem Statement

A StyleX class name is a hash of the canonical CSS declaration text. That makes
the canonical text a **compatibility contract**: any setup that mixes this
compiler with `@stylexjs/babel-plugin` — SSR built by one and client bundles by
the other, cached HTML, an incremental migration, snapshot tests written against
either — depends on both compilers producing byte-identical declarations for
identical source.

They do not. The reporter measured a production codebase of roughly 1,600 StyleX
files and found about 160 rules whose class names differ. The transform succeeds
and the CSS looks plausible in isolation; the damage only appears when the two
compilers' output meets, at which point styles silently fail to apply because
the markup names a class the stylesheet does not define.

Six normalizations were reported. Reproduced against
`@stylexjs/babel-plugin@0.19.0`, four are live on the current branch:

1. **Whitespace between value tokens is dropped.** `opacity 0.2s ease-in-out`
   becomes `opacity.2s ease-in-out`; `50% bottom` becomes `50%bottom`;
   `-50% -120%` becomes `-50%-120%`.
2. **Spacing around `*` in math functions is collapsed or displaced.**
   `calc(-1 * var(--spacing))` becomes `calc(-1*var(--spacing))`, and
   `100vw * 0.12` becomes `100vw* .12` — note the space did not vanish, it
   **moved**.
3. **6-digit hex colors are shortened.** `#ffffff` becomes `#fff`.
4. **Single-quoted strings are rewritten to double quotes.**
   `grid-template-areas: 'sidebar content'` becomes `"sidebar content"`.

Two more were reported and are already fixed on the current branch — transform
function names being lowercased (`translateY` → `translatey`) and large numbers
being rewritten in scientific notation (`-10000px` → `-1e4px`). Those fixes are
themselves part of the problem statement: each was closed by adding another
string-rewriting pass that tries to guess what the original text was.

**These are not six bugs.** They are one bug with six symptoms. Value
normalization currently parses the value into a full swc CSS `Stylesheet`,
re-serializes it through swc's code generator with `minify: true`, and then
attempts to undo the minifier with a sequence of hand-written string rescans.
Three things make that structurally incapable of parity:

- **Whitespace position is not represented in that AST.** The displaced space in
  `100vw* .12` is the proof: the original spacing is gone by the time the value
  is serialized, and every space in the output is re-synthesized from scratch —
  first by the code generator, then patched again by a hand-rolled spacing pass.
- **The minifier canonicalizes by design.** Shortening hex, preferring double
  quotes, lowercasing identifiers, and re-spelling numbers in exponent notation
  are the code generator working correctly. They are defects only because the
  contract says: do not touch what the reference implementation does not touch.
- **The normalizing visitor discards the raw spellings that did survive.** Every
  node it edits has its raw text and span cleared, so even where swc retained
  the author's original characters, they are gone before serialization.

The consequence is a ratchet. Every newly discovered symptom is answered with
another restoration pass, each pass is a heuristic reconstructing information
that was deliberately destroyed one step earlier, and each new pass is a new
opportunity to corrupt a value that was previously correct. The reported
divergences are the ones someone happened to measure; the design guarantees
there are more.

## Solution

Stop serializing through a minifier. Normalize the value with a faithful port of
the same value parser the reference implementation uses, so that anything no
normalizer explicitly rewrites survives **byte for byte**.

The reference implementation parses a declaration value into a deliberately
loose token list — words, spaces, separators, strings, functions — where every
token keeps its original text and its surrounding whitespace as first-class
data. It then runs a fixed, ordered list of nine small normalizers that mutate
only the specific tokens each one cares about, and re-emits the token list. A
hex color, an identifier's letter case, a quote character, and an exponent are
never examined by any of the nine, so they emerge exactly as the author wrote
them.

Port that parser and those nine normalizers, and make value normalization be
that pipeline. All four live divergences close by construction rather than by
correction — there is no code left that could shorten a hex color, because
nothing in the pipeline understands what a hex color is. The two already-fixed
divergences keep working for the same reason, and their restoration passes are
deleted along with the rest.

**The CSS parser is removed entirely.** Its only remaining consumer is a single
validation rule — reject a `var()` whose referenced name is not prefixed with
`--` — which the new token list answers directly and more cheaply. Nothing else
in the workspace uses swc's CSS support, so the whole CSS half of that
dependency stops being compiled.

This also makes normalization substantially cheaper. Per declaration, the
current path builds a synthetic rule string, runs a full CSS parse, walks the
result with a mutating visitor, runs code generation, and then makes five
separate full-string scans to extract and repair the result. The replacement is
one value parse, nine cheap token walks, and one emit. A memoizing cache in
front of normalization means repetitive files already skip much of this, so the
end-to-end gain is bounded — hence the benchmark below rather than a claim.

Nothing about the compiler's public interface changes. Authors who are not
mixing compilers see the same styles they see today, spelled the way the
reference implementation spells them.

## User Stories

1. As a developer running this compiler for SSR and `@stylexjs/babel-plugin` for
   client bundles, I want identical source to hash to identical class names, so
   that server-rendered markup finds its styles in the client stylesheet.
2. As a developer migrating a codebase incrementally, I want files compiled by
   either compiler to interoperate, so that I can migrate module by module
   instead of atomically.
3. As a developer with cached HTML in a CDN, I want class names to be stable
   across a compiler swap, so that cached pages do not lose their styling.
4. As a developer with snapshot tests written against the reference compiler, I
   want those snapshots to pass unchanged, so that adopting this compiler is not
   gated on rewriting my test suite.
5. As a developer writing `transition: 'opacity 0.2s ease-in-out'`, I want the
   space between the property and the duration preserved, so that the
   declaration is valid CSS and hashes the same as upstream.
6. As a developer writing `backgroundPosition: '50% bottom'`, I want the space
   after the percentage preserved, so that the keyword is not fused onto the
   number.
7. As a developer writing `translate: '-50% -120%'`, I want the two components
   to remain separate, so that the second value is not read as a subtraction.
8. As a developer writing `backgroundPosition: 'top 0.75rem left 0.625rem'`, I
   want spaces preserved after leading-zero stripping, so that a four-value
   position stays four values.
9. As a developer writing `outline: 'transparent dotted 0.125rem'`, I want the
   space before the width preserved, so that the shorthand parses.
10. As a developer writing a gradient with percentage color stops, I want spaces
    between each color and its position preserved, so that the gradient renders.
11. As a developer writing `calc(-1 * var(--spacing))`, I want the spaces around
    the multiplication operator preserved exactly where I put them, so that the
    expression matches upstream's canonical form.
12. As a developer writing `max(4.8125rem, 100vw * 0.12)`, I want the space
    before the operator to stay before the operator rather than migrating after
    it, so that the value is not silently altered.
13. As a developer writing `calc(var(--b) * var(--c))` inside a shorthand, I
    want operator spacing preserved, so that nested functions in a multi-value
    declaration hash correctly.
14. As a developer writing `color: '#ffffff'`, I want the six-digit form
    preserved, so that my class name matches upstream's.
15. As a developer writing a gradient containing `#000000`, I want the hex
    preserved inside the function too, so that function bodies are not a
    separate normalization regime.
16. As a developer writing `gridTemplateAreas: "'sidebar content'"`, I want my
    single quotes preserved, so that the quote character is not a hidden input
    to the hash.
17. As a developer writing `transform: 'translateX(-50%) translateY(-50%)'`, I
    want function-name capitalization preserved, so that the value is not
    lowercased.
18. As a developer writing `left: '-10000px'`, I want the plain decimal spelling
    preserved rather than an exponent, so that the emitted CSS reads the way I
    wrote it.
19. As a developer, I want the two already-fixed divergences to stay fixed after
    this rewrite, so that the change is not a regression trade.
20. As a developer writing `content` values containing quotes, escapes, or
    non-ASCII characters, I want them passed through untouched, so that
    generated content renders correctly.
21. As a developer writing a `url()` whose body contains characters that look
    like CSS syntax, I want the body copied verbatim, so that the URL is not
    corrupted.
22. As a developer writing a value containing a CSS comment, I want the comment
    handled without disturbing the surrounding value.
23. As a developer writing `var(--x)px`, I want no space inserted between the
    function and the unit, so that the previously fixed defect stays fixed.
24. As a developer writing `calc-size(fit-content, size / 2)` or any syntax
    newer than the compiler's knowledge, I want it normalized and emitted rather
    than rejected, so that new CSS features are not gated on compiler support.
25. As a developer using relative color syntax such as `oklch(from …)`, I want
    it preserved, so that modern color functions work without a special-case
    allowlist.
26. As a developer writing `500ms`, I want it converted to `.5s` exactly as
    upstream does, so that the established shortening still applies.
27. As a developer writing `5ms`, I want it left alone, so that the
    below-threshold exception is honored.
28. As a developer writing `0px`, I want the unit dropped, and writing `0deg`,
    `0s`, `0fr`, or `0%`, I want the unit kept — matching upstream's rules,
    including its treatment of zero values inside functions.
29. As a developer writing `0.5`, I want `.5`, so that leading-zero stripping
    still applies where upstream applies it.
30. As a developer writing a negative decimal such as `-0.24px`, I want the
    leading zero kept, so that the value is spelled as upstream spells it.
31. As a developer writing `transitionProperty: 'backgroundColor'`, I want it
    hyphenated, and writing a custom property name, I want it left alone.
32. As a developer setting a custom property, I want zero-value normalization
    skipped for it, matching upstream.
33. As a developer with font-size-to-rem conversion enabled, I want font sizes
    converted exactly as upstream converts them, and untouched when it is off.
34. As a developer who writes an unclosed function or an unclosed string, I want
    the same error the reference implementation raises, so that diagnostics are
    portable.
35. As a developer who writes an unclosed comment, I want the existing clear
    error rather than silent corruption.
36. As a developer who writes a value containing a character that could
    terminate the generated rule, I want it rejected, so that a style object
    cannot inject arbitrary CSS into the stylesheet.
37. As a developer who mistypes `var(x)` instead of `var(--x)`, I want the
    existing error, so that a silently-empty custom property is caught at
    compile time.
38. As a maintainer, I want one code path for every value, so that a bug found
    in one shape of value is not invisible in another because it took a bypass.
39. As a maintainer, I want no code whose purpose is to reverse an earlier step's
    canonicalization, so that the ratchet of restoration passes ends.
40. As a maintainer, I want the normalizer files to correspond one-to-one with
    upstream's, so that a future upstream change is a readable diff rather than
    an investigation.
41. As a maintainer, I want a runnable oracle that diffs this compiler against a
    given `@stylexjs/babel-plugin` release, so that the next upstream bump can be
    checked rather than assumed.
42. As a maintainer, I want the accumulated regression coverage from the deleted
    code preserved at a stable boundary, so that removing an implementation does
    not remove the knowledge of what it protected against.
43. As a maintainer, I want measured before-and-after normalization cost, so
    that the performance claim is evidence rather than assertion.
44. As a maintainer, I want swc's CSS support dropped from the build once
    nothing uses it, so that compile time and binary size reflect what the
    compiler actually does.
45. As a maintainer reading the crate's domain glossary, I want "normalizer" to
    describe what the code now is, so that the glossary does not mislead.
46. As the reporter of #1256, I want each of the six cases pinned by a permanent
    test, so that they cannot silently return.

## Implementation Decisions

**Vendored value parser.** Port `postcss-value-parser` — its parse, stringify,
walk, and unit-splitting modules — into a crate of its own,
`crates/postcss-value-parser`, with file names mirroring the original and a
header recording MIT provenance.

It is third-party code, so it stays visibly separate from StyleX logic, and a
crate boundary is what makes that separation real rather than advisory: the
crate declares no dependencies at all, and an empty `[dependencies]` is a thing
a future contributor has to deliberately edit, where a module inside
`stylex-css` could quietly reach for a sibling and start behaving like this
project's own code. It also keeps the layering honest — the scanner sits at
layer 0 with the other primitives, below everything that uses it. The workspace
`members` glob only matches `crates/stylex-*`, so it is listed explicitly in the
root `Cargo.toml`.

It is not merged into the existing CSS parser crate: that crate is a port of a
_different_ upstream package (a typed parser-combinator value library) with a
different purpose, and combining two unrelated vendored libraries under one
crate name would be misleading.

**Node representation.** The ported AST uses a single JS-shaped record with a
kind discriminant and optional fields (surrounding whitespace, quote character,
unclosed flag, children, source offsets), not a Rust enum. This is a deliberate
trade: an enum would be more type-safe, but the nine normalizers are written as
"inspect the kind, then assign to the value field," and every one of them would
have to be restructured into a match with a catch-all to satisfy an enum. The
brief forbids idiomatizing ported logic, and the record form keeps each
normalizer a line-for-line reading of its original. Each optional field is
documented with the kinds that populate it.

**Source offsets are mandatory.** The zero-dimension normalizer decides whether
a token sits inside a function by comparing source offsets, not by tracking
visitor state. The parser port must therefore carry per-node start and end
offsets; this is a load-bearing detail, not bookkeeping.

**Nine normalizers, ported verbatim, in upstream's order.** Unclosed-function
detection, unclosed-string detection, whitespace, timings, zero dimensions,
leading zero, quotes, camel-case value conversion, and — only when the
font-size-to-rem option is enabled — font-size conversion, appended last.
Ordering is significant and is preserved: timings runs before leading-zero
specifically so that a millisecond value converted to seconds is then stripped
of its leading zero. Each normalizer gets its own file named after its upstream
counterpart, and a single module holds the ordered list and the fold over it,
mirroring upstream's entry point.

**Behavioral deltas beyond the reported six are accepted.** Porting verbatim
changes more than #1256 describes: camel-case conversion will apply to every
top-level word token rather than only those the CSS parser classified as
identifiers; leading-zero stripping applies wherever upstream applies it rather
than being entangled with a negative-decimal repair pass; the
inside-a-function test becomes offset-based. Re-adding the old behavior on top
of the port would reintroduce exactly the divergence being fixed. If a delta
looks like an upstream _defect_ rather than a difference, upstream's behavior is
still adopted — hash parity outranks local correctness here — and the deviation
is called out in the commit message.

**JavaScript numeric semantics.** Three normalizers depend on JavaScript's
lenient string-to-float parsing and on JavaScript's number-to-string spelling.
The output side already exists as a shared utility. The input side is added as a
new shared utility implementing lenient prefix parsing: skip leading whitespace,
optional sign, digits with optional fraction and exponent, stop at the first
character that cannot continue the number, recognize the infinity literal, and
report failure where JavaScript would produce NaN. This is the highest
silent-divergence risk in the change — a float spelled with one digit's
difference produces a different hash and looks like an unrelated bug.

**The CSS parser is removed.** Parsing to a CSS stylesheet, the synthetic rule
wrapper built to feed it, the generic property name used to dodge
property-specific grammar, the parse-error handler, and the stylesheet-based
serializer are all deleted. lightningcss was considered as a faster replacement
and rejected: it is the same category of tool — a parser that emits canonical
CSS — so it cannot serve as the serializer, and using it as a validator would
be optimizing work that is being deleted rather than sped up. Removing the parse
outright is both faster and simpler than any parser swap.

**Custom-property validation moves to the value AST.** The rule — a `var()`
whose first argument does not begin with `--` is an error — is the only
consumer of the CSS parse. It is re-expressed against the token list: a function
token named `var` whose first word child lacks the prefix. This rule does not
exist upstream; it is a deliberate local addition and is knowingly retained,
because it changes only which programs are _rejected_, never the bytes of an
accepted program, and it catches a mistake that otherwise fails silently at
runtime.

**Two other local guards are retained** and are unaffected by the rewrite, since
both are plain string scans rather than parser-dependent: rejection of values
containing characters that could terminate the generated rule (an injection
defence with no upstream equivalent, because the reference implementation gets
it from its own host), and the unclosed-comment error.

**Two bypasses are deleted.** The allowlist that routed color functions and
relative-color syntax around the CSS parser existed solely to dodge color
canonicalization, which no longer happens. The fallback that preserved values
the CSS parser could not understand existed solely to dodge grammar gaps, which
no longer matter. Both become dead once there is one serializer, and keeping
either would leave a second path that could silently shadow the real one.
Support for newer syntax is not lost — it improves, because the ported parser
never rejects: it flags unclosed constructs rather than failing.

**Degenerate input matches upstream exactly**, including its failure. Upstream's
whitespace normalizer indexes the first node without guarding, so an empty value
throws there. The port reproduces the _behavior_ — normalization fails — with a
descriptive local message rather than an imitation of a JavaScript runtime error
string; nothing depends on the wording of an upstream internal crash. Whether an
empty value can reach normalization at all is confirmed first via the
differential harness.

**The public entry point is unchanged.** Value normalization keeps its existing
name, signature, crate, and callers. The rewrite is entirely internal, which is
what makes the primary test seam stable across it.

**The swc CSS feature set is dropped from the crate manifest as the final
step,** once nothing compiles against it. This is the proof of completion: if
the workspace builds without CSS parsing, code generation, AST, and visitor
support, nothing is quietly still on the old path.

**Delivery.** Twelve tickets on the current branch, broken out under
`issues/`. Three start immediately and in parallel: the differential harness,
the numeric utility, and the parser port. The harness comes **first**, not last:
it is the oracle that generates every expectation downstream, and it doubles as
a baseline divergence report. The two coverage-migration tickets come **before**
the swap rather than after it — re-expressing the implementation-coupled tests
at the public seam while the old implementation is still live means they pass on
both sides of the rewrite, which turns the swap from an unguarded change into
one with a net under it, and makes its diff reviewable because every changed
expectation was predicted in advance. The swap follows, then the validator move,
then deletion of the dead modules, then the regression tests, benchmark and
glossary, and finally the dependency removal as the proof of completion.

Tickets that add code without wiring it cannot make the JavaScript suite
meaningful — that suite exercises the compiled native artifact, which only
reflects reality after the swap — so the full suite is run at the swap and at
every ticket after it. The repository has no changeset directory, so no
changeset is added.

## Testing Decisions

**What a good test looks like here.** Assert on the normalized declaration text
and on the class name, never on the shape of the token list, the order in which
normalizers ran, or the presence of a particular helper. The entire point of the
change is that the internals are replaceable; tests that name internals would
have to be rewritten alongside them and would re-create the coupling being
removed. Every expectation is a string the reference compiler actually produced,
not a string a human believed it would produce.

**Three seams, two of which already exist.**

_Primary seam — value normalization._ The public normalization entry point,
which keeps its signature across the rewrite. Prior art: the existing
value-normalization test module in the `stylex-css` crate, already the largest
test file in that crate. This is the highest boundary that still lives inside
the crate being changed, and it is where the bulk of coverage lands.

_Issue seam — compiler output._ The full transform, asserting emitted class
names and rule text from the transform's style metadata. Prior art: the existing
value-normalization transform test module in the `stylex-transform` crate. The
six reported cases are pinned here rather than at the primary seam, because the
contract #1256 is about is the **class hash**, and only this seam can see it.

_New seam — the numeric utility._ The one new seam, and a deliberate exception
to preferring existing ones. Justified because a float-spelling divergence is
silent and surfaces at the other two seams as an unrelated-looking wrong digit,
with no way to localize it. Table-driven, against pairs generated from a
JavaScript runtime.

**Explicitly not seams:** the ported parse, stringify, walk, and unit-splitting
functions, and the nine normalizers individually. Testing those directly would
re-create precisely the implementation-coupled tests being deleted, and would
pin internals the port may need to adjust. The vendored library's own upstream
test suite is likewise not ported: it does not ship with the package, and the
differential harness proves faithfulness more directly — against the reference
compiler's real bytes, over this project's real corpus.

**The differential harness is an oracle, not a test.** A checked-in developer
script that runs a corpus through both this compiler and a pinned
`@stylexjs/babel-plugin` release and diffs the style metadata. It lives outside
the Rust test suite so that no Node toolchain becomes a prerequisite for
`cargo test`, and it is not wired into CI. Its job is to **generate** the
expectations used at the first two seams, so that no expectation is ever updated
by eye — a hand-edited expectation in this change is just the bug re-encoded.
It is checked in rather than discarded because this class of divergence will
recur at every upstream release, and a throwaway means the next person rebuilds
it from nothing. Corpus: every value literal harvested from the existing test
suites, the six reported cases, and a hand-written edge set covering non-ASCII
content, escapes, URLs, comments, importance annotations, empty strings, and
unclosed constructs.

**Coverage migration.** Roughly seventeen hundred lines of tests are bound to
code this change deletes. They are not discarded: each is mined for its _input_,
and those inputs are re-asserted at the primary seam with harness-verified
expectations. Those tests encode years of accumulated regressions — function
results followed by units, URL bodies, comments, non-ASCII content — and the
knowledge is worth more than the assertions' current form. This also relocates
coverage to where it should have been from the start: at a boundary that
survives an implementation change.

**Performance.** A criterion benchmark on the normalization entry point over a
representative value corpus, added alongside the crate's existing benchmark, run
before and after and reported with the change. Scoped to measurement; not added
as a CI gate.

## Out of Scope

- Any change to how class names are hashed, to the hashing algorithm, or to rule
  priority computation.
- Property-name normalization, right-to-left rule generation, media query
  handling, and the `when` construct — all untouched.
- The typed parser-combinator value parser crate, which serves a different
  upstream package and is not involved.
- Adopting lightningcss anywhere in the compiler. Evaluated and rejected above;
  should it be revisited, it would be for a different problem than this one.
- Wiring the differential harness into continuous integration, or making the
  Rust test suite depend on a Node toolchain.
- Divergences from the reference compiler outside value normalization. Related
  parity work is tracked separately; this spec covers only the value
  serialization path.
- Changing the compiler's public options, its transform interface, or any
  bundler plugin.
- Introducing the reference compiler as a runtime dependency of this project.

## Further Notes

The two divergences already fixed on the current branch are instructive rather
than incidental: both were closed by adding a pass that reconstructs information
an earlier step destroyed. That is the pattern this change ends. When reviewing,
treat any new code whose purpose is to reverse a previous step as a signal the
port has drifted back toward the old design.

The reported symptoms are a sample, not a census — they are what one production
codebase happened to surface. The success criterion is therefore not "the six
cases pass" but "the harness reports no diff across the full corpus." The six
cases are pinned as regression tests because they were reported, not because
they define done.

Worth stating plainly for whoever picks this up: the temptation at every step
will be to make the ported code more idiomatic. Resist it in the parser and the
nine normalizers specifically. Their value is that they can be diffed against
upstream by eye at the next release; anything gained by restructuring them is
lost the first time upstream changes and nobody can tell whether the difference
is intentional.
