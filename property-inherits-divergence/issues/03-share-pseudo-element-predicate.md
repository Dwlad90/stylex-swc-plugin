# 03 — Give the pseudo-element prefix test one name

**What to build:** The rule that decides whether a selector segment is a pseudo
_element_ — a `::` prefix, as against the single colon of a pseudo _class_ —
is stated once, under one name, and every site that applies it says the name
instead of re-deriving the rule from a string literal.

This is the actual fix for #1251. The behavioral change was one character; the
reason it took a user report to find is that the rule had no name, so eight
independent copies of it existed and no reviewer could see that one of them
disagreed with the other seven. After this ticket, a drift from `"::"` to `':'`
either changes the rule for every caller — visible in one line, in one review
comment — or does not compile.

Nothing about the compiler's output changes. The whole workspace snapshot suite
is the assertion: it must not move.

**Blocked by:** 02 — the reported input's pin belongs among the baselines being
held still, so that the property-registration call site is not converted
without its own reported case watching it.

**Status:** resolved

- [x] `is_pseudo_element(key: &str) -> bool` exists, `#[inline]`, returning
      whether the key carries a `::` prefix — so naming the rule costs nothing
      in generated code
- [x] It lives in a `pseudo` module under `stylex-css`'s utils, re-used by
      `stylex-transform`. Placement rationale: `stylex-utils` declares itself a
      leaf where no StyleX domain concept is defined, and pseudo element is a
      domain concept — a glossary term in both `stylex-css` and
      `stylex-transform`; `stylex-constants` is excluded by its own charter,
      which covers lookup tables and compile-time constants rather than
      predicates
- [x] Its doc comment states the rule intrinsically — `::` marks a pseudo
      element, a single colon marks a pseudo class — with no provenance claim
      about any other implementation
- [x] All eight sites currently testing a `::` prefix call it: the two
      pseudo-class-before-pseudo-element filters in selector assembly, the
      compound-chain priority sum, the standalone pseudo-element priority
      lookup, the `when` selector utility, the pre-rule nesting logic (both
      tests), and property registration in the create-call transform — the site
      that carried the bug
- [x] Every call site keeps its exact existing shape; only the literal moves
      behind the name. No control flow, ordering or naming is restructured
- [x] `stylex-css`'s glossary gains a pointer to the predicate, so the rule is
      discoverable from the crate that owns the code
- [x] **Zero snapshot movement across the whole workspace.** If any baseline
      moves, that is a defect in this ticket — stop and report it; do not
      re-baseline
- [x] `cargo test --workspace`, `cargo clippy`, `cargo fmt` and Taplo all clean.
      The Node suite is not run — nothing under Node changes
- [x] Committed as a single `refactor(…)` commit: the predicate and all eight
      call sites together, since a predicate with no callers is not a
      meaningful boundary and would trip dead-code lints

## Notes

An enum classifier over pseudo class / pseudo element was considered and
rejected: several call sites want a bare `bool` inside a filter, and the two
single-colon sites (see `04`) are not a binary classification at all — one means
"pseudo class or attribute selector", the other means "any pseudo" — so an
exhaustive type would not model them.

## Answer

`is_pseudo_element` lives in `stylex-css`'s new `utils::pseudo` module,
`#[inline]`, doc'd from the rule itself. All eight `::` sites call it: the two
filters in `push_selector`, `get_compound_pseudo_priority`,
`get_pseudo_element_priority`, `validate_pseudo_selector`, both tests in
`sort_pseudos`, and the property-registration site in the create transform. Call
sites kept their shape; `stylex-css`'s glossary gained a **Pseudo element**
entry. `cargo test --workspace`, clippy, fmt and Taplo clean, with zero snapshot
movement. Commit `1549e3772`.

Follow-on, outside the ticket as written: the colon-prefix rule got the same
treatment in commit `e9c94483d` — `is_pseudo_selector` (any pseudo) and
`is_pseudo_class` (a colon that is not `::`), replacing nine further literals in
nesting, validation, dynamic-style key paths and pseudo-class priority. Both
names are checked against upstream `5f51b2444`, where every single-colon test
is a bare `startsWith(':')` and only `getPseudoClassPriority` means pseudo class
— and only because `getPseudoElementPriority` runs first. Behavior unchanged;
snapshots still. This narrows `04` to its verdict-comment work.
