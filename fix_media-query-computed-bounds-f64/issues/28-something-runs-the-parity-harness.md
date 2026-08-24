# 28 — Something runs the parity harness

**What to build:** the corpus's answer arrives without anyone having to think of
asking for it.

The harness is the oracle every expectation in this effort was derived from, and
nothing runs it. The staleness check now runs ahead of one package's test suite,
which is a scan of Rust sources; the comparison itself — the part that actually
consults the reference compiler — runs only when a person remembers. That is how
the corpus came to be dozens of declarations behind without anyone noticing, and
the fix for that was to stop relying on remembering.

Its own README says why it is not already wired in: it needs a built `dist/` and
a Node toolchain, which is a real cost to put in front of every commit. That
argues against a pre-commit hook, not against running it at all. The two things
it exits non-zero for — an expectation that stopped holding, and a family no row
reached — are both regressions that get quieter the longer nobody looks.

Decide where it belongs given that cost. What it must not stay is nowhere.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The value comparison runs somewhere without a person choosing to run it,
      and where it runs is written down next to why that place and not a hook
- [x] A failure is attributable: the run says which compiler versions it
      resolved, since the reference plugin is held by the lockfile and a report
      that does not name it cannot be compared with an older one
- [x] The generated harness's cost is measured and its inclusion decided on that
      number — it crosses an alphabet and is the more expensive of the two
- [x] A run that fails says what to do about it, since the person who reads it
      first will not be the person who wrote the corpus
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass

## Closing note

Delivered. `parity`, `parity:positions` and `fuzz:pseudo-order` run in the
`checks` matrix's `parity` leg on every pull request, after a build, since all
three read `dist/` rather than the Rust sources. A failure is attributable: the
run prints the resolved version of each compiler it loaded, which matters because
the catalog pins no exact range.

The generated harness's cost was measured rather than guessed -- ~97s against the
curated harness's ~2.5s, about forty times -- and on that basis it runs on the
nightly schedule as `parity-sweep` instead of per commit. It is listed in the
validation gate's `needs` so a red sweep fails the gate rather than sitting green
beside it.

A failing run says what to do about it: the sweep's non-zero exit prints the
`--show` and `--json` invocations for the rows, and names the two outcomes --
fix the split, or add a refusal family stating the reason.
