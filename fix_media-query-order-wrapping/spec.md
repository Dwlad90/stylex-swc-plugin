# Media-query-order wrapping parity

Status: ready-for-agent

Tracks https://github.com/Dwlad90/stylex-swc-plugin/issues/1268.

Reference implementation: `@stylexjs/babel-plugin` 0.19.0, as installed by this
repo's lockfile. That build is the sole authority on expected output; where a
snapshot in this repo disagrees with it, the snapshot is wrong.

## Problem Statement

An author writes a conditional value map whose keys are a ladder of exclusive
`min-width`/`max-width` media queries, ending in a `max-width`-only query. The
official compiler and this one emit different CSS for it.

The official compiler rewrites the earlier entries into disjunctions containing
contradictory branches, which print as `not all` — for the first entry of a
six-rung ladder, a doubly nested `or` of four such branches around the authored
query. This compiler emits every authored query unchanged.

A StyleX class name is a hash of the canonical declaration text, so a difference
in rule text is a difference in class name. Any deployment that mixes the two
compilers — server-rendered markup from one and client bundles from the other,
cached HTML, a partial migration, snapshot tests written against either — breaks
silently: the markup names a class the stylesheet never defines, and nothing
errors. For the reported input, two of six class names diverge.

A second divergence is latent behind the first and is not visible until it is
fixed.

> **Correction, recorded during implementation.** This second divergence did
> not exist as stated. The transform did hand back a sequence, but its only
> consumer already wrote that sequence into an insertion-ordered map keyed by
> the property, so the collision already dropped a declaration exactly as the
> reference implementation does. An enumeration of 4032 ordered value maps over
> an alphabet built for collisions found no input on which the two disagreed.
> Ticket 06's tests therefore passed on arrival rather than landing red, and
> ticket 07 is a hardening refactor -- it moves the property to where it is
> produced instead of depending on the one caller that has it -- rather than a
> fix. See commit `02aa4391d`. The paragraph below describes the shape of the
> code, which was real; it is the observable divergence that was not. The official compiler's last-media-query-wins transform writes its
rewritten keys back into a plain object, deleting the old key and assigning the
new one. Once contradictory branches start collapsing, two authored entries can
normalize to the same query text; the second assignment then overwrites the
first, and one of the author's conditional values disappears from the output
entirely. This compiler accumulates rewritten keys in a sequence instead, so it
keeps both — a difference in rule count, not merely in rule text.

## Solution

Both divergences are removed in favour of the official compiler's behaviour,
byte for byte, including the two respects in which that behaviour is worse than
this compiler's current one.

For the wrapping: the canonicalization pass keeps contradictory disjunction
branches rather than discarding them, so they serialize as `not all` and the
surrounding nesting survives into the emitted query. This is achieved by
deleting a shortcut that has no counterpart in the reference implementation, not
by adding a wrapper — the last-media-query-wins transform already builds the
negation chain correctly.

For the declaration loss: the transform's output collection gains
insertion-ordered map semantics matching a JavaScript object, so that a
rewritten key colliding with one already present overwrites that entry's value
while keeping its original position, and one authored declaration is dropped
exactly as the reference implementation drops it.

Both are genuine upstream defects and are cross-reported to facebook/stylex
separately. The redundant wrapper is additionally rejected by lightningcss's
minifier, whose parser refuses the doubly parenthesised form. Emitting CSS that
a mainstream minifier will not parse is accepted knowingly: a rejected
stylesheet fails loudly, whereas a class-name divergence fails silently, and
matching the reference implementation is the point of this compiler.

## User Stories

1. As an author whose ladder of exclusive breakpoints includes a
   `max-width`-only rung, I want this compiler's rule text to match the official
   compiler's, so that the class names in my markup name rules my stylesheet
   actually defines.
2. As an author migrating a codebase one bundle at a time, I want both
   compilers to agree on every media query I have written, so that a
   half-migrated deployment renders correctly instead of silently losing styles.
3. As an author who server-renders with one compiler and bundles with the other,
   I want identical class names from both, so that hydration does not restyle
   the page.
4. As an author with cached HTML from a previous build, I want class names to be
   a function of my source and not of which compiler produced them, so that a
   compiler switch does not invalidate the cache silently.
