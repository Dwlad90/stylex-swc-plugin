# Spec — One name for the pseudo-element prefix test

**Status:** ready-for-agent

**Branch:** `fix_share-pseudo-element-predicate` (from `develop`)

**Related:** GitHub issue #1251 (already fixed by `d47510a92`);
`issues/01-align-property-inherits-with-upstream.md` (resolved) recorded this
follow-up in its "Not done" section.

**Upstream reference:** `~/Projects/Facebook/stylex` @ `5f51b2444` (the v0.19.0
release commit).

## Problem Statement

A dynamic style whose variable sits behind a pseudo-class — `:hover`, `:focus`,
`:active` — was registered as `@property … { syntax: "*"; inherits: true; }`
instead of `inherits: false`. Because `inherits` is not part of the class name's
hash input, the class name was identical to the one `@stylexjs/babel-plugin`
produces, so the same class resolved to different runtime cascade behavior
depending on which compiler built the sheet: with `inherits: true` the custom
property cascades into descendants, and a descendant using a dynamic style with
the same hashed variable picks up its ancestor's value instead of its own
initial value. This is the most dangerous shape a divergence can take — nothing
in the output looks wrong, and the bug only appears in a browser, in a nested
component.

The behavior is fixed. The defect that produced it is not.

**Property registration** turns on one rule: a selector segment is a pseudo
_element_ when it carries a `::` prefix, and everything else — every pseudo
_class_ — is not. That rule is currently spelled out as a bare string-prefix
test at eight independent sites across `stylex-css` and `stylex-transform`. One
of those eight copies had drifted to a single `':'`, which is precisely the bug
above. Eight copies of one rule means eight chances to drift, and nothing in
the type system, the compiler, or a reviewer's eye distinguishes the correct
copies from a drifted one — they are all just string literals.

A maintainer or agent editing any of these call sites today has no way to tell
that the literal they are reading is load-bearing domain logic rather than an
incidental string, and no single place to look up what the rule is.

## Solution

Give the rule one name, and let all eight sites say the name instead of
re-deriving the rule.

A single predicate — `is_pseudo_element(key) -> bool` — becomes the only place
the `::` prefix is tested. Every call site keeps its exact existing shape; only
the literal moves behind the name. After that, a future edit that would have
silently changed `"::"` to `':'` at one site either changes the rule for
everybody (visible, one line, one review comment) or does not compile.

Separately, the two places that deliberately test a **single** colon are
audited and given a written verdict, so the next reader does not have to guess
whether they are correct or are the next #1251 waiting to be found.

Nothing about the compiler's output changes. Users see no difference; this is
insurance against the same class of bug recurring.

## User Stories

1. As a StyleX user, I want the `@property` registration for a dynamic style
   behind `:hover` to declare `inherits: false`, so that the custom property
   does not leak into descendant elements that use the same dynamic style.
2. As a StyleX user, I want the `@property` registration for a dynamic style
   behind `::before` or `::after` to declare `inherits: true`, so that the
   pseudo element can reach the variable at all — it has no other route to it.
3. As a StyleX user, I want a class name emitted by this compiler to behave
   exactly as the same class name emitted by `@stylexjs/babel-plugin`, so that
   swapping compilers in a build never silently changes what my page looks
   like.
4. As a StyleX user, I want the fix for the divergence I reported to be pinned
   by a test using my exact reported input, so that it cannot regress and be
   rediscovered by another user in the same way.
5. As a StyleX user migrating an app between compilers, I want confidence that
   parity claims are backed by a run against the other compiler, not by an
   assertion this compiler wrote about itself.
6. As a maintainer, I want the pseudo-element rule stated once, so that a
   correctness review of it is a review of one function rather than of eight
   scattered literals.
7. As a maintainer, I want the pseudo-element rule to carry a name at every
   call site, so that reading a call site tells me the intent rather than
   leaving me to infer it from a prefix string.
8. As a maintainer, I want the refactor to produce zero snapshot movement, so
   that I can approve it by reading the diff rather than by re-deriving what
   every changed rule should now emit.
9. As a maintainer, I want any snapshot churn on this branch to appear in a
   commit _after_ the refactor, so that I can read every changed baseline as a
   deliberate behavior change rather than as refactor fallout.
10. As a maintainer, I want each behavior change on this branch to cite the
    upstream construct that proves the old behavior wrong, so that I am not
    asked to take a parity claim on trust.
11. As a maintainer, I want the two deliberate single-colon sites left alone
    unless upstream provably contradicts them, so that "resembles the bug we
    just fixed" never becomes sufficient grounds for changing working code.
