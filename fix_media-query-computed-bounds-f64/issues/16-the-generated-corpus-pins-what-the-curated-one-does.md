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

**Status:** ready-for-agent

- [ ] The report distinguishes pinned rows from unexpected ones, and prints the
      unexpected count as the number a reader acts on
- [ ] A row belonging to no pinned family is reported as news even when its
      verdict is one a pinned family also produces
- [ ] Growing the alphabet does not require editing expectations, unless it
      reaches a genuinely new refusal
- [ ] The family names match ticket 15's exactly, and there is one place they
      are written down
- [ ] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