5. As an author with snapshot tests written against the official compiler's
   output, I want this compiler to satisfy them unchanged, so that adopting it
   is not a test rewrite.
6. As an author whose conditional value map contains two entries that
   canonicalize to the same query, I want this compiler to keep the same entry
   the official compiler keeps, so that the two produce the same number of
   rules.
7. As an author who has opted out of media-query ordering, I want my authored
   query spelling hashed verbatim, so that opting out remains a way to avoid all
   of this.
8. As an author with a single media query in a conditional value map, I want it
   left alone, so that the common case gains no wrapper.
9. As an author whose queries mix units within one dimension, I want range
   simplification skipped exactly where the official compiler skips it, so that
   an `em` breakpoint beside a `px` one is not silently merged.
10. As an author using a media type or a feature the range merge does not
    understand, I want the merge to bail out to my authored rules where the
    official compiler bails out, so that `screen` in a ladder behaves the same
    in both.
11. As an author writing a strict range query, I want the computed bound to
    carry the same digits the official compiler computes, so that the wrapped
    forms this change introduces hash identically.
12. As an author with a breakpoint ladder long enough to exhaust the reference
    implementation's call stack, I want this compiler to degrade the same way
    rather than crash, so that a large design system compiles at all.
13. As an author whose media query is syntactically invalid, I want the same
    refusal I get today, so that this change does not turn a clear error into a
    mangled stylesheet.
14. As an author with non-media properties beside media queries in one
    conditional value map, I want their relative order preserved as the official
    compiler preserves it, so that the cascade resolves identically.
15. As an author using comma-separated queries, I want each disjunct wrapped as
    the official compiler wraps it, so that an `or` query behaves like the rest.
16. As an author whose ladder canonicalizes such that only contradictions
    remain, I want the whole query to print as the official compiler prints it,
    so that a dead rule is dead in both.
17. As a maintainer, I want the shortcut that caused this divergence removed
    rather than reshaped, so that no predicate in this compiler claims a
    property that cannot be checked against the reference implementation.
18. As a maintainer, I want the recursion bound that replaces the reference
    implementation's caught stack overflow to be justified by a measured
    comparison against it, so that the constant is not arbitrary.
19. As a maintainer, I want the two failure modes of the canonicalization pass —
    give up and return the input, versus refuse the whole declaration — to be
    visibly distinct in the code, so that a later change cannot conflate them.
20. As a maintainer, I want the emitted at-rule order verified against the
    reference implementation after the query text grows, so that a longer key
    sorting to a different position is caught rather than assumed away.
21. As a maintainer, I want the deliberate emission of CSS that lightningcss
    rejects recorded with its reasoning, so that a future reader does not
    "fix" it back.
22. As a maintainer, I want the declaration-dropping behaviour recorded as a
    port of an upstream defect rather than as intended design, so that it is
    revisited when the upstream report is resolved.
23. As a maintainer, I want each upstream defect cross-reported separately, so
    that either can be fixed upstream without waiting on the other.
24. As a maintainer, I want the reference implementation's version pinned in the
    report of any parity run, so that a failure after a dependency update is
    attributable.
25. As a reviewer, I want the divergence to land as a failing test before the
    fix, so that the history proves which change was responsible.
26. As a reviewer, I want the two defects in separate commits, so that the
    second is visibly a second cause rather than fallout from the first.
27. As a reviewer, I want any pre-existing expectation that contradicts the
    reference implementation listed with the reference output beside it before
    it is changed, so that no snapshot is quietly rewritten.
28. As an agent picking this up later, I want the vocabulary for the collapse
    behaviour in the crate glossary, so that the concept is named the same way
    everywhere.
29. As a performance-conscious maintainer, I want the cost of restoring the
    unbounded expansion measured at several ladder lengths, so that a follow-up
    optimization has a baseline.
30. As a performance-conscious maintainer, I want any future fast path to be
    justified separately from this parity fix, so that an unverifiable claim is
    never bundled into one.

## Implementation Decisions

