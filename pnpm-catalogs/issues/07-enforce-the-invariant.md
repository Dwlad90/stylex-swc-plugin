# 07 — Contract: make the invariant enforceable

**What to build:** A contributor who writes a literal version range into a
manifest is told so before the commit lands, and is told which catalog it
belongs in. Stale catalog entries are removed automatically.

This is the contract half of the migration. Every manifest now references a
catalog, so "versions are declared once, by name" becomes a property that can be
checked rather than a convention that erodes.

Two changes:

**The check.** Build one catalog-integrity command-line script with two modes.
This ticket delivers the manifest mode: reject any literal external range
outside `pnpm-workspace.yaml`. Ticket 08 adds the lockfile mode to the same
script. They are two assertions over the same data, so they share one seam and
one suite rather than becoming two scripts with two sets of wiring.

The existing version-mismatch check becomes a *caller* of it, running the
syncpack lint it runs today plus the new manifest mode. Both existing call
sites — the lefthook pre-commit job and the PR validation matrix — invoke that
script and so pick the new behaviour up with no change to their wiring.

The failure message is the whole point. `catalogMode: prefer` was chosen over
`strict` precisely so that this check, not a `pnpm add` failure, is what teaches
the convention — so it must name the offending file, the dependency, and a
suggested catalog. A check that just says "literal range found" pushes the
contributor into guessing at the taxonomy.

It must respect the same exclusion set ticket 01 established: fixture
manifests, generated platform manifests, virtual test apps and build output are
all legitimately literal. Test that the exclusions hold — a check that flags
fixtures will be disabled by the first person it inconveniences.

**The cleanup.** Flip `cleanupUnusedCatalogs` to `true`. It was held at `false`
through ticket 03 because the catalogs existed with zero referents and cleanup
would have deleted all of them. Now that every manifest references a catalog,
an unused entry is invisible rot with no reason to survive. Verify that turning
it on removes nothing — anything it deletes is an entry the migration failed to
wire up.

**Blocked by:** 04, 05, 06

**Status:** resolved

- [x] A catalog-integrity script rejects literal external ranges outside
      `pnpm-workspace.yaml` and passes on the migrated tree
- [x] Its failure output names the file, the dependency and a suggested catalog
- [x] Fixture, generated, virtual and build-output manifests are exempt, with a
      test covering the exemption
- [x] The version-mismatch check calls it; both existing call sites pick the new
      behaviour up with no change to their wiring
- [x] Tests drive the real script end-to-end using the shared script-test
      harness, asserting exit status and message rather than internals
- [x] `cleanupUnusedCatalogs` is `true`, and enabling it removes no entry
- [x] `pnpm test:scripts` passes; a deliberately introduced literal range fails
      both pre-commit and CI

## Comments

**Carried over from 06 — `cleanupUnusedCatalogs` will delete a live entry.**
`packages/unplugin/playground` is in syncpack's scope but is *not* a pnpm
workspace member: the `packages:` globs are `packages/*`, which does not reach
depth 3, and the lockfile has no importer for it. 06 migrated its two ranges as
the ticket asked, but pnpm never resolves them — the playground contributes
nothing to the lockfile's `catalogs:` snapshot. `vite-plugin-inspect` is
declared *only* there. Two facts are verified: `grep` finds exactly one referent
for it, and it is absent from the lockfile's `catalogs:` snapshot.

**What this means for the cleanup flip is untested, not established.** The
obvious worry is that `cleanupUnusedCatalogs: true` sees no referent and deletes
`bundlers.vite-plugin-inspect`, leaving the playground pointing at nothing. That
worry was *not* confirmed: flipping the flag and re-installing removed no entry,
including that one — but every attempt short-circuited on "Already up to date"
(plain install, `--force`, and `--lockfile-only` with the lockfile deleted), so
the cleanup pass may simply never have run. Treat this as the first thing to
check when the flip lands, not as a cleared risk.

Either way the playground is the one manifest whose catalog references nothing
resolves, so it is where this ticket's "anything it deletes is an entry the
migration failed to wire up" test needs a third answer beyond delete-or-keep.
The candidate fix is adding the playground to the workspace `packages:` globs,
which makes the reference real and the entry used — but that adds an importer
and installs into a directory that has never had a `node_modules`, so it is a
resolution change and belongs here rather than in 06.