12. As a maintainer, I want the audit's verdict recorded even when the verdict
    is "these are correct", so that the next person who notices them does not
    repeat the investigation.
13. As a maintainer, I want the predicate to live in a crate whose documented
    charter admits domain concepts, so that the crate boundaries in
    `CONTEXT-MAP.md` keep describing the code as it actually is.
14. As a maintainer, I want the predicate inlined at the machine-code level, so
    that naming the rule costs nothing measurable in compile output or run
    time.
15. As a maintainer, I want the predicate's documentation to state the rule on
    its own terms, so that the comment stays true regardless of what any other
    project does later.
16. As a maintainer, I want the reported divergence verified against a built
    release artifact, so that the verification exercises what would actually
    ship rather than a debug build.
17. As a maintainer, I want the differential check performed once and
    discarded, so that the repository does not acquire a second compiler as a
    permanent test-time dependency.
18. As an agent picking up work in this area, I want a glossary term naming the
    pseudo-element/pseudo-class distinction, so that I can find the rule before
    I touch a call site rather than after I break one.
19. As an agent, I want the predicate to be the single search hit for the
    pseudo-element rule, so that "find every place this rule is applied" is one
    lookup rather than a fragile grep for a punctuation string.
20. As an agent, I want the tracker to record why the predicate exists next to
    the bug that motivated it, so that a future reader can reconstruct the
    causal chain without reading git history.
21. As a reviewer, I want the test-pin commit separated from the refactor
    commit, so that each can be judged against its own standard.
22. As a reviewer, I want to be told which upstream source is authoritative for
    the `@property` emission text, so that I do not "discover" a divergence
    against a stale build artifact in the upstream checkout.
23. As a reviewer, I want to know that the existing `inherits: true` snapshots
    were checked against the authoritative upstream source, so that the
    pseudo-element side of the rule is as verified as the pseudo-class side.
24. As a release manager, I want the fix's parity confirmed before the version
    carrying it is published, so that the issue can be closed against a
    released artifact rather than against a branch.
25. As a contributor new to the codebase, I want each single-colon test to say
    in a comment why a single colon is correct there, so that I do not
    "correct" it into a bug.
26. As a contributor, I want the branch to be named for what its diff actually
    does, so that the branch list stays a usable index of in-flight work.

## Implementation Decisions

### Scope

The behavioral fix has already landed on `develop` — the create-call transform
computes property registration from a `::` prefix test, and the snapshots
covering pseudo-class, pseudo-class ordering and media-query-with-pseudo-class
cases all assert `inherits: false`. This spec covers **no behavior change**:
it is a test addition, a de-duplication, and an audit.

### Upstream authority for the emission text

The upstream checkout contains two conflicting `@property` emission forms. The
stale one lives in a build-artifact-only package directory (no `src`, last
touched by the workspace reorganization commit) and omits the `inherits` key
entirely for pseudo elements. The authoritative v0.19.0 source lives in the
`@stylexjs`-scoped workspace package's create visitor and emits an explicit
`inherits: true` / `inherits: false`. Our output matches the authoritative
form. Any future parity audit in this area must read the scoped package's
`src`, never the unscoped `lib`.

### The predicate

- One function, `is_pseudo_element(key: &str) -> bool`, returning whether the
  key carries a `::` prefix. `#[inline]`, so naming the rule has no codegen
  cost.
- **Placement: a `pseudo` module under `stylex-css`'s utils**, re-used by
  `stylex-transform`, which already depends on `stylex-css`.

  This deviates from a decision taken during grilling, which put the predicate
  in `stylex-utils`' string module. The deviation is deliberate and should be
  confirmed or overridden by the maintainer: `stylex-utils`' own `CONTEXT.md`
  declares it a leaf where _"no StyleX domain concept is defined … only the
  machinery the domain crates share"_, and pseudo element is a domain concept —
  it is a glossary term in both `stylex-css` and `stylex-transform`.
  `stylex-constants` is likewise excluded by its charter, which covers static
  lookup tables and compile-time constants rather than predicates. If the
  maintainer prefers the original placement, only the module path changes;
  every other decision here stands.

- Doc comment states the rule intrinsically — a `::` prefix marks a pseudo
  element, a single colon marks a pseudo class — with no provenance claim about
  any other implementation. Provenance comments rot when the other side moves.
- Call sites keep their current shape exactly. The predicate is substituted for
  the literal test; no control flow, ordering, or naming is restructured. This
  keeps the logic structurally identical to the source it was derived from
  while removing the duplication that allowed one copy to drift.

