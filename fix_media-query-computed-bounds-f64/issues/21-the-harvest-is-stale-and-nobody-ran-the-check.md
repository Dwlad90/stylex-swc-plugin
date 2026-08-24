# 21 — The harvest is stale, and nobody ran the check

**What to build:** `pnpm parity:harvest:check` passes, so the parity corpus
covers what the Rust suites cover rather than what they covered some commits
ago.

It is failing today, and it was already failing before ticket 15's work
touched anything — verified by regenerating the harvest against the tree with
and without those changes: both produce the same 776 declarations, 76 rows more
than the checked-in file, and none of the 76 comes from a test added by tickets
15–19. The values are the float-precision cases tickets 11–13 added:
`1.2345678901234567px 7%`, `1.7976931348623157e308px`, `-0px 1e21px`,
`+1px +2% 0.5px 000.5px`.

So the corpus is not measuring parity on the values the effort's own tests were
written to pin, which is the one place a stale corpus costs the most.

Not fixed alongside tickets 15–19 deliberately: regenerating it also rewrites
`crates/postcss-value-parser/src/tests/cases.rs`, whose row order *is* the
corpus order, so the change is two generated fixtures and belongs in a commit
a reviewer can read as that rather than as a footnote to a reporting change.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] `pnpm parity:harvest:check` passes
- [x] `cases.rs` is regenerated from the new corpus, per the chain the harvest
      script documents, and its diff is only the rows the corpus gained
- [x] The 76 new subjects are read: any divergence among them is a defect or a
      pinned family, not a number left in the report
- [x] Whether the check belongs in a hook or in CI is decided — it caught
      nothing here because nothing ran it
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