**Carried over from 01.** `.lefthook/pre-commit/manifests.sh` passes one
`--source` per staged path to `syncpack format`, and a command-line `--source`
overrides `.syncpackrc`'s negations rather than intersecting with them — so a
staged fixture or generated manifest is still formatted. Harmless today
(formatting only sorts keys), but the exemption list this ticket builds should
be the one source both the check and the hook read, rather than a second copy in
POSIX sh. See the trap recorded in `guidelines/git/HOOKS.md`.

## Answer

**The cleanup worry carried over from 06 was real, and confirming it took the
flip.** With `cleanupUnusedCatalogs: true` and the playground still outside the
workspace globs, one `pnpm install --lockfile-only` deleted exactly one entry --
`bundlers.vite-plugin-inspect` -- leaving `packages/unplugin/playground` pointing
at nothing. Every earlier attempt short-circuited on "Already up to date"; what
made the cleanup pass actually run was a lockfile-only install after the flag
changed. Nothing else was removed: the six `peers` entries pnpm does not record
in the lockfile's `catalogs:` snapshot survived it, so "not in the snapshot" is
not the same as "unused".

**Fixed by the candidate the ticket named:** `packages/unplugin/playground` is
now listed explicitly in `packages:`. It is depth 3, so `packages/*` never
reached it. With it a member, `vite-plugin-inspect` appears in the lockfile's
catalog snapshot, the reinstall removes nothing, and `pnpm -C playground run dev`
(the `play` script in `packages/unplugin`) installs something for the first time
-- previously its two devDependencies were declared and never resolved.

**The check** is `scripts/git/catalog-integrity.mjs`, one script with a mode
argument so 08 adds `lockfile` beside `manifests`. Scope comes from
`.syncpackrc` through the existing `lib/manifests.mjs`, so there is no second
exemption list. It rejects two things, the same invariant read both ways: a
literal external range, and a `catalog:` reference that resolves to nothing.
Catalogs are read by a new `lib/catalogs.mjs`, hand-parsed for the reason
`bump-version.mjs` hand-parses the same file -- the comments in
`pnpm-workspace.yaml` are load-bearing and there is no YAML dependency here.

The suggestion is the part worth reviewing: a literal in `peerDependencies` is
pointed at `peers`, not at the semantic catalog, so the failure message cannot
be the thing that silently narrows a consumer range.

`version-mismatch-check.sh` now runs both halves on every invocation instead of
short-circuiting on syncpack -- a manifest that is unformatted is usually the
one that reintroduced a literal range, and fixing them one per commit cycle is
what trains people into `--no-verify`. Its own suite,
`version-mismatch-check.test.mjs`, stubs only syncpack and runs the catalog half
for real, so what it asserts is exactly what the hook and the CI matrix see.

### Changed in review

- The exemption test now copies the real `.syncpackrc` `source` list verbatim,
  so all four families of ticket 01 are exercised rather than a sample of them.
  Its companion asserts each of those manifests *would* fail without its
  exclusion -- without that, a typo in a fixture path leaves the test passing
  for the wrong reason.
- `DEPENDENCY_FIELDS` and the literal-range predicate moved into
  `lib/manifests.mjs`. They were copied from `bump-version.mjs`, and the two
  scripts ask the same question for opposite reasons; two copies can disagree
  about a specifier form neither author was thinking of.
- The mode argument is now required and singular. It was variadic with an
  "all modes when none named" default, which is generality for a second mode
  that does not exist yet -- 08 adds one entry to `MODES` and inherits no
  default semantics to reason about.
- `writeJson`/`writeText` moved into `lib/test-harness.mjs`; three suites had
  identical copies.
- Added coverage for the two ways `lib/catalogs.mjs` refuses a file it cannot
  read. That branch is what stands between a malformed workspace file and a
  check that silently passes.

### Left open, deliberately

- **The `--source` trap from 01 is still there.** `manifests.sh` still overrides
  `.syncpackrc`'s negations for staged paths, so a staged fixture is still
  *formatted*. It is not a second copy of the exemption list, which is what the
  comment asked about, and the version-mismatch job is unaffected -- it passes no
  paths, so it reads the scope through `lib/manifests.mjs`. Recorded in
  `guidelines/git/HOOKS.md` rather than fixed.
- **The playground manifest has no `name`.** Now that it is a workspace member
  that is visible: `pnpm --filter` cannot address it and `turbo ls` silently
  omits it. Left alone because naming it pulls it into turbo's task graph, which
  is a wider change than making its catalog references resolve.
- **`pnpm deps:check` still reports unused dependencies.** Pre-existing, in
  manifests this ticket did not touch.
