# 09 — Record the decision and seed the glossary

**What to build:** A future reader who finds `catalog:internal` in a manifest
can discover why it is there, and the vocabulary this migration introduced is
written down once instead of re-derived by each person who meets it.

**The ADR.** One decision here clears the bar: *internal `@stylexswc/*`
dependencies live in a catalog, and the release bumper rewrites that catalog.*
It is hard to reverse (unwinding it means touching every manifest again), it is
surprising without context (the obvious alternative — leave internal deps as
literal versions the bumper already rewrites — is simpler and was rejected), and
it came from a real trade-off with alternatives that were considered and turned
down.

Record what the alternatives were and why they lost:

- Leave internal deps uncatalogued. Simpler, and the release bumper already
  handled them. Rejected because it leaves two mechanisms owning versions in one
  repo, and the boundary between them is invisible at the call site.
- Use the `workspace:` protocol instead. Rejected because the release pipeline
  bumps internal deps to literal published versions, and `workspace:` changes
  what gets published.

  **Check this reason before writing it into the ADR — as stated it looks
  wrong.** pnpm rewrites `workspace:` specifiers into real versions on publish,
  by the same mechanism it rewrites `catalog:` ones; ticket 05 verified that
  mechanism by packing `@stylexswc/rs-compiler`. So `workspace:` would probably
  *not* change what gets published, and this sentence would be disprovable by
  one `pnpm pack`. The decision may still be right — `saveWorkspaceProtocol:
  rolling` and `linkWorkspacePackages: deep` are both set, and the four-location
  bumper is built around literal versions — but it needs a reason that survives
  contact with a reader. A review of ticket 05 re-proposed `workspace:*`
  unprompted, which is the evidence that the current reason does not hold.

Record the consequence too: the release bumper is now the single point of
failure for four version locations that must agree, which is why ticket 02
required it to be tested and to treat a no-op as an error.

Note also the decision *not* to use `catalogMode: strict`, and why enforcement
lives in a check with a helpful message instead — that one is cheap to reverse
and does not need its own ADR, but it belongs in the consequences section.

**The glossary.** The repository documentation promises a root context map and
per-crate context files; none exist. Do not build that here — writing a context
file for every crate and package as a side effect of a dependency migration is
how both jobs end up half-done.

Seed a root `CONTEXT.md` with only the terms this migration actually settled:
*catalog*, *peers catalog*, *internal catalog*, *manifest*, *bumper*. Keep it a
glossary — definitions of terms, no implementation detail, no plan, no
decisions. The decisions belong in the ADR.

**Blocked by:** 07

**Status:** resolved

- [x] An ADR records the internal-deps-in-a-catalog decision, the alternatives
      rejected, and the release-bumper consequence
- [x] The `catalogMode: prefer` choice is captured in the ADR's consequences
- [x] A root `CONTEXT.md` defines the five settled terms and nothing else
- [x] No context map or per-crate context files are created
- [x] Markdown lint and format checks pass

## Comments

**The suspect `workspace:` reason was disprovable, and was disproved.** Packing
`@stylexswc/rollup-plugin` with `@stylexswc/rs-compiler` set to `workspace:^`
publishes `^0.18.3`; the same package with `catalog:internal` publishes
`0.18.3`. So `workspace:` does not leave a broken specifier on npm — pnpm
rewrites it by the same mechanism ticket 05 verified for `catalog:`. The ADR
says so explicitly rather than quietly dropping the claim, because a reader who
has heard the old reason needs to know it was retired on purpose.

**The reason that replaced it is the range *shape*, and it only holds because
of a setting.** `saveWorkspaceProtocol: rolling` makes `workspace:^` what `pnpm
add` writes, and `^0.18.3` lets a consumer of `@stylexswc/rollup-plugin@0.18.3`
resolve a different `@stylexswc/rs-compiler` from the one released beside it —
which matters because one of the packages is a native NAPI binding the others
are built against. The ADR concedes the rest: `workspace:*` *would* publish an
exact pin, so the honest reason to reject it is the first alternative's reason
— a second mechanism owning versions — not a publishing difference. That
concession is why the review's re-proposal should not recur.

**`docs/adr/` did not exist.** This is `0001`. No `CONTEXT-MAP.md` and no
per-crate `CONTEXT.md` were created; `docs/agents/domain.md` describes the root
map as something `/domain-modeling` grows lazily, and a dependency migration is
not the event that should grow it.

**Review caught three things, all fixed.** The ADR claimed
`catalog-integrity.mjs` checks the internal pin — it does not; it asserts that
manifests carry no literal ranges and that `catalog:` references resolve, and
has no `internal`-specific logic at all. The glossary asserted the seven
semantic catalogs are cut by what a dependency is *for*, which `internal`
contradicts, so the glossary no longer restates the cut principle — that
reasoning lives at the setting in `pnpm-workspace.yaml`. And three definitions
had grown into behaviour descriptions; they are back to one sentence each.

**The missing `CONTEXT-MAP.md` was answered in the glossary rather than by
building the map.** A bare root `CONTEXT.md` is the *single*-context shape, and
this repo is multi-context, so `docs/agents/domain.md`'s inference rule ("If
only a root `CONTEXT.md` exists, single context") would read it wrong. Building
the map is out of scope — the spec lists it under Out of Scope explicitly — so
the fix went into the one-line description `CONTEXT-FORMAT.md` already requires:
the file states that it is the root glossary, that it holds only cross-context
terms, and that a term particular to one context belongs in that context's own
file. That removes the mis-inference and constrains what may be added later,
without pre-empting the modelling work.

**The `catalogMode` consequence stays**, though it was flagged as a second
decision failing ADR-FORMAT's hard-to-reverse test. The ticket requires it in
the consequences section, and it now says of itself why it is a consequence
rather than an ADR.

**"Bumper is a coined name nobody uses" was rejected on evidence.** The term is
already this repository's vocabulary: `scripts/git/bump-version.test.mjs`,
`scripts/git/lib/manifests.mjs`, `.github/dependabot.yml` and
`.github/workflows/release.yml` all use it. Recording it is what a glossary is
for; the `_Avoid_` entries stand because `release script` collides with
`scripts/git/start-release.sh`, which is a different thing.

**DRY pass over the three files that now discuss catalogs.** An eight-word
shared-run scan across `pnpm-workspace.yaml`, the ADR and `CONTEXT.md` found the
`catalogMode` rationale duplicated between the yaml comment and the ADR — in a
paragraph that had just promised not to repeat it. The ADR now points at the
setting instead. The glossary also stated the catalog *count*, a fact
`pnpm-workspace.yaml` owns and that would rot the day a ninth catalog lands;
it now states the no-default rule, which is the part that is definitional. The
scan reports zero shared runs between any pair of the three files.

**The markdown check only reads tracked files** (`git ls-files`), so both new
files were also checked by explicit path — `pnpm run lint:markdown` alone would
have passed on 85 files without looking at either of them.