- **The reference implementation is the authority.** Expected output comes from
  running `@stylexjs/babel-plugin` 0.19.0, not from reasoning about what the
  output should be. Where an existing expectation in this repo disagrees with
  it, the expectation was a mis-port and is corrected. Any such correction is
  tabulated — expectation, reference output — and shown before it is applied.
  A future upstream change to this behaviour is a separate issue, not part of
  this work.

- **The wrapping fix is a deletion.** `merge_intervals_for_and` carries a
  predicate that detects a disjoint breakpoint ladder and returns an empty
  result, short-circuiting the DeMorgan expansion. The reference implementation
  has no such predicate. It is deleted along with its call site, and nothing
  else in the canonicalization pass changes.

  The mechanism, for the record: a contradictory branch in the reference
  implementation recurses to the bottom and yields a one-element result holding
  an empty disjunction. The parent's filter drops only *empty* results, so the
  one-element result survives and serializes as `not all`. This compiler's
  predicate returns a genuinely empty result instead, which the same filter
  discards, leaving one survivor that serialization then unwraps to the bare
  authored query. Restoring the reference behaviour therefore requires removing
  the predicate, not changing the filter or the serializer.

- **No reshaped fast path.** Making the predicate return the surviving shape
  instead of an empty one would preserve the string and the fast path, and is
  rejected: its correctness claim — that it fires exactly when the full
  expansion would contradict — has no upstream line to check against. If
  measurement shows a real cost, a fast path returns as a separate, benchmarked
  change with its own justification.

- **The transform's output collection gains JavaScript object semantics.** The
  last-media-query-wins transform currently appends rewritten keys to a
  sequence. It moves to an insertion-ordered map keyed by the rewritten query
  text, reproducing the reference implementation's delete-then-assign exactly:
  assigning a key already present keeps that key's original position and
  replaces only its value, while a key not yet present is appended. This is what
  makes one authored declaration disappear when two entries canonicalize to the
  same text, and that loss is the intended, faithful outcome. No diagnostic is
  emitted for it — the reference implementation emits none, and a warning it
  does not print would itself be a divergence in observable behaviour.

- **The caught stack overflow becomes an explicit recursion bound.** The
  reference implementation wraps its range merge in a `try`/`catch` that returns
  the input rules unchanged on any throw. Because the merge recurses into itself
  rather than through that wrapper, a deep enough ladder raises a call-stack
  error which that `catch` swallows — so the reference implementation's answer
  for a query too deep to merge is "emit it unmerged". This cannot be caught in
  Rust, where stack exhaustion aborts the process, so the merge gains a
  recursion depth bound that returns its input rules on exceedance.

  The bound is a number this compiler chooses, so byte parity past that depth is
  unattainable in principle. It is set at or above the ladder length at which
  the reference implementation itself gives up, found by bisecting ladder length
  against the installed plugin, and both the number and its provenance are
  recorded next to it. On exceedance the merge returns its input — it does not
  refuse the declaration; the reference implementation's inner recovery
  deliberately does not propagate to its outer refusal.

- **The two failure modes are separated in the code.** The inner recovery
  (give up merging, return the input) becomes a named boundary mirroring the
  reference implementation's wrapper, distinct from the outer refusal that turns
  an unparseable query into the invalid-media-query-syntax error. Making both
  visible is cheap now that the inner boundary must exist for the depth bound,
  and prevents the two from being conflated later.

- **The at-rule ordering is verified, not changed.** At-rule sorting compares
  the final key text, so a rewritten key that is much longer sorts differently
  among its siblings than the authored one did. The reference implementation
  sorts the same rewritten strings, so the comparator is expected to need no
  change — but the emitted order is checked against the reference
  implementation rather than assumed.

- **The ordering gate is audited, not widened.** Whether the panic-to-refusal
  conversion fires on exactly the inputs the reference implementation's catch
  fires on, and whether the ordering option's default matches, are both
  confirmed against the installed plugin. Changes are made only where it
  disagrees.

- **Commit structure.** The wrapping deletion and the declaration-loss port are
  separate commits, each preceded by the test that fails without it. Each
  upstream defect is cross-reported to facebook/stylex on its own.

