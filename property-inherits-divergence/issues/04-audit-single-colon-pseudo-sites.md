# 04 — Rule on the two deliberate single-colon sites

**What to build:** A written verdict on the two places that deliberately test a
**single** colon rather than `::`, so that the next reader does not have to
guess whether they are correct or are the next #1251 waiting to be found — and a
behavior fix at either site only where upstream provably contradicts the current
code.

The two sites are the `when` selector utility's validation of a pseudo selector
(which accepts a single colon *or* an attribute-selector bracket) and the check
in selector assembly that means "any pseudo". Both look, at a glance, exactly
like the drift that caused #1251. That resemblance is explicitly **not**
grounds for changing them: they may be entirely correct, and "looks like the bug
we just fixed" is not evidence.

The deliverable is the verdict, whichever way it falls. A finding of "both
correct" is a successful outcome of this ticket, not a wasted one — it closes a
question that would otherwise be re-investigated by whoever next reads those
lines.

**Blocked by:** 03 — the verdict comments land on lines that ticket rewrites,
and any behavior change must come *after* the churn-free refactor so that a
reviewer can read every moved baseline as deliberate rather than as refactor
fallout.

**Status:** resolved

**Since this was written:** `e9c94483d` renamed both sites' literal to
`is_pseudo_selector` — `crates/stylex-css/src/utils/when.rs:49` and
`crates/stylex-transform/src/shared/structures/pre_rule.rs:99`. That is a
naming change only; **no verdict has been reached**. The name makes the sites
read as adjudicated when they are not, which raises rather than lowers the
value of the one-line verdict comments below.

- [x] Each of the two single-colon sites is checked against the **authoritative**
      upstream source — the create/CSS logic in the `@stylexjs`-scoped workspace
      package's `src`, never the unscoped build-artifact-only `lib`, which
      carries an older `@property` emission form and will manufacture a
      divergence that does not exist
- [x] The verdict for each site is recorded in this file's `## Answer` section:
      correct as-is, or provably wrong with the upstream construct that proves
      it
- [x] Each site gains a one-line comment recording its verdict, so a reader
      staring at the line learns why a single colon belongs there
- [x] Behavior changes **only** where upstream provably contradicts the code. If
      neither site is provably wrong, no code changes and the Answer says why
- [x] Any such fix is its own commit, with its snapshot churn included and the
      upstream construct cited in the commit body — never folded into another
      commit
- [x] `cargo test --workspace` green; snapshot churn, if any, confined to
      baselines the cited upstream construct accounts for
- [x] No GitHub issue is filed or commented on — issue communication stays with
      the maintainer

## Answer

**Both sites are correct as-is. No behavior change.**

Checked against the authoritative upstream source at `5f51b2444` (v0.19.0) —
`packages/@stylexjs/babel-plugin/src`, never the unscoped `packages/babel-plugin`
build artifact.

### `crates/stylex-css/src/utils/when.rs:49` — the `when` utility's validation

Correct as-is. Upstream's `src/shared/when/when.js:56` spells the same gate:

```js
if (!(pseudo.startsWith(':') || pseudo.startsWith('['))) {
  throw new Error('Pseudo selector must start with ":" or "["');
}
if (pseudo.startsWith('::')) {
  throw new Error(
    'Pseudo selector cannot start with "::" (pseudo-elements are not supported)',
  );
}
```

The single colon is not a classification at all — it asks whether the argument
opens a selector. Pseudo elements *must* pass it, because the `::` check on the
next lines is what rejects them, with a message naming the actual problem. A
`::` test here would reject `::before` with "must start with ':' or '['", which
is both wrong and unhelpful.

### `crates/stylex-transform/src/shared/structures/pre_rule.rs:99` — "any pseudo"

Correct as-is. Upstream's `src/shared/preprocess-rules/PreRule.js:61-66`:

```js
get pseudos(): $ReadOnlyArray<string> {
  const unsortedPseudos = this.keyPath.filter(
    (key) => key.startsWith(':') || key.startsWith('['),
  );
  return sortPseudos(unsortedPseudos);
}
```

A rule's pseudo list is consumed by selector assembly, which needs both kinds:
narrowing to `::` would drop every `:hover` from every compiled selector. The
sibling `at_rules` and `const_rules` filters partition the same key path by
their own prefixes, confirming this one is a prefix partition rather than a
pseudo-element test.

### Why the resemblance to #1251 is superficial

#1251 was a site that needed the *element* half of the distinction and got the
whole set. Neither of these sites is making that distinction: one is a
well-formedness gate whose `::` case is handled separately and explicitly, the
other is a prefix partition over a key path. The shared literal is a
coincidence of syntax, not of intent — which is exactly what `is_pseudo_selector`
now says at both lines.

### Follow-up surfaced by review, not adjudicated here

`crates/stylex-css/src/css/common.rs:770` (`let is_pseudo = is_pseudo_selector(css_property);`,
feeding `build_css_rule`) is a third single-colon caller. This ticket scoped
the audit to two sites, so it is deliberately out of scope rather than
overlooked — but it is now the only unadjudicated one. Worth a follow-up
ticket if anyone wants the set closed.
