# 48 — The engine returns to the registry

**What to build:** The engine is an ordinary crates.io dependency again, and
everything this repository had to arrange around a carried copy of it is gone.

**The reason for carrying it expired.** `vendor/boa` exists because published
`boa_engine` 0.21.1 required `icu_normalizer ~2.0.0` and `boa_parser` required
`icu_properties ~2.0.0`, and neither can coexist with the `~2.3.0` that
`icu_collator 2.3.1` needs — same major, so Cargo has to resolve one version and
cannot. **0.22.0 asks for `~2.3.0` itself.** That is the line the rest of this
workspace was already on, so the conflict the vendoring answered no longer
exists and step 1 of `vendor/boa/README.md`'s own bump procedure applies:
delete the directory, drop `[patch.crates-io]`, depend on the release.

**It is not a free bump.** 0.22 is a breaking release by Cargo's reckoning, and
two things move with it:

- The engine reworded a `RangeError`, which one case asserts verbatim as the
  language's own sentence.
- Its parser spends about forty per cent more stack per nesting level. The case
  that folds a dead operand nested two hundred deep was sitting on the slack
  above the claim rather than on the claim's own promise, so the bump turns it
  from a passing test into a **process abort**.

The second is the interesting one: it is the failure mode `growable_stack`
exists to prevent, reaching the suite rather than a build, which is where it
belongs — but only because a test happened to be nested deep enough to find it.

**Blocked by:** none.

**Status:** resolved

- [x] The workspace depends on `boa_engine` 0.22.0 from the registry, with no
      `[patch.crates-io]` section and no `exclude` for a vendored tree
- [x] `vendor/` is deleted, and with it every exemption made for it: the
      `lefthook` excludes for five job groups, the `oxlint` and `oxfmt` ignore
      patterns, the `NOTICE.md` section, and the `deny.toml` advisory ignore for
      a crate the new release no longer pulls in
- [x] The reworded `RangeError` is asserted as the engine now words it
- [x] `a_dead_operand_deeper_than_the_ceiling_is_never_entered` asks for the
      depth the claim promises rather than the slack above it, and runs on a
      thread too small to hold the claim so the claim is what carries it
- [x] `BYTES_PER_LEVEL` keeps its value — 64 KiB is still more than twice what a
      level costs — and its record carries the re-measured cost and says that the
      number belongs to an engine version rather than to this compiler
- [x] ADR 0008 and ADR 0004 record what happened rather than describing a tree
      that is gone; the locale refusal's first reason is restated, since it
      rested on the conflict that expired
- [x] `cargo check`, `clippy` and the full test suite pass on the workspace

**Resolution:** the tree is gone and the graph is smaller for it — 492 packages
resolve under `--all-features` against 425 without the engine, and `paste`, whose
unmaintained advisory `deny.toml` carried an ignore for, is no longer in the
graph at all.

The stack measurement is the part worth keeping. On a claim of exactly four
megabytes — the default ceiling of 32, twice over, at 64 KiB a level — 0.22's
print-and-parse carries 148 levels of nested array literal and not 152, against
0.21's roughly 200. Nothing promised changed, since twice the ceiling is 64
levels either way, but the case pinned to the slack aborted. It now asks for
`DEFAULT_MAX_EVALUATION_DEPTH * UNWALKED_NESTING` on a 256 KiB thread, which
forces the claim to be allocated instead of running on whatever the test runner
had spare — so the next engine that grows its frames fails one case that measures
the margin, rather than a build.

Ticket 43, which would have put the vendored tree under a CI integrity check, is
closed `wontfix` against this one.