- **Glossary.** The crate glossary already names media query canonicalization
  and the last-media-query-wins transform, including that a contradiction prints
  as `not all`. It is extended to say that contradictory branches are retained
  rather than pruned, and that a collision between rewritten keys drops a
  declaration.

## Testing Decisions

A good test here asserts what a consumer of the compiler can observe: the
emitted CSS text, the class names, and the number of rules — never the shape of
an intermediate syntax tree, and never that a particular internal function was
called. The contract this work defends is the rule text, because the class name
is its hash; so a test that would still pass after the rule text changed is not
testing this.

Every expectation in a new test is taken from a run of the reference
implementation, not written by hand.

Three seams, in ascending order of level:

- **The transform's own unit seam** — the existing unit tests over the
  last-media-query-wins transform, which take a conditional value map and assert
  the rewritten keys without invoking the compiler. It carries the rewritten
  keys for the reported ladder, and it is the only seam that hosts the recursion
  bound, whose interesting property is that the input came back unmerged rather
  than the process aborting; asserting that through a higher seam would mean a
  very large literal that no reader benefits from. Prior art: the existing
  disjoint-range and unit-conflict cases in that file, whose counterparts in the
  reference implementation's own suite carry the same names — which is why
  removing the shortcut is expected to leave them passing rather than rewrite
  them.

- **The compiler's end-to-end seam** — the existing media-query canonicalization
  tests over `stylex.create`, asserting emitted CSS text and class names. This
  is the primary seam. The reported input goes here, as do the collision case
  (visible as a declaration absent from the output) and the at-rule order check.
  Snapshots carry the emitted CSS text so that a rehash shows up as a readable
  query-string diff rather than an opaque class-name change. Prior art: the
  existing canonicalization tests pinning the earlier canonicalization issue,
  and the computed-bounds tests beside them.

- **The parity seam** — the module corpus consulted by the parity harness, which
  runs both compilers and compares class name, rule text, and style-object
  shape. This is the only seam that can fail when the reference implementation
  changes rather than when this compiler does, which is what makes it required.
  The corpus already carries multi-module subjects with nested media queries, so
  the reported input needs no harness change. Prior art: the existing nested
  media-query subjects in that corpus.

The failing test lands before the fix in both cases, so the history shows which
change was responsible. Any pre-existing expectation that the reference
implementation contradicts is corrected only after being tabulated against the
reference output.

The full repository gate — typecheck, format, both linters, the Rust and JS
suites, and the parity harness — runs before this is called done. The JS suites
exercise the built artifact rather than the Rust sources, so a rebuild precedes
them.

## Out of Scope

- Fixing either defect upstream. Both are cross-reported; neither report gates
  this work.
- Tracking a future upstream change to this behaviour. If the reference
  implementation stops emitting the wrapper, that is a new issue.
- Pinning the reference implementation to an exact version in the dependency
  catalog. The caret range and its drift risk are noted, not changed here.
- Any fast path restoring the deleted shortcut's performance. Measured and
  reported, but deferred.
- A diagnostic, warning, or refusal for the dropped declaration.
- Changing the at-rule comparator, unless the reference implementation is found
  to disagree with it.
- Changing the ordering option's gate or default, unless the reference
  implementation is found to disagree with it.
- Style-level media query keys, which the ordering transform does not touch.

## Further Notes

The reference implementation's wrapped output is rejected by lightningcss's
minifier, which refuses the doubly parenthesised form. This is a known,
accepted consequence and belongs in the record next to the change, because it
is the one respect in which a reader will believe the current behaviour is
better — and it is, semantically. The reason it loses is that a class-name
divergence fails silently while a minifier rejection fails loudly.

The declaration-dropping port is the more serious of the two upstream defects:
redundant CSS is ugly, a missing declaration is a lost style. It is worth
saying plainly in the upstream report.

Two earlier efforts in this area — the canonicalization parity work and the
double-precision bounds work — both pinned expectations against this same
reference version. One of them added the shortcut this spec deletes. That is the
reason every expectation touched here is re-derived from the reference
implementation rather than trusted, and the reason the recursion bound's
provenance is written down: an earlier plan in this area recorded tickets marked
done with unchecked acceptance criteria and no evidence, which is the failure
mode to avoid.
