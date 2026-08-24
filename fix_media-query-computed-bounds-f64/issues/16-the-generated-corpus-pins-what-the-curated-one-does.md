# 16 — The generated corpus pins what the curated one does

**What to build:** `pnpm fuzz:shorthand` separates rows that are pinned from
rows that are news, so its output can be read the way the curated report is.

The harness currently prints roughly 10,600 divergent rows out of 129,744 and
none of them is a defect: every one is a member of a refusal family ticket 15
names. A reader cannot tell that from the report, which means the number has
to be explained by whoever ran it — the exact failure mode the `expected`
field exists to prevent, reproduced in the second harness.

The families are the same ones. Reuse the names ticket 15 settles rather than
describing them again, so the two harnesses cannot come to disagree about
which refusal is deliberate.

Sizing note: pinning here is by *family*, not by row. There are hundreds of
distinct generated values per family, and an expectation per row would be a
fixture nobody can read and a corpus that churns whenever the alphabet grows.

**Blocked by:** 15 — for the family names and their reasons, not for the
mechanism.

**Status:** done

- [x] The report distinguishes pinned rows from unexpected ones, and prints the
      unexpected count as the number a reader acts on
- [x] A row belonging to no pinned family is reported as news even when its
      verdict is one a pinned family also produces
- [x] Growing the alphabet does not require editing expectations, unless it
      reaches a genuinely new refusal
- [x] The family names match ticket 15's exactly, and there is one place they
      are written down
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Outcome

`pnpm fuzz:shorthand` splits its divergent rows into pinned and unexpected,
reading ticket 15's family list rather than describing the refusals again. Over
the grown alphabet it reports 153,624 subjects, 18,909 divergent, **0
unexpected**. `--show` prints unexpected rows only; `--json` carries both, the
pinned ones grouped by family name.

A row is asked about its family, never about its verdict alone, so a row whose
verdict a pinned family also produces is still news when the refusal underneath
it differs — covered by six cases in `__tests__/refusal-families.test.ts`. Two
families were too wide on first writing and were narrowed under review: `first
refusal to fire` now requires the reference compiler's complaint to be one of the
two this compiler's guard is known to preempt, and `style key off
Object.prototype` requires the shape an inherited method produces rather than
the key name alone.

Narrowing the first of those turned 238 generated rows into news, and reading
them is why a family carries a *set* of verdicts rather than one. Their shape is
a value this compiler refuses for a declaration-terminating token that also
crashes the reference compiler — the `reference TypeError` reason exactly, read
under a both-reject verdict instead of an acceptance one, because this side
happened to refuse too. The reason survives that; the single verdict did not.
Precedence settles the overlap with `first refusal to fire`: the crash sits
above it, because a crash is not a guard that spoke first.

Nothing is pinned by count, so growing the alphabet costs no expectation edit:
the three classes ticket 17 added moved every family's row count and required no
change here.