### Call sites converted

All eight sites currently testing a `::` prefix, spanning:

- selector assembly in `stylex-css`, where pseudo classes are emitted before
  pseudo elements (two sites, both bare `bool` uses inside filters — a reason
  the predicate is a `bool` and not an enum classifier);
- priority computation in `stylex-css`, both the compound-chain sum and the
  standalone pseudo-element priority lookup;
- the `when` selector utility and the pre-rule nesting logic in `stylex-css`;
- property registration in `stylex-transform`'s create-call transform — the
  site that carried the #1251 bug.

An enum classifier over pseudo class / pseudo element was considered and
rejected: several sites want a bare `bool` inside a filter, and two of the
single-colon sites are not a binary classification at all (one means "pseudo
class or attribute selector", the other means "any pseudo"), so an exhaustive
type would not model them.

### Accepted follow-on: the sibling predicates

**Amended after review.** The audit found the single-colon test spelled out as
a literal at ten further sites, where "starts with a colon" and "is a pseudo
class" read identically while meaning different things. Naming only the `::`
rule would leave that ambiguity in place, so the branch also adds:

- `is_pseudo_selector` — any pseudo, class or element. Taken by the nesting and
  validation sites, which have always meant "does this key open a selector".
- `is_pseudo_class` — a colon that is not `::`. Taken by
  `get_pseudo_class_priority`. This narrows the test: the bare colon it
  replaced also matched `::before`, which was unreachable because
  `get_priority` probes pseudo elements first. No output changes, and the
  narrowed form no longer depends on that ordering.
- `is_conditional_key` (`utils::condition`) — a pseudo selector, an at-rule, or
  an attribute selector: the three prefixes that open a nested block. It
  retires the four sites that spelled all three out. Sites admitting only a
  subset — the create-arg walker and dynamic-style functions exclude attribute
  selectors, pre-rule nesting handles at-rules on a separate pass — keep their
  literals, because widening them would change behavior.

This is a departure from "only the literal moves behind the name", and it is
deliberate: the same argument that justifies naming the `::` rule justifies
naming the rules it is confused with.

### The audit

Two sites deliberately test a **single** colon: the `when` utility's validation
of a pseudo selector, and a check in selector assembly that means "any pseudo".
Both are audited against the authoritative upstream source. Behavior changes
only where upstream provably contradicts the current code; a resemblance to the
#1251 shape is explicitly not sufficient. If the audit finds nothing provable,
nothing changes and the verdict says so. Each audited site gains a one-line
comment recording the outcome.

### Commit sequence

1. `test(stylex-transform)`: add the reported input as a second pin in the
   dynamic-styles create-call tests, keeping the existing bare-pseudo-class
   pin.
2. `refactor`: introduce the predicate and convert all eight call sites in the
   same commit — a predicate with no callers is not a meaningful boundary and
   would trip dead-code lints.
3. Any audit-driven behavior fix, one commit each, snapshot churn included,
   with the upstream construct cited in the commit body. Omitted entirely if
   the audit finds nothing.

Tracker updates are not commits — `.scratch` is never committed.

### Verification

`cargo test --workspace`, `cargo clippy`, `cargo fmt` and Taplo. The Node test
suite is not run: nothing under Node changes, and the type-aware lint would
require a build for no benefit. One exception — the differential check below
needs a **release** build of the compiler's native artifact, since the JS-facing
suites exercise the built artifact rather than the crate sources.

### Differential verification (one-off, not committed)

The reported module is run through the built release artifact and through
`@stylexjs/babel-plugin` v0.19.0 — already present in the workspace's package
store, requireable by absolute path, so no install is needed — with identical
options, and the emitted `@property` rules are compared for byte equality. The
runner is a throwaway in the session scratchpad and is deleted once green. The
durable pin is the snapshot test from commit 1.

This step exists because the regression pin added alongside the original fix
was self-referential: its input had no counterpart upstream, so it asserted
only that this compiler keeps doing what it currently does. The differential
run converts that into a checked fact once.

### Glossary

The **Property registration** term already added to `stylex-transform`'s
`CONTEXT.md` covers the pseudo-element/pseudo-class distinction. If the
predicate lands in `stylex-css`, that crate's glossary gains a pointer to it,
so the term is discoverable from the crate that owns the code.

**Amended after review.** `stylex-css`'s glossary gains one term per predicate
— **Pseudo element**, **Pseudo class**, **Pseudo selector**, **Conditional
key** — rather than a single pointer, following the accepted follow-on above.
Four names that a reader must not confuse are exactly what a glossary is for,
and `docs/agents/domain.md` requires each term's `_Avoid_` list not to ban a
term the glossary itself defines.

## Testing Decisions

**What makes a good test here.** A good test feeds a source module in at the
top of the transform and asserts the CSS text that comes out. It never reaches
for the predicate, the priority tables, or the selector assembler directly:
those are implementation, and #1251 proved the point — the bug was not in a
rule's definition but in one call site's spelling of it, which only an
end-to-end assertion on emitted CSS can catch.

**Seam: one, and it already exists.** The `stylex_test!` snapshot macro in
`stylex-transform`'s create-call test suite. It is the highest seam available —
source in, emitted CSS out, `@property` rules included — and it is where the
existing `inherits` pins live. No new seam is introduced.

**Modules tested.** Property registration in the create-call transform, via
that seam.

**Amended after review.** The predicates _are_ unit tested, reversing this
spec's original position. That position held while `is_pseudo_element` was the
only predicate: an assertion that it tests a `::` prefix restates its body, and
such a test would not have caught #1251, because the drifted copy was at a call
site rather than in a shared rule. Both arguments still stand for that
predicate alone. What changed is that there are now four predicates whose
extensions deliberately differ — `:` versus `::` versus `:`-or-`@`-or-`[` —
and the tests pin the *boundaries between them*: that `::before` is a pseudo
selector but not a pseudo class, that a legacy `:before` reads as a class, that
a subset predicate is not silently the same as the wider one. Those are
relationships no single body restates, and they follow `stylex-utils`'
per-module prior art rather than departing from it. The transform seam remains
the assertion for behavior; these pin vocabulary.

**Prior art.** The dynamic-styles create-call tests already pin both sides of
the rule: a bare pseudo-class case and a `:hover` nested inside `::before` for
`inherits: false`, and an `::after` case plus a `::before`-containing-pseudo-
classes case for `inherits: true`. The new case follows those exactly, differing
only in using the reporter's verbatim input.

**Refactor verification.** The existing workspace snapshot suite _is_ the
assertion for the de-duplication: the pass condition is zero snapshot movement.
If any baseline moves during the refactor commit, that is a defect in the
refactor and must be reported rather than re-baselined.

**Coverage of `inherits: true`.** Every upstream assertion of `inherits: true`
has exactly one counterpart here — no more, no fewer — verified during the
original fix against the authoritative upstream source. That correspondence is
what makes zero-churn a meaningful pass condition rather than a tautology.

## Out of Scope

- **Any change to compiler output.** If the audit produces a behavior fix, it
  is scoped by that audit's evidence, not by this spec.
- **A committed Babel-vs-Rust parity harness.** A CI job running both compilers
  over a corpus is a genuinely valuable project — it would have caught #1251
  before a user did — but it needs its own corpus, metadata-ordering
  normalization and upstream version pinning. It is not a rider on this branch.
  File it separately if wanted.
- **GitHub issue comms.** #1251 is not commented on or closed here. The fix is
  unreleased (workspace is at a release candidate; the reporter is on the prior
  published version), and issue communication stays with the maintainer.
- **A release.** Cutting the version that carries the fix is separate work.
- **Restructuring the pseudo-class priority tables**, the `when` selector
  ordering, or anything else the converted call sites happen to sit next to.
- **An enum or newtype model of pseudo kinds.** Considered and rejected above.
- **Expanding provenance comments** elsewhere in the codebase.

  **Amended after review.** The existing provenance comment in selector
  assembly was originally to be left as-is. It is instead rewritten to justify
  the normalization assumption on its own terms, dropping the claim about what
  another implementation does upstream — the same rule the predicate's own doc
  comment follows, and a maintainer instruction that provenance claims of this
  kind are not to be added. Removing one is not expanding any.

## Further Notes

- The branch `fix_media_queries_are_not_canonicalized` is stale — identical to
  `develop`, tracking a deleted remote — and is deleted as part of starting
  this work. Its six tracker issues are all resolved.
- The tracker's directory naming is inconsistent: most efforts are
  feature-named, one is branch-named. Feature-named ages better, since branches
  are deleted and features are not. Worth normalizing at some point; not part
  of this spec.
- The eight-fold duplication is the interesting artifact of this bug. The fix
  for #1251 was one character. The reason it took a user report to find is that
  the rule had no name, so no reviewer could see that one of its copies
  disagreed with the other seven. The de-duplication is the actual fix; the
  character was the symptom.
