# 08 — Move custom-property validation onto the value AST

**What to build:** A developer who mistypes a custom property reference — naming
it without the leading double hyphen — still gets the compile-time error they
get today, instead of a declaration that silently resolves to nothing at
runtime.

The rule itself does not change: a custom-property reference whose first
argument does not begin with a double hyphen is rejected. What changes is where
it reads from. Today it walks a CSS stylesheet, and that walk is the only
surviving reason the compiler parses CSS at all after ticket 07. The token list
answers the same question directly and more cheaply — a function token with the
reference's name whose first word child lacks the prefix.

Once this lands, nothing in the workspace consumes the CSS parse, and ticket 12
can remove the dependency.

Worth recording for whoever picks this up: this rule has **no upstream
equivalent**. The reference compiler accepts a malformed custom-property
reference without complaint. It is a deliberate local addition, knowingly
retained, because it changes only which programs are rejected and never the
bytes of an accepted program — so it cannot affect class-name parity — and it
catches a mistake that otherwise fails silently in a browser.

**Blocked by:** 07 — Swap normalization onto the ported pipeline.

**Status:** resolved

- [x] A malformed custom-property reference is rejected with the same error and
      message as before this change — with two departures recorded under
      *Departures from "the rule itself does not change"* below
- [x] A correctly prefixed reference is accepted, including inside nested
      functions and alongside a fallback argument
- [x] The check runs off the token list, not off a CSS stylesheet
- [x] The CSS parse has no remaining consumers anywhere in the workspace — no
      production consumer anywhere; the test-only residue is counted under
      *Consumers of the CSS parse* below and is ticket 09's to delete
- [x] Existing tests for this rule pass unchanged, or are re-expressed at the
      public entry point where they referenced the stylesheet type
- [x] The harness reports no divergence across the full corpus

## Answer

The rule is now `detect_unprefixed_custom_properties`, a pass over the token
list living beside the other passes in `css/normalizers/`. `css/validators/`
held nothing else and is gone.

**It joined `NORMALIZERS` rather than staying a call of its own in
`normalize_css_property_value`.** Two reasons. The array is the one place the
order of the passes is stated, and this rule's placement in that order is
behaviour: it has to run *after* the two unclosed detectors so `var(foo` —
unprefixed *and* unfinished — keeps reporting the unfinished function, which is
what this compiler has always said about it and what
`invalid_css_variable_unprefixed_and_unclosed_reports_the_unclosed_function`
pins. Second, the value is already scanned there, so the check costs a walk of
a list rather than a second parse on top of the CSS parse it replaces.

The array is not "the ported normalizers", so the new module's header says
`Local pass, not a port` where its siblings say `Ported normalizer N of 9`, and
the doc on `normalize_value` now says "passes" where it said "normalizers".

### Departures from "the rule itself does not change"

Two, both deliberate, both worth stating because the ticket asked for the rule
to arrive unchanged.

**The rejected set widened.** SWC parsed `var(1px)`, `var(#fff)` and `var(50%)`
into a dimension, a hex colour and a percentage — none of them an `Ident`, so
the old walk skipped them and the compiler accepted a reference to a property
that cannot exist. The token list calls all three a word, so they are now
rejected like `var(foo)`. This moves the code *toward* the ticket's own
statement of the rule — "a custom-property reference whose first argument does
not begin with a double hyphen is rejected" — and away from an exemption that
was an artefact of SWC's grammar rather than anybody's intent. Nothing in the
corpus is of that shape: only three entries carry an unprefixed reference at
all — `var(x)`, `var(foo)` and `var(foo` — and all three were rejected before.
So no accepted program changed and no class name moved.

**The diagnostic is spelled differently.** The old rule raised a bare
`assert!`, so the panic payload was exactly `Unprefixed custom properties`. It
now goes through `stylex_panic!` like every other pass in the list, which
renders `[StyleX] Unprefixed custom properties` and attaches a source location.
The message constant is untouched and every assertion on it is a substring
match, so nothing had to change; what the author sees is the same sentence
under the same banner as the unclosed-function and unclosed-string rejections
beside it, rather than a bare string that read as a different kind of failure.

Everything else is held exactly where it was: top-level references only — a
`var()` inside `calc()` is still not reached — and the function name still
matched case-sensitively, both now pinned by tests rather than left as
accidents of the SWC grammar.

**Coverage.** The 21 stylesheet-driven tests in `css/tests/validator_test.rs`
are re-expressed as 25 tests at the public entry point in
`css/tests/unprefixed_custom_properties_test.rs`, plus 12 predicate-level tests
in `tests/unprefixed_custom_properties_predicate_tests.rs` for node shapes the
scanner cannot produce (a function node with no argument list, an empty word, a
name spelled by a non-word kind). Those twelve go past what the ticket asked
for — it asked only that the old tests be re-expressed — and they are kept
because the predicate is private and its unreachable branches are otherwise
held up by reading alone. One of them, `an_empty_first_word_is_unprefixed`,
pins behaviour no input can reach; it records which way the branch falls rather
than claiming an author can observe it.

Two end-to-end rejections were added through the transform pass, one of them
inside a pseudo. The existing e2e test asserting "unclosed function" for
`var(foo` was renamed to say so.

`normalize_value_test`'s `var(a)` case moved out: that module asserts what the
fold spells, and this value now has no spelling.

**Consumers of the CSS parse.** None in production, anywhere. The `pub(crate)`
re-export of `swc_parse_css`, `stringify` and `get_value_from_ident` in
`stylex-transform`'s compatibility shim had no callers and came off with it.
`get_value_from_ident` itself is gone: the validator was its only caller, and
a function kept alive by nothing but its own two tests is dead code this
ticket created rather than inherited.

What remains is test-only, and counted rather than waved at — 56 call sites in
`css/normalizers/tests/base.rs`, which tests the superseded `CssFolder`, and 15
in `css/tests/common_test.rs`, which tests `swc_parse_css` and `stringify`
themselves. Both die in ticket 09 with the code they cover; deleting them here
would be ticket 09, not this one. `swc_parse_css` now says as much in its own
doc, because it was `pub` with no note and read like live API.

So the checklist item holds for production and not for the test tree, which is
the honest reading. Ticket 12's build with the CSS features switched off is
what actually proves no call site was missed.

**Harness, full corpus (790 declarations), `@stylexjs/babel-plugin` v0.19.0:**

| Verdict | Count |
| --- | --- |
| identical | 723 |
| **divergent** | **0** |
| structurally divergent | 15 |
| acceptance divergent | 26 |
| both reject | 26 |

The corpus grew from 781 as the harvester picked up the new test sources.
