# 07 — Cover the remaining API surfaces

**What to build:** Confidence that the fold reaches every public API, not just
the three the bug report happened to name. An author should not have to
discover that coercions work in `create` but not in `keyframes`.

Test-only. Every case here should already pass once ticket 03 lands — the
ticket exists to prove that rather than assume it, and to pin the behaviour so a
later change to the evaluator cannot quietly narrow where folding applies.

Surfaces to cover:

- `keyframes`, with a coerced value in a step
- A computed style key
- A nested value object, coercing at both the default and a pseudo branch
- Nested calls, showing coercions compose

Use the existing per-API transform test directories — the highest seam
available, where a module goes in and CSS and metadata come out. No new seam.
No test should reach for a function configuration, a callback variant, or the
shape of an evaluated value; all of those are implementation and would have to
be rewritten alongside any refactor.

Expected values come from measured output of `@stylexjs/babel-plugin` 0.19.0
resolved from `node_modules` — see the upstream commit pinned in the spec — not
from reading its source. The comparison stays a development instrument; do not
commit a harness that shells out to it.

**Blocked by:** 03 — there is nothing to cover until the fold exists.

**Found while implementing 06:** "should already pass" does not hold for
`keyframes` and `Object`. An animation step whose value is an object fails this
compiler ahead of any coercion — a bare `{ default: 'red' }` step is enough,
with no call anywhere — so `Object(null)`, which folds to an empty object,
fails there too where upstream leaves the step empty and compiles. The three
other callees are unaffected, because none of them folds to an object. Covering
this surface for `Object` therefore needs a change to how a step value is read,
not just a fixture; measure first and split that out rather than widening this
ticket.

**Status:** done

- [x] All four surfaces above are pinned in the existing per-API transform test
      directories — with one caveat: the `keyframes` surface is pinned for
      `String` and `Number` only, because `Object` and `Array` in a step fail
      ahead of the fold. That half is 09, filed as the note above directed.
- [x] No new test seam is introduced
- [x] No test asserts on evaluator internals
- [x] The rebuilt compiler artifact passes the JavaScript suite as well as the
      Rust one — the JavaScript tests exercise the built artifact, so a rebuild
      between the two is what makes that run mean anything

## Outcome

Nine cases, all measured against `@stylexjs/babel-plugin` 0.19.0 and all
matching it on class name, rule text and priority.

`keyframes` gets its own `global_builtin_calls.rs` beside the existing
`css_keyframes.rs`, covering a coerced step value for `String` and `Number`, a
coerced step key, the animation name a coerced step produces reaching the
declaration that refers to it, and a locally declared `String` still shadowing
the global inside a step.

The `create` file gains the positions its per-global cases did not reach: a
computed key, every branch of a nested value including a media query branch,
and the four globals composing in one expression.

### Review outcome

One finding applied: the composition case pinned four unrelated nestings under
a name that stated a category, and three of the four were already covered by
the per-global nest cases. It now pins the one claim those leave open — all
four globals in a single expression — under a name that says so.

Two findings checked and declined, both with the reasoning measured rather
than assumed:

- The local `fn stylex_transform` wrapper reads as a pass-through, but it is a
  convention across 77 test files in this crate, 67 of which add file-specific
  configuration through it. It is a named per-file seam, so a file can start
  configuring without touching its call sites. Removing it from the ten
  pass-through files would break the uniformity that makes the seam findable.
- The `create` fold file at 944 lines was read as one file changing for two
  reasons. Both sections change for the same reason — a change to the fold —
  and the file sits under its sibling `static_styles.rs` at 1,011 lines, so
  splitting would make the directory less consistent, not more.

**Split out as 09:** `Object` and `Array` in a keyframes step. Confirmed to be
the pre-existing defect the ticket predicted rather than a gap in the fold —
`convert_expr_to_str` refuses a non-string step value, and the panic reproduces
from a bare `{ default: 'red' }` step with no call anywhere. Measured what
upstream does for all three shapes and recorded it in the new ticket.
